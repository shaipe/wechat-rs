//! copyright © ecdata.cn 2021 - present
//! 宏处理
//! created by shaipe 20210228

/// 获取错误信息对象
#[macro_export]
macro_rules! logs {
    ($e: expr) => {{
        use std::fs::OpenOptions;
        use std::io::prelude::*;
        use std::time::SystemTime;

        let now = SystemTime::now();

        let file_path = "./logs.log";
        let content = format!("[{:?}]\n {}\n", now, $e);

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


/// 获取错误信息对象
#[macro_export]
macro_rules! error {
    // error! {code: i32, msg: String};
    (code: $code: expr, msg: $msg: expr) => {{
        crate::WechatError::custom($code, $msg)
    }};
    // error! {code: i32, msg: String,};
    (code: $code: expr, msg: $msg: expr,) => {{
        crate::WechatError::custom($code, $msg)
    }};

    // error!(msg);
    ($e: expr) => {{
        crate::WechatError::msg($e)
        // ::std::io::WechatError::new(::std::io::ErrorKind::Other, "wewe")
    }};

    ($code: expr, $msg: expr) => {{
        crate::WechatError::custom($code, $msg)
    }};

    // ($code: expr, $msg: expr, $s: tt) => {{
    //     crate::WechatError::custom_err($code, $msg, $s)
    // }};
}



/// 宏测试
#[test]
fn test_err() {
    use crate::WechatResult;

    fn te() -> WechatResult<String> {
        if false {
            Ok("".to_string())
        } else {
            Err(error!("没有给定必要的参数"))
        }
    }
    // let er = error!(4001, "test");
    // let x = error!("没有给定必要的参数");
    // let err = std::io::Error::new(std::io::ErrorKind::Other, "wewe");
    // let y = error!(4002, "没有给定必要的参数", err);
    // println!("{:?},     {:?} , {:?}", er, x, y);
    println!("{:?}", te());

    let t = error! {
        code:-3,
        msg:format!("Json decode error: {}", "e"),
    };
    let x = error! {
        code:-3,
        msg: format!("Json decode error: {}", "e")
    };

    println!("{:?} {:?}", t, x);
    assert_eq!(1 + 1, 1);
}
