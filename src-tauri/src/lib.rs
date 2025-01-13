#![allow(dead_code)]

use chat::command::{self, read_config};
pub mod chat;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    pretty_env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::greet,
            command::save_config,
            command::write_to_store,
            command::read_from_store,
            command::remove_from_store
        ])
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
            tauri::async_runtime::block_on(async {
                let _config = read_config(app.handle().clone()).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
