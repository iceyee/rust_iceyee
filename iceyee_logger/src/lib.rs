// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

#![feature(get_mut_unchecked)]

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
//! - @see [iceyee_time](../iceyee_time/index.html)
//! - @see [tokio](../tokio/index.html)

// Use.

use iceyee_time::DateTime;
use iceyee_time::Schedule;
use iceyee_time::Timer;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::io::Stdout;
use tokio::sync::Mutex as TokioMutex;

static LOGGER: TokioMutex<Option<Arc<Logger>>> = TokioMutex::const_new(None);

// Enum.

/// 日志等级, 从低到高分别是
/// [Debug](Level::Debug),
/// [Info](Level::Info),
/// [Warn](Level::Warn),
/// [Error](Level::Error)
/// .
/// 默认[Info](Level::Info).
#[derive(Clone, Debug, Default)]
pub enum Level {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<u64> for Level {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Debug,
            1 => Self::Info,
            2 => Self::Warn,
            3 => Self::Error,
            _ => Self::Debug,
        }
    }
}

impl Into<u64> for Level {
    fn into(self) -> u64 {
        return match self {
            Self::Debug => 0,
            Self::Info => 1,
            Self::Warn => 2,
            Self::Error => 3,
        };
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

// Trait.

// Struct.

/// 日志.
// #[allow(dead_code)]
pub struct Logger {
    timer: Timer,
    time: TokioMutex<String>,
    level: Level,
    project_name: Option<String>,
    target_directory: Option<String>,
    warn_file: TokioMutex<Option<File>>,
    error_file: TokioMutex<Option<File>>,
}

/// 更新时间.
struct ScheduleUpdateTime(Arc<Logger>);

impl Schedule for ScheduleUpdateTime {
    fn sleep_after_perform(&self) -> u64 {
        1
    }

    fn perform<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            *self.0.time.lock().await = DateTime::new().to_string();
            return true;
        });
    }
}

/// 文件重命名.
struct ScheduleRename(Arc<Logger>);

impl Schedule for ScheduleRename {
    fn schedule_by_pattern(&self) -> String {
        "01 00 00 * * *".to_string()
    }

    fn perform<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            if self.0.project_name.is_none()
                || self.0.project_name.as_ref().expect("NEVER").trim().len() == 0
            {
                return true;
            }
            let project_name: String = self
                .0
                .project_name
                .as_ref()
                .expect("NEVER")
                .trim()
                .to_string();
            let target_directory: String = match &self.0.target_directory {
                Some(target_directory) => {
                    if target_directory.trim().len() != 0 {
                        target_directory.trim().to_string()
                    } else {
                        default_target()
                    }
                }
                None => default_target(),
            };
            let path: String = target_directory.clone() + "/" + &project_name;
            // 刷新缓存, 然后关闭文件.
            let mut warn_file = self.0.warn_file.lock().await;
            if warn_file.is_some() {
                warn_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush()");
            }
            *warn_file = None;
            let mut error_file = self.0.error_file.lock().await;
            if error_file.is_some() {
                error_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush()");
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
            let warn_file_to: String =
                path.clone() + "/" + &project_name + "_warn" + &date + ".log";
            let error_file_to: String =
                path.clone() + "/" + &project_name + "_error" + &date + ".log";
            tokio::fs::rename(&warn_file_from, &warn_file_to)
                .await
                .expect("fs::rename()");
            tokio::fs::rename(&error_file_from, &error_file_to)
                .await
                .expect("fs::rename()");
            *warn_file = Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(warn_file_from)
                    .await
                    .expect("File::open()"),
            );
            *error_file = Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(error_file_from)
                    .await
                    .expect("File::open()"),
            );
            return true;
        });
    }
}

/// 删除两个月前的文件.
struct ScheduleDeleteOld(Arc<Logger>);

impl Schedule for ScheduleDeleteOld {
    fn schedule_by_pattern(&self) -> String {
        "01 01 00 * * *".to_string()
    }

