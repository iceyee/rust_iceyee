// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 日志.
//!
//! [debug], [info], [warn], [error], 这四个要求参数比须实现[ToString].
//!
//! [debug_object], [info_object], [warn_object], [error_object],
//!     这四个要求参数比须实现[Debug].
//!
//! # Example
//! ```
//! iceyee_logger::debug!(0, "hello world debug.", "second", "third", "fourth");
//! iceyee_logger::info!(1, "hello world debug.", "second", "third", "fourth");
//! iceyee_logger::warn!(2, "hello world debug.", "second", "third", "fourth");
//! iceyee_logger::error!(3, "hello world debug.", "second", "third", "fourth");
//! ```
//!
//! # Output
//! ```text
//! 2024-09-29T12:12:44.917+08:00 DEBUG # 0 hello world debug. second third fourth
//!
//! 2024-09-29T12:12:44.917+08:00 INFO  # 1 hello world debug. second third fourth
//!
//! 2024-09-29T12:12:44.917+08:00 WARN  # 2 hello world debug. second third fourth
//!
//! 2024-09-29T12:12:44.917+08:00 ERROR #
//!     iceyee_logger/tests/test_logger.rs:59:5 test_logger #
//!     3 hello world debug. second third fourth
//! ```
//!
//! - @see [iceyee_time](../iceyee_time/index.html)
//! - @see [tokio](../tokio/index.html)

// Use.

use iceyee_time::DateTime;
use iceyee_time::Schedule1;
use iceyee_time::Schedule2;
use iceyee_time::Schedule3;
use iceyee_time::Schedule4;
use iceyee_time::Timer;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::io::Stdout;
use tokio::sync::Mutex as TokioMutex;

lazy_static::lazy_static! {
static ref LOGGER: Logger = Logger {
    timer: Timer::new(),
    time: Arc::new(TokioMutex::new(DateTime::new().to_string())),
    level: Arc::new(AtomicUsize::new(Level::default().to_usize())),
    project_name: Arc::new(TokioMutex::new(None)),
    target_directory: Arc::new(TokioMutex::new(None)),
    warn_file: Arc::new(TokioMutex::new(None)),
    error_file: Arc::new(TokioMutex::new(None)),
};
}

// Enum.

/// 日志等级.
///
/// 从低到高分别是
/// [Debug](Level::Debug),
/// [Info](Level::Info),
/// [Warn](Level::Warn),
/// [Error](Level::Error)
/// .
/// 默认[Info](Level::Info).
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Level {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<usize> for Level {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Debug,
            1 => Self::Info,
            2 => Self::Warn,
            3 => Self::Error,
            _ => Self::Debug,
        }
    }
}

impl Level {
    fn to_usize(&self) -> usize {
        return match self {
            Self::Debug => 0,
            Self::Info => 1,
            Self::Warn => 2,
            Self::Error => 3,
        };
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x: usize = self.clone().to_usize();
        let y: usize = other.clone().to_usize();
        return x.partial_cmp(&y);
    }
}

impl ToString for Level {
    fn to_string(&self) -> String {
        return match self {
            Self::Debug => "DEBUG".to_string(),
            Self::Info => "INFO ".to_string(),
            Self::Warn => "WARN ".to_string(),
            Self::Error => "ERROR".to_string(),
        };
    }
}

impl Level {
    pub fn to_string_with_color(&self) -> String {
        return match self {
            Self::Debug => "\x1b[37mDEBUG\x1b[0m".to_string(),
            Self::Info => "\x1b[34mINFO \x1b[0m".to_string(),
            Self::Warn => "\x1b[33mWARN \x1b[0m".to_string(),
            Self::Error => "\x1b[31mERROR\x1b[0m".to_string(),
        };
    }
}

// Trait.

// Struct.

/// 日志.
#[derive(Clone)]
pub struct Logger {
    timer: Timer,
    time: Arc<TokioMutex<String>>,
    level: Arc<AtomicUsize>,
    project_name: Arc<TokioMutex<Option<String>>>,
    target_directory: Arc<TokioMutex<Option<String>>>,
    warn_file: Arc<TokioMutex<Option<File>>>,
    error_file: Arc<TokioMutex<Option<File>>>,
}

