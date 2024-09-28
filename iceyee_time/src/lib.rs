// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

#![feature(get_mut_unchecked)]

// Use.

use std::cmp::Ordering as CmpOrdering;
use std::cmp::PartialOrd;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use tokio::task::JoinHandle;
use tokio::time::Sleep;

const ONE_MILLISECOND: i64 = 1;
const ONE_SECOND: i64 = 1_000 * ONE_MILLISECOND;
const ONE_MINUTE: i64 = 60 * ONE_SECOND;
const ONE_HOUR: i64 = 60 * ONE_MINUTE;
const ONE_DAY: i64 = 24 * ONE_HOUR;
const ONE_WEEK: i64 = 7 * ONE_DAY;
// const ONE_MONTH: i64 = 31 * ONE_DAY;
const ONE_YEAR: i64 = 365 * ONE_DAY;
const FOUR_YEAR: i64 = 4 * ONE_YEAR + ONE_DAY;
const ONE_HUNDRED_YEAR: i64 = 25 * FOUR_YEAR - ONE_DAY;
const FOUR_HUNDRED_YEAR: i64 = 4 * ONE_HUNDRED_YEAR + ONE_DAY;
const TIME_0: i64 =
    4 * FOUR_HUNDRED_YEAR + 3 * ONE_HUNDRED_YEAR + ONE_DAY + 17 * FOUR_YEAR + 2 * ONE_YEAR;

// Enum.

// Trait.

/// 定时任务1.
///
/// sleep_before_perform(), sleep_after_perform(), schedule_by_pattern()表示三种不同的模式.
///
/// # Use
/// ```
/// use iceyee_time::Schedule1;
/// use iceyee_time::Timer;
/// use std::future::Future;
/// use std::pin::Pin;
/// use std::sync::Arc;
/// use std::sync::atomic::AtomicBool;
/// use std::sync::atomic::Ordering::SeqCst;
/// ```
/// - @see [Timer]
pub trait Schedule1: Send + Sync {
    /// 初始延迟, 默认0.
    fn delay1(&self) -> u64 {
        0
    }

    fn sleep_before_perform1(&self) -> u64 {
        0
    }

    fn sleep_after_perform1(&self) -> u64 {
        0
    }

    fn schedule_by_pattern1(&self) -> String {
        "".to_string()
    }

    /// 在循环任务开始之前执行.
    fn initialize1<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    /// 循环任务, 返回值表示是否继续循环.
    fn perform1<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b;

    /// 在循环任务结束之后执行.
    fn finish1<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn wrap1(self) -> Arc<dyn Schedule1>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

/// 定时任务2.
///
/// - @see [Timer]
pub trait Schedule2: Send + Sync {
    fn delay2(&self) -> u64 {
        0
    }

    fn sleep_before_perform2(&self) -> u64 {
        0
    }

    fn sleep_after_perform2(&self) -> u64 {
        0
    }

    fn schedule_by_pattern2(&self) -> String {
        "".to_string()
    }

    fn initialize2<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn perform2<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b;

    fn finish2<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn wrap2(self) -> Arc<dyn Schedule2>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

/// 定时任务3.
///
/// - @see [Timer]
pub trait Schedule3: Send + Sync {
    fn delay3(&self) -> u64 {
        0
    }

    fn sleep_before_perform3(&self) -> u64 {
        0
    }

    fn sleep_after_perform3(&self) -> u64 {
        0
    }

    fn schedule_by_pattern3(&self) -> String {
        "".to_string()
    }

    fn initialize3<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn perform3<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b;

    fn finish3<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn wrap3(self) -> Arc<dyn Schedule3>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

/// 定时任务4.
///
/// - @see [Timer]
pub trait Schedule4: Send + Sync {
    fn delay4(&self) -> u64 {
        0
    }

    fn sleep_before_perform4(&self) -> u64 {
        0
    }

    fn sleep_after_perform4(&self) -> u64 {
        0
    }

    fn schedule_by_pattern4(&self) -> String {
        "".to_string()
    }

    fn initialize4<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn perform4<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b;

    fn finish4<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn wrap4(self) -> Arc<dyn Schedule4>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

/// 定时任务5.
///
/// - @see [Timer]
pub trait Schedule5: Send + Sync {
    fn delay5(&self) -> u64 {
        0
    }

    fn sleep_before_perform5(&self) -> u64 {
        0
    }

    fn sleep_after_perform5(&self) -> u64 {
        0
    }

    fn schedule_by_pattern5(&self) -> String {
        "".to_string()
    }

    fn initialize5<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn perform5<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b;

    fn finish5<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async move {
            return;
        });
    }

