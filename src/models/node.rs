use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub links: Vec<String>,
}

impl Node {
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            created_at: Utc::now(),
            tags: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn with_tags(title: String, content: String, tags: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            created_at: Utc::now(),
            tags,
            links: Vec::new(),
        }
    }

    pub fn add_link(&mut self, link_id: String) {
        if !self.links.contains(&link_id) {
            self.links.push(link_id);
        }
    }

    /// Fixed: now pushes to self.tags instead of self.links
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_link(&mut self, link_id: String) {
        self.links.retain(|id| id != &link_id);
    }

    pub fn remove_tag(&mut self, tag: String) {
        self.tags.retain(|t| t != &tag);
    }
}
