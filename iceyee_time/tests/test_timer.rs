// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
/* clippy: 本crate风格是每个函数显式写return. */
#![allow(clippy::needless_return, clippy::println_empty_string)]

// Use.

use iceyee_time::Schedule1;
use iceyee_time::Schedule2;
use iceyee_time::Timer;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

// Enum.

// Trait.

// Struct.

struct A;

impl Schedule1 for A {
    fn delay1(&self) -> u64 {
        1_000
    }

    fn sleep_before_perform1(&self) -> u64 {
        1_000
    }

    fn initialize1<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("A initialize");
            return;
        });
    }

    fn finish1<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("A finish");
            return;
        });
    }

    fn perform1<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("A {}", iceyee_time::now_seconds());
            iceyee_time::sleep(1_000).await;
            return true;
        });
    }
}

struct B;

impl Schedule2 for B {
    fn delay2(&self) -> u64 {
        1_000
    }

    fn sleep_after_perform2(&self) -> u64 {
        1_000
    }

    fn initialize2<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("B initialize");
            return;
        });
    }

    fn finish2<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("B finish");
            return;
        });
    }

    fn perform2<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("B {}", iceyee_time::now_seconds());
            iceyee_time::sleep(1_000).await;
            return true;
        });
    }
}

struct C;

impl Schedule1 for C {
    fn schedule_by_pattern1(&self) -> String {
        "* * * * * *".to_string()
    }

    fn initialize1<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("C initialize");
            return;
        });
    }

    fn finish1<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("C finish");
            return;
        });
    }

    fn perform1<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("C {}", iceyee_time::now_seconds());
            iceyee_time::sleep(1_000).await;
            return true;
        });
    }
}

/* cron表达式由外部传入的定时任务. */
struct D(String);

impl Schedule1 for D {
    fn schedule_by_pattern1(&self) -> String {
        return self.0.clone();
    }

    fn perform1<'a, 'b>(
        &'a self,
        _stop: Arc<AtomicBool>,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            return true;
        });
    }
}

// Function.

#[tokio::test]
pub async fn test_timer_drop() {
    println!("");
    println!("测试Timer的Drop.");
    println!("创建时钟.");
    let timer: Timer = Timer::new();
    let _ = timer.clone();
    println!("任务会输出当前时间戳, 需要1秒完成.");
    println!("定时A, 初始延迟1秒, 间隔1秒, schedule_execute_before");
    println!("定时B, 初始延迟1秒, 间隔1秒, schedule_execute_after");
    println!("主线等5秒.");
    println!("当前时间戳{}", iceyee_time::now_seconds());
    timer.schedule1(A.wrap1()).await;
    timer.schedule2(B.wrap2()).await;
    iceyee_time::sleep(5_000).await;
    println!("主线等待结束");
    {
        println!("Timer::stop_and_wait()");
        timer.stop_and_wait().await;
        println!("等待2秒");
        iceyee_time::sleep(2_000).await;
    }
    return;
}

#[tokio::test]
pub async fn test_timer_pattern() {
    println!("");
    println!("创建时钟.");
    let timer: Timer = Timer::new();
    println!("任务会输出当前时间戳, 需要1秒完成.");
    println!("定时C, * * * * * *");
    println!("主线等5秒.");
    println!("当前时间戳{}", iceyee_time::now_seconds());
    timer.schedule1(C.wrap1()).await;
    iceyee_time::sleep(5_000).await;
    {
        println!("stop_and_wait.");
        timer.stop_and_wait().await;
    }
    return;
}

#[tokio::test]
pub async fn test_timer_pattern_bad() {
    println!("");
    println!("测试非法的cron表达式, 每一个都应该panic.");
    println!("其中步长为0的表达式在旧版本里是死循环, 现在改成panic.");
    let table: [&str; 16] = [
        "* * * * *",
        "* * * * * * *",
        "",
        "60 * * * * *",
        "0 0 0 32 1 1",
        "0 0 0 1 13 1",
        "0 0 0 1 1 8",
        "*abc * * * * *",
        "1-0 * * * * *",
        "-5 * * * * *",
        "a * * * * *",
        "*/a * * * * *",
        "*/0 * * * * *",
        "0-59/0 * * * * *",
        "0/0 * * * * *",
        "5-10/0 * * * * *",
    ];
    for x in table {
        println!("{x:?}");
        let timer: Timer = Timer::new();
        let result = tokio::spawn(async move {
            timer.schedule1(D(x.to_string()).wrap1()).await;
        })
        .await;
        assert!(result.is_err(), "{x:?} 应该panic");
    }
    return;
}

#[tokio::test]
pub async fn test_timer_pattern_ok() {
    println!("");
    println!("测试合法的cron表达式, 不应该panic.");
    let table: [&str; 10] = [
        "* * * * * *",
        "0 * * * * *",
        "*/5 * * * * *",
        "0-30/10 * * * * *",
        "1,2,3 * * * * *",
        "1，2，3 * * * * *",
        "0 0 0 1 1 1",
        "0 0 12 * * 1-5",
        "0 30 9,18 * * *",
        "0 0 0 1,15 * *",
    ];
    for x in table {
        println!("{x:?}");
        let timer: Timer = Timer::new();
        let task = tokio::spawn(async move {
            timer.schedule1(D(x.to_string()).wrap1()).await;
            iceyee_time::sleep(230).await;
            timer.stop_and_wait().await;
        });
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), task).await;
    }
    return;
}
