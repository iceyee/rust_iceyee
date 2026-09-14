// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
/* clippy: 本crate风格是每个函数显式写return; 取模判断比is_multiple_of可读. */
#![allow(clippy::manual_is_multiple_of, clippy::needless_return)]

/* Use. */

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use tokio::task::JoinHandle;
use tokio::time::Sleep;

/// 1毫秒, 单位:毫秒.
pub const ONE_MILLISECOND: i64 = 1;
/// 1秒, 单位:毫秒.
pub const ONE_SECOND: i64 = 1_000 * ONE_MILLISECOND;
/// 1分钟, 单位:毫秒.
pub const ONE_MINUTE: i64 = 60 * ONE_SECOND;
/// 1小时, 单位:毫秒.
pub const ONE_HOUR: i64 = 60 * ONE_MINUTE;
/// 1天, 单位:毫秒.
pub const ONE_DAY: i64 = 24 * ONE_HOUR;
/// 1周, 单位:毫秒.
pub const ONE_WEEK: i64 = 7 * ONE_DAY;
/// 31天, 单位:毫秒.
///
/// 这里只是一个固定的31天, 不是"日历上的一个月", 需要按月份计算时请用[DateTime].
pub const ONE_MONTH: i64 = 31 * ONE_DAY;
/// 1个平年, 365天, 单位:毫秒.
pub const ONE_YEAR: i64 = 365 * ONE_DAY;
/// 1个闰年周期, 4年, 1461天, 单位:毫秒.
pub const FOUR_YEAR: i64 = 4 * ONE_YEAR + ONE_DAY;
/// 1个世纪, 100年, 36524天, 单位:毫秒.
///
/// 100年里只有24个闰年, 所以是25个4年周期减去1天.
pub const ONE_HUNDRED_YEAR: i64 = 25 * FOUR_YEAR - ONE_DAY;
/// 1个公历大周期, 400年, 146097天, 单位:毫秒.
///
/// 400年里只有97个闰年, 所以是4个世纪加上1天.
pub const FOUR_HUNDRED_YEAR: i64 = 4 * ONE_HUNDRED_YEAR + ONE_DAY;
/// 公元0年1月1日0时0分0秒0毫秒的"时间戳", 单位:毫秒.
///
/// 这是本crate内部的纪元, [DateTime]的分解与合成都以它为基准.
/// 它等于 0000-01-01 到 1970-01-01 之间的毫秒数, 也就是719528天.
pub const TIME_0: i64 =
    4 * FOUR_HUNDRED_YEAR + 3 * ONE_HUNDRED_YEAR + ONE_DAY + 17 * FOUR_YEAR + 2 * ONE_YEAR;

/// [TIME_0]对应的天数, 即0000-01-01到1970-01-01之间的天数.
const EPOCH_DAY: i64 = TIME_0 / ONE_DAY;

/* Enum. */

/* 调度模式. */
#[derive(Clone, Copy)]
enum ScheduleMode {
    /* 每次执行之前等待. */
    SleepBeforePerform,
    /* 每次执行之后等待. */
    SleepAfterPerform,
    /* 按cron表达式. */
    ScheduleByPattern,
}

/* Trait. */

/* 内部统一接口, 把ScheduleN适配成同一套方法, 调度逻辑只写一遍. */
trait ScheduleAdapter: Send + Sync {
    fn delay(&self) -> u64;

    fn sleep_before_perform(&self) -> u64;

    fn sleep_after_perform(&self) -> u64;

    fn schedule_by_pattern(&self) -> String;

    fn initialize<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b;

