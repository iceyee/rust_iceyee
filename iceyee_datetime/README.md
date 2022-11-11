
# iceyee_datetime

## Supported Os

- [x] linux
- [ ] macos
- [x] windows

## Example

```rust
#[test]
fn function() {
    use iceyee_datetime::DateTime;
    println!("");
    println!("@TIME_OFFSET={:?}", unsafe { iceyee_datetime::TIME_OFFSET });
    println!("DateTime::now()={}", DateTime::now());
    let dt_new = DateTime::new();
    println!("DateTime::new()=\n{}", dt_new.to_string());
    let dt_new_utc = dt_new.to_utc();
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
    println!(
        "DateTime::new().to_utc()::from()=\n{}",
        dt_new_utc_from.to_string()
    );
    return;
}
```

```
test function ...
@TIME_OFFSET=Some(800)
DateTime::now()=1668051555891
DateTime::new()=
2022-11-10T11:39:15.891+08:00
DateTime::new().to_utc()=
2022-11-10T03:39:15.891Z
DateTime::new()::from()=
2022-11-10T11:39:15.0+08:00
DateTime::new().to_utc()::from()=
2022-11-10T03:39:15.0Z
```
