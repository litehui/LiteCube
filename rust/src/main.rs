use cl_table::cli::run;

fn main() {
    // 清理旧的临时文件
    let _ = cl_table::api::cleanup_old_temp_files();
    
    // 运行CLI命令
    run();
}
