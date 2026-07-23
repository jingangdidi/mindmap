use std::collections::HashMap;
use std::fs::{write, read_to_string, create_dir_all, remove_file};
use std::path::PathBuf;
use std::sync::RwLock;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::{event, Level};

pub mod parse_paras;
pub mod router;
pub mod error;
pub mod ctrlc;
pub mod node;

mod handlers;

use crate::{
    node::MindmapNode,
    parse_paras::PARAS,
};

/// mindmap default page
pub const DEFAULT_PAGE: &str = include_str!("../../assets/mindmap.html");
/// style for export png
pub const INDEX: &str = include_str!("../../assets/index.css");
pub const KATEX: &str = include_str!("../../assets/katex.css");

/// highlight color
const HIGHLIGHT_COLORS: &[&str; 9] = &["Yellow", "Pink", "SkyBlue", "LightGreen", "Orange", "Violet", "chartreuse", "Bisque", "Khaki"];

/// common parent nodes color will be set to gray
const HIGHLIGHT_GRAY: &str = "LightGray";

/// global data, store all mindmap
pub static DATA: Lazy<RwLock<MindMap>> = Lazy::new(|| RwLock::new(MindMap::new()));

/// highlight node
#[derive(Deserialize, Serialize)]
struct HighlightNode {
    all_nodes:      HashMap<String, usize>, // key: all highlighted node id, value: color index
    selected_nodes: HashMap<String, usize>, // key: selected node id, value: color index
}

impl HighlightNode {
    /// create empty struct
    fn new() -> Self {
        Self {
            all_nodes:      HashMap::new(),
            selected_nodes: HashMap::new(),
        }
    }
}

/// store local mindmap
pub struct MindMap {
    loaded:    HashMap<String, (String, Option<String>, bool)>, // loaded mindmap, key: uuid, value: (mindmap data, label string, updated)
    local:     HashMap<String, (PathBuf, Option<String>)>,      // all mindmap in outpath, key : uuid, value: (mindmap data file path, label string), e.g. (f66bedbd-9972-4ec3-9a30-9510d4fffe1c.html, f66bedbd-9972-4ec3-9a30-9510d4fffe1c.json, "my first mindmap")
    highlight: HashMap<String, HighlightNode>,                  // highlighted nodes, key: uuid, value: highlighted node
}

impl MindMap {
    /// get all mindmap file path from outpath
    fn new() -> Self {
        let mut local = HashMap::new();
        let mut highlight = HashMap::new();
        if let Ok(uuid_dirs) = PARAS.outpath.read_dir() {
            for i in uuid_dirs {
                if let Ok(entry) = i {
                    let uuid_path = entry.path();
                    if uuid_path.is_dir() {
                        if let Some(uuid) = uuid_path.file_name().unwrap().to_str() {
                            // load mindmap data
                            let json_path = uuid_path.join(&format!("{}.json", uuid));
                            let label_path = uuid_path.join(&format!("{}.txt", uuid));
                            if json_path.exists() && json_path.is_file() {
                                local.insert(
                                    uuid.to_string(),
                                    (
                                        json_path,
                                        if label_path.exists() && label_path.is_file() {
                                            match read_to_string(&label_path) {
                                                Ok(label) => Some(label),
                                                Err(e) => {
                                                    event!(Level::ERROR, "read {} to string: {}", label_path.display(), e);
                                                    None
                                                },
                                            }
                                        } else {
                                            None
                                        },
                                    ),
                                );
                            }
                            // load highlighted node data
                            let highlight_path = uuid_path.join(&format!("{}.highlight", uuid));
                            if highlight_path.exists() && highlight_path.is_file() {
                                match read_to_string(&highlight_path) {
                                    Ok(highlight_str) => {
                                        match serde_json::from_str::<HighlightNode>(&highlight_str) {
                                            Ok(data) => {
                                                highlight.insert(uuid.to_string(), data);
                                            },
                                            Err(e) => event!(Level::ERROR, "convert {} content to struct: {}", highlight_path.display(), e),
                                        }
                                    },
                                    Err(e) => event!(Level::ERROR, "read {} to string: {}", highlight_path.display(), e),
                                }
                            }
                        }
                    }
                }
            }
        }
        Self {
            loaded: HashMap::new(), // not load any mindmap
            local,
            highlight,
        }
    }