    fn perform<'a, 'b>(
        &'a self,
        stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b;

    fn finish<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b;
}

/* Struct. */

/// 时区所对应的时间偏移.
/// 比如用+0800表示东八区的时间偏移, 即+08:00.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct TimeOffset(pub i16);

impl std::default::Default for TimeOffset {
    /// 默认返回系统设置的时区.
    fn default() -> Self {
        thread_local! {
            /* 已经是const初始化; clippy 1.98 的missing_const_for_thread_local对const块会误报, 故忽略. */
            #[allow(clippy::missing_const_for_thread_local)]
            static TIME_OFFSET: Cell<Option<TimeOffset>> = const { Cell::new(None) };
        }
        if TIME_OFFSET.get().is_some() {
            return TIME_OFFSET.get().expect("NEVER");
        }
        #[cfg(target_os = "linux")]
        {
            // extern long timezone;
            // void tzset ();
            use std::ffi::c_long;
            unsafe extern "C" {
                static mut timezone: c_long;
                fn tzset();
            }
            unsafe {
                tzset();
                let offset_hour: i16 = timezone as i16 / 60 / 60;
                let offset_minute: i16 = timezone as i16 / 60 % 60;
                let t: i16 = -(offset_hour * 100 + offset_minute);
                TIME_OFFSET.set(Some(TimeOffset(t)));
            }
            return TIME_OFFSET.get().expect("NEVER");
        }
        #[cfg(target_os = "windows")]
        {
            /* typedef struct _SYSTEMTIME {
             *     WORD wYear;
             *     WORD wMonth;
             *     WORD wDayOfWeek;
             *     WORD wDay;
             *     WORD wHour;
             *     WORD wMinute;
             *     WORD wSecond;
             *     WORD wMilliseconds;
             * } SYSTEMTIME, *PSYSTEMTIME, *LPSYSTEMTIME;
             * typedef struct _TIME_ZONE_INFORMATION {
             *     LONG       Bias;
             *     WCHAR      StandardName[32];
             *     SYSTEMTIME StandardDate;
             *     LONG       StandardBias;
             *     WCHAR      DaylightName[32];
             *     SYSTEMTIME DaylightDate;
             *     LONG       DaylightBias;
             * } TIME_ZONE_INFORMATION, *PTIME_ZONE_INFORMATION, *LPTIME_ZONE_INFORMATION;
             * DWORD GetTimeZoneInformation(
             *         [out] LPTIME_ZONE_INFORMATION lpTimeZoneInformation
             *         ); */
            use std::ffi::c_int;
            use std::ffi::c_long;
            use std::ffi::c_ushort;
            #[allow(non_snake_case)]
            #[derive(Debug, Clone, Default)]
            #[repr(C)]
            struct SystemTime {
                wYear: c_ushort,
                wMonth: c_ushort,
                wDayOfWeek: c_ushort,
                wDay: c_ushort,
                wHour: c_ushort,
                wMinute: c_ushort,
                wSecond: c_ushort,
                wMilliseconds: c_ushort,
            }
            #[allow(non_snake_case)]
            #[derive(Debug, Clone, Default)]
            #[repr(C)]
            struct TimeZoneInformation {
                Bias: c_long,
                StandardName: [c_ushort; 32],
                StandardDate: SystemTime,
                StandardBias: c_long,
                DaylightName: [c_ushort; 32],
                DaylightDate: SystemTime,
                DaylightBias: c_long,
            }
            unsafe extern "C" {
                fn GetTimeZoneInformation(lpTimeZoneInformation: *mut TimeZoneInformation)
                -> c_int;
            }
            let mut tzi: TimeZoneInformation = Default::default();
            unsafe { GetTimeZoneInformation(&mut tzi) };
            let offset_hour: i16 = tzi.Bias as i16 / 60;
            let offset_minute: i16 = tzi.Bias as i16 % 60;
            let t: i16 = -(offset_hour * 100 + offset_minute);
            TIME_OFFSET.set(Some(TimeOffset(t)));
            return TIME_OFFSET.get().expect("NEVER");
        }
    }
}

/// 日期时间.
#[derive(Debug, Clone, PartialEq)]
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
    /// \[1, 366]
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
    /// let _: iceyee_time::DateTime =
    ///     iceyee_time::DateTime::from((iceyee_time::now(), None));
    /// ```
    // 不实现Default: 默认值会让month, day, weekday都落在非法范围里.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        return DateTime::from((now(), None));
    }

    /// 转成国际标准时间, 等同于:
    ///
    /// ```
    /// let dt: iceyee_time::DateTime = iceyee_time::DateTime::new();
    /// let _: iceyee_time::DateTime = iceyee_time::DateTime::from((
    ///     dt.timestamp,
    ///     Some(iceyee_time::TimeOffset(0)),
    /// ));
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
        let (timestamp, offset) = value;
        let offset: TimeOffset = offset.unwrap_or(TimeOffset::default());
        /* 换算到本地时间, 单位:毫秒. */
        let local: i64 = timestamp.wrapping_add(offset_to_millisecond(offset));
        /* 天数与当天剩余的毫秒, 用欧几里得除法保证余数非负. */
        let days: i64 = local.div_euclid(ONE_DAY).wrapping_add(EPOCH_DAY);
        let rest: i64 = local.rem_euclid(ONE_DAY);
        let (year, month, day): (i64, i64, i64) = days_to_civil(days);
        let hour: i64 = rest / ONE_HOUR;
        let minute: i64 = rest % ONE_HOUR / ONE_MINUTE;
        let second: i64 = rest % ONE_MINUTE / ONE_SECOND;
        let millisecond: i64 = rest % ONE_SECOND;
        let day_of_year: i64 = days.wrapping_sub(civil_to_days(year, 1, 1)) + 1;
        /* 0000-01-01是周六, 所以补5再取模. */
        let weekday: i64 = days.wrapping_add(5).rem_euclid(7) + 1;
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
            timestamp,
            offset,
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
    /// 例如, `月`的范围是1~12, 如果对应的入参是0或者13, 就会panic.
    ///
    /// 例如, 某一年的二月份, 只有28天, 但是参数`日`的入参是29, 就会panic.
    fn from(value: (u64, u64, u64, u64, u64, u64, u64, Option<TimeOffset>)) -> Self {
        let (year, month, day, hour, minute, second, millisecond, offset) = value;
        if month == 0 || 12 < month {
            panic!("@month={month:?}, 范围[1, 12]");
        }
        let max_days: u64 = days_in_month(year, month);
        if day == 0 || max_days < day {
            panic!("@day={day:?}, {year}年{month}月的范围[1, {max_days}]");
        }
        if 23 < hour {
            panic!("@hour={hour:?}, 范围[0, 23]");
        }
        if 59 < minute {
            panic!("@minute={minute:?}, 范围[0, 59]");
        }
        if 59 < second {
            panic!("@second={second:?}, 范围[0, 59]");
        }
        if 999 < millisecond {
            panic!("@millisecond={millisecond:?}, 范围[0, 999]");
        }
        let offset: TimeOffset = offset.unwrap_or(TimeOffset::default());
        /* 先算出本地时间的"距1970-01-01的毫秒数", 再减掉时区偏移得到时间戳. */
        let mut timestamp: i64 = civil_to_days(year as i64, month as i64, day as i64)
            .wrapping_sub(EPOCH_DAY)
            .wrapping_mul(ONE_DAY);
        timestamp = timestamp.wrapping_add(hour as i64 * ONE_HOUR);
        timestamp = timestamp.wrapping_add(minute as i64 * ONE_MINUTE);
        timestamp = timestamp.wrapping_add(second as i64 * ONE_SECOND);
        timestamp = timestamp.wrapping_add(millisecond as i64 * ONE_MILLISECOND);
        timestamp = timestamp.wrapping_sub(offset_to_millisecond(offset));
        return Self::from((timestamp, Some(offset)));
    }
}