    fn wrap5(self) -> Arc<dyn Schedule5>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

// Struct.

/// 时区所对应的时间偏移.
/// 比如用+0800表示东八区的时间偏移, 即+08:00.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeOffset(pub i16);

impl std::default::Default for TimeOffset {
    /// 默认返回系统设置的时区.
    fn default() -> Self {
        static mut TIME_OFFSET: Option<TimeOffset> = None;
        unsafe {
            if TIME_OFFSET.is_some() {
                return TIME_OFFSET.as_ref().expect("NEVER").clone();
            }
            #[cfg(target_os = "linux")]
            {
                // extern long timezone;
                // void tzset ();
                use std::ffi::c_long;
                extern "C" {
                    static mut timezone: c_long;
                    fn tzset();
                }
                tzset();
                let offset_hour: i16 = timezone as i16 / 60 / 60;
                let offset_minute: i16 = timezone as i16 / 60 % 60;
                let t: i16 = -(offset_hour * 100 + offset_minute);
                TIME_OFFSET = Some(TimeOffset(t));
            }
            #[cfg(target_os = "windows")]
            {
                // typedef struct _SYSTEMTIME {
                //     WORD wYear;
                //     WORD wMonth;
                //     WORD wDayOfWeek;
                //     WORD wDay;
                //     WORD wHour;
                //     WORD wMinute;
                //     WORD wSecond;
                //     WORD wMilliseconds;
                // } SYSTEMTIME, *PSYSTEMTIME, *LPSYSTEMTIME;
                // typedef struct _TIME_ZONE_INFORMATION {
                //     LONG       Bias;
                //     WCHAR      StandardName[32];
                //     SYSTEMTIME StandardDate;
                //     LONG       StandardBias;
                //     WCHAR      DaylightName[32];
                //     SYSTEMTIME DaylightDate;
                //     LONG       DaylightBias;
                // } TIME_ZONE_INFORMATION, *PTIME_ZONE_INFORMATION, *LPTIME_ZONE_INFORMATION;
                // DWORD GetTimeZoneInformation(
                //         [out] LPTIME_ZONE_INFORMATION lpTimeZoneInformation
                //         );
                use std::ffi::c_int;
                use std::ffi::c_long;
                use std::ffi::c_ushort;
                #[allow(non_snake_case)]
                #[derive(Debug, Clone, Default)]
                #[repr(C)]
                struct SYSTEMTIME {
                    wYear: c_ushort,
                    wMonth: c_ushort,
                    wDayOfWeek: c_ushort,
                    wDay: c_ushort,
                    wHour: c_ushort,
                    wMinute: c_ushort,
                    wSecond: c_ushort,
                    wMilliseconds: c_ushort,
                }
                #[allow(non_camel_case_types)]
                #[allow(non_snake_case)]
                #[derive(Debug, Clone, Default)]
                #[repr(C)]
                struct TIME_ZONE_INFORMATION {
                    Bias: c_long,
                    StandardName: [c_ushort; 32],
                    StandardDate: SYSTEMTIME,
                    StandardBias: c_long,
                    DaylightName: [c_ushort; 32],
                    DaylightDate: SYSTEMTIME,
                    DaylightBias: c_long,
                }
                extern "C" {
                    fn GetTimeZoneInformation(
                        lpTimeZoneInformation: *mut TIME_ZONE_INFORMATION,
                    ) -> c_int;
                }
                let mut tzi: TIME_ZONE_INFORMATION = Default::default();
                GetTimeZoneInformation(&mut tzi);
                let offset_hour: i16 = tzi.Bias as i16 / 60;
                let offset_minute: i16 = tzi.Bias as i16 % 60;
                let t: i16 = -(offset_hour * 100 + offset_minute);
                TIME_OFFSET = Some(TimeOffset(t));
            }
            return TIME_OFFSET.as_ref().expect("NEVER").clone();
        }
    }
}

/// 日期时间.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateTime {
    /// \[0, +oo)
    pub year: u64,
    /// \[1, 12]
    pub month: u64,
    /// \[1, 31]
    pub day: u64,
    /// \[0, 23]
    pub hour: u64,
    /// \[0, 59]
    pub minute: u64,
    /// \[0, 59]
    pub second: u64,
    /// \[0, 999]
    pub millisecond: u64,
    /// \[1, 365]
    pub day_of_year: u64,
    /// \[1, 7]
    pub weekday: u64,
    /// 时间戳, 单位:毫秒.
    pub timestamp: i64,
    /// 时间偏移, 比如用+0800表示东八区的时间偏移, 即+08:00.
    pub offset: TimeOffset,
}

impl DateTime {
    /// 返回当前时间, 等同于:
    ///
    /// ```
    /// DateTime::from((iceyee_time::now(), None));
    /// ```
    pub fn new() -> Self {
        return DateTime::from((now(), None));
    }

