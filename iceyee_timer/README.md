
# iceyee_timer

## Supported Os

- [x] linux
- [ ] macos
- [x] windows

## Example

```rust
#[tokio::test]
async fn test_schedule_pattern() {
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
    timer.stop().await;
    println!("{}", DateTime::now());
    return;
}
```

```
test test_schedule_pattern ...
1668060460760
2022-11-10T14:07:40.760+08:00
2022-11-10T14:07:41.211+08:00
2022-11-10T14:07:42.207+08:00
2022-11-10T14:07:43.209+08:00
1668060464098
```

```rust
#[tokio::test]
async fn test_execute() {
    use iceyee_datetime::DateTime;
    use iceyee_timer::Timer;
    println!("");
    println!("{}", DateTime::now());
    let mut timer: Timer = Timer::new();

    timer.schedule_execute_before(0, 1000, || async {
        println!("{} - before", DateTime::new().to_string());
        Timer::sleep(1000).await;
    });

    timer.schedule_execute_after(0, 1000, || async {
        println!("{} - after", DateTime::new().to_string());
        Timer::sleep(1000).await;
    });

    Timer::sleep(5000).await;
    timer.stop().await;
    println!("{}", DateTime::now());
    return;
}
```

```
test test_execute ...
1668060543189
2022-11-10T14:09:03.200+08:00 - before
2022-11-10T14:09:03.200+08:00 - after
2022-11-10T14:09:04.212+08:00 - before
2022-11-10T14:09:05.209+08:00 - after
2022-11-10T14:09:05.225+08:00 - before
2022-11-10T14:09:06.238+08:00 - before
2022-11-10T14:09:07.218+08:00 - after
2022-11-10T14:09:07.240+08:00 - before
1668060548241
```
