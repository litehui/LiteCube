use rusqlite::{Connection, Result};

pub fn get_connection(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    init_tables(&conn)?;
    Ok(conn)
}

fn init_tables(conn: &Connection) -> Result<()> {
    // 创建表信息表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tables (
            table_id TEXT PRIMARY KEY,
            table_name TEXT NOT NULL,
            table_type TEXT NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;
    
    // 创建字段信息表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fields (
            field_id TEXT PRIMARY KEY,
            table_id TEXT NOT NULL,
            field_name TEXT NOT NULL,
            field_type TEXT NOT NULL,
            properties TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (table_id) REFERENCES tables(table_id)
        )",
        [],
    )?;
    
    // 创建索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_fields_table_id ON fields(table_id)",
        [],
    )?;
    
    Ok(())
}
