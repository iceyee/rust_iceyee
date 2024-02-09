// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_logger::Level;

// Enum.

// Trait.

// Struct.

// Function.

pub async fn init_none() {
    iceyee_logger::init(Some(Level::Debug), None, None).await;
    return;
}

pub async fn init_test() {
    iceyee_logger::init(Some(Level::Debug), Some("test"), None).await;
    return;
}

// #[tokio::test]
pub async fn test_no_init() {
    #[allow(dead_code)]
    #[derive(Clone, Debug, Default)]
    struct T465 {
        pub a: usize,
        pub b: bool,
    }
    println!("");
    println!("测试不初始化logger.");
    let a001: T465 = T465::default();
    iceyee_logger::debug_object(&a001).await;
    iceyee_logger::debug(vec![
        "hello world debug.".to_string(),
        "second".to_string(),
        "third".to_string(),
        "fourth".to_string(),
    ])
    .await;
    iceyee_logger::info_object(&a001).await;
    iceyee_logger::info(vec![
        "hello world info.".to_string(),
        "second".to_string(),
        "third".to_string(),
        "fourth".to_string(),
    ])
    .await;
    iceyee_logger::warn_object(&a001).await;
    iceyee_logger::warn(vec![
        "hello world warn.".to_string(),
        "second".to_string(),
        "third".to_string(),
        "fourth".to_string(),
    ])
    .await;
    iceyee_logger::error_object(&a001).await;
    iceyee_logger::error(vec![
        "hello world debug.".to_string(),
        "second".to_string(),
        "third".to_string(),
        "fourth".to_string(),
    ])
    .await;
    iceyee_logger::error(vec![
        "\n".to_string(),
        "hello world debug.\n".to_string(),
        "second\n".to_string(),
        "third\n".to_string(),
        "fourth\n".to_string(),
    ])
    .await;
    return;
}

// #[tokio::test]
pub async fn test_none() {
    println!("");
    println!("初始化None.");
    init_none().await;
    iceyee_logger::debug(vec!["hello world debug.".to_string()]).await;
    iceyee_logger::info(vec!["hello world info.".to_string()]).await;
    iceyee_logger::warn(vec!["hello world warn.".to_string()]).await;
    iceyee_logger::error(vec!["hello world error.".to_string()]).await;
    return;
}

// #[tokio::test]
pub async fn test_test() {
    println!("");
    println!("测试写到文件, 并且结束后不使用flush.");
    println!("初始化'test'.");
    init_test().await;
    iceyee_logger::debug(vec!["hello world debug.".to_string()]).await;
    iceyee_logger::info(vec!["hello world info.".to_string()]).await;
    iceyee_logger::warn(vec!["hello world warn.".to_string()]).await;
    iceyee_logger::error(vec!["hello world error.".to_string()]).await;
    return;
}

// #[tokio::test]
pub async fn test_move() {
    println!("");
    println!("测试写到文件, 并且结束后不使用flush, 文件rename.");
    println!("初始化'test'.");
    init_test().await;
    iceyee_logger::debug(vec!["hello world debug.".to_string()]).await;
    iceyee_logger::info(vec!["hello world info.".to_string()]).await;
    iceyee_logger::warn(vec!["hello world warn.".to_string()]).await;
    iceyee_logger::error(vec!["hello world error.".to_string()]).await;
    println!("等待11秒.");
    iceyee_time::sleep(11_000).await;
    println!("再写入一条记录");
    iceyee_logger::warn(vec!["hello world warn.".to_string()]).await;
    return;
}

// #[tokio::test]
pub async fn test_sleep() {
    println!("");
    println!("间隔1秒输出, 持续10秒, Level是Info.");
    println!("初始化None.");
    iceyee_logger::init(Some(Level::Info), None, None).await;
    for _ in 0..10 {
        iceyee_logger::debug(vec!["hello world debug.".to_string()]).await;
        iceyee_logger::info(vec!["hello world info.".to_string()]).await;
        iceyee_logger::warn(vec!["hello world warn.".to_string()]).await;
        iceyee_logger::error(vec!["hello world error.".to_string()]).await;
        iceyee_time::sleep(1_000).await;
    }
    return;
}
