// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

/// 当前系统正在使用的时区所对应的时间偏移.
/// 比如用+0800表示东八区的时间偏移, 即+08:00.
pub static mut TIME_OFFSET: Option<i16> = None;

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

#[ctor::ctor]
fn init() {
    #[cfg(target_os = "linux")]
    unsafe {
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
        TIME_OFFSET = Some(-(offset_hour * 100 + offset_minute));
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
            fn GetTimeZoneInformation(lpTimeZoneInformation: *mut TIME_ZONE_INFORMATION) -> c_int;
        }
        let mut tzi: TIME_ZONE_INFORMATION = Default::default();
        GetTimeZoneInformation(&mut tzi);
        let offset_hour: i16 = tzi.Bias as i16 / 60;
        let offset_minute: i16 = tzi.Bias as i16 % 60;
        TIME_OFFSET = Some(-(offset_hour * 100 + offset_minute));
    }
    return;
}

// Enum.

// Trait.

// Struct.

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateTime {
    /// \[0, +oo)
    pub year: usize,
    /// \[1, 12\]
    pub month: usize,
    /// \[1, 31\]
    pub day: usize,
    /// \[0, 23\]
    pub hour: usize,
    /// \[0, 59\]
    pub minute: usize,
    /// \[0, 59\]
    pub second: usize,
    /// \[0, 999\]
    pub millisecond: usize,
    /// \[1, 365\]
    pub day_of_year: usize,
    /// \[1, 7\]
    pub weekday: usize,
    /// 时间戳, 单位:毫秒.
    pub timestamp: i64,
    /// 时间偏移, 比如用+0800表示东八区的时间偏移, 即+08:00.
    pub offset: i16,
}

impl DateTime {
    /// 返回当前时间, 等同于:
    ///
    /// ```
    /// DateTime::from((DateTime::now(), None));
    /// ```
    pub fn new() -> Self {
        return Self::from((Self::now(), None));
    }

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

    /// 转成国际标准时间.
    pub fn to_utc(&self) -> Self {
        return Self::from((self.timestamp, Some(0)));
    }
}

impl From<(i64, Option<i16>)> for DateTime {
    /// 从时间戳转成[DateTime].
    ///
    /// - @param value (timestamp, offset)
    /// - @param value$0 时间戳, 单位:毫秒.
    /// - @param value$1 时间偏移, 默认是系统设置的时区所对应的偏移.
    fn from(value: (i64, Option<i16>)) -> Self {
        let (timestamp_, offset) = value;
        let offset: i16 = match offset {
            Some(offset) => offset,
            None => unsafe { *TIME_OFFSET.as_ref().unwrap() },
        };
        let offset_hour: i16 = offset / 100 % 100;
        let offset_minute: i16 = offset % 100;
        let mut timestamp: i64 = TIME_0
            + offset_hour as i64 * 60 * 60 * 1000
            + offset_minute as i64 * 60 * 1000
            + timestamp_;
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
        // println!("@year={:?}", year);
        let day_of_year: i64 = timestamp / ONE_DAY + 1;
        // println!("@day_of_year={:?}", day_of_year);
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
        // println!("@month={:?}", month);
        // 日.
        let day: i64 = 1 + timestamp / ONE_DAY;
        timestamp %= ONE_DAY;
        // println!("@day={:?}", day);
        // 时分秒.
        let hour: i64 = timestamp / ONE_HOUR;
        timestamp %= ONE_HOUR;
        let minute: i64 = timestamp / ONE_MINUTE;
        timestamp %= ONE_MINUTE;
        let second: i64 = timestamp / ONE_SECOND;
        timestamp %= ONE_SECOND;
        let millisecond: i64 = timestamp / ONE_MILLISECOND;
        // println!(
        //     "@hour,minute,second={:02}:{:02}:{:02}",
        //     hour, minute, second
        // );
        // 周几.
        let mut weekday: i64 = (TIME_0
            + offset_hour as i64 * 60 * 60 * 1000
            + offset_minute as i64 * 60 * 1000
            + timestamp_)
            % ONE_WEEK
            / ONE_DAY
            + 1
            + 5;
        if 7 < weekday {
            weekday -= 7;
        }
        // println!("@week={:?}", weekday);
        return Self {
            year: year as usize,
            month: month as usize,
            day: day as usize,
            hour: hour as usize,
            minute: minute as usize,
            second: second as usize,
            millisecond: millisecond as usize,
            day_of_year: day_of_year as usize,
            weekday: weekday as usize,
            timestamp: timestamp_,
            offset: offset,
        };
    }
}

impl From<(usize, usize, usize, usize, usize, usize, usize, Option<i16>)> for DateTime {
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
    fn from(value: (usize, usize, usize, usize, usize, usize, usize, Option<i16>)) -> Self {
        let (year, month, day, hour, minute, second, millisecond, offset) = value;
        if month == 0 || 12 < month {
            panic!("@month={:?}", month);
        }
        let max_days: usize = match month {
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
        // println!("@timestamp={:?}", timestamp);
        // 时间差.
        let offset: i16 = match offset {
            Some(offset) => offset,
            None => unsafe { *TIME_OFFSET.as_ref().unwrap() },
        };
        let offset_hour: i16 = offset / 100 % 100;
        let offset_minute: i16 = offset % 100;
        timestamp -= TIME_0;
        timestamp -= offset_hour as i64 * 60 * 60 * 1000;
        timestamp -= offset_minute as i64 * 60 * 1000;
        return Self::from((timestamp, Some(offset)));
        // // println!("@timestamp={:?}", timestamp);
        // // 周几.
        // let mut weekday: i64 = timestamp % ONE_WEEK / ONE_DAY + 1 + 3;
        // if 7 < weekday {
        //     weekday -= 7;
        // }
        // return Self {
        //     year: year as usize,
        //     month: month as usize,
        //     day: day as usize,
        //     hour: hour as usize,
        //     minute: minute as usize,
        //     second: second as usize,
        //     day_of_year: day_of_year as usize,
        //     weekday: weekday as usize,
        //     timestamp: timestamp,
        //     offset: offset,
        // };
    }
}

impl ToString for DateTime {
    /// 转成字符串, 使用RFC3339标准, 格式'xx-xx-xxTxx:xx:xx.xxx\[+/-\]xx:xx'.
    fn to_string(&self) -> String {
        let v1: String = format!(
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}.{}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.timestamp % 1000
        );
        let v2: &str = if self.offset == 0 {
            ""
        } else if self.offset < 0 {
            "-"
        } else {
            "+"
        };
        let offset: i16 = if self.offset < 0 {
            -self.offset
        } else {
            self.offset
        };
        let v3: String = if offset == 0 {
            "Z".to_string()
        } else {
            format!("{:02}:{:02}", offset / 100 % 100, offset % 100)
        };
        let result: String = v1 + v2 + &v3;
        let result: String = format!("{result:29}");
        return result;
    }
}

// Function.