impl std::fmt::Display for DateTime {
    /// 转成字符串, 使用RFC3339标准, 格式'xx-xx-xxTxx:xx:xx.xxx\[+/-\]xx:xx'.
    ///
    /// 年份不足4位时补0, 超过4位时按RFC3339在前面加'+'.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let year: &str = if 9_999 < self.year { "+" } else { "" };
        if self.offset.0 == 0 {
            return write!(
                f,
                "{year}{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
                self.year,
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second,
                self.millisecond
            );
        }
        let offset: i32 = (self.offset.0 as i32).abs();
        return write!(
            f,
            "{year}{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{}{:02}:{:02}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
            if self.offset.0 < 0 { "-" } else { "+" },
            offset / 100 % 100,
            offset % 100
        );
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
        static TIMER: OnceLock<Timer> = OnceLock::new();
        return TIMER.get_or_init(Timer::new).clone();
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

    /// 停止定时器并等待所有任务结束.
    pub async fn stop_and_wait(&self) {
        let mut thread_handles = self.thread_handles.lock().await;
        self.stop.store(true, SeqCst);
        while let Some(handle) = thread_handles.pop() {
            handle.await.expect("JoinHandle::await");
        }
        return;
    }

    /* 统一的调度逻辑, 三种模式共用. */
    async fn run(&self, schedule: Arc<dyn ScheduleAdapter>, mode: ScheduleMode) {
        let stop: Arc<AtomicBool> = self.stop.clone();
        /* cron表达式在spawn之前解析, 解析失败直接把panic抛给调用方. */
        let table: Option<CronTable> = if let ScheduleMode::ScheduleByPattern = mode {
            Some(parse_cron(&schedule.schedule_by_pattern()))
        } else {
            None
        };
        let handle: JoinHandle<()> = tokio::task::spawn(async move {
            /* 1 初始延迟. */
            sleep_by_second(&stop, schedule.delay()).await;
            schedule.initialize().await;
            /* 2 执行. */
            match mode {
                ScheduleMode::SleepBeforePerform => {
                    let period: u64 = schedule.sleep_before_perform();
                    while !stop.load(SeqCst) {
                        spawn_perform(schedule.clone(), stop.clone());
                        sleep_by_second(&stop, period).await;
                    }
                }
                ScheduleMode::SleepAfterPerform => {
                    let period: u64 = schedule.sleep_after_perform();
                    while !stop.load(SeqCst) {
                        if !schedule.perform(stop.clone()).await {
                            break;
                        }
                        sleep_by_second(&stop, period).await;
                    }
                }
                ScheduleMode::ScheduleByPattern => {
                    let table: CronTable = table.expect("NEVER");
                    while !stop.load(SeqCst) {
                        if table.matches(&DateTime::new()) {
                            spawn_perform(schedule.clone(), stop.clone());
                        }
                        /* 对齐到整秒之后的200毫秒, 避免同一秒里重复触发. */
                        let t: u64 = 200 + 1_000 - now() as u64 % 1_000;
                        sleep(t).await;
                    }
                }
            }
            /* 3 结束. */
            schedule.finish().await;
            return;
        });
        /* 4 handle管理. */
        self.thread_handles.lock().await.push(handle);
        return;
    }
}

