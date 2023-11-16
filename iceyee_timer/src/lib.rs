// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 时钟.

// Use.

use std::future::Future;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tokio::task::JoinHandle;
use tokio::time::Sleep;

// Enum.

/// Error.
///
/// - @see [Timer]
#[derive(Debug, Clone)]
pub enum TimerError {
    InvalidFormat,
}

impl std::fmt::Display for TimerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidFormat => {
                f.write_str("错误的格式.")?;
            }
        }
        return Ok(());
    }
}

impl std::error::Error for TimerError {}

// Trait.

// Struct.

/// 时钟, 精度50ms ~ 100ms.

#[derive(Clone, Debug)]
pub struct Timer {
    thread_handles: Arc<TokioMutex<Vec<JoinHandle<()>>>>,
    is_stop: Arc<AtomicBool>,
}

impl Timer {
    pub fn new() -> Self {
        return Self {
            thread_handles: Arc::new(TokioMutex::new(Vec::new())),
            is_stop: Arc::new(AtomicBool::new(false)),
        };
    }

    /// 延时, 单位:毫秒.
    pub fn sleep(t: u64) -> Sleep {
        use std::time::Duration;

        return tokio::time::sleep(Duration::from_millis(t));
    }

    #[deprecated(
        since = "2.0.0",
        note = "停止时钟放到了Drop中, 只需要在主程序等待一段时间, 即可正确地释放资源."
    )]
    /// 停止时钟并结束所有与其绑定的定时任务.
    pub async fn stop(&mut self) {
        self.is_stop.store(true, Ordering::SeqCst);
        loop {
            match {
                let handle = self.thread_handles.lock().await.pop();
                handle
            } {
                Some(handle) => handle.await.unwrap(),
                None => break,
            }
        }
        return;
    }

    /// 定时任务, 模式匹配.
    ///
    /// - @param pattern "秒 分 时 日 月 周几", "second minute hour day month weekday", 可以参考linux的crontab.
    /// - @param f
    /// - @exception [TimerError::InvalidFormat] pattern参数, 格式错误.
    pub fn schedule_pattern<F1, F2>(&mut self, pattern: &str, mut f: F1) -> Result<(), TimerError>
    where
        F1: FnMut() -> F2 + Send + 'static,
        F2: Future<Output = ()> + Send + 'static,
    {
        // 在'*'可能有'/', 即SLASH.
        enum Status {
            MIN,
            MAX,
            SEPARATION,
            SLASH,
        }
        let expand = |mut min: usize, max: usize, separation: usize| {
            let mut result: Vec<usize> = Vec::new();
            while min <= max {
                result.push(min);
                min += separation;
            }
            return result;
        };
        let mut table: [([bool; 60], usize, usize); 6] = [
            ([false; 60], 0, 59),
            ([false; 60], 0, 59),
            ([false; 60], 0, 59),
            ([false; 60], 1, 31),
            ([false; 60], 1, 12),
            ([false; 60], 1, 7),
        ];
        let mut pattern: String = pattern.to_string();
        while pattern.contains("  ") {
            pattern = pattern.replace("  ", " ");
        }
        if pattern.split(' ').count() != table.len() {
            return Err(TimerError::InvalidFormat);
        }
        let mut index: usize = 0;
        for x in pattern.split(' ') {
            if x.len() == 0 {
                return Err(TimerError::InvalidFormat);
            }
            for y in x.split(',') {
                if y.len() == 0 {
                    return Err(TimerError::InvalidFormat);
                }
                let mut status: Status = Status::MIN;
                let mut min: Vec<u8> = Vec::new();
                let mut max: Vec<u8> = Vec::new();
                let mut separation: Vec<u8> = Vec::new();
                for z in y.as_bytes() {
                    match status {
                        Status::MIN => {
                            if (*z as char).is_ascii_digit() {
                                min.push(*z);
                            } else if *z == b'-' {
                                status = Status::MAX;
                            } else if *z == b'*' {
                                if 0 < min.len() {
                                    return Err(TimerError::InvalidFormat);
                                }
                                min.extend_from_slice(table[index].1.to_string().as_bytes());
                                max.extend_from_slice(table[index].2.to_string().as_bytes());
                                status = Status::SLASH;
                            } else {
                                return Err(TimerError::InvalidFormat);
                            }
                        }
                        Status::MAX => {
                            if (*z as char).is_ascii_digit() {
                                max.push(*z);
                            } else if *z == b'/' {
                                status = Status::SEPARATION;
                            } else {
                                return Err(TimerError::InvalidFormat);
                            }
                        }
                        Status::SEPARATION => {
                            if (*z as char).is_ascii_digit() {
                                separation.push(*z);
                            } else {
                                return Err(TimerError::InvalidFormat);
                            }
                        }
                        Status::SLASH => {
                            if *z == b'/' {
                                status = Status::SEPARATION;
                            }
                        }
                    }
                }
                match status {
                    Status::MIN => {
                        if min.len() == 0 {
                            return Err(TimerError::InvalidFormat);
                        } else {
                            max = min.clone();
                        }
                    }
                    Status::MAX => {
                        if max.len() == 0 {
                            return Err(TimerError::InvalidFormat);
                        }
                    }
                    Status::SEPARATION => {
                        if separation.len() == 0 {
                            return Err(TimerError::InvalidFormat);
                        }
                    }
                    Status::SLASH => {}
                }
                let min: usize = if min.len() == 0 {
                    table[index].1
                } else {
                    String::from_utf8(min)
                        .map_err(|_| TimerError::InvalidFormat)?
                        .parse::<usize>()
                        .map_err(|_| TimerError::InvalidFormat)?
                };
                let max: usize = if max.len() == 0 {
                    table[index].2
                } else {
                    String::from_utf8(max)
                        .map_err(|_| TimerError::InvalidFormat)?
                        .parse::<usize>()
                        .map_err(|_| TimerError::InvalidFormat)?
                };
                let separation: usize = if separation.len() == 0 {
                    1
                } else {
                    String::from_utf8(separation)
                        .map_err(|_| TimerError::InvalidFormat)?
                        .parse::<usize>()
                        .map_err(|_| TimerError::InvalidFormat)?
                };
                if min < table[index].1
                    || table[index].2 < min
                    || max < table[index].1
                    || table[index].2 < max
                    || max < min
                {
                    return Err(TimerError::InvalidFormat);
                }
                for z in expand(min, max, separation) {
                    table[index].0[z] = true;
                }
            } // for y in x.split(',') {...}
            index += 1;
        } // for x in pattern.split(' ') {...}
        let is_stop: Arc<AtomicBool> = self.is_stop.clone();
        let handle = tokio::task::spawn(async move {
            use iceyee_datetime::DateTime;
            let second = &table[0];
            let minute = &table[1];
            let hour = &table[2];
            let day = &table[3];
            let month = &table[4];
            let weekday = &table[5];
            while !is_stop.load(Ordering::SeqCst) {
                let t: u64 = (1000 - DateTime::now() as u64 % 1000) + 200;
                let sl = tokio::task::spawn(Self::sleep(t));
                let dt: DateTime = DateTime::new();
                if second.0[dt.second]
                    && minute.0[dt.minute]
                    && hour.0[dt.hour]
                    && day.0[dt.day]
                    && month.0[dt.month]
                    && weekday.0[dt.weekday]
                {
                    f().await;
                }
                while !is_stop.load(Ordering::SeqCst) && !sl.is_finished() {
                    Self::sleep(50).await;
                }
            }
        });
        let thread_handles = self.thread_handles.clone();
        tokio::task::spawn(async move {
            thread_handles.lock().await.push(handle);
        });
        return Ok(());
    }

    /// 定时任务, 任务执行的同时等待.
    pub fn schedule_execute_before<F1, F2>(&mut self, delay: u64, period: u64, mut f: F1)
    where
        F1: FnMut() -> F2 + Send + 'static,
        F2: Future<Output = ()> + Send + 'static,
    {
        let is_stop: Arc<AtomicBool> = self.is_stop.clone();
        let handle = tokio::task::spawn(async move {
            let sl = tokio::task::spawn(Timer::sleep(delay));
            while !is_stop.load(Ordering::SeqCst) && !sl.is_finished() {
                Timer::sleep(50).await;
            }
            while !is_stop.load(Ordering::SeqCst) {
                let sl = tokio::task::spawn(Timer::sleep(period));
                f().await;
                while !is_stop.load(Ordering::SeqCst) && !sl.is_finished() {
                    Timer::sleep(50).await;
                }
            }
        });
        let thread_handles = self.thread_handles.clone();
        tokio::task::spawn(async move {
            thread_handles.lock().await.push(handle);
        });
        return;
    }

    /// 定时任务, 在任务执行完成后等待.
    pub fn schedule_execute_after<F1, F2>(&mut self, delay: u64, period: u64, mut f: F1)
    where
        F1: FnMut() -> F2 + Send + 'static,
        F2: Future<Output = ()> + Send + 'static,
    {
        let is_stop: Arc<AtomicBool> = self.is_stop.clone();
        let handle = tokio::task::spawn(async move {
            let sl = tokio::task::spawn(Self::sleep(delay));
            while !is_stop.load(Ordering::SeqCst) && !sl.is_finished() {
                Self::sleep(50).await;
            }
            while !is_stop.load(Ordering::SeqCst) {
                f().await;
                let sl = tokio::task::spawn(Self::sleep(period));
                while !is_stop.load(Ordering::SeqCst) && !sl.is_finished() {
                    Self::sleep(50).await;
                }
            }
        });
        let thread_handles = self.thread_handles.clone();
        tokio::task::spawn(async move {
            thread_handles.lock().await.push(handle);
        });
        return;
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let is_stop = self.is_stop.clone();
        let thread_handles = self.thread_handles.clone();
        tokio::task::spawn(async move {
            is_stop.store(true, Ordering::SeqCst);
            loop {
                match {
                    let handle = thread_handles.lock().await.pop();
                    handle
                } {
                    Some(handle) => handle.await.unwrap(),
                    None => break,
                }
            }
        });
        return;
    }
}

// Function.
