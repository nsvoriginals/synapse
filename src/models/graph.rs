use super::{node::Node, edge::Edge};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn remove_node(&mut self, node_id: String) {
        self.nodes.remove(&node_id);
    }
      //FIXING MUTATION
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn list_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        if self.nodes.contains_key(&from) && self.nodes.contains_key(&to) {
            let edge = Edge::new(from, to);
            self.edges.push(edge);
        }
    }

    pub fn get_edges(&self, node_id: &str) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|e| e.from == node_id || e.to == node_id)
            .collect()
    }

    pub fn search_filter(&self, query: String) -> Vec<&Node> {
        let query_lower = query.to_lowercase();
        self.nodes
            .values()
            .filter(|node| {
                node.title.to_lowercase().contains(&query_lower)
                    || node.content.to_lowercase().contains(&query_lower)
                    || node.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let graph = serde_json::from_str(&json)?;
        Ok(graph)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
