use std::error::Error;
use rusqlite::{Transaction, Result as RusqliteResult};
use crate::types::TableInfo;

pub fn insert_table_info(tx: &Transaction, table_info: &TableInfo) -> Result<String, Box<dyn Error>> {
    let sql = "INSERT INTO tables (table_id, table_name, table_type, description, created_at, updated_at) 
               VALUES (?, ?, ?, ?, ?, ?)";
    
    tx.execute(
        sql,
        &[&table_info.table_id, &table_info.table_name, &table_info.table_type, 
          &table_info.description, &table_info.created_at.to_rfc3339(), &table_info.updated_at.to_rfc3339()],
    )?;
    
    // 创建数据存储表，将连字符替换为下划线以避免SQL语法错误
    let safe_table_id = table_info.table_id.replace('-', "_");
    let create_table_sql = format!(
        "CREATE TABLE IF NOT EXISTS data_{} (
            id TEXT PRIMARY KEY,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        safe_table_id
    );
    tx.execute(&create_table_sql, [])?;
    
    Ok(table_info.table_id.clone())
}
