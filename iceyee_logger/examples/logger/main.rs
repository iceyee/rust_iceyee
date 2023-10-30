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

// #[tokio::test]
pub async fn test_logger_no_project() {
    use iceyee_logger::Level;
    use iceyee_logger::Logger;
    use iceyee_timer::Timer;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    println!("");
    let mut logger: Logger = Logger::new(Some(Level::Info), None, None).await;
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
            let mut logger = logger_clone.lock().await;
            logger.debug(message.as_str()).await;
            logger.info(message.as_str()).await;
            logger.warn(message.as_str()).await;
            logger.error(message.as_str()).await;
            Timer::sleep(3_000).await;
        }
    });
    Timer::sleep(10_000).await;
    logger.lock().await.stop().await;
    println!("");
    return;
}

// #[tokio::test]
pub async fn test_logger_project_1() {
    use iceyee_logger::Level;
    use iceyee_logger::Logger;
    use iceyee_timer::Timer;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    println!("");
    let mut logger: Logger = Logger::new(Some(Level::Info), Some("test"), None).await;
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
            let mut logger = logger_clone.lock().await;
            logger.debug(message.as_str()).await;
            logger.info(message.as_str()).await;
            logger.warn(message.as_str()).await;
            logger.error(message.as_str()).await;
            Timer::sleep(3_000).await;
        }
    });
    Timer::sleep(10_000).await;
    logger.lock().await.stop().await;
    println!("");
    return;
}

// #[tokio::test]
pub async fn test_logger_project_2() {
    use iceyee_logger::Level;
    use iceyee_logger::Logger;
    use iceyee_timer::Timer;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    println!("");
    let mut logger: Logger = Logger::new(
        Some(Level::Info),
        Some("test"),
        Some(unsafe { iceyee_logger::DEFAULT.as_ref() }.unwrap().as_str()),
    )
    .await;
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
            let mut logger = logger_clone.lock().await;
            logger.debug(message.as_str()).await;
            logger.info(message.as_str()).await;
            logger.warn(message.as_str()).await;
            logger.error(message.as_str()).await;
            Timer::sleep(3_000).await;
        }
    });
    Timer::sleep(10_000).await;
    logger.lock().await.stop().await;
    println!("");
    return;
}

fn main() {
    println!("hello world.");
    return;
}
