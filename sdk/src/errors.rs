//! copyright © ecdata.cn 2020 - present
//! 自定义错误信息处理
//! created by shaipe

use chrono::{DateTime, Local};
use std::convert::Into;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    Msg(String),
    Io(::std::io::Error),
    Custom { code: i32, msg: String },
}

/// The WechatError type
#[derive(Debug)]
pub struct WechatError {
    /// Kind of error
    pub kind: ErrorKind,
    pub source: Option<Box<dyn StdError>>,
}
unsafe impl Sync for WechatError {}
unsafe impl Send for WechatError {}

/// 继承标准接口
impl StdError for WechatError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        let source = self.source.as_ref().map(|c| &**c);
        if source.is_none() {
            match self.kind {
                // ErrorKind::Custom(ref err) => source = err.source(),
                _ => (),
            };
        }

        source
    }
}

/// 格式化显示设置
impl fmt::Display for WechatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Msg(ref message) => write!(f, "{}", message),
            ErrorKind::Custom { code, ref msg } => {
                write!(f, "custom error code: {}, message: {}", code, msg)
            }
            ErrorKind::Io(ref e) => write!(f, "{}", e),
        }
    }
}

impl WechatError {
    /// Creates generic error
    pub fn msg(value: impl ToString) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: None,
        }
    }

    /// Creates generic error with a cause
    pub fn chain(value: impl ToString, source: impl Into<Box<dyn StdError>>) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: Some(source.into()),
        }
    }

    /// 自定义错误
    pub fn custom(code: i32, msg: impl ToString) -> Self {
        Self {
            kind: ErrorKind::Custom {
                code,
                msg: msg.to_string(),
            },
            source: None,
        }
    }

    /// 自定义错误
    pub fn custom_err(code: i32, msg: impl ToString, source: impl Into<Box<dyn StdError>>) -> Self {
        Self {
            kind: ErrorKind::Custom {
                code,
                msg: msg.to_string(),
            },
            source: Some(source.into()),
        }
    }

    /// 写入文件到日志
    // #[allow(dead_code)]
    pub fn write_to_file(content: String) {
        use std::fs::OpenOptions;
        use std::io::prelude::*;
        let mut name = String::from("logs");
        // 获取当前应用名称
        if let Ok(p) = std::env::current_exe() {
            let s = p.file_name().unwrap().to_str().unwrap();
            name = String::from(s);
        }

        let mut file_path = format!("log/{}.log", name);
        let mut dir_path_str = "log/".to_owned();
        // 只有非调试模式下才使用下面的配置
        if !cfg!(debug_assertions) {
            // 给定了相对顶层路径时不处理
            if !file_path.starts_with("/") {
                if let Ok(p) = std::env::current_exe() {
                    let workdir = format!("{}", p.parent().unwrap().display());
                    file_path = format!("{}/{}", &workdir, file_path.replace("./", ""));
                    dir_path_str = format!("{}/{}", &workdir, dir_path_str.replace("./",""));
                }
            }
        }

        let dir_path = std::path::Path::new(&dir_path_str);
        if !dir_path.exists() {
            let _ = std::fs::create_dir_all(dir_path);
        }
        // println!("sdsdds{}", file_path);
        // 以读,写,创建,追加的方式打开文件
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(file_path);

        let local: DateTime<Local> = Local::now();
        let now_str = local.format("%Y-%m-%d %H:%M:%S").to_string();

        // 向文件中写入内容
        match file {
            Ok(mut stream) => {
                stream
                    .write_all(format!("[{}] {}\n", now_str, content).as_bytes())
                    .unwrap();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}

impl From<&str> for WechatError {
    fn from(e: &str) -> Self {
        Self::msg(e)
    }
}
impl From<String> for WechatError {
    fn from(e: String) -> Self {
        Self::msg(e)
    }
}
impl From<::std::io::Error> for WechatError {
    fn from(e: ::std::io::Error) -> Self {
        Self {
            kind: ErrorKind::Io(e),
            source: None,
        }
    }
}
