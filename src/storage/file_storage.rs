use crate::models::graph::Graph;
use std::fs;

pub fn save(graph: &Graph, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(graph)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load(path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(path)?;
    let graph = serde_json::from_str(&json)?;
    Ok(graph)
}
