// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_time::Timer;
use std::sync::atomic::Ordering::SeqCst;

// Enum.

// Trait.

// Struct.

// Function.

// #[tokio::test]
pub async fn test_timer_drop() {
    println!("");
    println!("测试Timer的Drop.");
    println!("创建时钟.");
    let mut timer: Timer = Timer::new();
    let _ = timer.clone();
    println!("任务会输出当前时间戳, 需要1秒完成.");
    println!("定时1, 初始延迟1秒, 间隔1秒, schedule_execute_before");
    println!("定时2, 初始延迟1秒, 间隔1秒, schedule_execute_after");
    println!("主线等5秒.");
    println!("当前时间戳{}", iceyee_time::now_seconds());
    timer.schedule_execute_before(1_000, 1_000, |stop| async move {
        if !stop.load(SeqCst) {
            println!("@id={:03}, @time={}", 1, iceyee_time::now_seconds());
            iceyee_time::sleep(1_000).await;
        }
    });
    timer.schedule_execute_after(1_000, 1_000, |stop| async move {
        if !stop.load(SeqCst) {
            println!("@id={:03}, @time={}", 2, iceyee_time::now_seconds());
            iceyee_time::sleep(1_000).await;
        }
    });
    iceyee_time::sleep(5_000).await;
    println!("主线等待结束");
    {
        println!("Timer::stop()");
        timer.stop();
        println!("等待2秒");
        iceyee_time::sleep(2_000).await;
    }
    return;
}

// #[tokio::test]
pub async fn test_timer_pattern() {
    println!("");
    println!("测试Timer的重启.");
    println!("创建时钟.");
    let mut timer: Timer = Timer::new();
    println!("任务会输出当前时间戳, 需要1秒完成.");
    println!("定时1, * * * * * *");
    println!("主线等5秒.");
    println!("当前时间戳{}", iceyee_time::now_seconds());
    timer.schedule_pattern("* * * * * *", |stop| async move {
        if !stop.load(SeqCst) {
            println!("@id={:03}, @time={}", 1, iceyee_time::now_seconds());
            iceyee_time::sleep(1_000).await;
        }
    });
    iceyee_time::sleep(5_000).await;
    println!("主线等待结束");
    println!("不调用stop, 而是直接drop.");
    drop(timer);
    println!("主线等待2秒");
    iceyee_time::sleep(2_000).await;
    return;
}
