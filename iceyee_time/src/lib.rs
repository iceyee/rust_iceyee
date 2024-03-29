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
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::Sleep;

const ONE_MILLISECOND: i64 = 1;
const ONE_SECOND: i64 = 1000 * ONE_MILLISECOND;
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
            self.timestamp % 1000
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
        if self.timestamp < other.timestamp {
            return Some(CmpOrdering::Less);
        } else if self.timestamp == other.timestamp {
            return Some(CmpOrdering::Equal);
        } else if self.timestamp > other.timestamp {
            return Some(CmpOrdering::Greater);
        } else {
            return None;
        }
    }
}

/// 时钟, 精度0ms ~ 100ms.
#[derive(Clone, Debug)]
pub struct Timer {
    thread_handles: Arc<Vec<JoinHandle<()>>>,
    stop_flag: Arc<AtomicBool>,
}

/// 默认的时钟, 这是全局变量.
impl std::default::Default for Timer {
    fn default() -> Self {
        static mut TIMER: Option<Timer> = None;
        unsafe {
            if TIMER.is_none() {
                TIMER = Some(Timer::new());
            }
            return TIMER.as_ref().expect("NEVER").clone();
        }
    }
}

impl Drop for Timer {
    /// 关闭时钟.
    fn drop(&mut self) {
        if Arc::strong_count(&self.thread_handles) == 1 {
            println!("Timer::drop().");
            let thread_handles = unsafe { Arc::get_mut_unchecked(&mut self.thread_handles) };
            thread_handles.clear();
            self.stop_flag.store(true, Ordering::SeqCst);
        }
        return;
    }
}

impl Timer {
    /// 创建新的时钟, 默认开启状态.
    pub fn new() -> Self {
        return Timer {
            thread_handles: Arc::new(Vec::new()),
            stop_flag: Arc::new(AtomicBool::new(false)),
        };
    }

    /// 启动时钟.
    pub fn start(&mut self) {
        let thread_handles = unsafe { Arc::get_mut_unchecked(&mut self.thread_handles) };
        thread_handles.clear();
        self.stop_flag.store(false, Ordering::SeqCst);
        return;
    }

    /// 停止时钟.
    pub fn stop(&mut self) {
        let thread_handles = unsafe { Arc::get_mut_unchecked(&mut self.thread_handles) };
        thread_handles.clear();
        self.stop_flag.store(true, Ordering::SeqCst);
        return;
    }