/* Function. */

/// 当前系统的时间戳, 单位:毫秒.
pub fn now() -> i64 {
    #[cfg(target_os = "linux")]
    {
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
        struct TimeValue {
            pub tv_sec: c_long,
            pub tv_usec: c_long,
        }
        #[derive(Debug, Clone, Default, PartialEq)]
        #[repr(C)]
        struct TimeZone {
            pub tz_minuteswest: c_int,
            pub tz_dsttime: c_int,
        }
        unsafe extern "C" {
            fn gettimeofday(tv: *mut TimeValue, tz: *mut TimeZone) -> c_int;
        }
        let mut tv: TimeValue = Default::default();
        let mut tz: TimeZone = Default::default();
        if unsafe { gettimeofday(&mut tv, &mut tz) } != 0 {
            return 0;
        }
        return tv.tv_sec as i64 * 1_000 + tv.tv_usec as i64 / 1_000;
    }
    #[cfg(target_os = "windows")]
    {
        /* typedef struct _SYSTEMTIME {
         *     WORD wYear;
         *     WORD wMonth;
         *     WORD wDayOfWeek;
         *     WORD wDay;
         *     WORD wHour;
         *     WORD wMinute;
         *     WORD wSecond;
         *     WORD wMilliseconds;
         * } SYSTEMTIME, *PSYSTEMTIME, *LPSYSTEMTIME;
         * void GetLocalTime(
         *         [out] LPSYSTEMTIME lpSystemTime
         *         );
         * time_t time(time_t *); */
        use std::ffi::c_short;
        #[allow(non_snake_case)]
        #[derive(Debug, Clone, Default)]
        #[repr(C)]
        struct SystemTime {
            wYear: c_short,
            wMonth: c_short,
            wDayOfWeek: c_short,
            wDay: c_short,
            wHour: c_short,
            wMinute: c_short,
            wSecond: c_short,
            wMilliseconds: c_short,
        }
        unsafe extern "C" {
            fn GetLocalTime(lpSystemTime: *mut SystemTime);
            fn time(t: *mut i64) -> i64;
        }
        let mut st: SystemTime = Default::default();
        unsafe { GetLocalTime(&mut st) };
        let mut t: i64 = 0;
        unsafe { time(&mut t) };
        return t * 1_000 + st.wMilliseconds as i64;
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

/* 把时区偏移换算成毫秒, 比如+0800换算成+8小时. */
fn offset_to_millisecond(offset: TimeOffset) -> i64 {
    let hour: i64 = (offset.0 / 100 % 100) as i64;
    let minute: i64 = (offset.0 % 100) as i64;
    return (hour * 60 + minute) * 60 * 1_000;
}

/* 是否闰年. */
fn is_leap_year(year: u64) -> bool {
    return year % 400 == 0 || year % 100 != 0 && year % 4 == 0;
}

/* 某年某月有多少天. */
fn days_in_month(year: u64, month: u64) -> u64 {
    return match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    };
}

/* 把公历日期换算成"距0000-01-01的天数", 支持负年份.
 *
 * 算法来自 Howard Hinnant 的 days_from_civil.
 * 以3月为一年之首(1月2月算作上一年的末尾), 这样闰日永远落在年末, 不用分支判断. */
fn civil_to_days(year: i64, month: i64, day: i64) -> i64 {
    /* 以3月为一年之首, 1月和2月归到上一年. */
    let y: i64 = if month <= 2 {
        year.wrapping_sub(1)
    } else {
        year
    };
    let era: i64 = if 0 <= y { y } else { y.wrapping_sub(399) } / 400;
    let yoe: i64 = y.wrapping_sub(era.wrapping_mul(400));
    let mp: i64 = if 2 < month {
        month.wrapping_sub(3)
    } else {
        month.wrapping_add(9)
    };
    let doy: i64 = (153 * mp + 2) / 5 + day - 1;
    let doe: i64 = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    return era
        .wrapping_mul(146_097)
        .wrapping_add(doe)
        .wrapping_sub(719_468)
        .wrapping_add(EPOCH_DAY);
}

/* 把"距0000-01-01的天数"换算成公历日期, 支持负年份.
 *
 * 算法来自 Howard Hinnant 的 civil_from_days. */
fn days_to_civil(days: i64) -> (i64, i64, i64) {
    let z: i64 = days.wrapping_sub(EPOCH_DAY).wrapping_add(719_468);
    let era: i64 = if 0 <= z { z } else { z.wrapping_sub(146_096) } / 146_097;
    let doe: i64 = z.wrapping_sub(era.wrapping_mul(146_097));
    let yoe: i64 = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y: i64 = yoe.wrapping_add(era.wrapping_mul(400));
    let doy: i64 = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp: i64 = (5 * doy + 2) / 153;
    let day: i64 = doy - (153 * mp + 2) / 5 + 1;
    let month: i64 = mp + if mp < 10 { 3 } else { -9 };
    let year: i64 = y + if month <= 2 { 1 } else { 0 };
    return (year, month, day);
}

/* 按秒等待, 每秒检查一次停止标志, 让停止动作能在一秒内生效. */
async fn sleep_by_second(stop: &Arc<AtomicBool>, t: u64) {
    let a: u64 = t / 1_000;
    let b: u64 = t % 1_000;
    for _ in 0..a {
        if !stop.load(SeqCst) {
            sleep(1_000).await;
        }
    }
    if !stop.load(SeqCst) && 0 < b {
        sleep(b).await;
    }
    return;
}

/* 执行一次任务, 返回false则停止整个调度. */
fn spawn_perform(schedule: Arc<dyn ScheduleAdapter>, stop: Arc<AtomicBool>) {
    tokio::task::spawn(async move {
        if !schedule.perform(stop.clone()).await {
            stop.store(true, SeqCst);
        }
        return;
    });
    return;
}

/* cron表达式的解析结果. */
struct CronTable {
    second: [bool; 60],
    minute: [bool; 60],
    hour: [bool; 24],
    day: [bool; 32],
    month: [bool; 13],
    weekday: [bool; 8],
}

impl CronTable {
    fn new() -> Self {
        return Self {
            second: [false; 60],
            minute: [false; 60],
            hour: [false; 24],
            day: [false; 32],
            month: [false; 13],
            weekday: [false; 8],
        };
    }

    /* 按下标取字段表. */
    fn field(&mut self, index: usize) -> &mut [bool] {
        return match index {
            0 => &mut self.second,
            1 => &mut self.minute,
            2 => &mut self.hour,
            3 => &mut self.day,
            4 => &mut self.month,
            _ => &mut self.weekday,
        };
    }

    /* 是否命中. */
    fn matches(&self, dt: &DateTime) -> bool {
        return self.second[dt.second as usize]
            && self.minute[dt.minute as usize]
            && self.hour[dt.hour as usize]
            && self.day[dt.day as usize]
            && self.month[dt.month as usize]
            && self.weekday[dt.weekday as usize];
    }
}

// 解析cron表达式.
//
// 6个字段, 依次是: 秒 分 时 日 月 周.
// 每个字段支持: 星号(全部), n(单个值), n-m(区间), 星号/步长, n-m/步长, n/步长,
// 也可以用逗号分隔多个值, 逗号支持全角.
// 'n/步长'表示从n开始, 直到该字段的上限.
fn parse_cron(pattern: &str) -> CronTable {
    let names: [&str; 6] = ["秒", "分", "时", "日", "月", "周"];
    let limits: [(u64, u64); 6] = [(0, 59), (0, 59), (0, 23), (1, 31), (1, 12), (1, 7)];
    /* 归一化空白. */
    let mut pattern: String = pattern.to_string();
    while pattern.contains("  ") {
        pattern = pattern.replace("  ", " ");
    }
    let pattern: String = pattern.trim().to_string();
    let fields: Vec<&str> = pattern.split(' ').collect();
    if fields.len() != names.len() {
        panic!(
            "bad pattern: 需要{:?}这{}个字段, 实际是{pattern:?}",
            names,
            names.len()
        );
    }
    let mut table: CronTable = CronTable::new();
    for (index, field) in fields.iter().enumerate() {
        let (min_limit, max_limit) = limits[index];
        for item in field.split([',', '，']) {
            let (min, max, step): (u64, u64, u64) =
                parse_cron_item(item, min_limit, max_limit, names[index], field);
            let t: &mut [bool] = table.field(index);
            let mut v: u64 = min;
            while v <= max {
                t[v as usize] = true;
                v = match v.checked_add(step) {
                    Some(next) => next,
                    None => break,
                };
            }
        }
    }
    return table;
}

// 解析cron表达式里的一个字段项.
//
// - @param item 字段项, 比如"星号/5".
// - @param min_limit 该字段的取值下限.
// - @param max_limit 该字段的取值上限.
// - @param name 该字段的名称, 用于报错.
// - @param field 该字段的原文, 用于报错.
//
// - @return (下限, 上限, 步长)
//
// # Panics
//
// 数值越界, 起始值大于结束值, 步长为0, 或出现未预期的字符. */
fn parse_cron_item(
    item: &str,
    min_limit: u64,
    max_limit: u64,
    name: &str,
    field: &str,
) -> (u64, u64, u64) {
    let (body, has_step, step): (&str, bool, u64) = match item.split_once('/') {
        Some((body, step)) => {
            let step: u64 = step.parse().unwrap_or_else(|_| {
                panic!("bad pattern: {name}字段的步长不是数字: 字段={field:?}, 项={item:?}")
            });
            if step == 0 {
                panic!("bad pattern: {name}字段的步长是0: 字段={field:?}, 项={item:?}");
            }
            (body, true, step)
        }
        None => (item, false, 1),
    };
    let (min, max): (u64, u64) = if body == "*" {
        (min_limit, max_limit)
    } else if let Some((min, max)) = body.split_once('-') {
        (
            parse_cron_number(min, min_limit, max_limit, name, field),
            parse_cron_number(max, min_limit, max_limit, name, field),
        )
    } else {
        let v: u64 = parse_cron_number(body, min_limit, max_limit, name, field);
        // 'n/步长' 表示从n开始直到该字段的上限.
        if has_step { (v, max_limit) } else { (v, v) }
    };
    if max < min {
        panic!("bad pattern: {name}字段的起始值大于结束值: 字段={field:?}, 项={item:?}");
    }
    return (min, max, step);
}

// 解析cron表达式里的一个数值.
//
// # Panics
//
// 不是数字, 或者超出该字段的范围.
fn parse_cron_number(text: &str, min_limit: u64, max_limit: u64, name: &str, field: &str) -> u64 {
    let value: u64 = text
        .parse()
        .unwrap_or_else(|_| panic!("bad pattern: {name}字段不是数字: 字段={field:?}, 值={text:?}"));
    if value < min_limit || max_limit < value {
        panic!(
            "bad pattern: {name}字段的值{value}超出范围[{min_limit}, {max_limit}]: 字段={field:?}"
        );
    }
    return value;
}

/* Macro. */

/* 生成ScheduleN trait, 以及对应的适配器和Timer::scheduleN.
 *
 * 各个编号的调度逻辑完全一样, 只有方法名不同, 所以用宏生成, 逻辑只在Timer::run里写一遍. */
macro_rules! define_schedule {
    (
        $trait_name:ident,
        $schedule:ident,
        $delay:ident,
        $sleep_before:ident,
        $sleep_after:ident,
        $by_pattern:ident,
        $initialize:ident,
        $perform:ident,
        $finish:ident,
        $wrap:ident
    ) => {
        /// 定时任务.
        ///
        /// sleep_before_perform, sleep_after_perform, schedule_by_pattern表示三种不同的模式,
        /// 三者都为空时会panic.
        ///
        /// - @see [Timer]
        pub trait $trait_name: Send + Sync {
            /// 初始延迟, 单位:毫秒, 默认0.
            fn $delay(&self) -> u64 {
                return 0;
            }

            /// 每次执行之前的间隔, 单位:毫秒, 默认0.
            fn $sleep_before(&self) -> u64 {
                return 0;
            }

            /// 每次执行之后的间隔, 单位:毫秒, 默认0.
            fn $sleep_after(&self) -> u64 {
                return 0;
            }

            /// cron表达式, 默认空字符串.
            fn $by_pattern(&self) -> String {
                return "".to_string();
            }

            /// 在循环任务开始之前执行.
            fn $initialize<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
            where
                'a: 'b,
            {
                return Box::pin(async move {
                    return;
                });
            }

            /// 循环任务, 返回值表示是否继续循环.
            fn $perform<'a, 'b>(
                &'a self,
                _stop: Arc<AtomicBool>,
            ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
            where
                'a: 'b;

            /// 在循环任务结束之后执行.
            fn $finish<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
            where
                'a: 'b,
            {
                return Box::pin(async move {
                    return;
                });
            }

            /// 包装成[Arc].
            fn $wrap(self) -> Arc<dyn $trait_name>
            where
                Self: Sized + 'static,
            {
                return Arc::new(self);
            }
        }

        /* 适配到统一的内部接口. */
        impl ScheduleAdapter for Arc<dyn $trait_name> {
            fn delay(&self) -> u64 {
                return (**self).$delay();
            }

            fn sleep_before_perform(&self) -> u64 {
                return (**self).$sleep_before();
            }

            fn sleep_after_perform(&self) -> u64 {
                return (**self).$sleep_after();
            }

            fn schedule_by_pattern(&self) -> String {
                return (**self).$by_pattern();
            }

            fn initialize<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
            where
                'a: 'b,
            {
                return (**self).$initialize();
            }

            fn perform<'a, 'b>(
                &'a self,
                stop: Arc<AtomicBool>,
            ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
            where
                'a: 'b,
            {
                return (**self).$perform(stop);
            }

            fn finish<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
            where
                'a: 'b,
            {
                return (**self).$finish();
            }
        }

        impl Timer {
            /// 定时任务.
            pub async fn $schedule(&self, schedule: Arc<dyn $trait_name>) {
                let mode: ScheduleMode = if schedule.$sleep_before() != 0 {
                    ScheduleMode::SleepBeforePerform
                } else if schedule.$sleep_after() != 0 {
                    ScheduleMode::SleepAfterPerform
                } else if !schedule.$by_pattern().is_empty() {
                    ScheduleMode::ScheduleByPattern
                } else {
                    panic!("trait [Schedule]必须实现 sleep_before_perform, sleep_after_perform, schedule_by_pattern 中的任意一个");
                };
                let schedule: Arc<dyn ScheduleAdapter> = Arc::new(schedule);
                return self.run(schedule, mode).await;
            }
        }
    };
}

define_schedule!(
    Schedule0,
    schedule0,
    delay0,
    sleep_before_perform0,
    sleep_after_perform0,
    schedule_by_pattern0,
    initialize0,
    perform0,
    finish0,
    wrap0
);

define_schedule!(
    Schedule1,
    schedule1,
    delay1,
    sleep_before_perform1,
    sleep_after_perform1,
    schedule_by_pattern1,
    initialize1,
    perform1,
    finish1,
    wrap1
);

define_schedule!(
    Schedule2,
    schedule2,
    delay2,
    sleep_before_perform2,
    sleep_after_perform2,
    schedule_by_pattern2,
    initialize2,
    perform2,
    finish2,
    wrap2
);

define_schedule!(
    Schedule3,
    schedule3,
    delay3,
    sleep_before_perform3,
    sleep_after_perform3,
    schedule_by_pattern3,
    initialize3,
    perform3,
    finish3,
    wrap3
);

define_schedule!(
    Schedule4,
    schedule4,
    delay4,
    sleep_before_perform4,
    sleep_after_perform4,
    schedule_by_pattern4,
    initialize4,
    perform4,
    finish4,
    wrap4
);

define_schedule!(
    Schedule5,
    schedule5,
    delay5,
    sleep_before_perform5,
    sleep_after_perform5,
    schedule_by_pattern5,
    initialize5,
    perform5,
    finish5,
    wrap5
);

define_schedule!(
    Schedule6,
    schedule6,
    delay6,
    sleep_before_perform6,
    sleep_after_perform6,
    schedule_by_pattern6,
    initialize6,
    perform6,
    finish6,
    wrap6
);

define_schedule!(
    Schedule7,
    schedule7,
    delay7,
    sleep_before_perform7,
    sleep_after_perform7,
    schedule_by_pattern7,
    initialize7,
    perform7,
    finish7,
    wrap7
);

define_schedule!(
    Schedule8,
    schedule8,
    delay8,
    sleep_before_perform8,
    sleep_after_perform8,
    schedule_by_pattern8,
    initialize8,
    perform8,
    finish8,
    wrap8
);

define_schedule!(
    Schedule9,
    schedule9,
    delay9,
    sleep_before_perform9,
    sleep_after_perform9,
    schedule_by_pattern9,
    initialize9,
    perform9,
    finish9,
    wrap9
);