    fn perform<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            if self.0.project_name.is_none()
                || self.0.project_name.as_ref().expect("NEVER").trim().len() == 0
            {
                return true;
            }
            let project_name: String = self
                .0
                .project_name
                .as_ref()
                .expect("NEVER")
                .trim()
                .to_string();
            let target_directory: String = match &self.0.target_directory {
                Some(target_directory) => {
                    if target_directory.trim().len() != 0 {
                        target_directory.trim().to_string()
                    } else {
                        default_target()
                    }
                }
                None => default_target(),
            };
            let path: String = target_directory.clone() + "/" + &project_name;
            let mut dirs = tokio::fs::read_dir(&path).await.expect("fs::read_dir()");
            // 删除两个月前的文件.
            while let Ok(Some(entry)) = dirs.next_entry().await {
                let t = entry
                    .metadata()
                    .await
                    .expect("Entry::metadata()")
                    .modified()
                    .expect("metadata().modified()");
                if 1 * 60 * 60 * 24 * 60
                    < SystemTime::now()
                        .duration_since(t)
                        .expect("time::duration_since()")
                        .as_secs()
                {
                    tokio::fs::remove_file(entry.path().as_path())
                        .await
                        .expect("fs::remove_file()");
                }
            }
            return true;
        });
    }
}

/// 刷新缓存.
struct ScheduleFlush(Arc<Logger>);

impl Schedule for ScheduleFlush {
    fn schedule_by_pattern(&self) -> String {
        "00 * * * * *".to_string()
    }

    fn perform<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let mut warn_file = self.0.warn_file.lock().await;
            if warn_file.is_some() {
                warn_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush()");
            }
            drop(warn_file);
            let mut error_file = self.0.error_file.lock().await;
            if error_file.is_some() {
                error_file
                    .as_mut()
                    .expect("NEVER")
                    .flush()
                    .await
                    .expect("File::flush()");
            }
            drop(error_file);
            return true;
        });
    }
}

impl Logger {
    pub async fn new(
        level: Option<Level>,
        project_name: Option<&str>,
        target_directory: Option<&str>,
    ) -> Arc<Self> {
        let level: Level = level.unwrap_or(Level::default());
        let timer = Timer::new();
        let time: TokioMutex<String> = TokioMutex::new(DateTime::new().to_string());
        let this: Logger = Logger {
            timer: timer,
            time: time,
            level: level,
            project_name: project_name.map(|x| x.to_string()),
            target_directory: target_directory.map(|x| x.to_string()),
            warn_file: TokioMutex::new(None),
            error_file: TokioMutex::new(None),
        };
        let this: Arc<Logger> = Arc::new(this);
        // 打开文件.
        {
            let (warn_file, error_file) = Self::create_file(this.clone()).await;
            *this.warn_file.lock().await = warn_file;
            *this.error_file.lock().await = error_file;
        }
        // 更新时间.
        this.timer
            .schedule(ScheduleUpdateTime(this.clone()).wrap())
            .await;
        // 重命名.
        this.timer
            .schedule(ScheduleRename(this.clone()).wrap())
            .await;
        // 删除两个月前的文件.
        this.timer
            .schedule(ScheduleDeleteOld(this.clone()).wrap())
            .await;
        // 更新缓存.
        this.timer
            .schedule(ScheduleFlush(this.clone()).wrap())
            .await;
        return this;
    }

    // 创建目录文件.
    async fn create_file(logger: Arc<Logger>) -> (Option<File>, Option<File>) {
        if logger.project_name.is_none()
            || logger.project_name.as_ref().expect("NEVER").trim().len() == 0
        {
            return (None, None);
        }
        let project_name: String = logger
            .project_name
            .as_ref()
            .expect("NEVER")
            .trim()
            .to_string();
        let target_directory: String = match &logger.target_directory {
            Some(target_directory) => {
                if target_directory.trim().len() != 0 {
                    target_directory.trim().to_string()
                } else {
                    default_target()
                }
            }
            None => default_target(),
        };
        let path: String = target_directory.clone() + "/" + &project_name;
        let _ = tokio::fs::create_dir_all(&path).await;
        let warn_file: String = path.clone() + "/" + &project_name + "_warn.log";
        let error_file: String = path.clone() + "/" + &project_name + "_error.log";
        let warn_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(warn_file)
            .await
            .expect("File::open()");
        let error_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(error_file)
            .await
            .expect("File::open()");
        return (Some(warn_file), Some(error_file));
    }

