// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_time::Schedule;
use iceyee_time::Timer;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;

// Enum.

// Trait.

// Struct.

struct A;

impl Schedule for A {
    fn delay(&self) -> u64 {
        1_000
    }

    fn sleep_before_perform(&self) -> u64 {
        1_000
    }

    fn initialize<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("A initialize");
            return;
        });
    }

    fn finish<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("A finish");
            return;
        });
    }

    fn perform<'a, 'b>(
        &'a self,
        stop: Arc<AtomicBool>,
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

impl Schedule for B {
    fn delay(&self) -> u64 {
        1_000
    }

    fn sleep_after_perform(&self) -> u64 {
        1_000
    }

    fn initialize<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("B initialize");
            return;
        });
    }

    fn finish<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("B finish");
            return;
        });
    }

    fn perform<'a, 'b>(
        &'a self,
        stop: Arc<AtomicBool>,
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

impl Schedule for C {
    fn schedule_by_pattern(&self) -> String {
        "* * * * * *".to_string()
    }

    fn initialize<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("C initialize");
            return;
        });
    }

    fn finish<'a, 'b>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            println!("C finish");
            return;
        });
    }

    fn perform<'a, 'b>(
        &'a self,
        stop: Arc<AtomicBool>,
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

// Function.

// #[tokio::test]
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
    timer.schedule(A.wrap()).await;
    timer.schedule(B.wrap()).await;
    iceyee_time::sleep(5_000).await;
    println!("主线等待结束");
    {
        println!("Timer::stop()");
        timer.stop().await;
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
    timer.schedule(C.wrap()).await;
    iceyee_time::sleep(5_000).await;
    {
        println!("stop_and_wait.");
        timer.stop_and_wait().await;
    }
    // {
    //     println!("不调用stop, 而是直接drop.");
    //     drop(timer);
    //     println!("主线等待2秒");
    //     iceyee_time::sleep(2_000).await;
    // }
    return;
}