    /// 转成国际标准时间, 等同于:
    ///
    /// ```
    /// DateTime::from((self.timestamp, Some(0)));
    /// ```
    pub fn to_utc(&self) -> Self {
        return Self::from((self.timestamp, Some(TimeOffset(0))));
    }
}

impl From<(i64, Option<TimeOffset>)> for DateTime {
    /// 从时间戳转成[DateTime].
    ///
    /// - @param value (timestamp, offset)
    /// - @param value$0 时间戳, 单位:毫秒.
    /// - @param value$1 时间偏移, 默认是系统设置的时区所对应的偏移.
    fn from(value: (i64, Option<TimeOffset>)) -> Self {
        let (_timestamp, offset) = value;
        let offset: TimeOffset = offset.unwrap_or(TimeOffset::default());
        let offset: i16 = offset.0;
        let offset_hour: i16 = offset / 100 % 100;
        let offset_minute: i16 = offset % 100;
        let mut timestamp: i64 = TIME_0
            + offset_hour as i64 * 60 * 60 * 1000
            + offset_minute as i64 * 60 * 1000
            + _timestamp;
        // 年.
        let mut year: i64 = 0;
        year += timestamp / FOUR_HUNDRED_YEAR * 400;
        timestamp %= FOUR_HUNDRED_YEAR;
        if ONE_HUNDRED_YEAR + ONE_DAY <= timestamp {
            timestamp -= ONE_HUNDRED_YEAR;
            timestamp -= ONE_DAY;
            year += 100;
            if ONE_HUNDRED_YEAR <= timestamp {
                timestamp -= ONE_HUNDRED_YEAR;
                year += 100;
            }
            if ONE_HUNDRED_YEAR <= timestamp {
                timestamp -= ONE_HUNDRED_YEAR;
                year += 100;
            }
        }
        if year % 400 != 0 && 4 * ONE_YEAR <= timestamp {
            timestamp -= 4 * ONE_YEAR;
            year += 4;
        }
        year += timestamp / FOUR_YEAR * 4;
        timestamp %= FOUR_YEAR;
        if ONE_YEAR + ONE_DAY <= timestamp {
            timestamp -= ONE_YEAR;
            timestamp -= ONE_DAY;
            year += 1;
            if ONE_YEAR <= timestamp {
                timestamp -= ONE_YEAR;
                year += 1;
            }
            if ONE_YEAR <= timestamp {
                timestamp -= ONE_YEAR;
                year += 1;
            }
        }
        let day_of_year: i64 = timestamp / ONE_DAY + 1;
        // 月.
        let mut month: i64 = 0;
        for x in 1..=12 {
            let max_days: i64 = match x {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                2 => {
                    if year % 400 == 0 || year % 100 != 0 && year % 4 == 0 {
                        29
                    } else {
                        28
                    }
                }
                _ => 30,
            };
            if max_days * ONE_DAY <= timestamp {
                timestamp -= max_days * ONE_DAY;
                continue;
            } else {
                month = x;
                break;
            }
        }
        // 日.
        let day: i64 = 1 + timestamp / ONE_DAY;
        timestamp %= ONE_DAY;
        // 时分秒.
        let hour: i64 = timestamp / ONE_HOUR;
        timestamp %= ONE_HOUR;
        let minute: i64 = timestamp / ONE_MINUTE;
        timestamp %= ONE_MINUTE;
        let second: i64 = timestamp / ONE_SECOND;
        timestamp %= ONE_SECOND;
        let millisecond: i64 = timestamp / ONE_MILLISECOND;
        // 周几.
        let mut weekday: i64 = (TIME_0
            + offset_hour as i64 * 60 * 60 * 1000
            + offset_minute as i64 * 60 * 1000
            + _timestamp)
            % ONE_WEEK
            / ONE_DAY
            + 1
            + 5;
        while 7 < weekday {
            weekday -= 7;
        }
        return Self {
            year: year as u64,
            month: month as u64,
            day: day as u64,
            hour: hour as u64,
            minute: minute as u64,
            second: second as u64,
            millisecond: millisecond as u64,
            day_of_year: day_of_year as u64,
            weekday: weekday as u64,
            timestamp: _timestamp,
            offset: TimeOffset(offset),
        };
    }
}

impl From<(u64, u64, u64, u64, u64, u64, u64, Option<TimeOffset>)> for DateTime {
    /// 从设置好的时间, 转成[DateTime].
    ///
    /// - @param value (year, month, day, hour, minute, second, millisecond, offset)
    /// - @param value$0 年.
    /// - @param value$1 月.
    /// - @param value$2 日.
    /// - @param value$3 时.
    /// - @param value$4 分.
    /// - @param value$5 秒.
    /// - @param value$6 毫秒.
    /// - @param value$7 时间偏移, 默认是系统设置的时区所对应的偏移.
    ///
    /// # Panics
    ///
    /// 如果参数不符合特定的数值范围就会panic.
    ///
    /// 例如, '时'的范围是0~23, 如果对应的入参是24, 就会panic.
    ///
    /// 例如, 某一年的二月份, 只有28天, 但是参数'日'的入参是29, 就会panic.
    fn from(value: (u64, u64, u64, u64, u64, u64, u64, Option<TimeOffset>)) -> Self {
        let (year, month, day, hour, minute, second, millisecond, offset) = value;
        if month == 0 || 12 < month {
            panic!("@month={:?}", month);
        }
        let max_days: u64 = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 => {
                if year % 400 == 0 || year % 100 != 0 && year % 4 == 0 {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        };
        if day == 0 || max_days < day {
            panic!("@day={:?}", day);
        }
        if 23 < hour {
            panic!("@hour={:?}", hour);
        }
        if 59 < minute {
            panic!("@minute={:?}", minute);
        }
        if 59 < second {
            panic!("@second={:?}", second);
        }
        if 999 < millisecond {
            panic!("@millisecond={:?}", millisecond);
        }
        let mut timestamp: i64 = 0;
        let mut t: i64 = year as i64;
        // 年.
        timestamp += t * ONE_YEAR;
        timestamp += t / 400 * 97 * ONE_DAY;
        t %= 400;
        if t != 0 {
            timestamp += (t / 100 * 24 + 1) * ONE_DAY;
            t %= 100;
        }
        if t != 0 {
            timestamp -= ONE_DAY;
            if t % 4 != 0 {
                timestamp += ONE_DAY;
            }
            timestamp += (t / 4) * ONE_DAY;
            t %= 4;
        }
        let _ = t;
        // 月.
        t = 0;
        for x in 1..=12 {
            if month <= x {
                break;
            }
            let max_days: i64 = match x {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                2 => {
                    if year % 400 == 0 || year % 100 != 0 && year % 4 == 0 {
                        29
                    } else {
                        28
                    }
                }
                _ => 30,
            };
            timestamp += max_days * ONE_DAY;
            t += max_days;
        }
        // let day_of_year: i64 = t + day as i64;
        // 日.
        timestamp += (day as i64 - 1) * ONE_DAY;
        // 时分秒.
        timestamp += hour as i64 * ONE_HOUR;
        timestamp += minute as i64 * ONE_MINUTE;
        timestamp += second as i64 * ONE_SECOND;
        timestamp += millisecond as i64 * ONE_MILLISECOND;
        // 时间差.
        let offset: TimeOffset = offset.unwrap_or(TimeOffset::default());
        let offset: i16 = offset.0;
        let offset_hour: i16 = offset / 100 % 100;
        let offset_minute: i16 = offset % 100;
        timestamp -= TIME_0;
        timestamp -= offset_hour as i64 * 60 * 60 * 1000;
        timestamp -= offset_minute as i64 * 60 * 1000;
        return Self::from((timestamp, Some(TimeOffset(offset))));
        // // 周几.
        // let mut weekday: i64 = timestamp % ONE_WEEK / ONE_DAY + 1 + 3;
        // while 7 < weekday {
        //     weekday -= 7;
        // }
        // return Self {
        //     year: year as u64,
        //     month: month as u64,
        //     day: day as u64,
        //     hour: hour as u64,
        //     minute: minute as u64,
        //     second: second as u64,
        //     day_of_year: day_of_year as u64,
        //     weekday: weekday as u64,
        //     timestamp: timestamp,
        //     offset: offset,
        // };
    }
}

impl ToString for DateTime {
    /// 转成字符串, 使用RFC3339标准, 格式'xx-xx-xxTxx:xx:xx.xxx\[+/-\]xx:xx'.
    fn to_string(&self) -> String {
        let v1: String = format!(
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.timestamp % 1_000
        );
        let v2: &str = if self.offset.0 == 0 {
            ""
        } else if self.offset.0 < 0 {
            "-"
        } else {
            "+"
        };
        let offset: i16 = if self.offset.0 < 0 {
            -self.offset.0
        } else {
            self.offset.0
        };
        let v3: String = if offset == 0 {
            "Z".to_string()
        } else {
            format!("{:02}:{:02}", offset / 100 % 100, offset % 100)
        };
        let result: String = v1 + v2 + &v3;
        return format!("{result:29}");
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        return self.timestamp.partial_cmp(&other.timestamp);
    }
}

/// 定时器.
///
/// @see [Schedule1]
#[derive(Clone)]
pub struct Timer {
    thread_handles: Arc<TokioMutex<Vec<JoinHandle<()>>>>,
    stop: Arc<AtomicBool>,
}

/// 默认的定时器, 这是全局变量.
impl std::default::Default for Timer {
    fn default() -> Self {
        use std::sync::Mutex;
        static TIMER: Mutex<Option<Timer>> = Mutex::new(None);
        let mut timer = TIMER.lock().expect("Mutex::lock");
        if timer.is_none() {
            *timer = Some(Timer::new());
        }
        return timer.as_ref().expect("NEVER").clone();
    }
}

impl Drop for Timer {
    /// 关闭定时器.
    fn drop(&mut self) {
        if Arc::get_mut(&mut self.thread_handles).is_some() {
            println!("Timer::drop");
            self.stop.store(true, SeqCst);
        }
        return;
    }
}

impl Timer {
    /// 创建新的定时器, 默认开启状态.
    pub fn new() -> Self {
        return Timer {
            thread_handles: Arc::new(TokioMutex::new(Vec::new())),
            stop: Arc::new(AtomicBool::new(false)),
        };
    }

