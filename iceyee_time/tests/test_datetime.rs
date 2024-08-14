// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_time::DateTime;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_datetime() {
    println!("");
    println!("\niceyee_time::now()={}", iceyee_time::now());
    let dt_new = DateTime::new();
    // println!("DateTime::new()=\n{:?}", dt_new);
    println!("\nDateTime::new()=\n{}", dt_new.to_string());
    let dt_new_utc = dt_new.to_utc();
    // println!("DateTime::new().to_utc()=\n{:?}", dt_new_utc);
    println!("\nDateTime::new().to_utc()=\n{}", dt_new_utc.to_string());
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
    // println!("DateTime::new()::from()=\n{:?}", dt_new_from);
    println!("\nDateTime::new()::from()=\n{}", dt_new_from.to_string());
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
    // println!("DateTime::new().to_utc()::from()=\n{:?}", dt_new_utc_from);
    println!(
        "\nDateTime::new().to_utc()::from()=\n{}",
        dt_new_utc_from.to_string()
    );
    return;
}
