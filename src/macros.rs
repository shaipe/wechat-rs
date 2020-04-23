//! copyright


/// 获取错误信息对象
#[macro_export]
macro_rules! logs {
    ($e: expr) => {{
        use std::fs::OpenOptions;
        use std::io::prelude::*;
        use std::time::SystemTime;

        let now = SystemTime::now();

        let file_path = "./logs.log";
        let content = format!("[{:?}] {}\n", now, $e);

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


/// ## Usage
/// ```rust
/// use simple_timer::timeit;
/// 
/// fn hello_world() {
///     println!("hello world");
/// }
/// 
/// fn main() {
///     timeit!("time_1", hello_world());
///     timeit!("time_two",
///         {
///             println!("great weather");
///             println!("i agree");
///         }
///     );
/// }
/// ```
#[macro_export]
macro_rules! watch_time {
    ($t: expr, $x:expr) => {
        {
            use std::time::Instant;
            let start = Instant::now();
            let res = $x;
            let end = start.elapsed();
            println!("=== excute({}) === use time: {}.{:03}s", $t, end.as_secs(), end.subsec_millis());
            res
        }
    };
}

