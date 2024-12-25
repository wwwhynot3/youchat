// use std::sync::Arc;

// use chrono::Utc;
// use lazy_static::lazy_static;
// use serde::{Deserialize, Serialize};

// // use super::config::{ChatConfig};

// lazy_static! {}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Chat {
//     config: ChatConfig,
//     messages: Arc<Vec<ChatMessage>>,
// }
// impl Chat {
//     pub fn new(config: &ChatConfig) -> Self {
//         Self {
//             config: config.clone(),
//             messages: Arc::new(Vec::new()),
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct ChatMessage {
//     from: String,
//     timestamp: i64,
//     message: String,
// }
// impl ChatMessage {
//     pub fn new(from: String, message: String) -> Self {
//         Self {
//             from,
//             timestamp: Utc::now().timestamp(),
//             message,
//         }
//     }
// }
