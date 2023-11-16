
# iceyee_timer

## Supported Os

- [x] linux
- [ ] macos
- [x] windows

## Example

```rust
#[tokio::test]
pub async fn test_schedule_pattern_1() {
    use iceyee_datetime::DateTime;
    use iceyee_timer::Timer;

    println!("");
    println!("{}", DateTime::now());
    let mut timer: Timer = Timer::new();

    timer
        .schedule_pattern("* * * * * *", || async {
            println!("{}", DateTime::new().to_string());
        })
        .unwrap();
    Timer::sleep(3333).await;

    drop(timer);
    println!("{}", DateTime::now());
    Timer::sleep(2000).await;
    return;
}
```

```
test test_schedule_pattern_1 ...
1700106934456
2023-11-16T11:55:34.456+08:00
2023-11-16T11:55:35.222+08:00
2023-11-16T11:55:36.246+08:00
2023-11-16T11:55:37.220+08:00
1700106937791
```

```rust
#[tokio::test]
pub async fn test_execute() {
    use iceyee_datetime::DateTime;
    use iceyee_timer::Timer;

    println!("");
    println!("{}", DateTime::now());
    let mut timer = Timer::new();

    timer.schedule_execute_before(0, 1000, || async {
        println!("{} - before", DateTime::new().to_string());
        Timer::sleep(1000).await;
    });

    timer.schedule_execute_after(0, 1000, || async {
        println!("{} - after", DateTime::new().to_string());
        Timer::sleep(1000).await;
    });

    Timer::sleep(5000).await;
    println!("{}", DateTime::now());
    Timer::sleep(2000).await;
    return;
}
```

```
test test_execute ...
1700107070376
2023-11-16T11:57:50.428+08:00 - before
2023-11-16T11:57:50.428+08:00 - after
2023-11-16T11:57:51.481+08:00 - before
2023-11-16T11:57:52.452+08:00 - after
2023-11-16T11:57:52.534+08:00 - before
2023-11-16T11:57:53.586+08:00 - before
2023-11-16T11:57:54.481+08:00 - after
2023-11-16T11:57:54.638+08:00 - before
1700107075378
2023-11-16T11:57:55.691+08:00 - before
2023-11-16T11:57:56.509+08:00 - after
2023-11-16T11:57:56.744+08:00 - before
```
