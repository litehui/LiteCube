use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileState {
    pub file_path: Option<PathBuf>,
    pub temp_file_path: PathBuf,
    pub is_unsaved: bool,
    pub original_exists: bool,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

impl FileState {
    pub fn new(temp_path: PathBuf) -> Self {
        let now = Utc::now();
        Self {
            file_path: None,
            temp_file_path: temp_path,
            is_unsaved: true,
            original_exists: false,
            created_at: now,
            last_modified: now,
        }
    }

    pub fn from_original(original_path: PathBuf, temp_path: PathBuf) -> Self {
        let now = Utc::now();
        Self {
            file_path: Some(original_path),
            temp_file_path: temp_path,
            is_unsaved: false,
            original_exists: true,
            created_at: now,
            last_modified: now,
        }
    }

    pub fn get_db_path(&self) -> &PathBuf {
        &self.temp_file_path
    }
}