/// 更新时间.

impl Schedule1 for Logger {
    fn sleep_after_perform1(&self) -> u64 {
        100
    }

    fn perform1<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            *LOGGER.time.lock().await = DateTime::new().to_string();
            return true;
        });
    }
}

/// 文件重命名.
impl Schedule2 for Logger {
    fn schedule_by_pattern2(&self) -> String {
        "01 00 00 * * *".to_string()
    }

    fn perform2<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let project_name: Option<String> = LOGGER.project_name.lock().await.clone();
            if project_name.is_none() {
                return true;
            }
            let project_name: String = project_name.as_ref().expect("NEVER").clone();
            let target_directory: String = LOGGER
                .target_directory
                .lock()
                .await
                .clone()
                .unwrap_or_else(|| default_target());
            let path: String = target_directory.clone() + "/" + &project_name;
            // 刷新缓存, 然后关闭文件.
            let mut warn_file = LOGGER.warn_file.lock().await;
            if warn_file.is_some() {
                warn_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush");
            }
            *warn_file = None;
            let mut error_file = LOGGER.error_file.lock().await;
            if error_file.is_some() {
                error_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush");
            }
            *error_file = None;
            // 重命名.
            let t: i64 = iceyee_time::now() - 1_000 * 60 * 60 * 1;
            let datetime: DateTime = DateTime::from((t, None));
            let date: String = format!(
                "_{}_{:02}_{:02}",
                datetime.year, datetime.month, datetime.day
            );
            let warn_file_from: String = path.clone() + "/" + &project_name + "_warn.log";
            let error_file_from: String = path.clone() + "/" + &project_name + "_error.log";
            let warn_file_to: String = path.clone() + "/" + &project_name + &date + "_warn.log";
            let error_file_to: String = path.clone() + "/" + &project_name + &date + "_error.log";
            tokio::fs::rename(&warn_file_from, &warn_file_to)
                .await
                .expect("fs::rename");
            tokio::fs::rename(&error_file_from, &error_file_to)
                .await
                .expect("fs::rename");
            *warn_file = Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(warn_file_from)
                    .await
                    .expect("File::open"),
            );
            *error_file = Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(error_file_from)
                    .await
                    .expect("File::open"),
            );
            return true;
        });
    }
}

/// 删除两个月前的文件.
impl Schedule3 for Logger {
    fn schedule_by_pattern3(&self) -> String {
        "01 01 00 * * *".to_string()
    }

    fn perform3<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let project_name: Option<String> = LOGGER.project_name.lock().await.clone();
            if project_name.is_none() {
                return true;
            }
            let project_name: String = project_name.as_ref().expect("NEVER").clone();
            let target_directory: String = LOGGER
                .target_directory
                .lock()
                .await
                .clone()
                .unwrap_or_else(|| default_target());
            let path: String = target_directory.clone() + "/" + &project_name;
            let mut dirs = tokio::fs::read_dir(&path).await.expect("fs::read_dir");
            // 删除两个月前的文件.
            while let Ok(Some(entry)) = dirs.next_entry().await {
                let t: SystemTime = entry
                    .metadata()
                    .await
                    .map(|x| x.modified())
                    .expect("Entry::metadata::modified")
                    .expect("Entry::metadata::modified");
                if 1 * 60 * 60 * 24 * 60
                    < SystemTime::now()
                        .duration_since(t)
                        .expect("time::duration_since")
                        .as_secs()
                {
                    tokio::fs::remove_file(entry.path().as_path())
                        .await
                        .expect("fs::remove_file");
                }
            }
            return true;
        });
    }
}

/// 刷新缓存.
impl Schedule4 for Logger {
    fn schedule_by_pattern4(&self) -> String {
        "00 * * * * *".to_string()
    }

    fn perform4<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let mut warn_file = LOGGER.warn_file.lock().await;
            if warn_file.is_some() {
                warn_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush");
            }
            drop(warn_file);
            let mut error_file = LOGGER.error_file.lock().await;
            if error_file.is_some() {
                error_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush");
            }
            drop(error_file);
            return true;
        });
    }

    fn finish4<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let mut warn_file = LOGGER.warn_file.lock().await;
            if warn_file.is_some() {
                warn_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush");
            }
            drop(warn_file);
            let mut error_file = LOGGER.error_file.lock().await;
            if error_file.is_some() {
                error_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush");
            }
            drop(error_file);
            return;
        });
    }
}