    /// get all mindmap uuid pulldown option string
    pub fn pulldown_uuid(&self, uuid: &str) -> String {
        let mut pulldown: HashMap<String, String> = HashMap::new(); // key: uuid, value: pulldown option string
        let mut selected: &str;
        let mut uuid_inserted = false;
        // loaded, some new created mindmap in loaded but not in local, so first insert loaded
        for (k, v) in &self.loaded {
            selected = if k == uuid {
                uuid_inserted = true;
                " selected"
            } else {
                ""
            };
            pulldown.insert(
                k.clone(),
                match &v.1 {
                    Some(label) => format!("<option value='{}'{}>{}({})</option>", k, selected, k, label),
                    None => format!("<option value='{}'{}>{}</option>", k, selected, k),
                }
            );
        }
        // local, insert mindmap not in loaded but in local
        for (k, v) in &self.local {
            if !pulldown.contains_key(k) {
                selected = if k == uuid {
                    uuid_inserted = true;
                    " selected"
                } else {
                    ""
                };
                pulldown.insert(
                    k.clone(),
                    match &v.1 {
                        Some(label) => format!("<option value='{}'{}>{}({})</option>", k, selected, k, label),
                        None => format!("<option value='{}'{}>{}</option>", k, selected, k),
                    }
                );
            }
        }
        // if uuid not inserted, insert it
        if !uuid_inserted {
            pulldown.insert(
                uuid.to_string(),
                format!("<option value='{}' selected>{}</option>", uuid, uuid),
            );
        }
        pulldown.into_iter().map(|(_, v)| v).collect::<Vec<_>>().join("\n          ")
    }

    /// get all mindmap node pulldown option string
    fn pulldown_node(&self, uuid: &str, id_topic: Vec<(String, String)>) -> String {
        let mut node_options: Vec<String> = vec![
            "<option value='0' selected>highlight node</option>".to_string(),
            "<option value='clear'>❎ clear all highlight</option>".to_string(),
        ];
        let tmp_highlight = &self.highlight.get(uuid);
        let mut tmp_topic: String;
        for (k, v) in id_topic {
            tmp_topic = truncate_str(&v, 40).replace("<", "&lt;").replace(">", "&gt;");
            if let Some(tmp_hl) = &tmp_highlight {
                if tmp_hl.all_nodes.contains_key(&k) {
                    node_options.push(format!("<option value='{}'>✅ {}</option>", k, tmp_topic));
                } else {
                    node_options.push(format!("<option value='{}'>{}</option>", k, tmp_topic));
                }
            } else {
                node_options.push(format!("<option value='{}'>{}</option>", k, tmp_topic));
            }
        }
        node_options.join("\n          ")
    }

