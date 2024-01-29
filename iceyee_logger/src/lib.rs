// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

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

static LOGGER: Mutex<Option<Logger>> = Mutex::const_new(None);

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
#[allow(dead_code)]
#[derive(Debug)]
struct Logger {
    timer: Option<Timer>,
    time: Arc<AtomicPtr<String>>,
    level: Level,
    project_name: Option<String>,
    target_directory: Option<String>,
    warn_file: Arc<Mutex<Option<File>>>,
    error_file: Arc<Mutex<Option<File>>>,
}

impl std::clone::Clone for Logger {
    // 克隆不带timer.
    fn clone(&self) -> Self {
        return Logger {
            timer: None,
            time: self.time.clone(),
            level: self.level.clone(),
            project_name: self.project_name.clone(),
            target_directory: self.target_directory.clone(),
            warn_file: self.warn_file.clone(),
            error_file: self.error_file.clone(),
        };
    }
}

impl Logger {
    // 更新时间.
    pub async fn update_time(logger: Logger) {
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
    pub async fn create_file(logger: Logger) -> (Option<File>, Option<File>) {
        if logger.project_name.is_none()
            || logger
                .project_name
                .as_ref()
                .expect("iceyee_logger/lib.rs 833")
                .trim()
                .len()
                == 0
        {
            return (None, None);
        }
        let project_name: String = logger
            .project_name
            .expect("iceyee_logger/lib.rs 041")
            .trim()
            .to_string();
        let target_directory: String = match logger.target_directory {
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
            .expect("iceyee_logger/lib.rs 225");
        let error_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(error_file)
            .await
            .expect("iceyee_logger/lib.rs 473");
        return (Some(warn_file), Some(error_file));
    }

    // 刷新缓存.
    pub async fn flush(logger: Logger) {
        let mut warn_file = logger.warn_file.lock().await;
        if warn_file.is_some() {
            warn_file
                .as_mut()
                .expect("iceyee_logger/lib.rs 481")
                .flush()
                .await
                .expect("iceyee_logger/lib.rs 449");
        }
        drop(warn_file);
        let mut error_file = logger.error_file.lock().await;
        if error_file.is_some() {
            error_file
                .as_mut()
                .expect("iceyee_logger/lib.rs 577")
                .flush()
                .await
                .expect("iceyee_logger/lib.rs 065");
        }
        drop(error_file);
        return;
    }

    // 文件管理.
    pub async fn manage(logger: Logger) {
        if logger.project_name.is_none()
            || logger
                .project_name
                .as_ref()
                .expect("iceyee_logger/lib.rs 113")
                .trim()
                .len()
                == 0
        {
            return;
        }
        let project_name: String = logger
            .project_name
            .expect("iceyee_logger/lib.rs 921")
            .trim()
            .to_string();
        let target_directory: String = match logger.target_directory {
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
        let mut dirs = tokio::fs::read_dir(&path)
            .await
            .expect("iceyee_logger/lib.rs 905");
        // 删除两个月前的文件.
        while let Ok(Some(entry)) = dirs.next_entry().await {
            let t = entry
                .metadata()
                .await
                .expect("iceyee_logger/lib.rs 753")
                .modified()
                .expect("iceyee_logger/lib.rs 361");
            if 1 * 60 * 60 * 24 * 60
                < SystemTime::now()
                    .duration_since(t)
                    .expect("iceyee_logger/lib.rs 929")
                    .as_secs()
            {
                tokio::fs::remove_file(entry.path().as_path())
                    .await
                    .expect("iceyee_logger/lib.rs 657");
            }
        }
        // 刷新缓存, 然后关闭文件.
        let mut warn_file = logger.warn_file.lock().await;
        if warn_file.is_some() {
            warn_file
                .as_mut()
                .expect("iceyee_logger/lib.rs 745")
                .flush()
                .await
                .expect("iceyee_logger/lib.rs 393");
        }
        *warn_file = None;
        // drop(warn_file);
        let mut error_file = logger.error_file.lock().await;
        if error_file.is_some() {
            error_file
                .as_mut()
                .expect("iceyee_logger/lib.rs 177")
                .flush()
                .await
                .expect("iceyee_logger/lib.rs 665");
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
        let warn_file_to: String =
            path.clone() + "/" + &project_name + "_warn" + date.as_str() + ".log";
        let error_file_to: String =
            path.clone() + "/" + &project_name + "_error" + date.as_str() + ".log";
        // println!("{warn_file_from}");
        // println!("{warn_file_to}");
        // println!("{error_file_from}");
        // println!("{error_file_to}");
        tokio::fs::rename(&warn_file_from, &warn_file_to)
            .await
            .expect("iceyee_logger/lib.rs 713");
        tokio::fs::rename(&error_file_from, &error_file_to)
            .await
            .expect("iceyee_logger/lib.rs 521");
        *warn_file = Some(
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(warn_file_from)
                .await
                .expect("iceyee_logger/lib.rs 289"),
        );
        *error_file = Some(
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(error_file_from)
                .await
                .expect("iceyee_logger/lib.rs 217"),
        );
        return;
    }
}

impl Logger {
    pub async fn new(
        level: Level,
        project_name: Option<&str>,
        target_directory: Option<&str>,
    ) -> Self {
        let timer = Timer::new();
        static mut EMPTY_STRING: String = String::new();
        let time: Arc<AtomicPtr<String>> = Arc::new(AtomicPtr::new(unsafe { &mut EMPTY_STRING }));
        let this: Logger = Logger {
            timer: Some(timer),
            time: time,
            level: level,
            project_name: project_name.map(|x| x.to_string()),
            target_directory: target_directory.map(|x| x.to_string()),
            warn_file: Arc::new(Mutex::new(None)),
            error_file: Arc::new(Mutex::new(None)),
        };
        // 更新时间.
        Self::update_time(this.clone()).await;
        let this_clone = this.clone();
        this.timer
            .as_ref()
            .expect("iceyee_logger/lib.rs 505")
            .schedule_execute_before(1, 1, move |_| Self::update_time(this_clone.clone()));
        let (warn_file, error_file) = Self::create_file(this.clone()).await;
        *this.warn_file.lock().await = warn_file;
        *this.error_file.lock().await = error_file;
        // 更新缓存.
        let this_clone = this.clone();
        this.timer
            .as_ref()
            .expect("iceyee_logger/lib.rs 353")
            .schedule_execute_before(0, 60_000, move |_| Self::flush(this_clone.clone()));
        // 文件管理.
        {
            let this_clone = this.clone();
            this.timer
                .as_ref()
                .expect("iceyee_logger/lib.rs 961")
                .schedule_pattern("57 59 23 * * *", move |_| Self::manage(this_clone.clone()));
        }
        // {
        //     let datetime: DateTime = DateTime::from((iceyee_time::now() + 10_000, None));
        //     let pattern: String = format!(
        //         "{} {} {} * * *",
        //         datetime.second, datetime.minute, datetime.hour
        //     );
        //     let this_clone = this.clone();
        //     this.timer
        //         .as_ref()
        //         .expect("iceyee_logger/lib.rs 529")
        //         .schedule_pattern(&pattern, move |_| Self::manage(this_clone.clone()));
        // }
        return this;
    }
}

impl Logger {
    async fn print<S>(&self, message: S, level: Level)
    where
        S: AsRef<str>,
    {
        static STDOUT: Mutex<Option<Stdout>> = Mutex::const_new(None);
        let mut stdout = STDOUT.lock().await;
        if stdout.is_none() {
            *stdout = Some(tokio::io::stdout());
        }
        let x: usize = level.clone().into();
        let y: usize = self.level.clone().into();
        if x < y {
            return;
        }
        // let time: String = unsafe { (*self.time.load(SeqCst)).clone() };
        let time: String = unsafe {
            self.time
                .load(SeqCst)
                .as_ref()
                .expect("iceyee_logger/lib.rs 257")
                .clone()
        };
        let message: String = message.as_ref().replace("\n", "\n    ");
        match level {
            Level::Debug | Level::Info => {
                let message: String = format!("\n{} {} # {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("iceyee_logger/lib.rs 345")
                    .write_all(message.as_bytes())
                    .await
                    .expect("iceyee_logger/lib.rs 993");
                drop(stdout);
            }
            Level::Warn => {
                let message: String = format!("\n{} {} # {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("iceyee_logger/lib.rs 401")
                    .write_all(message.as_bytes())
                    .await
                    .expect("iceyee_logger/lib.rs 769");
                drop(stdout);
                let mut warn_file = self.warn_file.lock().await;
                if warn_file.is_some() {
                    warn_file
                        .as_mut()
                        .expect("iceyee_logger/lib.rs 297")
                        .write_all(message.as_bytes())
                        .await
                        .expect("iceyee_logger/lib.rs 185");
                }
                drop(warn_file);
            }
            Level::Error => {
                let message: String =
                    format!("\n{} {} # \n    {}\n", time, level.to_string(), message);
                stdout
                    .as_mut()
                    .expect("iceyee_logger/lib.rs 633")
                    .write_all(message.as_bytes())
                    .await
                    .expect("iceyee_logger/lib.rs 841");
                drop(stdout);
                let mut error_file = self.error_file.lock().await;
                if error_file.is_some() {
                    error_file
                        .as_mut()
                        .expect("iceyee_logger/lib.rs 009")
                        .write_all(message.as_bytes())
                        .await
                        .expect("iceyee_logger/lib.rs 337");
                }
                drop(error_file);
            }
        }
        return;
    }

    pub async fn debug<S>(&self, message: S)
    where
        S: AsRef<str>,
    {
        Self::print(self, message, Level::Debug).await;
        return;
    }

    pub async fn debug_2<S1, S2>(&self, s1: S1, s2: S2)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let message: String = format!("{} {}", s1.as_ref(), s2.as_ref());
        Self::print(self, message, Level::Debug).await;
        return;
    }

    pub async fn debug_3<S1, S2, S3>(&self, s1: S1, s2: S2, s3: S3)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let message: String = format!("{} {} {}", s1.as_ref(), s2.as_ref(), s3.as_ref(),);
        Self::print(self, message, Level::Debug).await;
        return;
    }

    pub async fn debug_4<S1, S2, S3, S4>(&self, s1: S1, s2: S2, s3: S3, s4: S4)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
        S4: AsRef<str>,
    {
        let message: String = format!(
            "{} {} {} {}",
            s1.as_ref(),
            s2.as_ref(),
            s3.as_ref(),
            s4.as_ref()
        );
        Self::print(self, message, Level::Debug).await;
        return;
    }

    pub async fn info<S>(&self, message: S)
    where
        S: AsRef<str>,
    {
        Self::print(self, message, Level::Info).await;
        return;
    }

    pub async fn info_2<S1, S2>(&self, s1: S1, s2: S2)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let message: String = format!("{} {}", s1.as_ref(), s2.as_ref());
        Self::print(self, message, Level::Info).await;
        return;
    }

    pub async fn info_3<S1, S2, S3>(&self, s1: S1, s2: S2, s3: S3)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let message: String = format!("{} {} {}", s1.as_ref(), s2.as_ref(), s3.as_ref(),);
        Self::print(self, message, Level::Info).await;
        return;
    }

    pub async fn info_4<S1, S2, S3, S4>(&self, s1: S1, s2: S2, s3: S3, s4: S4)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
        S4: AsRef<str>,
    {
        let message: String = format!(
            "{} {} {} {}",
            s1.as_ref(),
            s2.as_ref(),
            s3.as_ref(),
            s4.as_ref()
        );
        Self::print(self, message, Level::Info).await;
        return;
    }

    pub async fn warn<S>(&self, message: S)
    where
        S: AsRef<str>,
    {
        Self::print(self, message, Level::Warn).await;
        return;
    }

    pub async fn warn_2<S1, S2>(&self, s1: S1, s2: S2)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let message: String = format!("{} {}", s1.as_ref(), s2.as_ref());
        Self::print(self, message, Level::Warn).await;
        return;
    }

    pub async fn warn_3<S1, S2, S3>(&self, s1: S1, s2: S2, s3: S3)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let message: String = format!("{} {} {}", s1.as_ref(), s2.as_ref(), s3.as_ref());
        Self::print(self, message, Level::Warn).await;
        return;
    }

