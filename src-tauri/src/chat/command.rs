use serde_json::Value;
use std::path::{Path, PathBuf};
use tauri::Runtime;
use tauri_plugin_store::StoreExt;

use crate::chat::config::DEFAULT_PATH;

use super::config::{ChatConfig, CHAT_CONFIG, CONFIG_NAME, CONFIG_PATH};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
pub async fn save_config<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let config = CHAT_CONFIG.read().await;
    let config = config.as_ref().ok_or("Failed to get chat config")?;
    let store = app
        .store(Path::new(CONFIG_PATH))
        .map_err(|e| e.to_string())?;
    store.set(
        CONFIG_NAME,
        serde_json::to_string(config).map_err(|e| e.to_string())?,
    );
    store.close_resource();
    Ok(())
}
#[tauri::command]
pub async fn read_config<R: Runtime>(app: tauri::AppHandle<R>) -> Result<ChatConfig, String> {
    let store = app
        .store(Path::new(CONFIG_PATH))
        .map_err(|e| format!("Failed to Own Store(Path:{})\nErr:{}", CONFIG_PATH, e))?;
    let value = store.get(CONFIG_NAME);
    store.close_resource();
    let config = match value {
        Some(value) => {
            let value_clone = value.clone();
            serde_json::from_value(value).map_err(|e| {
                format!(
                    "Failed to Deserialize Config (Path:{})\nValue:{}\nErr:{}",
                    CONFIG_PATH, value_clone, e
                )
            })?
        }
        None => ChatConfig::new(),
    };
    let mut config_opt = CHAT_CONFIG.write().await;
    *config_opt = Some(config.clone());

    Ok(config)
}
#[tauri::command]
pub async fn write_to_store<R: Runtime>(
    app: tauri::AppHandle<R>,
    path: Option<String>,
    key: String,
    value: String,
) -> Result<(), String> {
    eprintln!("1111111111111111111111");
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(DEFAULT_PATH),
    };
    // let path = PathBuf::from(DEFAULT_PATH);
    eprintln!("2222222222222222222222");
    let store = app.store(path).map_err(|e| e.to_string())?;
    println!("3333333333333333333333");
    store.set(key, value);
    println!("4444444444444444444444");
    // store.close_resource();
    println!("5555555555555555555555");
    Ok(())
}
#[tauri::command]
pub fn read_from_store<R: Runtime>(
    app: tauri::AppHandle<R>,
    path: Option<String>,
    key: String,
) -> Result<Value, String> {
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(DEFAULT_PATH),
    };
    let store = app.store(path).map_err(|e| e.to_string())?;
    let value = store.get(key.clone());
    store.close_resource();
    // Ok(value.map(|v| v.to_string()))
    value.ok_or(format!("Unexistent Key:{}", key))
}
#[tauri::command]
pub async fn remove_from_store<R: Runtime>(
    app: tauri::AppHandle<R>,
    path: Option<String>,
    key: String,
) -> Result<bool, String> {
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(DEFAULT_PATH),
    };
    let store = app.store(path).map_err(|e| e.to_string())?;
    let deleted = store.delete(key);
    println!("Deleted:{}", deleted);
    store.close_resource();
    Ok(deleted)
}
