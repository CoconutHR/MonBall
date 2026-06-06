#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};
use std::fs;
use std::path::PathBuf;

/// 数据请求命令：向远程 Agent 拉取系统指标
#[tauri::command]
fn fetch_stats(ip: String, port: u16, token: String) -> Result<serde_json::Value, String> {
    let url = format!("http://{}:{}/api/v1/stats", ip, port);
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .get(&url)
        .header("x-monitor-token", &token)
        .send()
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
    Ok(json)
}

/// 读取配置
#[tauri::command]
fn read_config(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let path = get_config_path(&app)?;
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    } else {
        // 默认配置
        Ok(serde_json::json!({
            "items": [
                {"id": "cpu", "label": "CPU", "show": true},
                {"id": "mem", "label": "MEM", "show": true},
                {"id": "disk", "label": "DISK", "show": true},
                {"id": "net", "label": "NET", "show": true}
            ],
            "server": {"ip": "127.0.0.1", "port": 8080, "token": "secret123"}
        }))
    }
}

/// 保存配置
#[tauri::command]
fn save_config(app: tauri::AppHandle, config: serde_json::Value) -> Result<(), String> {
    let path = get_config_path(&app)?;
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

/// 辅助：获取配置文件路径
fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("overlay_config.json"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 构建托盘菜单
            let lock_item = MenuItemBuilder::with_id("lock", "锁定 (穿透)").build(app)?;
            let unlock_item = MenuItemBuilder::with_id("unlock", "解锁 (可拖动)").build(app)?;
            let config_item = MenuItemBuilder::with_id("config", "配置").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&lock_item, &unlock_item, &config_item, &quit_item])
                .build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "lock" => {
                        if let Some(win) = app.get_webview_window("overlay") {
                            let _ = win.set_ignore_cursor_events(true);
                        }
                    }
                    "unlock" => {
                        if let Some(win) = app.get_webview_window("overlay") {
                            let _ = win.set_ignore_cursor_events(false);
                        }
                    }
                    "config" => {
                        if let Some(win) = app.get_webview_window("config") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_stats,
            read_config,
            save_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