    /// get local mindmap by uuid, return ((mindmap data, label), pulldown_uuids, pulldown_nodes)
    pub fn get_local_mindmap(&mut self, uuid: &str, highlight_id: Option<String>) -> (Option<(String, Option<String>)>, String, Option<String>) {
        let mut add_to_loaded = false;
        let pulldown_ids = self.pulldown_uuid(uuid);
        let mut result = match (self.local.get(uuid), self.loaded.get(uuid)) {
            (Some(_), Some((content, label, _))) => Some((content.clone(), label.clone())), // already loaded mindmap
            (Some((json_file, label)), None) => { // in local, but not loaded
                match read_to_string(&json_file) {
                    Ok(content) => {
                        add_to_loaded = true;
                        Some((content, label.clone()))
                    },
                    Err(e) => {
                        event!(Level::ERROR, "read {} to string: {}", json_file.display(), e);
                        None
                    },
                }
            },
            (None, Some((content, label, _))) => Some((content.clone(), label.clone())), // created new mindmap, not in local
            (None, None) => None,
        };
        let mut update_loaded = false;
        let pulldown_nodes = if let Some((mut content, label)) = result {
            let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
            let pulldown_nodes: Option<String> = match serde_json::from_value::<MindmapNode>(parsed["nodeData"].clone()) {
                Ok(root) => {
                    // build parent and children relatinship
                    let parent_map = root.build_parent_map();
                    // hoghlight
                    if let Some(id) = highlight_id {
                        if id != "0" { // "0" means not select any node
                            if !add_to_loaded {
                                update_loaded = true;
                            }
                            if id == "clear" { // clear all selected nodes
                                if let Some(node_id_topic) = self.highlight.remove(uuid) {
                                    let colors_num = HIGHLIGHT_COLORS.len();
                                    for (k, v) in node_id_topic.all_nodes {
                                        if v == colors_num { // gray
                                            content = content.replace(&format!("\"id\":\"{}\",\"style\":{{\"background\":\"{}\"}}", k, HIGHLIGHT_GRAY), &format!("\"id\":\"{}\"", k));
                                        } else {
                                            content = content.replace(&format!("\"id\":\"{}\",\"style\":{{\"background\":\"{}\"}}", k, HIGHLIGHT_COLORS[v]), &format!("\"id\":\"{}\"", k));
                                        }
                                    }
                                    // delete local file
                                    let save_path = PARAS.outpath.join(uuid);
                                    let file_name = format!("{}.highlight", uuid);
                                    let highlight_path = save_path.join(file_name);
                                    if highlight_path.exists() && highlight_path.is_file() {
                                        if let Err(e) = remove_file(&highlight_path) {
                                            event!(Level::ERROR, "delete {} error: {}", highlight_path.display(), e);
                                        }
                                    }
                                }
                            } else {
                                // next color index
                                let color_index = match self.highlight.get(uuid) {
                                    Some(nodes) => {
                                        let color_index = nodes.selected_nodes.len();
                                        if color_index >= 9 {
                                            0
                                        } else {
                                            color_index
                                        }
                                    },
                                    None => {
                                        self.highlight.insert(uuid.to_string(), HighlightNode::new());
                                        0
                                    },
                                };
                                let node_id_color_map = self.highlight.get_mut(uuid).unwrap();
                                node_id_color_map.selected_nodes.insert(id.clone(), color_index); // insert selected node id
                                let colors_num = HIGHLIGHT_COLORS.len();
                                for (i, _, background) in root.get_ancestors(id, &parent_map) {
                                    match node_id_color_map.all_nodes.get_mut(&i) {
                                        Some(idx) => {
                                            if *idx < colors_num { // not gray, set to gray
                                                *idx = colors_num;
                                                content = content.replace(&format!("\"id\":\"{}\",\"style\":{{\"background\":\"{}\"}}", i, background.unwrap()), &format!("\"id\":\"{}\",\"style\":{{\"background\":\"{}\"}}", i, HIGHLIGHT_GRAY));
                                            }
                                        },
                                        None => {
                                            content = content.replace(&format!("\"id\":\"{}\"", i), &format!("\"id\":\"{}\",\"style\":{{\"background\":\"{}\"}}", i, HIGHLIGHT_COLORS[color_index]));
                                            node_id_color_map.all_nodes.insert(i, color_index); // insert selected and it's parent node id
                                        },
                                    }
                                }
                            }
                        }
                    }
                    // get all node id and topic
                    Some(self.pulldown_node(uuid, root.build_children_vec()))
                },
                Err(e) => {
                    event!(Level::ERROR, "{} convert mindmap data to struct: {}", uuid, e);
                    None
                },
            };
            // add to loaded hashmap
            if add_to_loaded {
                self.loaded.insert(uuid.to_string(), (content.clone(), label.clone(), false));
            } else if update_loaded {
                if let Some(loaded) = self.loaded.get_mut(uuid) {
                    loaded.0 = content.clone();
                    loaded.2 = true;
                }
            }
            result = Some((content, label));
            pulldown_nodes
        } else {
            None
        };

        (result, pulldown_ids, pulldown_nodes)
    }

    /// update loaded mindmap
    pub fn update_loaded_mindmap(&mut self, uuid: String, content: String, label: Option<String>) {
        match self.loaded.get_mut(&uuid) {
            Some(data) => {
                if label.is_none() {
                    let save_path = PARAS.outpath.join(&uuid);
                    let file_name = format!("{}.txt", uuid);
                    let label_path = save_path.join(file_name);
                    if label_path.exists() && label_path.is_file() {
                        if let Err(e) = remove_file(&label_path) {
                            event!(Level::ERROR, "delete {} error: {}", label_path.display(), e);
                        }
                    }
                }
                event!(Level::INFO, "{} update mindmap in server", &uuid);
                *data = (content, label, true);
            },
            None => {
                event!(Level::INFO, "create {} mindmap in server", uuid);
                self.loaded.insert(uuid, (content, label, true));
            },
        }
    }

