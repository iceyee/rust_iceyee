// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
/* clippy: 本crate风格是每个函数显式写return. */
#![allow(
    clippy::needless_return,
    clippy::println_empty_string,
    clippy::type_complexity
)]

// Use.

use iceyee_time::DateTime;
use iceyee_time::TimeOffset;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_datetime() {
    println!("");
    println!("\niceyee_time::now()={}", iceyee_time::now());
    let dt_new = DateTime::new();
    println!("\nDateTime::new()=\n{dt_new}");
    let dt_new_utc = dt_new.to_utc();
    println!("\nDateTime::new().to_utc()=\n{dt_new_utc}");
    let dt_new_from = DateTime::from((
        dt_new.year,
        dt_new.month,
        dt_new.day,
        dt_new.hour,
        dt_new.minute,
        dt_new.second,
        dt_new.millisecond,
        Some(dt_new.offset),
    ));
    println!("\nDateTime::new()::from()=\n{dt_new_from}");
    let dt_new_utc_from = DateTime::from((
        dt_new_utc.year,
        dt_new_utc.month,
        dt_new_utc.day,
        dt_new_utc.hour,
        dt_new_utc.minute,
        dt_new_utc.second,
        dt_new_utc.millisecond,
        Some(dt_new_utc.offset),
    ));
    println!("\nDateTime::new().to_utc()::from()=\n{dt_new_utc_from}");
    return;
}

#[test]
pub fn test_datetime_epoch() {
    println!("");
    println!("测试纪元的边界值.");
    /* 时间戳0就是1970-01-01T00:00:00.000Z. */
    let table: [(i64, u64, u64, u64, u64, u64, u64, u64, u64, u64); 8] = [
        (0, 1970, 1, 1, 0, 0, 0, 0, 1, 4),
        (1, 1970, 1, 1, 0, 0, 0, 1, 1, 4),
        (999, 1970, 1, 1, 0, 0, 0, 999, 1, 4),
        (1_000, 1970, 1, 1, 0, 0, 1, 0, 1, 4),
        (86_399_999, 1970, 1, 1, 23, 59, 59, 999, 1, 4),
        (86_400_000, 1970, 1, 2, 0, 0, 0, 0, 2, 5),
        (-1, 1969, 12, 31, 23, 59, 59, 999, 365, 3),
        (-1_000, 1969, 12, 31, 23, 59, 59, 0, 365, 3),
    ];
    for (timestamp, year, month, day, hour, minute, second, millisecond, day_of_year, weekday) in
        table
    {
        let dt: DateTime = DateTime::from((timestamp, Some(TimeOffset(0))));
        println!("{timestamp} -> {dt}");
        assert_eq!(dt.year, year);
        assert_eq!(dt.month, month);
        assert_eq!(dt.day, day);
        assert_eq!(dt.hour, hour);
        assert_eq!(dt.minute, minute);
        assert_eq!(dt.second, second);
        assert_eq!(dt.millisecond, millisecond);
        assert_eq!(dt.day_of_year, day_of_year);
        assert_eq!(dt.weekday, weekday);
    }
    return;
}

#[test]
pub fn test_datetime_century() {
    println!("");
    println!("测试世纪边界, 旧版本在'上一年是100的倍数但不是400的倍数'时整体错一天.");
    println!("1970年之后只有2000年这一个世纪, 所以旧版本的自测发现不了这个问题.");
    /* (输入日期, 期望的读回结果) */
    let table: [(u64, u64, u64); 21] = [
        (1700, 1, 1),
        (1700, 3, 1),
        (1701, 1, 1),
        (1701, 2, 28),
        (1701, 3, 1),
        (1701, 12, 31),
        (1702, 6, 15),
        (1703, 12, 31),
        (1704, 1, 1),
        (1800, 1, 1),
        (1801, 1, 1),
        (1900, 1, 1),
        (1900, 3, 1),
        (1901, 1, 1),
        (1901, 3, 1),
        (1903, 12, 31),
        (1904, 1, 1),
        (2000, 2, 29),
        (2000, 3, 1),
        (2100, 3, 1),
        (2400, 2, 29),
    ];
    for (year, month, day) in table {
        let dt: DateTime = DateTime::from((year, month, day, 0, 0, 0, 0, Some(TimeOffset(0))));
        println!("{} -> {}", dt, dt.timestamp);
        assert_eq!(dt.year, year, "{year}-{month}-{day}");
        assert_eq!(dt.month, month, "{year}-{month}-{day}");
        assert_eq!(dt.day, day, "{year}-{month}-{day}");
        assert!(1 <= dt.month && dt.month <= 12);
        assert!(1 <= dt.day && dt.day <= 31);
        assert!(1 <= dt.weekday && dt.weekday <= 7);
        assert!(1 <= dt.day_of_year && dt.day_of_year <= 366);
        /* 往返: 读回的字段再合成一次, 时间戳应该不变. */
        let dt2: DateTime = DateTime::from((
            dt.year,
            dt.month,
            dt.day,
            dt.hour,
            dt.minute,
            dt.second,
            dt.millisecond,
            Some(TimeOffset(0)),
        ));
        assert_eq!(dt2.timestamp, dt.timestamp, "{year}-{month}-{day} 往返");
    }
    println!("测试1901年每一天的周几, 1901-01-01是周二.");
    for day in 1..=31 {
        let dt: DateTime = DateTime::from((1901, 1, day, 0, 0, 0, 0, Some(TimeOffset(0))));
        assert_eq!(dt.year, 1901);
        assert_eq!(dt.month, 1);
        assert_eq!(dt.day, day);
        assert_eq!(dt.day_of_year, day);
    }
    return;
}

