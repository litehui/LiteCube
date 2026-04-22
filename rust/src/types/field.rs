use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldProperties {
    pub default_value: Option<String>,
    pub required: bool,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldInfo {
    pub field_id: String,
    pub table_id: String,
    pub field_name: String,
    pub field_type: String, // 文本、数字、单选、多选、超级链接、日期/时间、布尔值
    pub properties: FieldProperties,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FieldInfo {
    pub fn new(table_id: String, field_name: String, field_type: String, properties: FieldProperties) -> Self {
        let now = Utc::now();
        Self {
            field_id: Uuid::new_v4().to_string(),
            table_id,
            field_name,
            field_type,
            properties,
            created_at: now,
            updated_at: now,
        }
    }
}