    pub async fn warn_4<S1, S2, S3, S4>(&self, s1: S1, s2: S2, s3: S3, s4: S4)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
        S4: AsRef<str>,
    {
        let message: String = format!(
            "{} {} {} {}",
            s1.as_ref(),
            s2.as_ref(),
            s3.as_ref(),
            s4.as_ref()
        );
        Self::print(self, message, Level::Warn).await;
        return;
    }

    pub async fn error<S>(&self, message: S)
    where
        S: AsRef<str>,
    {
        Self::print(self, message, Level::Error).await;
        return;
    }

    pub async fn error_2<S1, S2>(&self, s1: S1, s2: S2)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let message: String = format!("{} {}", s1.as_ref(), s2.as_ref());
        Self::print(self, message, Level::Error).await;
        return;
    }

    pub async fn error_3<S1, S2, S3>(&self, s1: S1, s2: S2, s3: S3)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let message: String = format!("{} {} {}", s1.as_ref(), s2.as_ref(), s3.as_ref());
        Self::print(self, message, Level::Error).await;
        return;
    }

    pub async fn error_4<S1, S2, S3, S4>(&self, s1: S1, s2: S2, s3: S3, s4: S4)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
        S4: AsRef<str>,
    {
        let message: String = format!(
            "{} {} {} {}",
            s1.as_ref(),
            s2.as_ref(),
            s3.as_ref(),
            s4.as_ref()
        );
        Self::print(self, message, Level::Error).await;
        return;
    }
}

