use crate::types::{TableInfo, FieldInfo};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ValidationError {}

pub fn validate_table_info(table_info: &TableInfo) -> Result<(), ValidationError> {
    // 验证表名称
    if table_info.table_name.is_empty() {
        return Err(ValidationError { message: "表名称不能为空".to_string() });
    }
    
    if table_info.table_name.len() > 50 {
        return Err(ValidationError { message: "表名称长度不能超过50个字符".to_string() });
    }
    
    // 验证表描述
    if table_info.description.len() > 200 {
        return Err(ValidationError { message: "表描述长度不能超过200个字符".to_string() });
    }
    
    Ok(())
}

pub fn validate_field_info(field_info: &FieldInfo) -> Result<(), ValidationError> {
    // 验证字段名称
    if field_info.field_name.is_empty() {
        return Err(ValidationError { message: "字段名称不能为空".to_string() });
    }
    
    if field_info.field_name.len() > 30 {
        return Err(ValidationError { message: "字段名称长度不能超过30个字符".to_string() });
    }
    
    // 验证字段类型
    let valid_types = vec!["文本", "数字", "单选", "多选", "超级链接", "日期/时间", "布尔值"];
    if !valid_types.contains(&field_info.field_type.as_str()) {
        return Err(ValidationError { message: "无效的字段类型".to_string() });
    }
    
    // 验证字段属性
    match field_info.field_type.as_str() {
        "单选" | "多选" => {
            if field_info.properties.options.is_none() || field_info.properties.options.as_ref().unwrap().is_empty() {
                return Err(ValidationError { message: "单选和多选字段必须设置选项列表".to_string() });
            }
        }
        _ => {}
    }
    
    Ok(())
}
