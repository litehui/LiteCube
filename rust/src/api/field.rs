use std::error::Error;
use rusqlite::{Connection, Transaction, Result as RusqliteResult};
use chrono;
use serde_json;
use crate::types::FieldInfo;
use crate::db::{get_connection, insert_field_info};
use crate::api::validators::validate_field_info;

pub fn add_field(db_path: &str, field_info: FieldInfo) -> Result<FieldInfo, Box<dyn Error>> {
    // 验证字段信息
    validate_field_info(&field_info)?;
    
    // 获取数据库连接
    let mut conn = get_connection(db_path)?;
    
    // 开始事务
    let tx = conn.transaction()?;
    
    // 插入字段信息
    insert_field_info(&tx, &field_info)?;
    
    // 提交事务
    tx.commit()?;
    
    // 返回创建的字段信息
    Ok(field_info)
}

pub fn get_fields(db_path: &str, table_id: &str) -> Result<Vec<FieldInfo>, Box<dyn Error>> {
    let conn = get_connection(db_path)?;
    let mut stmt = conn.prepare("SELECT field_id, table_id, field_name, field_type, properties, created_at, updated_at FROM fields WHERE table_id = ?")?;
    
    let field_iter = stmt.query_map(&[table_id], |row| {
        let properties_json: String = row.get(4)?;
        let properties: crate::types::FieldProperties = serde_json::from_str(&properties_json).unwrap();
        
        Ok(FieldInfo {
            field_id: row.get(0)?,
            table_id: row.get(1)?,
            field_name: row.get(2)?,
            field_type: row.get(3)?,
            properties,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .unwrap()
                .with_timezone(&chrono::Utc),
        })
    })?;
    
    let fields: RusqliteResult<Vec<_>> = field_iter.collect();
    Ok(fields?)
}
