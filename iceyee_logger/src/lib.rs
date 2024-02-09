// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

#![feature(get_mut_unchecked)]

//! 日志.

// Use.

use iceyee_time::DateTime;
use iceyee_time::Timer;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::io::Stdout;
use tokio::sync::Mutex;

static LOGGER: Mutex<Option<Arc<Logger>>> = Mutex::const_new(None);

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
#[allow(dead_code)]
struct Logger {
    timer: Arc<Timer>,
    time: Arc<AtomicPtr<String>>,
    level: Level,
    project_name: Option<String>,
    target_directory: Option<String>,
    warn_file: Mutex<Option<File>>,
    error_file: Mutex<Option<File>>,
}

impl Logger {
    // 更新时间.
    pub async fn update_time(logger: Arc<Logger>) {
        static mut SWITCH: usize = 0;
        static mut TIMES: [String; 2] = [String::new(), String::new()];
        unsafe {
            SWITCH = (SWITCH + 1) % 2;
            TIMES[SWITCH] = DateTime::new().to_string();
            logger.time.store(&mut TIMES[SWITCH], SeqCst);
            return;
        }
    }

    // 创建目录文件.
    pub async fn create_file(logger: Arc<Logger>) -> (Option<File>, Option<File>) {
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

    // 文件重命名.
    pub async fn rename(logger: Arc<Logger>) {
        if logger.project_name.is_none()
            || logger.project_name.as_ref().expect("NEVER").trim().len() == 0
        {
            return;
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
        // 刷新缓存, 然后关闭文件.
        let mut warn_file = logger.warn_file.lock().await;
        if warn_file.is_some() {
            warn_file
                .as_mut()
                .expect("NEVER")
                .flush()
                .await
                .expect("File::flush()");
        }
        *warn_file = None;
        // drop(warn_file);
        let mut error_file = logger.error_file.lock().await;
        if error_file.is_some() {
            error_file
                .as_mut()
                .expect("NEVER")
                .flush()
                .await
                .expect("File::flush()");
        }
        *error_file = None;
        // drop(error_file);
        // 重命名.
        let t: i64 = iceyee_time::now() - 1_000 * 60 * 60 * 1;
        let datetime: DateTime = DateTime::from((t, None));
        let date: String = format!(
            "_{}_{:02}_{:02}",
            datetime.year, datetime.month, datetime.day
        );
        let warn_file_from: String = path.clone() + "/" + &project_name + "_warn.log";
        let error_file_from: String = path.clone() + "/" + &project_name + "_error.log";
        let warn_file_to: String = path.clone() + "/" + &project_name + "_warn" + &date + ".log";
        let error_file_to: String = path.clone() + "/" + &project_name + "_error" + &date + ".log";
        // println!("{warn_file_from}");
        // println!("{warn_file_to}");
        // println!("{error_file_from}");
        // println!("{error_file_to}");
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
        return;
    }

    // 删除两个月前的文件.
    pub async fn delete_old(logger: Arc<Logger>) {
        if logger.project_name.is_none()
            || logger.project_name.as_ref().expect("NEVER").trim().len() == 0
        {
            return;
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
                    .expect("iceyee_logger/lib.rs 617")
                    .as_secs()
            {
                tokio::fs::remove_file(entry.path().as_path())
                    .await
                    .expect("fs::remove_file()");
            }
        }
        return;
    }

    // 刷新缓存.
    pub async fn flush(logger: Arc<Logger>) {
        let mut warn_file = logger.warn_file.lock().await;
        if warn_file.is_some() {
            warn_file
                .as_mut()
                .expect("NEVER")
                .flush()
                .await
                .expect("File::flush()");
        }
        drop(warn_file);
        let mut error_file = logger.error_file.lock().await;
        if error_file.is_some() {
            error_file
                .as_mut()
                .expect("NEVER")
                .flush()
                .await
                .expect("File::flush()");
        }
        drop(error_file);
        return;
    }
}

impl Logger {
    pub async fn new(
        level: Option<Level>,
        project_name: Option<&str>,
        target_directory: Option<&str>,
    ) -> Arc<Self> {
        static mut EMPTY_STRING: String = String::new();
        let level: Level = level.unwrap_or(Level::default());
        let timer = Timer::new();
        let time: Arc<AtomicPtr<String>> = Arc::new(AtomicPtr::new(unsafe { &mut EMPTY_STRING }));
        let this: Logger = Logger {
            timer: Arc::new(timer),
            time: time,
            level: level,
            project_name: project_name.map(|x| x.to_string()),
            target_directory: target_directory.map(|x| x.to_string()),
            warn_file: Mutex::new(None),
            error_file: Mutex::new(None),
        };
        let this: Arc<Logger> = Arc::new(this);
        let mut timer = this.timer.clone();
        let timer = unsafe { Arc::get_mut_unchecked(&mut timer) };
        // 更新时间.
        {
            Self::update_time(this.clone()).await;
            let this_clone = this.clone();
            timer.schedule_execute_before(1, 1, move |_| Self::update_time(this_clone.clone()));
        }
        // 打开文件.
        {
            let (warn_file, error_file) = Self::create_file(this.clone()).await;
            *this.warn_file.lock().await = warn_file;
            *this.error_file.lock().await = error_file;
        }
        // 重命名.
        {
            let this_clone = this.clone();
            timer.schedule_pattern("57 59 23 * * *", move |_| Self::rename(this_clone.clone()));
        }
        // 删除两个月前的文件.
        {
            let this_clone = this.clone();
            timer.schedule_pattern("00 00 00 * * *", move |_| {
                Self::delete_old(this_clone.clone())
            });
        }
        // 更新缓存.
        {
            let this_clone = this.clone();
            timer.schedule_execute_before(0, 60_000, move |_| Self::flush(this_clone.clone()));
        }
        return this;
    }

    pub async fn print(&self, message: &str, level: Level) {
        static STDOUT: Mutex<Option<Stdout>> = Mutex::const_new(None);
        let mut stdout = STDOUT.lock().await;
        if stdout.is_none() {
            *stdout = Some(tokio::io::stdout());
        }
        let x: u64 = level.clone().into();
        let y: u64 = self.level.clone().into();
        if x < y {
            return;
        }
        let time: String = unsafe { self.time.load(SeqCst).as_ref().expect("NEVER").clone() };
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
    // #[cfg(target_os = "linux")]
    // {
    //     return std::env::var("HOME").expect("std::env::var('HOME')") + "/.iceyee_log";
    // }
    // #[cfg(target_os = "windows")]
    // {
    //     return std::env::var("USERPROFILE").expect("std::env::var('USERPROFILE')")
    //         + "/.iceyee_log";
    // }
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
        unsafe {
            let mut timer: Arc<Timer> = Arc::get_mut_unchecked(logger.as_mut().expect("NEVER"))
                .timer
                .clone();
            let timer = Arc::get_mut_unchecked(&mut timer);
            timer.stop_and_wait().await;
        }
    }
    *logger = Some(Logger::new(level, project_name, target_directory).await);
    return;
}

async fn default_logger() {
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(None, None, None).await);
    }
    return;
}

