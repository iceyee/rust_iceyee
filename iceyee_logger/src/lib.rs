// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 日志.

// Use.

use iceyee_timer::Timer;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::sync::Mutex;

/// $HOME
pub static mut HOME: Option<String> = None;

/// $HOME/.iceyee_log
pub static mut DEFAULT: Option<String> = None;

#[allow(dead_code)]
#[ctor::ctor]
fn init() {
    #[cfg(target_os = "linux")]
    unsafe {
        DEFAULT = Some(std::env::var("HOME").unwrap() + "/.iceyee_log");
        HOME = Some(std::env::var("HOME").unwrap());
    }
    #[cfg(target_os = "windows")]
    unsafe {
        DEFAULT = Some(std::env::var("USERPROFILE").unwrap() + "\\.iceyee_log");
        HOME = Some(std::env::var("USERPROFILE").unwrap());
    }
    return;
}

// Enum.

/// 日志等级, 从低到高分别是(
/// [Debug](Level::Debug),
/// [Info](Level::Info),
/// [Warn](Level::Warn),
/// [Error](Level::Error),
/// ), 默认[Debug](Level::Debug).
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Level {
    #[default]
    Debug,
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
            _ => panic!("@value={}", value),
        }
    }
}

impl Into<usize> for Level {
    fn into(self) -> usize {
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
#[derive(Debug)]
pub struct Logger {
    timer: Timer,
    time: Arc<AtomicPtr<String>>,
    level: Level,
    warn_file: Option<Arc<Mutex<File>>>,
    error_file: Option<Arc<Mutex<File>>>,
}

impl Logger {
    /// new.
    ///
    /// project_name和target_directory同时不为空, 才是有效的.
    /// 否则只会把信息输出到stdout, 而不会保存在文件中.
    ///
    /// - @param level 默认Debug.
    /// - @param project_name 项目名.
    /// - @param target_directory 把日志保存在哪一个目录.
    pub async fn new(
        level: Option<Level>,
        project_name: Option<&str>,
        target_directory: Option<&str>,
    ) -> Self {
        let mut timer = Timer::new();
        static mut EMPTY_STRING: String = String::new();
        let time: Arc<AtomicPtr<String>> = Arc::new(AtomicPtr::new(unsafe { &mut EMPTY_STRING }));
        // 更新时间.
        Self::update_time(time.clone()).await;
        let time_clone = time.clone();
        timer.schedule_execute_before(0, 10, move || Self::update_time(time_clone.clone()));
        let level: Level = level.unwrap_or(Level::Debug);
        let (warn_file, error_file) = if !project_name.is_none() && !target_directory.is_none() {
            let project_name: &str = project_name.unwrap();
            let target_directory: &str = target_directory.unwrap();
            if project_name.len() == 0 || target_directory.len() == 0 {
                panic!("project_name或target_directory不能长度为0.");
            }
            let (warn_file, error_file) = Self::create(project_name, target_directory).await;
            let (warn_file, error_file) = (
                Arc::new(Mutex::new(warn_file)),
                Arc::new(Mutex::new(error_file)),
            );
            // 更新缓存.
            let (warn_file_clone, error_file_clone) = (warn_file.clone(), error_file.clone());
            timer.schedule_execute_before(60_000, 60_000, move || {
                Self::flush(warn_file_clone.clone(), error_file_clone.clone())
            });
            // 文件管理.
            let project_name_clone: String = project_name.to_string();
            let target_directory_clone: String = target_directory.to_string();
            let (warn_file_clone, error_file_clone) = (warn_file.clone(), error_file.clone());
            timer
                .schedule_pattern("50 59 23 * * *", move || {
                    Self::manage(
                        project_name_clone.clone(),
                        target_directory_clone.clone(),
                        warn_file_clone.clone(),
                        error_file_clone.clone(),
                    )
                })
                .unwrap();
            (Some(warn_file), Some(error_file))
        } else {
            (None, None)
        };
        return Self {
            timer: timer,
            time: time,
            level: level,
            warn_file: warn_file,
            error_file: error_file,
        };
    }

    pub async fn stop(&mut self) {
        self.timer.stop().await;
        return;
    }

    // 更新时间.
    async fn update_time(time: Arc<AtomicPtr<String>>) {
        use iceyee_datetime::DateTime;
        static mut SWITCH: usize = 0;
        static mut TIMES: [String; 2] = [String::new(), String::new()];
        unsafe {
            SWITCH = (SWITCH + 1) % 2;
            TIMES[SWITCH] = DateTime::new().to_string();
            time.store(&mut TIMES[SWITCH], Ordering::SeqCst);
            return;
        }
    }

    // 创建目录文件.
    async fn create(project_name: &str, target_directory: &str) -> (File, File) {
        let path: String = target_directory.to_string() + "/" + project_name;
        if let Err(_) = tokio::fs::create_dir_all(&path).await {}
        let warn_file: String = path.clone() + "/" + project_name + "_warn.log";
        let error_file: String = path.clone() + "/" + project_name + "_error.log";
        let warn_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(warn_file)
            .await
            .unwrap();
        let error_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(error_file)
            .await
            .unwrap();
        return (warn_file, error_file);
    }

    // 刷新缓存.
    async fn flush(warn_file: Arc<Mutex<File>>, error_file: Arc<Mutex<File>>) {
        use tokio::io::AsyncWriteExt;
        warn_file.lock().await.flush().await.unwrap();
        error_file.lock().await.flush().await.unwrap();
        return;
    }

    // 文件管理.
    async fn manage(
        project_name: String,
        target_directory: String,
        warn_file: Arc<Mutex<File>>,
        error_file: Arc<Mutex<File>>,
    ) {
        use iceyee_datetime::DateTime;
        use std::time::SystemTime;
        use tokio::io::AsyncWriteExt;
        let path: String = target_directory.to_string() + "/" + project_name.as_str();
        let mut dirs = tokio::fs::read_dir(&path).await.unwrap();
        // 删除.
        while let Ok(Some(entry)) = dirs.next_entry().await {
            let t = entry.metadata().await.unwrap().modified().unwrap();
            if 1 * 60 * 60 * 24 * 45 < SystemTime::now().duration_since(t).unwrap().as_secs() {
                tokio::fs::remove_file(entry.path().as_path())
                    .await
                    .unwrap();
            }
        }
        // 刷新缓存.
        warn_file.lock().await.flush().await.unwrap();
        error_file.lock().await.flush().await.unwrap();
        // 重命名.
        let datetime: DateTime = DateTime::new();
        let date = format!(
            "{}_{:02}_{:02}",
            datetime.year, datetime.month, datetime.day
        );
        let warn_file_from: String = path.clone() + "/" + project_name.as_str() + "_warn.log";
        let error_file_from: String = path.clone() + "/" + project_name.as_str() + "_error.log";
        let warn_file_to: String =
            path.clone() + "/" + project_name.as_str() + "_warn_" + date.as_str() + ".log";
        let error_file_to: String =
            path.clone() + "/" + project_name.as_str() + "_error_" + date.as_str() + ".log";
        // println!("{warn_file_from}");
        // println!("{warn_file_to}");
        // println!("{error_file_from}");
        // println!("{error_file_to}");
        tokio::fs::rename(&warn_file_from, &warn_file_to)
            .await
            .unwrap();
        tokio::fs::rename(&error_file_from, &error_file_to)
            .await
            .unwrap();
        *warn_file.lock().await = OpenOptions::new()
            .create(true)
            .write(true)
            .open(warn_file_from)
            .await
            .unwrap();
        *error_file.lock().await = OpenOptions::new()
            .create(true)
            .write(true)
            .open(error_file_from)
            .await
            .unwrap();
        return;
    }
}

impl Logger {
    async fn print(&mut self, message: &str, level: Level) {
        use tokio::io::AsyncWriteExt;
        use tokio::io::Stdout;
        static STDOUT: Mutex<Option<Stdout>> = Mutex::const_new(None);
        let mut stdout = STDOUT.lock().await;
        if stdout.is_none() {
            *stdout = Some(tokio::io::stdout());
        }
        let a: usize = level.clone().into();
        let b: usize = self.level.clone().into();
        if a < b {
            return;
        }
        let time: String = unsafe { (*self.time.load(Ordering::SeqCst)).clone() };
        match level {
            Level::Debug | Level::Info => {
                let message: String = format!(
                    "\n{} {} # {}\n",
                    time,
                    level.to_string(),
                    message.replace("\n", "\n    ")
                );
                stdout
                    .as_mut()
                    .unwrap()
                    .write_all(message.as_bytes())
                    .await
                    .unwrap();
                drop(stdout);
            }
            Level::Warn => {
                let message: String = format!(
                    "\n{} {} # {}\n",
                    time,
                    level.to_string(),
                    message.replace("\n", "\n    ")
                );
                stdout
                    .as_mut()
                    .unwrap()
                    .write_all(message.as_bytes())
                    .await
                    .unwrap();
                drop(stdout);
                if self.warn_file.is_some() {
                    self.warn_file
                        .as_mut()
                        .unwrap()
                        .lock()
                        .await
                        .write_all(message.as_bytes())
                        .await
                        .unwrap();
                }
            }
            Level::Error => {
                let message: String = format!(
                    "\n{} {} # \n    {}\n",
                    time,
                    level.to_string(),
                    message.replace("\n", "\n    ")
                );
                stdout
                    .as_mut()
                    .unwrap()
                    .write_all(message.as_bytes())
                    .await
                    .unwrap();
                drop(stdout);
                if self.error_file.is_some() {
                    self.error_file
                        .as_mut()
                        .unwrap()
                        .lock()
                        .await
                        .write_all(message.as_bytes())
                        .await
                        .unwrap();
                }
            }
        }
        return;
    }

    /// Debug.
    pub async fn debug(&mut self, message: &str) {
        Self::print(self, message, Level::Debug).await;
        return;
    }

    /// Info.
    pub async fn info(&mut self, message: &str) {
        Self::print(self, message, Level::Info).await;
        return;
    }

    /// Warn.
    pub async fn warn(&mut self, message: &str) {
        Self::print(self, message, Level::Warn).await;
        return;
    }

    /// Error.
    pub async fn error(&mut self, message: &str) {
        Self::print(self, message, Level::Error).await;
        return;
    }
}

// Function.
