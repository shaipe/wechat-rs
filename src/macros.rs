/// 获取错误信息对象
#[macro_export]
macro_rules! logs {
    ($e: expr) => {{
        use std::fs::OpenOptions;
        use std::io::prelude::*;
        let file_path = "./logs.log";
        let content = format!("{}\n", $e);

        // 以读,写,创建,追加的方式打开文件
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(file_path);

        // 向文件中写入内容
        match file {
            Ok(mut stream) => {
                stream.write_all(content.as_bytes()).unwrap();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }};
}
