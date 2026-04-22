use clap::{App, Arg, SubCommand};
use crate::api::{create_table, add_field, create_new_temp_file, save_file};
use crate::types::{TableInfo, FieldInfo, FieldProperties};
use std::path::Path;

pub fn run() {
    let matches = App::new("cl-table")
        .version("1.0")
        .about("多维表命令行工具")
        .subcommand(SubCommand::with_name("create-table")
            .about("创建多维表")
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("表名称")
                .required(true))
            .arg(Arg::with_name("description")
                .short("d")
                .long("description")
                .value_name("DESCRIPTION")
                .help("表描述"))
            .arg(Arg::with_name("db")
                .short("b")
                .long("db")
                .value_name("DATABASE")
                .help("数据库文件路径")
                .default_value("./data.cl")))
        .subcommand(SubCommand::with_name("add-field")
            .about("添加字段")
            .arg(Arg::with_name("table-id")
                .short("t")
                .long("table-id")
                .value_name("TABLE_ID")
                .help("表ID")
                .required(true))
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("字段名称")
                .required(true))
            .arg(Arg::with_name("type")
                .short("y")
                .long("type")
                .value_name("TYPE")
                .help("字段类型: 文本, 数字, 单选, 多选, 超级链接, 日期/时间, 布尔值")
                .required(true))
            .arg(Arg::with_name("options")
                .short("o")
                .long("options")
                .value_name("OPTIONS")
                .help("选项列表，用逗号分隔，仅用于单选和多选字段"))
            .arg(Arg::with_name("required")
                .short("r")
                .long("required")
                .help("是否必填"))
            .arg(Arg::with_name("default")
                .short("e")
                .long("default")
                .value_name("DEFAULT")
                .help("默认值"))
            .arg(Arg::with_name("db")
                .short("b")
                .long("db")
                .value_name("DATABASE")
                .help("数据库文件路径")
                .default_value("./data.cl")))
        .get_matches();

    match matches.subcommand() {
        ("create-table", Some(sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            let description = sub_m.value_of("description").unwrap_or("");
            let db_path = sub_m.value_of("db").unwrap();
            
            let table_info = TableInfo::new(name.to_string(), description.to_string());
            
            match create_table(db_path, table_info) {
                Ok(table) => println!("表创建成功: {} ({})\n", table.table_name, table.table_id),
                Err(e) => println!("表创建失败: {}\n", e),
            }
        },
        ("add-field", Some(sub_m)) => {
            let table_id = sub_m.value_of("table-id").unwrap();
            let name = sub_m.value_of("name").unwrap();
            let field_type = sub_m.value_of("type").unwrap();
            let options = sub_m.value_of("options").map(|o| o.split(",").map(|s| s.trim().to_string()).collect());
            let required = sub_m.is_present("required");
            let default_value = sub_m.value_of("default").map(|s| s.to_string());
            let db_path = sub_m.value_of("db").unwrap();
            
            let properties = FieldProperties {
                default_value,
                required,
                options,
            };
            
            let field_info = FieldInfo::new(table_id.to_string(), name.to_string(), field_type.to_string(), properties);
            
            match add_field(db_path, field_info) {
                Ok(field) => println!("字段添加成功: {} ({})\n", field.field_name, field.field_id),
                Err(e) => println!("字段添加失败: {}\n", e),
            }
        },
        _ => {
            println!("使用 --help 查看帮助信息\n");
        }
    }
}
