// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;
use tauri::command;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    base_path: String,
    last_left_path: String,
    last_source_selections: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Preset {
    name: String,
    selections: Vec<String>,
}

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

// 获取配置文件路径
fn get_config_path() -> Result<std::path::PathBuf, String> {
    let exe_path = std::env::current_exe().map_err(|_| "无法获取程序路径".to_string())?;
    let exe_dir = exe_path.parent().ok_or("无法获取程序目录".to_string())?;
    Ok(exe_dir.join("last_path.json"))
}

// 获取预设文件路径
fn get_presets_path() -> Result<std::path::PathBuf, String> {
    let exe_path = std::env::current_exe().map_err(|_| "无法获取程序路径".to_string())?;
    let exe_dir = exe_path.parent().ok_or("无法获取程序目录".to_string())?;
    Ok(exe_dir.join("presets.json"))
}

// 保存配置
#[command]
fn save_config(config: AppConfig) -> Result<String, String> {
    let config_json = serde_json::to_string_pretty(&config)
        .map_err(|_| "序列化配置失败".to_string())?;
    
    let config_path = get_config_path()?;
    fs::write(config_path, config_json)
        .map_err(|e| format!("保存配置失败: {}", e))?;
    
    Ok("配置保存成功".to_string())
}

// 加载配置
#[command]
fn load_config() -> Result<AppConfig, String> {
    let config_path = get_config_path()?;
    match fs::read_to_string(config_path) {
        Ok(content) => {
            serde_json::from_str(&content)
                .map_err(|e| format!("解析配置失败: {}", e))
        }
        Err(_) => Ok(AppConfig {
            base_path: String::new(),
            last_left_path: String::new(),
            last_source_selections: vec![String::new(); 4],
        })
    }
}

// 保存预设
#[command]
fn save_presets(presets: HashMap<String, Vec<String>>) -> Result<String, String> {
    let presets_json = serde_json::to_string_pretty(&presets)
        .map_err(|_| "序列化预设失败".to_string())?;
    
    let presets_path = get_presets_path()?;
    fs::write(presets_path, presets_json)
        .map_err(|e| format!("保存预设失败: {}", e))?;
    
    Ok("预设保存成功".to_string())
}

// 加载预设
#[command]
fn load_presets() -> Result<HashMap<String, Vec<String>>, String> {
    let presets_path = get_presets_path()?;
    match fs::read_to_string(presets_path) {
        Ok(content) => {
            serde_json::from_str(&content)
                .map_err(|e| format!("解析预设失败: {}", e))
        }
        Err(_) => Ok(HashMap::new())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_subdirectories,
            copy_directory,
            save_config,
            load_config,
            save_presets,
            load_presets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}