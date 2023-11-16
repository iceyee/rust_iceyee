
# iceyee_logger

## Supported Os

- [x] linux
- [ ] macos
- [x] windows

## Example

```rust
#[tokio::test]
pub async fn test_logger_no_project() {
    use iceyee_logger::Level;
    use iceyee_logger::Logger;
    use iceyee_timer::Timer;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    println!("");
    let logger: Logger = Logger::new(Some(Level::Info), None, None).await;
    logger.debug("hello world.").await;
    logger.info("hello world.").await;
    logger.warn("hello world.").await;
    logger.error("hello world.").await;
    let logger: Arc<Mutex<Logger>> = Arc::new(Mutex::new(logger));
    let logger_clone = logger.clone();
    tokio::task::spawn(async move {
        let mut counter: usize = 0;
        loop {
            counter += 1;
            let message = counter.to_string();
            let logger = logger_clone.lock().await;
            logger.debug(message.as_str()).await;
            logger.info(message.as_str()).await;
            logger.warn(message.as_str()).await;
            logger.error(message.as_str()).await;
            Timer::sleep(3_000).await;
        }
    });
    Timer::sleep(10_000).await;
    drop(logger);
    Timer::sleep(2_000).await;
    println!("");
    return;
}
```

```
test test_logger_project_2 ...

2023-11-16T12:10:40.543+08:00 INFO  # hello world.

2023-11-16T12:10:40.543+08:00 WARN  # hello world.

2023-11-16T12:10:40.543+08:00 ERROR #
    hello world.

2023-11-16T12:10:40.543+08:00 INFO  # 1

2023-11-16T12:10:40.543+08:00 WARN  # 1

2023-11-16T12:10:40.543+08:00 ERROR #
    1

2023-11-16T12:10:43.513+08:00 INFO  # 2

2023-11-16T12:10:43.513+08:00 WARN  # 2

2023-11-16T12:10:43.513+08:00 ERROR #
    2

2023-11-16T12:10:46.543+08:00 INFO  # 3

2023-11-16T12:10:46.543+08:00 WARN  # 3

2023-11-16T12:10:46.543+08:00 ERROR #
    3

2023-11-16T12:10:49.514+08:00 INFO  # 4

2023-11-16T12:10:49.514+08:00 WARN  # 4

2023-11-16T12:10:49.514+08:00 ERROR #
    4
```