    /// 停止时钟并等待所有任务结束.
    pub async fn stop_and_wait(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);
        let thread_handles = unsafe { Arc::get_mut_unchecked(&mut self.thread_handles) };
        loop {
            match thread_handles.pop() {
                Some(handle) => handle.await.expect("JoinHandle::await"),
                None => break,
            }
        }
        return;
    }

    /// 定时任务, 模式匹配.
    ///
    /// - @param pattern "秒 分 时 日 月 周几", "second minute hour day month weekday", 可以参考linux的crontab.
    /// - @param f 任务, 参数是stop标志, 表示是否已经发出停止的信号.
    pub fn schedule_pattern<F1, F2>(&mut self, pattern: &str, mut f: F1) -> bool
    where
        F1: FnMut(Arc<AtomicBool>) -> F2 + Send + 'static,
        F2: Future<Output = ()> + Send + 'static,
    {
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
            return false;
        }
        let mut index: usize = 0;
        for x in pattern.split(' ') {
            if x.len() == 0 {
                return false;
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
                                    return false;
                                }
                                min.extend_from_slice(table[index].1.to_string().as_bytes());
                                max.extend_from_slice(table[index].2.to_string().as_bytes());
                                status = Status::SLASH;
                            } else {
                                return false;
                            }
                        }
                        Status::MAX => {
                            if z.is_ascii_digit() {
                                max.push(*z);
                            } else if *z == b'/' {
                                status = Status::SEPARATION;
                            } else {
                                return false;
                            }
                        }
                        Status::SEPARATION => {
                            if z.is_ascii_digit() {
                                separation.push(*z);
                            } else {
                                return false;
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
                            return false;
                        } else {
                            max = min.clone();
                        }
                    }
                    Status::MAX => {
                        if max.len() == 0 {
                            return false;
                        }
                    }
                    Status::SEPARATION => {
                        if separation.len() == 0 {
                            return false;
                        }
                    }
                    Status::SLASH => {}
                }
                let min: u64 = if min.len() == 0 {
                    table[index].1
                } else {
                    match String::from_utf8(min) {
                        Ok(s) => match s.parse::<u64>() {
                            Ok(s) => s,
                            Err(_) => return false,
                        },
                        Err(_) => return false,
                    }
                };
                let max: u64 = if max.len() == 0 {
                    table[index].2
                } else {
                    match String::from_utf8(max) {
                        Ok(s) => match s.parse::<u64>() {
                            Ok(s) => s,
                            Err(_) => return false,
                        },
                        Err(_) => return false,
                    }
                };
                let separation: u64 = if separation.len() == 0 {
                    1
                } else {
                    match String::from_utf8(separation) {
                        Ok(s) => match s.parse::<u64>() {
                            Ok(s) => s,
                            Err(_) => return false,
                        },
                        Err(_) => return false,
                    }
                };
                if min < table[index].1
                    || table[index].2 < min
                    || max < table[index].1
                    || table[index].2 < max
                    || max < min
                {
                    return false;
                }
                for z in expand(min, max, separation) {
                    table[index].0[z as usize] = true;
                }
            } // for y in x.split(',') {...}
            index += 1;
        } // for x in pattern.split(' ') {...}
          // 2 执行.
        let stop_flag_clone: Arc<AtomicBool> = self.stop_flag.clone();
        let handle = tokio::task::spawn(async move {
            let stop_flag = stop_flag_clone;
            let second = &table[0];
            let minute = &table[1];
            let hour = &table[2];
            let day = &table[3];
            let month = &table[4];
            let weekday = &table[5];
            while !stop_flag.load(Ordering::SeqCst) {
                let t: u64 = 200 + 1000 - now() as u64 % 1000;
                let sl = tokio::task::spawn(sleep(t as u64));
                let dt: DateTime = DateTime::new();
                if second.0[dt.second as usize]
                    && minute.0[dt.minute as usize]
                    && hour.0[dt.hour as usize]
                    && day.0[dt.day as usize]
                    && month.0[dt.month as usize]
                    && weekday.0[dt.weekday as usize]
                {
                    f(stop_flag.clone()).await;
                }
                while !stop_flag.load(Ordering::SeqCst) && !sl.is_finished() {
                    sleep(100).await;
                }
            }
        });
        // 3 handle管理.
        let thread_handles = unsafe { Arc::get_mut_unchecked(&mut self.thread_handles) };
        thread_handles.push(handle);
        return true;
    }

    /// 定时任务, 任务执行的同时等待.
    ///
    /// - @param delay 初始延迟, 单位:毫秒.
    /// - @param period 每轮任务的时间间隔, 单位:毫秒.
    /// - @param f 任务, 参数是stop标志, 表示是否已经发出停止的信号.
    pub fn schedule_execute_before<F1, F2>(&mut self, delay: u64, period: u64, mut f: F1)
    where
        F1: FnMut(Arc<AtomicBool>) -> F2 + Send + 'static,
        F2: Future<Output = ()> + Send + 'static,
    {
        let stop_flag_clone: Arc<AtomicBool> = self.stop_flag.clone();
        let handle = tokio::task::spawn(async move {
            let stop_flag = stop_flag_clone;
            // 1 初始延迟.
            let sl = tokio::task::spawn(sleep(delay));
            while !stop_flag.load(Ordering::SeqCst) && !sl.is_finished() {
                sleep(100).await;
            }
            while !stop_flag.load(Ordering::SeqCst) {
                // 2 等待并执行.
                let sl = tokio::task::spawn(sleep(period));
                f(stop_flag.clone()).await;
                while !stop_flag.load(Ordering::SeqCst) && !sl.is_finished() {
                    sleep(100).await;
                }
            }
        });
        // 3 handle管理.
        let thread_handles = unsafe { Arc::get_mut_unchecked(&mut self.thread_handles) };
        thread_handles.push(handle);
        return;
    }

    /// 定时任务, 在任务执行完成后等待.
    ///
    /// - @param delay 初始延迟, 单位:毫秒.
    /// - @param period 每轮任务的时间间隔, 单位:毫秒.
    /// - @param f 任务, 参数是stop标志, 表示是否已经发出停止的信号.
    pub fn schedule_execute_after<F1, F2>(&mut self, delay: u64, period: u64, mut f: F1)
    where
        F1: FnMut(Arc<AtomicBool>) -> F2 + Send + 'static,
        F2: Future<Output = ()> + Send + 'static,
    {
        let stop_flag_clone: Arc<AtomicBool> = self.stop_flag.clone();
        let handle = tokio::task::spawn(async move {
            let stop_flag = stop_flag_clone;
            // 1 初始延迟.
            let sl = tokio::task::spawn(sleep(delay));
            while !stop_flag.load(Ordering::SeqCst) && !sl.is_finished() {
                sleep(100).await;
            }
            while !stop_flag.load(Ordering::SeqCst) {
                // 2 执行并等待.
                f(stop_flag.clone()).await;
                let sl = tokio::task::spawn(sleep(period));
                while !stop_flag.load(Ordering::SeqCst) && !sl.is_finished() {
                    sleep(100).await;
                }
            }
        });
        // 3 handle管理.
        let thread_handles = unsafe { Arc::get_mut_unchecked(&mut self.thread_handles) };
        thread_handles.push(handle);
        return;
    }
}