    /// 启动定时器.
    pub async fn start(&self) {
        if !self.stop.load(SeqCst) {
            return;
        }
        self.stop.store(false, SeqCst);
        self.thread_handles.lock().await.clear();
        return;
    }

    /// 停止定时器.
    pub async fn stop(&self) {
        self.stop.store(true, SeqCst);
        self.thread_handles.lock().await.clear();
        return;
    }

    /// 停止定时器并等待所有任务结束.
    pub async fn stop_and_wait(&self) {
        let mut thread_handles = self.thread_handles.lock().await;
        self.stop.store(true, SeqCst);
        loop {
            match thread_handles.pop() {
                Some(handle) => handle.await.expect("JoinHandle::await"),
                None => break,
            }
        }
        return;
    }

    /// 定时任务.
    pub async fn schedule1(&self, schedule: Arc<dyn Schedule1>) {
        if schedule.sleep_before_perform1() != 0 {
            self.schedule_1_1(schedule).await;
        } else if schedule.sleep_after_perform1() != 0 {
            self.schedule_1_2(schedule).await;
        } else if schedule.schedule_by_pattern1().len() != 0 {
            self.schedule_1_3(schedule).await;
        } else {
            panic!("trait [Schedule]必须实现 sleep_before_perform, sleep_after_perform, schedule_pattern 中的任意一个");
        }
        return;
    }