    /// prepare uuid mindmap html content for download
    pub fn html_content(&self, uuid: &str) -> Option<String> {
        if let Some(value) = self.loaded.get(uuid) {
            let mut html = DEFAULT_PAGE
                .replace("<option value='mindmap' selected>mindmap</option>", &format!("<option value='{}' selected>{}</option>", uuid, uuid))
                .replace("mindmap.png", &format!("{}.png", uuid))
                .replace("highlight?uuid=&", &format!("highlight?uuid={}&", uuid))
                .replace("editable: true,", "editable: false,")
                .replace("const style = ``;", &format!("const style = `{}`;", INDEX))
                .replace("const katex = ``;", &format!("const katex = `{}`;", KATEX))
                .replace(" = MindElixir.new('root')", &format!(" = JSON.parse('{}')", &value.0.replace("\\", "\\\\").replace("-->", "--\\>").replace("'", "\\'")));
            if PARAS.language != "en" {
                html = html.replace("locale: 'en'", &format!("locale: '{}'", PARAS.language));
            }
            if let Some(l) = &value.1 {
                html = html.replace("    let mind;", &format!("    let mind;\n    document.getElementById('input-label').value = '{}';", l));
            }
            Some(html)
        } else {
            None
        }
    }

    /// save all updated mindmap to local
    pub fn save_mindmap(&self, uuid: Option<String>) {
        let mut stop = false;
        for (k, v) in &self.loaded {
            if v.2 {
                if let Some(u) = &uuid {
                    if k == u {
                        stop = true;
                    } else {
                        continue
                    }
                }
                // check path exist
                let save_path = PARAS.outpath.join(k);
                if !(save_path.exists() && save_path.is_dir()) {
                    if let Err(e) = create_dir_all(&save_path) {
                        event!(Level::ERROR, "create dir {} error: {}", save_path.display(), e);
                        continue
                    }
                }
                // save html
                let file_name = format!("{}.html", k);
                let file_path = save_path.join(file_name);
                let mut html = DEFAULT_PAGE
                    .replace("<option value='mindmap' selected>mindmap</option>", &format!("<option value='{}' selected>{}</option>", k, k))
                    .replace("mindmap.png", &format!("{}.png", k))
                    .replace("highlight?uuid=&", &format!("highlight?uuid={}&", k))
                    .replace("editable: true,", "editable: false,")
                    .replace("const style = ``;", &format!("const style = `{}`;", INDEX))
                    .replace("const katex = ``;", &format!("const katex = `{}`;", KATEX))
                    .replace(" = MindElixir.new('root')", &format!(" = JSON.parse('{}')", &v.0.replace("\\", "\\\\").replace("-->", "--\\>").replace("'", "\\'")));
                if PARAS.language != "en" {
                    html = html.replace("locale: 'en'", &format!("locale: '{}'", PARAS.language));
                }
                if let Some(l) = &v.1 {
                    html = html.replace("    let mind;", &format!("    let mind;\n    document.getElementById('input-label').value = '{}';", l));
                }
                if let Err(e) = write(&file_path, html) {
                    event!(Level::ERROR, "{}: save mindmap html {} error: {}", k, file_path.display(), e);
                }
                // save mindmap data
                let file_name = format!("{}.json", k);
                let file_path = save_path.join(file_name);
                if let Err(e) = write(&file_path, &v.0) {
                    event!(Level::ERROR, "{}: save mindmap data {} error: {}", k, file_path.display(), e);
                }
                // save label
                if let Some(l) = &v.1 {
                    let file_name = format!("{}.txt", k);
                    let file_path = save_path.join(file_name);
                    if let Err(e) = write(&file_path, l) {
                        event!(Level::ERROR, "{}: save mindmap label {} error: {}", k, file_path.display(), e);
                    }
                }
                if stop {
                    event!(Level::INFO, "{}: save mindmap done", k);
                    break
                }
            }
        }
        // save highlight node
        stop = false;
        for (k, v) in &self.highlight {
            if let Some(u) = &uuid {
                if k != u {
                    stop = true;
                } else {
                    continue
                }
            }
            let save_path = PARAS.outpath.join(k);
            let file_name = format!("{}.highlight", k);
            let file_path = save_path.join(file_name);
            match serde_json::to_string(&v) {
                Ok(json_str) => if let Err(e) = write(&file_path, &json_str) {
                    event!(Level::ERROR, "{}: save highlighted nodes {} error: {}", k, file_path.display(), e);
                },
                Err(e) => event!(Level::ERROR, "{}: convert highlighted nodes HashMap to string error: {}", k, e),
            }
            if stop {
                break
            }
        }
    }
}

/// truncate string
fn truncate_str(s: &str, len: usize) -> String {
    let chars: Vec<char> = s.chars().take(len).collect();
    chars.into_iter().collect()
}
