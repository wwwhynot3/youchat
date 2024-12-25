use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
lazy_static! {}


#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct ChatConfig(HashMap<String, HashMap<String, String>>);

impl ChatConfig {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn export_to_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize ChatConfig to TOML")
    }
    pub fn add(&mut self, host: String, port: u16, topic: String, role: String) {
        let key = format!("{}:{}", host, port);
        let roles = self.0.entry(key).or_default();
        roles.insert(topic, role);
    }
    pub fn get(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.0
    }
    pub fn get_mut(&mut self) -> &mut HashMap<String, HashMap<String, String>> {
        &mut self.0
    }
}
