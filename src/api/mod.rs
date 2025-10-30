use std::collections::HashMap;
use std::fs::{write, read_to_string, create_dir_all};
use std::path::PathBuf;
use std::sync::RwLock;

use once_cell::sync::Lazy;
use tracing::{event, Level};

pub mod parse_paras;
pub mod router;
pub mod error;
pub mod ctrlc;

mod handlers;

use crate::parse_paras::PARAS;

/// mindmap default page
pub const DEFAULT_PAGE: &str = include_str!("../../assets/mindmap.html");
/// style for export png
pub const INDEX: &str = include_str!("../../assets/index.css");
pub const KATEX: &str = include_str!("../../assets/katex.css");

/// global data, store all mindmap
pub static DATA: Lazy<RwLock<MindMap>> = Lazy::new(|| RwLock::new(MindMap::new()));

/// store local mindmap
pub struct MindMap {
    loaded: HashMap<String, (String, Option<String>, bool)>, // loaded mindmap, key: uuid, value: (mindmap data, label string, updated)
    local:  HashMap<String, (PathBuf, Option<String>)>, // all mindmap in outpath, key : uuid, value: (mindmap data file path, label string), e.g. (f66bedbd-9972-4ec3-9a30-9510d4fffe1c.html, f66bedbd-9972-4ec3-9a30-9510d4fffe1c.json, "my first mindmap")
}

impl MindMap {
    /// get all mindmap file path from outpath
    fn new() -> Self {
        let mut local = HashMap::new();
        if let Ok(uuid_dirs) = PARAS.outpath.read_dir() {
            for i in uuid_dirs {
                if let Ok(entry) = i {
                    let uuid_path = entry.path();
                    if uuid_path.is_dir() {
                        if let Some(uuid) = uuid_path.file_name().unwrap().to_str() {
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
                        }
                    }
                }
            }
        }
        Self {
            loaded: HashMap::new(), // not load any mindmap
            local,
        }
    }

    /// get all mindmap pulldown option string vec
    fn pulldown(&self, uuid: &str) -> Vec<String> {
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
        pulldown.into_iter().map(|(_, v)| v).collect()
    }

    /// get all mindmap pulldown option string
    pub fn html_pulldown(&self, uuid: &str) -> String {
        self.pulldown(uuid).join("\n          ")
    }

    /// get local mindmap by uuid, return ((mindmap data, label), pulldown)
    pub fn get_local_mindmap(&mut self, uuid: &str) -> (Option<(String, Option<String>)>, String) {
        let mut add_to_loaded = false;
        let pulldown = self.pulldown(uuid);
        let result = match (self.local.get(uuid), self.loaded.get(uuid)) {
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
        if add_to_loaded {
            if let Some((content, label)) = &result {
                self.loaded.insert(uuid.to_string(), (content.clone(), label.clone(), false));
            }
        }
        (result, pulldown.join("\n          "))
    }

    /// update loaded mindmap
    pub fn update_loaded_mindmap(&mut self, uuid: String, content: String, label: Option<String>) {
        match self.loaded.get_mut(&uuid) {
            Some(data) => {
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
                .replace("const style = ``;", &format!("const style = `{}`;", INDEX))
                .replace("const katex = ``;", &format!("const katex = `{}`;", KATEX))
                .replace("MindElixir.new('root')", &format!("JSON.parse('{}')", &value.0));
            if PARAS.language != "en" {
                html = html.replace("locale: 'en'", &format!("locale: '{}'", PARAS.language));
            }
            if let Some(l) = &value.1 {
                html = html.replace("placeholder='mindmap label'>", &format!("placeholder='{}'>", l));
            }
            Some(html)
        } else {
            None
        }
    }

    /// save all updated mindmap to local
    pub fn save_mindmap(&self) {
        for (k, v) in &self.loaded {
            if v.2 {
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
                    .replace("const style = ``;", &format!("const style = `{}`;", INDEX))
                    .replace("const katex = ``;", &format!("const katex = `{}`;", KATEX))
                    .replace("MindElixir.new('root')", &format!("JSON.parse('{}')", &v.0));
                if PARAS.language != "en" {
                    html = html.replace("locale: 'en'", &format!("locale: '{}'", PARAS.language));
                }
                if let Some(l) = &v.1 {
                    html = html.replace("placeholder='mindmap label'>", &format!("placeholder='{}'>", l));
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
            }
        }
    }
}
