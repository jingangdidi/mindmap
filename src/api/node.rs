use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// mindmap node data
#[derive(Deserialize, Serialize)]
pub struct MindmapNode {
    pub id: String, // node id
    pub topic: String, // node name
    pub style: Option<HashMap<String, String>>, // background color
    #[serde(default)]
    pub children: Vec<MindmapNode>, // children node
}

impl MindmapNode {
    /// child_id -> (parent_id, child topic, child style)
    pub fn build_parent_map(&self) -> HashMap<String, (String, String, Option<HashMap<String, String>>)> {
        let mut parent_map = HashMap::new();
        Self::collect_parents(&self, None, &mut parent_map);
        parent_map
    }

    /// get parent and child relationship
    fn collect_parents(node: &MindmapNode, parent_id: Option<String>, parent_map: &mut HashMap<String, (String, String, Option<HashMap<String, String>>)>) {
        if let Some(parent) = parent_id {
            parent_map.insert(node.id.clone(), (parent, node.topic.clone(), node.style.clone()));
        }
        for child in &node.children {
            Self::collect_parents(child, Some(node.id.clone()), parent_map);
        }
    }

    /// Vec<(child_id, child topic)>
    pub fn build_children_vec(&self) -> Vec<(String, String)> {
        let mut children_vec = Vec::new();
        Self::collect_children(&self, &mut children_vec);
        children_vec
    }

    /// get all child id and topic
    fn collect_children(node: &MindmapNode, children_vec: &mut Vec<(String, String)>) {
        for child in &node.children {
            children_vec.push((child.id.clone(), child.topic.clone()));
            Self::collect_children(child, children_vec);
        }
    }

    /// get all parents id, topic, style based on specified id
    pub fn get_ancestors(&self, id: String, parent_map: &HashMap<String, (String, String, Option<HashMap<String, String>>)>) -> Vec<(String, String, Option<String>)> {
        let mut current_id = id.clone();
        let mut path = match parent_map.get(&current_id) {
            Some(tmp) => vec![
                (
                    id,
                    tmp.1.clone(),
                    match &tmp.2 {
                        Some(style) => match style.get("background") {
                            Some(bg) => Some(bg.clone()),
                            None => None,
                        },
                        None => None,
                    },
                )
            ],
            None => Vec::new(),
        };

        // from current id to root id
        while let Some((parent_id, _, _)) = parent_map.get(&current_id) {
            if let Some(tmp) = parent_map.get(parent_id) {
                path.push(
                    (
                        parent_id.clone(),
                        tmp.1.clone(),
                        match &tmp.2 {
                            Some(style) => match style.get("background") {
                                Some(bg) => Some(bg.clone()),
                                None => None,
                            },
                            None => None,
                        },
                    )
                );
            }
            current_id = parent_id.clone();
        }

        path
    }
}