// pub struct Logger {
//     timer: Timer,
//     time: TokioMutex<String>,
//     level: AtomicUsize,
//     project_name: TokioMutex<Option<String>>,
//     target_directory: TokioMutex<Option<String>>,
//     warn_file: TokioMutex<Option<File>>,
//     error_file: TokioMutex<Option<File>>,
// }

impl Logger {
    pub async fn init(
        level: Option<Level>,
        project_name: Option<&str>,
        target_directory: Option<&str>,
    ) {
        LOGGER.timer.stop_and_wait().await;
        LOGGER.timer.start().await;
        let level = level.unwrap_or_else(|| Level::default());
        LOGGER.level.store(level.to_usize(), SeqCst);
        *LOGGER.project_name.lock().await = project_name.clone().and_then(|x| Some(x.to_string()));
        *LOGGER.target_directory.lock().await =
            target_directory.clone().and_then(|x| Some(x.to_string()));
        *LOGGER.warn_file.lock().await = None;
        *LOGGER.error_file.lock().await = None;
        // 打开文件.
        {
            let (warn_file, error_file) = Self::create_file().await;
            *LOGGER.warn_file.lock().await = warn_file;
            *LOGGER.error_file.lock().await = error_file;
        }
        // 更新时间.
        LOGGER.timer.schedule1(LOGGER.clone().wrap1()).await;
        // 重命名.
        LOGGER.timer.schedule2(LOGGER.clone().wrap2()).await;
        // 删除两个月前的文件.
        LOGGER.timer.schedule3(LOGGER.clone().wrap3()).await;
        // 更新缓存.
        LOGGER.timer.schedule4(LOGGER.clone().wrap4()).await;
        return;
    }

    // 创建目录文件.
    async fn create_file() -> (Option<File>, Option<File>) {
        if LOGGER.project_name.lock().await.is_none() {
            return (None, None);
        }
        let project_name: String = LOGGER
            .project_name
            .lock()
            .await
            .as_ref()
            .expect("NEVER")
            .clone();
        let target_directory: String = LOGGER
            .target_directory
            .lock()
            .await
            .clone()
            .unwrap_or_else(|| default_target());
        let path: String = target_directory.clone() + "/" + &project_name;
        let _ = tokio::fs::create_dir_all(&path).await;
        let warn_file: String = path.clone() + "/" + &project_name + "_warn.log";
        let error_file: String = path.clone() + "/" + &project_name + "_error.log";
        let warn_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(warn_file)
            .await
            .expect("File::open");
        let error_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(error_file)
            .await
            .expect("File::open");
        return (Some(warn_file), Some(error_file));
    }

