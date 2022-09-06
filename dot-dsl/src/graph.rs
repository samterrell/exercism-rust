pub mod graph_items;
use std::collections::HashMap;

use graph_items::edge::Edge;
use graph_items::node::Node;
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            attrs: HashMap::new(),
        }
    }
    pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
        Self {
            nodes: nodes.to_vec(),
            ..self
        }
    }
    pub fn with_edges(self, edges: &Vec<Edge>) -> Self {
        Self {
            edges: edges.to_vec(),
            ..self
        }
    }
    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Self {
            attrs: HashMap::from_iter(
                attrs
                    .iter()
                    .map(|(k, v)| (String::from(*k), String::from(*v))),
            ),
            ..self
        }
    }
    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }
}