    pub async fn print(&self, level: Level, message: &str) {
        static STDOUT: TokioMutex<Option<Stdout>> = TokioMutex::const_new(None);
        let mut stdout = STDOUT.lock().await;
        if stdout.is_none() {
            *stdout = Some(tokio::io::stdout());
        }
        let x: u64 = level.clone().into();
        let y: u64 = self.level.clone().into();
        if x < y {
            return;
        }
        // /// 日志.
        // // #[allow(dead_code)]
        // pub struct Logger {
        //     timer: Timer,
        //     time: AtomicPtr<String>,
        //     time_buffer: TokioMutex<[String; 2]>,
        //     time_index: AtomicUsize,
        //     level: Level,
        //     project_name: Option<String>,
        //     target_directory: Option<String>,
        //     warn_file: TokioMutex<Option<File>>,
        //     error_file: TokioMutex<Option<File>>,
        // }
        let time: String = self.time.lock().await.clone();
        let message: String = message.to_string().replace("\n", "\n    ");
        match level {
            Level::Debug | Level::Info => {
                let message: String = format!("\n{} {} # {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("NEVER")
                    .write_all(message.as_bytes())
                    .await
                    .expect("Stdout::write()");
                drop(stdout);
            }
            Level::Warn => {
                let message: String = format!("\n{} {} # {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("NEVER")
                    .write_all(message.as_bytes())
                    .await
                    .expect("Stdout::write()");
                drop(stdout);
                let mut warn_file = self.warn_file.lock().await;
                if warn_file.is_some() {
                    warn_file
                        .as_mut()
                        .expect("NEVER")
                        .write_all(message.as_bytes())
                        .await
                        .expect("File::write()");
                }
                drop(warn_file);
            }
            Level::Error => {
                let message: String =
                    format!("\n{} {} # \n    {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("NEVER")
                    .write_all(message.as_bytes())
                    .await
                    .expect("File::write()");
                drop(stdout);
                let mut error_file = self.error_file.lock().await;
                if error_file.is_some() {
                    error_file
                        .as_mut()
                        .expect("NEVER")
                        .write_all(message.as_bytes())
                        .await
                        .expect("File::write()");
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
    return home() + "/.iceyee_log";
}

/// 用户主目录.
pub fn home() -> String {
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
    let mut logger = LOGGER.lock().await;
    if logger.is_some() {
        logger.as_mut().expect("NEVER").timer.stop().await;
    }
    *logger = Some(Logger::new(level, project_name, target_directory).await);
    return;
}

/// 输出日志.
pub async fn print(level: Level, message: &str) {
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(None, None, None).await);
    }
    logger.as_ref().expect("NEVER").print(level, message).await;
    return;
}

#[macro_export]
macro_rules! debug {
    ($($x:expr),* $(,)?) => {
        {
            let mut message: String = String::new();
            $(
                let x: String = $x.to_string();
                message.push_str(&x);
                if !x.ends_with('\n') {
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
            let mut message: String = String::new();
            $(
                let x: String = $x.to_string();
                message.push_str(&x);
                if !x.ends_with('\n') {
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
            let mut message: String = String::new();
            $(
                let x: String = $x.to_string();
                message.push_str(&x);
                if !x.ends_with('\n') {
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
            let mut message: String = String::new();
            message.push_str(&format!("{}:{}:{} {} | \n", file!(), line!(), column!(), module_path!()));
            $(
                let x: String = $x.to_string();
                message.push_str(&x);
                if !x.ends_with('\n') {
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
            let mut message: String = String::new();
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
            let mut message: String = String::new();
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
            let mut message: String = String::new();
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
            let mut message: String = String::new();
            message.push_str(&format!("{}:{}:{} {} | ", file!(), line!(), column!(), module_path!()));
            $(
                message.push_str(&format!("\n{:?}", $x));
            )*
            iceyee_logger::print(iceyee_logger::Level::Error, &message).await;
        }
    };
}
