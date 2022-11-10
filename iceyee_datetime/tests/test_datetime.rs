// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

// Enum.

// Trait.

// Struct.

// Function.

#[test]
fn function() {
    use iceyee_datetime::DateTime;
    println!("");
    println!("@TIME_OFFSET={:?}", unsafe { iceyee_datetime::TIME_OFFSET });
    println!("DateTime::now()={}", DateTime::now());
    let dt_new = DateTime::new();
    // println!("DateTime::new()=\n{:?}", dt_new);
    println!("DateTime::new()=\n{}", dt_new.to_string());
    let dt_new_utc = dt_new.to_utc();
    // println!("DateTime::new().to_utc()=\n{:?}", dt_new_utc);
    println!("DateTime::new().to_utc()=\n{}", dt_new_utc.to_string());
    let dt_new_from = DateTime::from((
        dt_new.year,
        dt_new.month,
        dt_new.day,
        dt_new.hour,
        dt_new.minute,
        dt_new.second,
        Some(dt_new.offset),
    ));
    // println!("DateTime::new()::from()=\n{:?}", dt_new_from);
    println!("DateTime::new()::from()=\n{}", dt_new_from.to_string());
    let dt_new_utc_from = DateTime::from((
        dt_new_utc.year,
        dt_new_utc.month,
        dt_new_utc.day,
        dt_new_utc.hour,
        dt_new_utc.minute,
        dt_new_utc.second,
        Some(dt_new_utc.offset),
    ));
    // println!("DateTime::new().to_utc()::from()=\n{:?}", dt_new_utc_from);
    println!(
        "DateTime::new().to_utc()::from()=\n{}",
        dt_new_utc_from.to_string()
    );
    return;
}
