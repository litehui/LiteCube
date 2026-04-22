use std::error::Error;
use rusqlite::{Connection, Transaction, Result as RusqliteResult};
use chrono;
use crate::types::TableInfo;
use crate::db::{get_connection, insert_table_info};
use crate::api::validators::validate_table_info;

pub fn create_table(db_path: &str, table_info: TableInfo) -> Result<TableInfo, Box<dyn Error>> {
    // 验证表信息
    validate_table_info(&table_info)?;
    
    // 获取数据库连接
    let mut conn = get_connection(db_path)?;
    
    // 开始事务
    let tx = conn.transaction()?;
    
    // 插入表信息
    insert_table_info(&tx, &table_info)?;
    
    // 提交事务
    tx.commit()?;
    
    // 返回创建的表信息
    Ok(table_info)
}

pub fn get_table(db_path: &str, table_id: &str) -> Result<TableInfo, Box<dyn Error>> {
    let conn = get_connection(db_path)?;
    let mut stmt = conn.prepare("SELECT table_id, table_name, table_type, description, created_at, updated_at FROM tables WHERE table_id = ?")?;
    
    let table_info = stmt.query_row(&[table_id], |row| {
        Ok(TableInfo {
            table_id: row.get(0)?,
            table_name: row.get(1)?,
            table_type: row.get(2)?,
            description: row.get(3)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .unwrap()
                .with_timezone(&chrono::Utc),
        })
    })?;
    
    Ok(table_info)
}

pub fn list_tables(db_path: &str) -> Result<Vec<TableInfo>, Box<dyn Error>> {
    let conn = get_connection(db_path)?;
    let mut stmt = conn.prepare("SELECT table_id, table_name, table_type, description, created_at, updated_at FROM tables")?;
    
    let table_iter = stmt.query_map([], |row| {
        Ok(TableInfo {
            table_id: row.get(0)?,
            table_name: row.get(1)?,
            table_type: row.get(2)?,
            description: row.get(3)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .unwrap()
                .with_timezone(&chrono::Utc),
        })
    })?;
    
    let tables: RusqliteResult<Vec<_>> = table_iter.collect();
    Ok(tables?)
}