// Function.

/// 返回当前系统的时间戳, 单位:毫秒.
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
        return tv.tv_sec as i64 * 1000 + tv.tv_usec as i64 / 1000;
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
        return t as i64 * 1000 + st.wMilliseconds as i64;
    }
}

/// 返回当前系统的时间戳, 单位:秒.
pub fn now_seconds() -> i64 {
    return now() / 1000;
}

/// 延时, 单位:毫秒.
pub fn sleep(t: u64) -> Sleep {
    return tokio::time::sleep(Duration::from_millis(t));
}

// pub fn init() {
//     #[cfg(target_os = "linux")]
//     unsafe {
//         // extern long timezone;
//         // void tzset ();
//         use std::ffi::c_long;
//         extern "C" {
//             static mut timezone: c_long;
//             fn tzset();
//         }
//         tzset();
//         let offset_hour: i16 = timezone as i16 / 60 / 60;
//         let offset_minute: i16 = timezone as i16 / 60 % 60;
//         TIME_OFFSET = Some(-(offset_hour * 100 + offset_minute));
//     #[cfg(target_os = "windows")]
//         // typedef struct _SYSTEMTIME {
//         //     WORD wYear;
//         //     WORD wMonth;
//         //     WORD wDayOfWeek;
//         //     WORD wDay;
//         //     WORD wHour;
//         //     WORD wMinute;
//         //     WORD wSecond;
//         //     WORD wMilliseconds;
//         // } SYSTEMTIME, *PSYSTEMTIME, *LPSYSTEMTIME;
//         // typedef struct _TIME_ZONE_INFORMATION {
//         //     LONG       Bias;
//         //     WCHAR      StandardName[32];
//         //     SYSTEMTIME StandardDate;
//         //     LONG       StandardBias;
//         //     WCHAR      DaylightName[32];
//         //     SYSTEMTIME DaylightDate;
//         //     LONG       DaylightBias;
//         // } TIME_ZONE_INFORMATION, *PTIME_ZONE_INFORMATION, *LPTIME_ZONE_INFORMATION;
//         // DWORD GetTimeZoneInformation(
//         //         [out] LPTIME_ZONE_INFORMATION lpTimeZoneInformation
//         //         );
//         use std::ffi::c_int;
//         use std::ffi::c_long;
//         use std::ffi::c_ushort;
//         #[allow(non_snake_case)]
//         #[derive(Debug, Clone, Default)]
//         #[repr(C)]
//         struct SYSTEMTIME {
//             wYear: c_ushort,
//             wMonth: c_ushort,
//             wDayOfWeek: c_ushort,
//             wDay: c_ushort,
//             wHour: c_ushort,
//             wMinute: c_ushort,
//             wSecond: c_ushort,
//             wMilliseconds: c_ushort,
//         }
//         #[allow(non_camel_case_types)]
//         #[allow(non_snake_case)]
//         #[derive(Debug, Clone, Default)]
//         #[repr(C)]
//         struct TIME_ZONE_INFORMATION {
//             Bias: c_long,
//             StandardName: [c_ushort; 32],
//             StandardDate: SYSTEMTIME,
//             StandardBias: c_long,
//             DaylightName: [c_ushort; 32],
//             DaylightDate: SYSTEMTIME,
//             DaylightBias: c_long,
//         }
//         extern "C" {
//             fn GetTimeZoneInformation(lpTimeZoneInformation: *mut TIME_ZONE_INFORMATION) -> c_int;
//         }
//         let mut tzi: TIME_ZONE_INFORMATION = Default::default();
//         GetTimeZoneInformation(&mut tzi);
//         let offset_hour: i16 = tzi.Bias as i16 / 60;
//         let offset_minute: i16 = tzi.Bias as i16 % 60;
//         TIME_OFFSET = Some(-(offset_hour * 100 + offset_minute));
//     }
//     return;
// }
