// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_time::Timer;

// Enum.

// Trait.

// Struct.

// Function.

pub struct Executor(usize);

impl Executor {
    pub async fn execute(&self) {
        println!("@id={:03}, @time={}", self.0, iceyee_time::now_seconds());
        iceyee_time::sleep(1_000).await;
    }
}

// #[tokio::test]
pub async fn test_timer_drop() {
    println!("");
    println!("测试Timer的Drop.");
    println!("创建时钟.");
    let timer: Timer = Timer::new();
    println!("任务会输出当前时间戳, 需要1秒完成.");
    println!("定时1, 初始延迟1秒, 间隔1秒, schedule_execute_before");
    println!("定时2, 初始延迟1秒, 间隔1秒, schedule_execute_after");
    println!("主线等5秒.");
    println!("当前时间戳{}", iceyee_time::now_seconds());
    timer.schedule_execute_before(1_000, 1_000, |_| Executor(1).execute());
    timer.schedule_execute_after(1_000, 1_000, |_| Executor(2).execute());
    iceyee_time::sleep(5_000).await;
    println!("主线等待结束");
    {
        println!("主动stop.");
        timer.stop();
        println!("主线等待1秒");
        iceyee_time::sleep(1_000).await;
    }
    // {
    //     println!("不调用stop, 而是直接drop.");
    //     drop(timer);
    //     println!("主线等待1秒");
    //     iceyee_time::sleep(1_000).await;
    // }
    // {
    //     println!("clone timer.");
    //     let _timer = timer.clone();
    //     println!("drop原来的timer.");
    //     drop(timer);
    //     println!("主线等待1秒");
    //     iceyee_time::sleep(1_000).await;
    // }
    // {
    //     println!("clone timer.");
    //     let _timer = timer.clone();
    //     println!("drop原来的timer.");
    //     drop(timer);
    //     println!("drop clone timer.");
    //     drop(_timer);
    //     println!("主线等待1秒");
    //     iceyee_time::sleep(1_000).await;
    // }
    return;
}

// #[tokio::test]
pub async fn test_timer_restart() {
    println!("");
    println!("测试Timer的重启.");
    println!("创建时钟.");
    let timer: Timer = Timer::new();
    println!("任务会输出当前时间戳, 需要1秒完成.");
    println!("定时1, 初始延迟1秒, 间隔1秒, schedule_execute_before");
    println!("定时2, 初始延迟1秒, 间隔1秒, schedule_execute_after");
    println!("主线等5秒.");
    println!("当前时间戳{}", iceyee_time::now_seconds());
    timer.schedule_execute_before(1_000, 1_000, |_| Executor(1).execute());
    timer.schedule_execute_after(1_000, 1_000, |_| Executor(2).execute());
    iceyee_time::sleep(5_000).await;
    println!("主线等待结束");
    println!("主动stop.");
    timer.stop();
    println!("主线等待1秒");
    iceyee_time::sleep(1_000).await;
    println!("重启时钟");
    timer.start();
    println!("定时1, 初始延迟1秒, 间隔1秒, schedule_execute_before");
    println!("定时2, 初始延迟1秒, 间隔1秒, schedule_execute_after");
    println!("主线等5秒.");
    println!("当前时间戳{}", iceyee_time::now_seconds());
    timer.schedule_execute_before(1_000, 1_000, |_| Executor(1).execute());
    timer.schedule_execute_after(1_000, 1_000, |_| Executor(2).execute());
    iceyee_time::sleep(5_000).await;
    println!("主线等待结束");
    println!("不调用stop, 而是直接drop.");
    drop(timer);
    println!("主线等待1秒");
    iceyee_time::sleep(1_000).await;
    return;
}

// #[tokio::test]
pub async fn test_timer_pattern() {
    println!("");
    println!("测试Timer的重启.");
    println!("创建时钟.");
    let timer: Timer = Timer::new();
    println!("任务会输出当前时间戳, 需要1秒完成.");
    println!("定时1, * * * * * *");
    println!("主线等5秒.");
    println!("当前时间戳{}", iceyee_time::now_seconds());
    timer.schedule_pattern("* * * * * *", |_| Executor(1).execute());
    iceyee_time::sleep(5_000).await;
    println!("主线等待结束");
    println!("不调用stop, 而是直接drop.");
    drop(timer);
    println!("主线等待1秒");
    iceyee_time::sleep(1_000).await;
    return;
}
