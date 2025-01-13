use std::collections::HashMap;

use chrono::Utc;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
// use tokio::sync::RwLock;
use tauri::async_runtime::RwLock;
pub static CONFIG_PATH: &str = "config.json";
pub static CONFIG_NAME: &str = "chat_config";
pub static DEFAULT_PATH: &str = "store.json";
lazy_static! {
    // pub static ref APP_HANDLE: RwLock<Option<AppHandle<Wry>>> = RwLock::new(None); //useless
    pub static ref CHAT_CONFIG: RwLock<Option<ChatConfig>> = RwLock::new(Option::None);
}

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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    from: String,
    timestamp: i64,
    message: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MqMessage {
    pub sha256: [u8; 32],
    pub message: ChatMessage,
}
impl ChatMessage {
    pub fn new(from: String, message: String) -> Self {
        Self {
            from,
            timestamp: Utc::now().timestamp(),
            message,
        }
    }
    // todo city hash ❌ sha256 ✅
    pub fn from_json(msg: String) -> Result<Self, String> {
        let msg: ChatMessage = serde_json::from_str(&msg).map_err(|e| e.to_string())?;
        Ok(msg)
    }
    pub fn from_binary(msg: Vec<u8>) -> Result<Self, String> {
        let msg: ChatMessage = bincode::deserialize(&msg).map_err(|e| e.to_string())?;
        Ok(msg)
    }
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }
    pub fn to_binary(&self) -> Result<Vec<u8>, String> {
        bincode::serialize(self).map_err(|e| e.to_string())
    }
    pub fn calculate_sha256(&self) -> [u8; 32] {
        Sha256::digest(self.to_binary().expect("Failed to Binarilize ChatMessage")).into()
    }
}
impl MqMessage {
    pub fn new(from: String, message: String) -> Self {
        let message = ChatMessage::new(from, message);
        let sha256 = message.calculate_sha256();
        Self { sha256, message }
    }
    pub fn from_json(msg: String) -> Result<Self, String> {
        serde_json::from_str(&msg).map_err(|e| e.to_string())
    }
    pub fn from_binary(msg: Vec<u8>) -> Result<Self, String> {
        bincode::deserialize(&msg).map_err(|e| e.to_string())?
    }
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }
    pub fn to_binary(&self) -> Result<Vec<u8>, String> {
        bincode::serialize(self).map_err(|e| e.to_string())
    }
}