    async fn schedule_1_1(&self, schedule: Arc<dyn Schedule1>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay1();
            let period: u64 = schedule.sleep_before_perform1();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize1().await;
            while !stop.load(SeqCst) {
                // 2 等待并执行.
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform1(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish1().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_1_2(&self, schedule: Arc<dyn Schedule1>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay1();
            let period: u64 = schedule.sleep_after_perform1();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize1().await;
            while !stop.load(SeqCst) {
                // 2 执行后等待.
                if !schedule.perform1(stop.clone()).await {
                    break;
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish1().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_1_3(&self, schedule: Arc<dyn Schedule1>) {
        let pattern: String = schedule.schedule_by_pattern1();
        // 1 解析.
        // 在'*'可能有'/', 即SLASH.
        enum Status {
            MIN,
            MAX,
            SEPARATION,
            SLASH,
        }
        let expand = |mut min: u64, max: u64, separation: u64| {
            let mut output: Vec<u64> = Vec::new();
            while min <= max {
                output.push(min);
                min += separation;
            }
            return output;
        };
        let mut table: [([bool; 60], u64, u64); 6] = [
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
            panic!("bad pattern");
        }
        let mut index: usize = 0;
        for x in pattern.split(' ') {
            if x.len() == 0 {
                panic!("bad pattern");
            }
            for y in x.split([',', '，']) {
                let mut status: Status = Status::MIN;
                let mut min: Vec<u8> = Vec::new();
                let mut max: Vec<u8> = Vec::new();
                let mut separation: Vec<u8> = Vec::new();
                for z in y.as_bytes() {
                    match status {
                        Status::MIN => {
                            if z.is_ascii_digit() {
                                min.push(*z);
                            } else if *z == b'-' {
                                status = Status::MAX;
                            } else if *z == b'*' {
                                if 0 < min.len() {
                                    panic!("bad pattern");
                                }
                                min.extend_from_slice(table[index].1.to_string().as_bytes());
                                max.extend_from_slice(table[index].2.to_string().as_bytes());
                                status = Status::SLASH;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::MAX => {
                            if z.is_ascii_digit() {
                                max.push(*z);
                            } else if *z == b'/' {
                                status = Status::SEPARATION;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::SEPARATION => {
                            if z.is_ascii_digit() {
                                separation.push(*z);
                            } else {
                                panic!("bad pattern");
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
                            panic!("bad pattern");
                        } else {
                            max = min.clone();
                        }
                    }
                    Status::MAX => {
                        if max.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SEPARATION => {
                        if separation.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SLASH => {}
                }
                let min: u64 = if min.len() == 0 {
                    table[index].1
                } else {
                    String::from_utf8(min)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let max: u64 = if max.len() == 0 {
                    table[index].2
                } else {
                    String::from_utf8(max)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let separation: u64 = if separation.len() == 0 {
                    1
                } else {
                    String::from_utf8(separation)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                if min < table[index].1
                    || table[index].2 < min
                    || max < table[index].1
                    || table[index].2 < max
                    || max < min
                {
                    panic!("bad pattern");
                }
                for z in expand(min, max, separation) {
                    table[index].0[z as usize] = true;
                }
            } // for y in x.split(',') {...}
            index += 1;
        } // for x in pattern.split(' ') {...}
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            // 2 初始延迟 开始.
            let delay: u64 = schedule.delay1();
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize1().await;
            let second = &table[0];
            let minute = &table[1];
            let hour = &table[2];
            let day = &table[3];
            let month = &table[4];
            let weekday = &table[5];
            // 3 执行.
            while !stop.load(SeqCst) {
                let dt: DateTime = DateTime::new();
                if second.0[dt.second as usize]
                    && minute.0[dt.minute as usize]
                    && hour.0[dt.hour as usize]
                    && day.0[dt.day as usize]
                    && month.0[dt.month as usize]
                    && weekday.0[dt.weekday as usize]
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform1(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                let t: u64 = 200 + 1_000 - now() as u64 % 1_000;
                sleep(t).await;
            }
            // 4 结束.
            schedule.finish1().await;
        });
        // 5 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    /// 定时任务.
    pub async fn schedule2(&self, schedule: Arc<dyn Schedule2>) {
        if schedule.sleep_before_perform2() != 0 {
            self.schedule_2_1(schedule).await;
        } else if schedule.sleep_after_perform2() != 0 {
            self.schedule_2_2(schedule).await;
        } else if schedule.schedule_by_pattern2().len() != 0 {
            self.schedule_2_3(schedule).await;
        } else {
            panic!("trait [Schedule]必须实现 sleep_before_perform, sleep_after_perform, schedule_pattern 中的任意一个");
        }
        return;
    }

    async fn schedule_2_1(&self, schedule: Arc<dyn Schedule2>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay2();
            let period: u64 = schedule.sleep_before_perform2();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize2().await;
            while !stop.load(SeqCst) {
                // 2 等待并执行.
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform2(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish2().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_2_2(&self, schedule: Arc<dyn Schedule2>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay2();
            let period: u64 = schedule.sleep_after_perform2();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize2().await;
            while !stop.load(SeqCst) {
                // 2 执行后等待.
                if !schedule.perform2(stop.clone()).await {
                    break;
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish2().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_2_3(&self, schedule: Arc<dyn Schedule2>) {
        let pattern: String = schedule.schedule_by_pattern2();
        // 1 解析.
        // 在'*'可能有'/', 即SLASH.
        enum Status {
            MIN,
            MAX,
            SEPARATION,
            SLASH,
        }
        let expand = |mut min: u64, max: u64, separation: u64| {
            let mut output: Vec<u64> = Vec::new();
            while min <= max {
                output.push(min);
                min += separation;
            }
            return output;
        };
        let mut table: [([bool; 60], u64, u64); 6] = [
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
            panic!("bad pattern");
        }
        let mut index: usize = 0;
        for x in pattern.split(' ') {
            if x.len() == 0 {
                panic!("bad pattern");
            }
            for y in x.split([',', '，']) {
                let mut status: Status = Status::MIN;
                let mut min: Vec<u8> = Vec::new();
                let mut max: Vec<u8> = Vec::new();
                let mut separation: Vec<u8> = Vec::new();
                for z in y.as_bytes() {
                    match status {
                        Status::MIN => {
                            if z.is_ascii_digit() {
                                min.push(*z);
                            } else if *z == b'-' {
                                status = Status::MAX;
                            } else if *z == b'*' {
                                if 0 < min.len() {
                                    panic!("bad pattern");
                                }
                                min.extend_from_slice(table[index].1.to_string().as_bytes());
                                max.extend_from_slice(table[index].2.to_string().as_bytes());
                                status = Status::SLASH;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::MAX => {
                            if z.is_ascii_digit() {
                                max.push(*z);
                            } else if *z == b'/' {
                                status = Status::SEPARATION;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::SEPARATION => {
                            if z.is_ascii_digit() {
                                separation.push(*z);
                            } else {
                                panic!("bad pattern");
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
                            panic!("bad pattern");
                        } else {
                            max = min.clone();
                        }
                    }
                    Status::MAX => {
                        if max.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SEPARATION => {
                        if separation.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SLASH => {}
                }
                let min: u64 = if min.len() == 0 {
                    table[index].1
                } else {
                    String::from_utf8(min)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let max: u64 = if max.len() == 0 {
                    table[index].2
                } else {
                    String::from_utf8(max)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let separation: u64 = if separation.len() == 0 {
                    1
                } else {
                    String::from_utf8(separation)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                if min < table[index].1
                    || table[index].2 < min
                    || max < table[index].1
                    || table[index].2 < max
                    || max < min
                {
                    panic!("bad pattern");
                }
                for z in expand(min, max, separation) {
                    table[index].0[z as usize] = true;
                }
            } // for y in x.split(',') {...}
            index += 1;
        } // for x in pattern.split(' ') {...}
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            // 2 初始延迟 开始.
            let delay: u64 = schedule.delay2();
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize2().await;
            let second = &table[0];
            let minute = &table[1];
            let hour = &table[2];
            let day = &table[3];
            let month = &table[4];
            let weekday = &table[5];
            // 3 执行.
            while !stop.load(SeqCst) {
                let dt: DateTime = DateTime::new();
                if second.0[dt.second as usize]
                    && minute.0[dt.minute as usize]
                    && hour.0[dt.hour as usize]
                    && day.0[dt.day as usize]
                    && month.0[dt.month as usize]
                    && weekday.0[dt.weekday as usize]
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform2(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                let t: u64 = 200 + 1_000 - now() as u64 % 1_000;
                sleep(t).await;
            }
            // 4 结束.
            schedule.finish2().await;
        });
        // 5 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    /// 定时任务.
    pub async fn schedule3(&self, schedule: Arc<dyn Schedule3>) {
        if schedule.sleep_before_perform3() != 0 {
            self.schedule_3_1(schedule).await;
        } else if schedule.sleep_after_perform3() != 0 {
            self.schedule_3_2(schedule).await;
        } else if schedule.schedule_by_pattern3().len() != 0 {
            self.schedule_3_3(schedule).await;
        } else {
            panic!("trait [Schedule]必须实现 sleep_before_perform, sleep_after_perform, schedule_pattern 中的任意一个");
        }
        return;
    }

    async fn schedule_3_1(&self, schedule: Arc<dyn Schedule3>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay3();
            let period: u64 = schedule.sleep_before_perform3();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize3().await;
            while !stop.load(SeqCst) {
                // 2 等待并执行.
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform3(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish3().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_3_2(&self, schedule: Arc<dyn Schedule3>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay3();
            let period: u64 = schedule.sleep_after_perform3();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize3().await;
            while !stop.load(SeqCst) {
                // 2 执行后等待.
                if !schedule.perform3(stop.clone()).await {
                    break;
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish3().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_3_3(&self, schedule: Arc<dyn Schedule3>) {
        let pattern: String = schedule.schedule_by_pattern3();
        // 1 解析.
        // 在'*'可能有'/', 即SLASH.
        enum Status {
            MIN,
            MAX,
            SEPARATION,
            SLASH,
        }
        let expand = |mut min: u64, max: u64, separation: u64| {
            let mut output: Vec<u64> = Vec::new();
            while min <= max {
                output.push(min);
                min += separation;
            }
            return output;
        };
        let mut table: [([bool; 60], u64, u64); 6] = [
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
            panic!("bad pattern");
        }
        let mut index: usize = 0;
        for x in pattern.split(' ') {
            if x.len() == 0 {
                panic!("bad pattern");
            }
            for y in x.split([',', '，']) {
                let mut status: Status = Status::MIN;
                let mut min: Vec<u8> = Vec::new();
                let mut max: Vec<u8> = Vec::new();
                let mut separation: Vec<u8> = Vec::new();
                for z in y.as_bytes() {
                    match status {
                        Status::MIN => {
                            if z.is_ascii_digit() {
                                min.push(*z);
                            } else if *z == b'-' {
                                status = Status::MAX;
                            } else if *z == b'*' {
                                if 0 < min.len() {
                                    panic!("bad pattern");
                                }
                                min.extend_from_slice(table[index].1.to_string().as_bytes());
                                max.extend_from_slice(table[index].2.to_string().as_bytes());
                                status = Status::SLASH;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::MAX => {
                            if z.is_ascii_digit() {
                                max.push(*z);
                            } else if *z == b'/' {
                                status = Status::SEPARATION;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::SEPARATION => {
                            if z.is_ascii_digit() {
                                separation.push(*z);
                            } else {
                                panic!("bad pattern");
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
                            panic!("bad pattern");
                        } else {
                            max = min.clone();
                        }
                    }
                    Status::MAX => {
                        if max.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SEPARATION => {
                        if separation.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SLASH => {}
                }
                let min: u64 = if min.len() == 0 {
                    table[index].1
                } else {
                    String::from_utf8(min)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let max: u64 = if max.len() == 0 {
                    table[index].2
                } else {
                    String::from_utf8(max)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let separation: u64 = if separation.len() == 0 {
                    1
                } else {
                    String::from_utf8(separation)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                if min < table[index].1
                    || table[index].2 < min
                    || max < table[index].1
                    || table[index].2 < max
                    || max < min
                {
                    panic!("bad pattern");
                }
                for z in expand(min, max, separation) {
                    table[index].0[z as usize] = true;
                }
            } // for y in x.split(',') {...}
            index += 1;
        } // for x in pattern.split(' ') {...}
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            // 2 初始延迟 开始.
            let delay: u64 = schedule.delay3();
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize3().await;
            let second = &table[0];
            let minute = &table[1];
            let hour = &table[2];
            let day = &table[3];
            let month = &table[4];
            let weekday = &table[5];
            // 3 执行.
            while !stop.load(SeqCst) {
                let dt: DateTime = DateTime::new();
                if second.0[dt.second as usize]
                    && minute.0[dt.minute as usize]
                    && hour.0[dt.hour as usize]
                    && day.0[dt.day as usize]
                    && month.0[dt.month as usize]
                    && weekday.0[dt.weekday as usize]
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform3(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                let t: u64 = 200 + 1_000 - now() as u64 % 1_000;
                sleep(t).await;
            }
            // 4 结束.
            schedule.finish3().await;
        });
        // 5 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    /// 定时任务.
    pub async fn schedule4(&self, schedule: Arc<dyn Schedule4>) {
        if schedule.sleep_before_perform4() != 0 {
            self.schedule_4_1(schedule).await;
        } else if schedule.sleep_after_perform4() != 0 {
            self.schedule_4_2(schedule).await;
        } else if schedule.schedule_by_pattern4().len() != 0 {
            self.schedule_4_3(schedule).await;
        } else {
            panic!("trait [Schedule]必须实现 sleep_before_perform, sleep_after_perform, schedule_pattern 中的任意一个");
        }
        return;
    }

    async fn schedule_4_1(&self, schedule: Arc<dyn Schedule4>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay4();
            let period: u64 = schedule.sleep_before_perform4();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize4().await;
            while !stop.load(SeqCst) {
                // 2 等待并执行.
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform4(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish4().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_4_2(&self, schedule: Arc<dyn Schedule4>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay4();
            let period: u64 = schedule.sleep_after_perform4();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize4().await;
            while !stop.load(SeqCst) {
                // 2 执行后等待.
                if !schedule.perform4(stop.clone()).await {
                    break;
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish4().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_4_3(&self, schedule: Arc<dyn Schedule4>) {
        let pattern: String = schedule.schedule_by_pattern4();
        // 1 解析.
        // 在'*'可能有'/', 即SLASH.
        enum Status {
            MIN,
            MAX,
            SEPARATION,
            SLASH,
        }
        let expand = |mut min: u64, max: u64, separation: u64| {
            let mut output: Vec<u64> = Vec::new();
            while min <= max {
                output.push(min);
                min += separation;
            }
            return output;
        };
        let mut table: [([bool; 60], u64, u64); 6] = [
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
            panic!("bad pattern");
        }
        let mut index: usize = 0;
        for x in pattern.split(' ') {
            if x.len() == 0 {
                panic!("bad pattern");
            }
            for y in x.split([',', '，']) {
                let mut status: Status = Status::MIN;
                let mut min: Vec<u8> = Vec::new();
                let mut max: Vec<u8> = Vec::new();
                let mut separation: Vec<u8> = Vec::new();
                for z in y.as_bytes() {
                    match status {
                        Status::MIN => {
                            if z.is_ascii_digit() {
                                min.push(*z);
                            } else if *z == b'-' {
                                status = Status::MAX;
                            } else if *z == b'*' {
                                if 0 < min.len() {
                                    panic!("bad pattern");
                                }
                                min.extend_from_slice(table[index].1.to_string().as_bytes());
                                max.extend_from_slice(table[index].2.to_string().as_bytes());
                                status = Status::SLASH;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::MAX => {
                            if z.is_ascii_digit() {
                                max.push(*z);
                            } else if *z == b'/' {
                                status = Status::SEPARATION;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::SEPARATION => {
                            if z.is_ascii_digit() {
                                separation.push(*z);
                            } else {
                                panic!("bad pattern");
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
                            panic!("bad pattern");
                        } else {
                            max = min.clone();
                        }
                    }
                    Status::MAX => {
                        if max.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SEPARATION => {
                        if separation.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SLASH => {}
                }
                let min: u64 = if min.len() == 0 {
                    table[index].1
                } else {
                    String::from_utf8(min)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let max: u64 = if max.len() == 0 {
                    table[index].2
                } else {
                    String::from_utf8(max)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let separation: u64 = if separation.len() == 0 {
                    1
                } else {
                    String::from_utf8(separation)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                if min < table[index].1
                    || table[index].2 < min
                    || max < table[index].1
                    || table[index].2 < max
                    || max < min
                {
                    panic!("bad pattern");
                }
                for z in expand(min, max, separation) {
                    table[index].0[z as usize] = true;
                }
            } // for y in x.split(',') {...}
            index += 1;
        } // for x in pattern.split(' ') {...}
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            // 2 初始延迟 开始.
            let delay: u64 = schedule.delay4();
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize4().await;
            let second = &table[0];
            let minute = &table[1];
            let hour = &table[2];
            let day = &table[3];
            let month = &table[4];
            let weekday = &table[5];
            // 3 执行.
            while !stop.load(SeqCst) {
                let dt: DateTime = DateTime::new();
                if second.0[dt.second as usize]
                    && minute.0[dt.minute as usize]
                    && hour.0[dt.hour as usize]
                    && day.0[dt.day as usize]
                    && month.0[dt.month as usize]
                    && weekday.0[dt.weekday as usize]
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform4(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                let t: u64 = 200 + 1_000 - now() as u64 % 1_000;
                sleep(t).await;
            }
            // 4 结束.
            schedule.finish4().await;
        });
        // 5 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    /// 定时任务.
    pub async fn schedule5(&self, schedule: Arc<dyn Schedule5>) {
        if schedule.sleep_before_perform5() != 0 {
            self.schedule_5_1(schedule).await;
        } else if schedule.sleep_after_perform5() != 0 {
            self.schedule_5_2(schedule).await;
        } else if schedule.schedule_by_pattern5().len() != 0 {
            self.schedule_5_3(schedule).await;
        } else {
            panic!("trait [Schedule]必须实现 sleep_before_perform, sleep_after_perform, schedule_pattern 中的任意一个");
        }
        return;
    }

    async fn schedule_5_1(&self, schedule: Arc<dyn Schedule5>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay5();
            let period: u64 = schedule.sleep_before_perform5();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize5().await;
            while !stop.load(SeqCst) {
                // 2 等待并执行.
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform5(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish5().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_5_2(&self, schedule: Arc<dyn Schedule5>) {
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            let delay: u64 = schedule.delay5();
            let period: u64 = schedule.sleep_after_perform5();
            // 1 初始延迟 开始.
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize5().await;
            while !stop.load(SeqCst) {
                // 2 执行后等待.
                if !schedule.perform5(stop.clone()).await {
                    break;
                }
                {
                    let a = period / 1_000;
                    let b = period % 1_000;
                    for _ in 0..a {
                        if !stop.load(SeqCst) {
                            sleep(1_000).await;
                        }
                    }
                    if !stop.load(SeqCst) && 0 < b {
                        sleep(b).await;
                    }
                }
            }
            // 3 结束.
            schedule.finish5().await;
        });
        // 4 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }

    async fn schedule_5_3(&self, schedule: Arc<dyn Schedule5>) {
        let pattern: String = schedule.schedule_by_pattern5();
        // 1 解析.
        // 在'*'可能有'/', 即SLASH.
        enum Status {
            MIN,
            MAX,
            SEPARATION,
            SLASH,
        }
        let expand = |mut min: u64, max: u64, separation: u64| {
            let mut output: Vec<u64> = Vec::new();
            while min <= max {
                output.push(min);
                min += separation;
            }
            return output;
        };
        let mut table: [([bool; 60], u64, u64); 6] = [
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
            panic!("bad pattern");
        }
        let mut index: usize = 0;
        for x in pattern.split(' ') {
            if x.len() == 0 {
                panic!("bad pattern");
            }
            for y in x.split([',', '，']) {
                let mut status: Status = Status::MIN;
                let mut min: Vec<u8> = Vec::new();
                let mut max: Vec<u8> = Vec::new();
                let mut separation: Vec<u8> = Vec::new();
                for z in y.as_bytes() {
                    match status {
                        Status::MIN => {
                            if z.is_ascii_digit() {
                                min.push(*z);
                            } else if *z == b'-' {
                                status = Status::MAX;
                            } else if *z == b'*' {
                                if 0 < min.len() {
                                    panic!("bad pattern");
                                }
                                min.extend_from_slice(table[index].1.to_string().as_bytes());
                                max.extend_from_slice(table[index].2.to_string().as_bytes());
                                status = Status::SLASH;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::MAX => {
                            if z.is_ascii_digit() {
                                max.push(*z);
                            } else if *z == b'/' {
                                status = Status::SEPARATION;
                            } else {
                                panic!("bad pattern");
                            }
                        }
                        Status::SEPARATION => {
                            if z.is_ascii_digit() {
                                separation.push(*z);
                            } else {
                                panic!("bad pattern");
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
                            panic!("bad pattern");
                        } else {
                            max = min.clone();
                        }
                    }
                    Status::MAX => {
                        if max.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SEPARATION => {
                        if separation.len() == 0 {
                            panic!("bad pattern");
                        }
                    }
                    Status::SLASH => {}
                }
                let min: u64 = if min.len() == 0 {
                    table[index].1
                } else {
                    String::from_utf8(min)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let max: u64 = if max.len() == 0 {
                    table[index].2
                } else {
                    String::from_utf8(max)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                let separation: u64 = if separation.len() == 0 {
                    1
                } else {
                    String::from_utf8(separation)
                        .expect("String::from_utf8")
                        .parse::<u64>()
                        .expect("str::parse")
                };
                if min < table[index].1
                    || table[index].2 < min
                    || max < table[index].1
                    || table[index].2 < max
                    || max < min
                {
                    panic!("bad pattern");
                }
                for z in expand(min, max, separation) {
                    table[index].0[z as usize] = true;
                }
            } // for y in x.split(',') {...}
            index += 1;
        } // for x in pattern.split(' ') {...}
        let stop = self.stop.clone();
        let handle = tokio::task::spawn(async move {
            // 2 初始延迟 开始.
            let delay: u64 = schedule.delay5();
            {
                let a = delay / 1_000;
                let b = delay % 1_000;
                for _ in 0..a {
                    if !stop.load(SeqCst) {
                        sleep(1_000).await;
                    }
                }
                if !stop.load(SeqCst) && 0 < b {
                    sleep(b).await;
                }
            }
            schedule.initialize5().await;
            let second = &table[0];
            let minute = &table[1];
            let hour = &table[2];
            let day = &table[3];
            let month = &table[4];
            let weekday = &table[5];
            // 3 执行.
            while !stop.load(SeqCst) {
                let dt: DateTime = DateTime::new();
                if second.0[dt.second as usize]
                    && minute.0[dt.minute as usize]
                    && hour.0[dt.hour as usize]
                    && day.0[dt.day as usize]
                    && month.0[dt.month as usize]
                    && weekday.0[dt.weekday as usize]
                {
                    let schedule = schedule.clone();
                    let stop = stop.clone();
                    tokio::task::spawn(async move {
                        if !schedule.perform5(stop.clone()).await {
                            stop.store(true, SeqCst);
                        }
                    });
                }
                let t: u64 = 200 + 1_000 - now() as u64 % 1_000;
                sleep(t).await;
            }
            // 4 结束.
            schedule.finish5().await;
        });
        // 5 handle管理.
        self.thread_handles.lock().await.push(handle);
        return;
    }
}

// Function.

/// 当前系统的时间戳, 单位:毫秒.
pub fn now() -> i64 {
    #[cfg(target_os = "linux")]
    unsafe {
        // struct timeval {
        //     time_t      tv_sec;     /* seconds */
        //     suseconds_t tv_usec;    /* microseconds */
        // };
        // struct timezone {
        //     int tz_minuteswest;     /* minutes west of Greenwich */
        //     int tz_dsttime;         /* type of DST correction */
        // };
        // int gettimeofday(struct timeval *tv, struct timezone *tz);
        use std::ffi::c_int;
        use std::ffi::c_long;
        #[derive(Debug, Clone, Default, PartialEq)]
        #[repr(C)]
        pub struct TimeValue {
            pub tv_sec: c_long,
            pub tv_usec: c_long,
        }
        #[derive(Debug, Clone, Default, PartialEq)]
        #[repr(C)]
        pub struct TimeZone {
            pub tz_minuteswest: c_int,
            pub tz_dsttime: c_int,
        }
        extern "C" {
            fn gettimeofday(tv: *mut TimeValue, tz: *mut TimeZone) -> c_int;
        }
        let mut tv: TimeValue = Default::default();
        let mut tz: TimeZone = Default::default();
        if gettimeofday(&mut tv, &mut tz) != 0 {
            return 0;
        }
        return tv.tv_sec as i64 * 1_000 + tv.tv_usec as i64 / 1_000;
    }
    #[cfg(target_os = "windows")]
    unsafe {
        // typedef struct _SYSTEMTIME {
        //     WORD wYear;
        //     WORD wMonth;
        //     WORD wDayOfWeek;
        //     WORD wDay;
        //     WORD wHour;
        //     WORD wMinute;
        //     WORD wSecond;
        //     WORD wMilliseconds;
        // } SYSTEMTIME, *PSYSTEMTIME, *LPSYSTEMTIME;
        // void GetLocalTime(
        //         [out] LPSYSTEMTIME lpSystemTime
        //         );
        // time_t time(time_t *);
        use std::ffi::c_long;
        use std::ffi::c_short;
        #[allow(non_snake_case)]
        #[derive(Debug, Clone, Default)]
        #[repr(C)]
        struct SYSTEMTIME {
            wYear: c_short,
            wMonth: c_short,
            wDayOfWeek: c_short,
            wDay: c_short,
            wHour: c_short,
            wMinute: c_short,
            wSecond: c_short,
            wMilliseconds: c_short,
        }
        extern "C" {
            fn GetLocalTime(lpSystemTime: *mut SYSTEMTIME);
            fn time(t: *mut c_long) -> c_long;
        }
        let mut st: SYSTEMTIME = Default::default();
        GetLocalTime(&mut st);
        let mut t: c_long = 0;
        time(&mut t);
        return t as i64 * 1_000 + st.wMilliseconds as i64;
    }
}

/// 当前系统的时间戳, 单位:秒.
pub fn now_seconds() -> i64 {
    return now() / 1_000;
}

/// 延时, 单位:毫秒.
pub fn sleep(t: u64) -> Sleep {
    return tokio::time::sleep(Duration::from_millis(t));
}