// Function.

/// 日志的默认路径.
pub fn default_target() -> String {
    #[cfg(target_os = "linux")]
    {
        return std::env::var("HOME").expect("iceyee_logger/lib.rs 025") + "/.iceyee_log";
    }
    #[cfg(target_os = "windows")]
    {
        return std::env::var("USERPROFILE").expect("iceyee_logger/lib.rs 273") + "\\.iceyee_log";
    }
}

/// 用户主目录.
pub fn home() -> String {
    #[cfg(target_os = "linux")]
    {
        return std::env::var("HOME").expect("iceyee_logger/lib.rs 281");
    }
    #[cfg(target_os = "windows")]
    {
        return std::env::var("USERPROFILE").expect("iceyee_logger/lib.rs 249");
    }
}

/// 初始化.
pub async fn init(level: Level, project_name: Option<&str>, target_directory: Option<&str>) {
    *LOGGER.lock().await = Some(Logger::new(level, project_name, target_directory).await);
    return;
}

/// Debug.
pub async fn debug<S>(message: S)
where
    S: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 865")
        .debug(message)
        .await;
}

/// Debug.
pub async fn debug_2<S1, S2>(s1: S1, s2: S2)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 913")
        .debug_2(s1, s2)
        .await;
}

/// Debug.
pub async fn debug_3<S1, S2, S3>(s1: S1, s2: S2, s3: S3)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 721")
        .debug_3(s1, s2, s3)
        .await;
}

