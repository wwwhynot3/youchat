#![allow(dead_code)]
use std::{
    path::{self, Path, PathBuf},
    sync::RwLock,
};

use chat::config::ChatConfig;
use chrono::SecondsFormat;
use lazy_static::lazy_static;
use serde_json::{json, Value};
use tauri::{App, AppHandle, Manager, Runtime, Wry};
use tauri_plugin_store::StoreExt;
pub mod chat;
pub static CONFIG_PATH: &str = "config.json";
pub static CONFIG_NAME: &str = "chat_config";
pub static DEFAULT_PATH: &str = "store.json";
lazy_static! {
    // pub static ref APP_HANDLE: RwLock<Option<AppHandle<Wry>>> = RwLock::new(None); //useless
    pub static ref CHAT_CONFIG: RwLock<Option<ChatConfig>> = RwLock::new(Option::None);
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn save_config<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    let config = CHAT_CONFIG
        .read()
        .map_err(|_| "Failed to read chat config")?;
    let config = config.as_ref().ok_or("Save Chat Config Before Load")?;
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
async fn write_to_store<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    path: Option<String>,
    key: String,
    value: String,
) -> Result<(), String> {
    eprintln!("1111111111111111111111");
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(DEFAULT_PATH),
    };
    eprintln!("2222222222222222222222");
    let store = app.store(path).map_err(|e| e.to_string())?;
    println!("3333333333333333333333");
    store.set(key, value);
    println!("4444444444444444444444");
    store.close_resource();
    Ok(())
}
#[tauri::command]
fn read_from_store<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    path: Option<String>,
    key: String,
) -> Result<Option<String>, String> {
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(DEFAULT_PATH),
    };
    let store = app.store(path).map_err(|e| e.to_string())?;
    let value = store.get(key);
    store.close_resource();
    Ok(value.map(|v| v.to_string()))
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![save_config])
        .invoke_handler(tauri::generate_handler![write_to_store])
        .invoke_handler(tauri::generate_handler![read_from_store])
        // 获取App的句柄,存入全局变量
        // .setup(|app| {
        //     let mut app_handle = APP_HANDLE
        //         .write()
        //         .expect("Failed to get app handle in initalizing app store");
        //     *app_handle = Some(app.handle().clone());
        //     Ok(())
        // })
        // 读取配置文件
        .setup(|app| {
            // 读取配置文件
            let store = app.store(Path::new(CONFIG_PATH))?;
            let value = store.get(CONFIG_NAME);
            let mut config = CHAT_CONFIG.write().expect("Failed to own chat config");
            *config = match value {
                Some(value) => {
                    Some(serde_json::from_value(value).expect("Failed to parse chat config"))
                }
                None => Some(ChatConfig::new()),
            };
            store.close_resource();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