    pub async fn print(&self, level: Level, message: &str) {
        if level.to_usize() < self.level.load(SeqCst) {
            return;
        }
        static STDOUT: TokioMutex<Option<Stdout>> = TokioMutex::const_new(None);
        let mut stdout = STDOUT.lock().await;
        if stdout.is_none() {
            *stdout = Some(tokio::io::stdout());
        }
        let time: String = self.time.lock().await.clone();
        let message: String = message.to_string().replace("\n", "\n    ");
        match level {
            Level::Debug | Level::Info => {
                let message_a: String = format!(
                    "\n{} {} # {}\n",
                    time,
                    level.to_string_with_color(),
                    message
                );
                // let message_b: String = format!("\n{} {} # {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("NEVER")
                    .write_all(message_a.as_bytes())
                    .await
                    .expect("Stdout::write");
                drop(stdout);
            }
            Level::Warn => {
                let message_a: String = format!(
                    "\n{} {} # {}\n",
                    time,
                    level.to_string_with_color(),
                    message
                );
                let message_b: String = format!("\n{} {} # {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("NEVER")
                    .write_all(message_a.as_bytes())
                    .await
                    .expect("Stdout::write");
                drop(stdout);
                let mut warn_file = self.warn_file.lock().await;
                if warn_file.is_some() {
                    warn_file
                        .as_mut()
                        .expect("NEVER")
                        .write_all(message_b.as_bytes())
                        .await
                        .expect("File::write");
                }
                drop(warn_file);
            }
            Level::Error => {
                let message_a: String = format!(
                    "\n{} {} # \n    {}\n",
                    time,
                    level.to_string_with_color(),
                    message
                );
                let message_b: String =
                    format!("\n{} {} # \n    {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("NEVER")
                    .write_all(message_a.as_bytes())
                    .await
                    .expect("File::write");
                drop(stdout);
                let mut error_file = self.error_file.lock().await;
                if error_file.is_some() {
                    error_file
                        .as_mut()
                        .expect("NEVER")
                        .write_all(message_b.as_bytes())
                        .await
                        .expect("File::write");
                }
                drop(error_file);
            }
        }
        return;
    }
}

// Function.

/// 日志的默认路径.
pub fn default_target() -> String {
    return home_dir() + "/.iceyee_log";
}

/// 用户主目录.
pub fn home_dir() -> String {
    #[cfg(target_os = "linux")]
    {
        return std::env::var("HOME").expect("std::env::var('HOME')");
    }
    #[cfg(target_os = "windows")]
    {
        return std::env::var("USERPROFILE").expect("std::env::var('USERPROFILE')");
    }
}

/// 初始化.
pub async fn init(
    level: Option<Level>,
    project_name: Option<&str>,
    target_directory: Option<&str>,
) {
    Logger::init(level, project_name, target_directory).await;
    return;
}

/// 输出日志.
pub async fn print(level: Level, message: &str) {
    LOGGER.print(level, message).await;
    return;
}

#[macro_export]
macro_rules! debug {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::with_capacity(0xFF);
            $(
                let x: String = $x.to_string();
                message.push_str(&x);
                if !message.ends_with("\n") {
                    message.push_str(" ");
                }
            )*
            iceyee_logger::print(iceyee_logger::Level::Debug, &message).await;
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::with_capacity(0xFF);
            $(
                let x: String = $x.to_string();
                message.push_str(&x);
                if !message.ends_with("\n") {
                    message.push_str(" ");
                }
            )*
            iceyee_logger::print(iceyee_logger::Level::Info, &message).await;
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::with_capacity(0xFF);
            $(
                let x: String = $x.to_string();
                message.push_str(&x);
                if !message.ends_with("\n") {
                    message.push_str(" ");
                }
            )*
            iceyee_logger::print(iceyee_logger::Level::Warn, &message).await;
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::with_capacity(0xFF);
            message.push_str(&format!("{}:{}:{} {} # \n", file!(), line!(), column!(), module_path!()));
            $(
                let x: String = $x.to_string();
                message.push_str(&x);
                if !message.ends_with("\n") {
                    message.push_str(" ");
                }
            )*
            iceyee_logger::print(iceyee_logger::Level::Error, &message).await;
        }
    };
}

#[macro_export]
macro_rules! debug_object {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::with_capacity(0xFF);
            $(
                message.push_str(&format!("\n{:?}", $x));
            )*
            iceyee_logger::print(iceyee_logger::Level::Debug, &message).await;
        }
    };
}

#[macro_export]
macro_rules! info_object {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::with_capacity(0xFF);
            $(
                message.push_str(&format!("\n{:?}", $x));
            )*
            iceyee_logger::print(iceyee_logger::Level::Info, &message).await;
        }
    };
}

#[macro_export]
macro_rules! warn_object {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::with_capacity(0xFF);
            $(
                message.push_str(&format!("\n{:?}", $x));
            )*
            iceyee_logger::print(iceyee_logger::Level::Warn, &message).await;
        }
    };
}

#[macro_export]
macro_rules! error_object {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::with_capacity(0xFF);
            message.push_str(&format!("{}:{}:{} {} | ", file!(), line!(), column!(), module_path!()));
            $(
                message.push_str(&format!("\n{:?}", $x));
            )*
            iceyee_logger::print(iceyee_logger::Level::Error, &message).await;
        }
    };
}
