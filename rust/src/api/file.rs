use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use std::env;
use crate::types::file::FileState;
use chrono::{Utc, DateTime};

/// 获取系统临时文件夹路径
fn get_temp_dir() -> PathBuf {
    let mut temp_dir = env::temp_dir();
    temp_dir.push("cl-table");
    // 确保临时文件夹存在
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).unwrap();
    }
    temp_dir
}

/// 创建唯一的临时文件名
fn create_temp_filename() -> String {
    let uuid = Uuid::new_v4().to_string();
    format!("cl-table-{}.cl", uuid)
}

/// 创建新的临时文件，用于新建表
pub fn create_new_temp_file() -> Result<FileState, io::Error> {
    let temp_dir = get_temp_dir();
    let temp_filename = create_temp_filename();
    let temp_file_path = temp_dir.join(temp_filename);
    
    // 创建空的临时文件（后面会通过db操作创建表结构）
    let file = File::create(&temp_file_path)?;
    drop(file);
    
    let file_state = FileState::new(temp_file_path);
    Ok(file_state)
}

/// 从已有文件创建临时文件，用于打开表
pub fn create_temp_from_original(original_path: &Path) -> Result<FileState, io::Error> {
    let temp_dir = get_temp_dir();
    let temp_filename = create_temp_filename();
    let temp_file_path = temp_dir.join(temp_filename);
    
    // 复制原始文件到临时位置
    fs::copy(original_path, &temp_file_path)?;
    
    let file_state = FileState::from_original(original_path.to_path_buf(), temp_file_path);
    Ok(file_state)
}

/// 保存临时文件到指定路径
pub fn save_file(file_state: &FileState, target_path: &Path) -> Result<(), io::Error> {
    fs::copy(&file_state.temp_file_path, target_path)?;
    
    Ok(())
}

/// 另存为到新位置
pub fn save_as(file_state: &mut FileState, new_path: &Path) -> Result<(), io::Error> {
    fs::copy(&file_state.temp_file_path, new_path)?;
    
    file_state.file_path = Some(new_path.to_path_buf());
    file_state.is_unsaved = false;
    file_state.last_modified = Utc::now();
    
    Ok(())
}

/// 删除临时文件
pub fn cleanup_temp_file(file_state: &FileState) -> Result<(), io::Error> {
    if file_state.temp_file_path.exists() {
        fs::remove_file(&file_state.temp_file_path)?;
    }
    Ok(())
}

/// 清理所有过期的临时文件（启动时调用
pub fn cleanup_old_temp_files() -> Result<(), io::Error> {
    let temp_dir = get_temp_dir();
    if !temp_dir.exists() {
        return Ok(());
    }
    
    let twenty_four_hours_ago = Utc::now() - chrono::Duration::hours(24);
    
    for entry in fs::read_dir(temp_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(filename) = path.file_name() {
            let filename_str = filename.to_string_lossy();
            if filename_str.starts_with("cl-table-") && filename_str.ends_with(".cl") {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        let modified_datetime: DateTime<Utc> = modified.into();
                        if modified_datetime < twenty_four_hours_ago {
                            let _ = fs::remove_file(&path);
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}