#[test]
pub fn test_datetime_offset() {
    println!("");
    println!("测试时区偏移.");
    /* (偏移, 期望的小时, 期望的分钟) */
    let table: [(i16, u64, u64); 8] = [
        (0, 0, 0),
        (800, 8, 0),
        (-800, 16, 0),
        (530, 5, 30),
        (-530, 18, 30),
        (330, 3, 30),
        (1_245, 12, 45),
        (-1_245, 11, 15),
    ];
    for (offset, hour, minute) in table {
        let dt: DateTime = DateTime::from((0, Some(TimeOffset(offset))));
        println!("offset={offset}, {dt}");
        assert_eq!(dt.hour, hour);
        assert_eq!(dt.minute, minute);
        /* 时间戳不受偏移影响. */
        assert_eq!(dt.timestamp, 0);
        assert_eq!(dt.offset, TimeOffset(offset));
        /* 同一时间戳换一个时区, 时间戳不变. */
        let utc: DateTime = DateTime::from((0, Some(TimeOffset(0))));
        assert_eq!(utc.timestamp, dt.timestamp);
    }
    return;
}

#[test]
pub fn test_datetime_format() {
    println!("");
    println!("测试格式化, 使用RFC3339, 年份补齐4位, 毫秒取millisecond字段.");
    let table: [(i64, i16, &str); 8] = [
        (0, 0, "1970-01-01T00:00:00.000Z"),
        (1_000, 0, "1970-01-01T00:00:01.000Z"),
        (1_000, 800, "1970-01-01T08:00:01.000+08:00"),
        (1_000, -800, "1969-12-31T16:00:01.000-08:00"),
        (1_000, 530, "1970-01-01T05:30:01.000+05:30"),
        (-1, 0, "1969-12-31T23:59:59.999Z"),
        (-999, 0, "1969-12-31T23:59:59.001Z"),
        (-1_000, 800, "1970-01-01T07:59:59.000+08:00"),
    ];
    for (timestamp, offset, expect) in table {
        let dt: DateTime = DateTime::from((timestamp, Some(TimeOffset(offset))));
        println!("{timestamp}, {offset} -> {dt}");
        assert_eq!(dt.to_string(), expect);
    }
    println!("测试年份补齐4位.");
    let dt: DateTime = DateTime::from((1, 1, 1, 0, 0, 0, 0, Some(TimeOffset(0))));
    println!("{}", dt);
    assert_eq!(dt.to_string(), "0001-01-01T00:00:00.000Z");
    return;
}

#[test]
pub fn test_datetime_panic() {
    println!("");
    println!("测试非法参数, 应该panic.");
    let table: [(u64, u64, u64, u64, u64, u64, u64); 8] = [
        (2024, 0, 1, 0, 0, 0, 0),
        (2024, 13, 1, 0, 0, 0, 0),
        (2024, 1, 0, 0, 0, 0, 0),
        (2024, 1, 32, 0, 0, 0, 0),
        (2023, 2, 29, 0, 0, 0, 0),
        (2024, 1, 1, 24, 0, 0, 0),
        (2024, 1, 1, 0, 60, 0, 0),
        (2024, 1, 1, 0, 0, 0, 1_000),
    ];
    for (year, month, day, hour, minute, second, millisecond) in table {
        println!("{year}-{month}-{day} {hour}:{minute}:{second}.{millisecond}");
        let result = std::panic::catch_unwind(|| {
            return DateTime::from((
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
                Some(TimeOffset(0)),
            ));
        });
        assert!(result.is_err(), "{year}-{month}-{day} 应该panic");
    }
    println!("测试闰年的2月29日, 只有闰年才是合法的.");
    for year in [1600u64, 1972, 2000, 2024, 2400] {
        println!("{year}");
        let result = std::panic::catch_unwind(|| {
            return DateTime::from((year, 2, 29, 0, 0, 0, 0, Some(TimeOffset(0))));
        });
        assert!(result.is_ok(), "{year}是闰年, 不该panic");
    }
    for year in [1700u64, 1800, 1900, 2023, 2100, 2200] {
        println!("{year}");
        let result = std::panic::catch_unwind(|| {
            return DateTime::from((year, 2, 29, 0, 0, 0, 0, Some(TimeOffset(0))));
        });
        assert!(result.is_err(), "{year}不是闰年, 应该panic");
    }
    return;
}
