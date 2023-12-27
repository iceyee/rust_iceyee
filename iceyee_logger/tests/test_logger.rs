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
    iceyee_logger::init(Level::Debug, None, None).await;
    return;
}

pub async fn init_test() {
    iceyee_logger::init(Level::Debug, Some("test"), None).await;
    return;
}

// #[tokio::test]
pub async fn test_no_init() {
    println!("");
    println!("测试不初始化logger.");
    iceyee_logger::debug("hello world.").await;
    iceyee_logger::info("hello world info.").await;
    iceyee_logger::warn("hello world warn.").await;
    iceyee_logger::error("hello world error.").await;
    return;
}

// #[tokio::test]
pub async fn test_none() {
    println!("");
    println!("初始化None.");
    init_none().await;
    iceyee_logger::debug("hello world debug.").await;
    iceyee_logger::info("hello world info.").await;
    iceyee_logger::warn("hello world warn.").await;
    iceyee_logger::error("hello world error.").await;
    return;
}

// #[tokio::test]
pub async fn test_test() {
    println!("");
    println!("测试写到文件, 并且结束后不使用flush.");
    println!("初始化'test'.");
    init_test().await;
    iceyee_logger::debug("hello world debug.").await;
    iceyee_logger::info("hello world info.").await;
    iceyee_logger::warn("hello world warn.").await;
    iceyee_logger::error("hello world error.").await;
    return;
}

// #[tokio::test]
pub async fn test_move() {
    println!("");
    println!("测试写到文件, 并且结束后不使用flush, 文件rename.");
    println!("初始化'test'.");
    init_test().await;
    iceyee_logger::debug("hello world debug.").await;
    iceyee_logger::info("hello world info.").await;
    iceyee_logger::warn("hello world warn.").await;
    iceyee_logger::error("hello world error.").await;
    println!("等待11秒.");
    iceyee_time::sleep(11_000).await;
    println!("再写入一条记录");
    iceyee_logger::warn("hello world warn.").await;
    return;
}

// #[tokio::test]
pub async fn test_sleep() {
    println!("");
    println!("间隔1秒输出, 持续10秒, Level是Info.");
    println!("初始化None.");
    iceyee_logger::init(Level::Info, None, None).await;
    for _ in 0..10 {
        iceyee_logger::debug("hello world debug.").await;
        iceyee_logger::info("hello world info.").await;
        iceyee_logger::warn("hello world warn.").await;
        iceyee_logger::error("hello world error.").await;
        iceyee_time::sleep(1_000).await;
    }
    return;
}
