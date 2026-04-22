use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableInfo {
    pub table_id: String,
    pub table_name: String,
    pub table_type: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TableInfo {
    pub fn new(table_name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            table_id: Uuid::new_v4().to_string(),
            table_name,
            table_type: "多维表".to_string(),
            description,
            created_at: now,
            updated_at: now,
        }
    }
}