/// Debug.
pub async fn debug_4<S1, S2, S3, S4>(s1: S1, s2: S2, s3: S3, s4: S4)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
    S4: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 489")
        .debug_4(s1, s2, s3, s4)
        .await;
}

/// Debug.
pub async fn debug_object<S>(prompt: S, object: &impl std::fmt::Debug)
where
    S: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    let message: String = format!("{} {:?}", prompt.as_ref(), object);
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 377")
        .debug(message)
        .await;
}

/// Info.
pub async fn info<S>(message: S)
where
    S: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 417")
        .info(message)
        .await;
}

/// Info.
pub async fn info_2<S1, S2>(s1: S1, s2: S2)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 705")
        .info_2(s1, s2)
        .await;
}

/// Info.
pub async fn info_3<S1, S2, S3>(s1: S1, s2: S2, s3: S3)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 553")
        .info_3(s1, s2, s3)
        .await;
}

/// Info.
pub async fn info_4<S1, S2, S3, S4>(s1: S1, s2: S2, s3: S3, s4: S4)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
    S4: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 161")
        .info_4(s1, s2, s3, s4)
        .await;
}

/// Info.
pub async fn info_object<S>(prompt: S, object: &impl std::fmt::Debug)
where
    S: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    let message: String = format!("{} {:?}", prompt.as_ref(), object);
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 377")
        .info(message)
        .await;
}
/// Warn.
pub async fn warn<S>(message: S)
where
    S: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 729")
        .warn(message)
        .await;
}

/// Warn.
pub async fn warn_2<S1, S2>(s1: S1, s2: S2)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 457")
        .warn_2(s1, s2)
        .await;
}

/// Warn.
pub async fn warn_3<S1, S2, S3>(s1: S1, s2: S2, s3: S3)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 545")
        .warn_3(s1, s2, s3)
        .await;
}

/// Warn.
pub async fn warn_4<S1, S2, S3, S4>(s1: S1, s2: S2, s3: S3, s4: S4)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
    S4: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 193")
        .warn_4(s1, s2, s3, s4)
        .await;
}

/// Warn.
pub async fn warn_object<S>(prompt: S, object: &impl std::fmt::Debug)
where
    S: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    let message: String = format!("{} {:?}", prompt.as_ref(), object);
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 377")
        .warn(message)
        .await;
}

/// Error.
pub async fn error<S>(message: S)
where
    S: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 601")
        .error(message)
        .await;
}

/// Error.
pub async fn error_2<S1, S2>(s1: S1, s2: S2)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 969")
        .error_2(s1, s2)
        .await;
}

/// Error.
pub async fn error_3<S1, S2, S3>(s1: S1, s2: S2, s3: S3)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 497")
        .error_3(s1, s2, s3)
        .await;
}

/// Error.
pub async fn error_4<S1, S2, S3, S4>(s1: S1, s2: S2, s3: S3, s4: S4)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
    S4: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 385")
        .error_4(s1, s2, s3, s4)
        .await;
}

/// Error.
pub async fn error_object<S>(prompt: S, object: &impl std::fmt::Debug)
where
    S: AsRef<str>,
{
    let mut logger = LOGGER.lock().await;
    if logger.is_none() {
        *logger = Some(Logger::new(Level::default(), None, None).await);
    }
    let message: String = format!("{} {:?}", prompt.as_ref(), object);
    return logger
        .as_ref()
        .expect("iceyee_logger/lib.rs 377")
        .error(message)
        .await;
}