fn build_message(message: Vec<String>) -> String {
    let mut output: String = String::with_capacity(0xFFF);
    for x in message {
        output.push_str(&x);
        if !x.ends_with('\n') {
            output.push_str(" ");
        }
    }
    return output;
}

/// debug.
pub async fn debug(message: Vec<String>) {
    default_logger().await;
    let message: String = build_message(message);
    return LOGGER
        .lock()
        .await
        .as_ref()
        .expect("NEVER")
        .print(&message, Level::Debug)
        .await;
}

/// info.
pub async fn info(message: Vec<String>) {
    default_logger().await;
    let message: String = build_message(message);
    return LOGGER
        .lock()
        .await
        .as_ref()
        .expect("NEVER")
        .print(&message, Level::Info)
        .await;
}

/// warn.
pub async fn warn(message: Vec<String>) {
    default_logger().await;
    let message: String = build_message(message);
    return LOGGER
        .lock()
        .await
        .as_ref()
        .expect("NEVER")
        .print(&message, Level::Warn)
        .await;
}

/// error.
pub async fn error(message: Vec<String>) {
    default_logger().await;
    let message: String = build_message(message);
    return LOGGER
        .lock()
        .await
        .as_ref()
        .expect("NEVER")
        .print(&message, Level::Error)
        .await;
}

/// debug.
pub async fn debug_object<O>(object: O)
where
    O: std::fmt::Debug,
{
    default_logger().await;
    let message: String = format!("{:?}", object);
    return LOGGER
        .lock()
        .await
        .as_ref()
        .expect("NEVER")
        .print(&message, Level::Debug)
        .await;
}

/// info.
pub async fn info_object<O>(object: O)
where
    O: std::fmt::Debug,
{
    default_logger().await;
    let message: String = format!("{:?}", object);
    return LOGGER
        .lock()
        .await
        .as_ref()
        .expect("NEVER")
        .print(&message, Level::Info)
        .await;
}

/// warn.
pub async fn warn_object<O>(object: O)
where
    O: std::fmt::Debug,
{
    default_logger().await;
    let message: String = format!("{:?}", object);
    return LOGGER
        .lock()
        .await
        .as_ref()
        .expect("NEVER")
        .print(&message, Level::Warn)
        .await;
}

/// error.
pub async fn error_object<O>(object: O)
where
    O: std::fmt::Debug,
{
    default_logger().await;
    let message: String = format!("{:?}", object);
    return LOGGER
        .lock()
        .await
        .as_ref()
        .expect("NEVER")
        .print(&message, Level::Error)
        .await;
}
