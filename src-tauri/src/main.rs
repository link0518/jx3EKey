// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::command;


// 获取子目录列表
#[command]
fn get_subdirectories(path: String) -> Result<Vec<String>, String> {
    let path = Path::new(&path);
    if !path.exists() {
        return Err("路径不存在".to_string());
    }

    let mut subdirs = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            subdirs.push(name.to_string());
                        }
                    }
                }
            }
        }
        Err(_) => return Err("无法读取目录".to_string()),
    }

    subdirs.sort();
    Ok(subdirs)
}

// 复制目录
#[command]
fn copy_directory(source: String, target: String) -> Result<String, String> {
    let source_path = Path::new(&source);
    let target_path = Path::new(&target);

    if !source_path.exists() {
        return Err("源路径不存在".to_string());
    }

    // 如果目标目录存在，先删除
    if target_path.exists() {
        if let Err(_) = fs::remove_dir_all(target_path) {
            return Err("无法清理目标目录".to_string());
        }
    }

    // 创建目标目录
    if let Err(_) = fs::create_dir_all(target_path) {
        return Err("无法创建目标目录".to_string());
    }

    // 复制文件和目录
    copy_dir_recursive(source_path, target_path)?;

    Ok("复制成功".to_string())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(|_| "创建目录失败".to_string())?;
    }

    for entry in fs::read_dir(src).map_err(|_| "读取源目录失败".to_string())? {
        let entry = entry.map_err(|_| "读取目录项失败".to_string())?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|_| "复制文件失败".to_string())?;
        }
    }

    Ok(())
}


// 获取应用数据文件路径
fn get_app_data_path() -> Result<std::path::PathBuf, String> {
    let exe_path = std::env::current_exe().map_err(|_| "无法获取程序路径".to_string())?;
    let exe_dir = exe_path.parent().ok_or("无法获取程序目录".to_string())?;
    Ok(exe_dir.join("app_data.json"))
}

// 保存应用数据
#[command]
fn save_app_data(data: serde_json::Value) -> Result<String, String> {
    let app_data_json = serde_json::to_string_pretty(&data).map_err(|_| "序列化应用数据失败".to_string())?;
    
    let app_data_path = get_app_data_path()?;
    fs::write(app_data_path, app_data_json).map_err(|e| format!("保存应用数据失败: {}", e))?;
    
    Ok("应用数据保存成功".to_string())
}

// 加载应用数据
#[command]
fn load_app_data() -> Result<serde_json::Value, String> {
    let app_data_path = get_app_data_path()?;
    match fs::read_to_string(app_data_path) {
        Ok(content) => serde_json::from_str(&content).map_err(|e| format!("解析应用数据失败: {}", e)),
        Err(_) => {
            // 返回默认的应用数据结构
            let default_data = serde_json::json!({
                "config": {
                    "base_path": "",
                    "last_left_path": "",
                    "last_source_selections": ["", "", "", ""]
                },
                "presets": {},
                "version": "3.0.0"
            });
            Ok(default_data)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_subdirectories,
            copy_directory,
            save_app_data,
            load_app_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
