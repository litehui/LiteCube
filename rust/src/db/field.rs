use std::error::Error;
use rusqlite::{Transaction, Result as RusqliteResult};
use serde_json::to_string;
use crate::types::FieldInfo;

pub fn insert_field_info(tx: &Transaction, field_info: &FieldInfo) -> Result<String, Box<dyn Error>> {
    let sql = "INSERT INTO fields (field_id, table_id, field_name, field_type, properties, created_at, updated_at) 
               VALUES (?, ?, ?, ?, ?, ?, ?)";
    
    let properties_json = to_string(&field_info.properties)?;
    
    tx.execute(
        sql,
        &[&field_info.field_id, &field_info.table_id, &field_info.field_name, 
          &field_info.field_type, &properties_json, &field_info.created_at.to_rfc3339(), &field_info.updated_at.to_rfc3339()],
    )?;
    
    // 在数据存储表中添加字段，将连字符替换为下划线以避免SQL语法错误
    let safe_table_id = field_info.table_id.replace('-', "_");
    let safe_field_id = field_info.field_id.replace('-', "_");
    let alter_table_sql = format!(
        "ALTER TABLE data_{} ADD COLUMN {} TEXT",
        safe_table_id, safe_field_id
    );
    tx.execute(&alter_table_sql, [])?;
    
    Ok(field_info.field_id.clone())
}
