// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

// #![feature(async_closure)]

// Enum.

// Trait.

// Struct.

// Function.

// #[tokio::test]
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

// #[tokio::test]
pub async fn test_schedule_pattern_2() {
    use iceyee_datetime::DateTime;
    use iceyee_timer::Timer;
    println!("");
    println!("{}", DateTime::now());
    let mut timer: Timer = Timer::new();

    timer
        .schedule_pattern("*/2 * * * * *", || async {
            println!("{}", DateTime::new().to_string());
        })
        .unwrap();
    Timer::sleep(6666).await;

    println!("{}", DateTime::now());
    Timer::sleep(2000).await;
    return;
}

// #[tokio::test]
pub async fn test_schedule_pattern_3() {
    use iceyee_datetime::DateTime;
    use iceyee_timer::Timer;
    println!("");
    println!("{}", DateTime::now());
    let mut timer: Timer = Timer::new();

    timer
        .schedule_pattern("1-50/2,59 * * * * *", || async {
            println!("{}", DateTime::new().to_string());
        })
        .unwrap();
    Timer::sleep(66666).await;

    println!("{}", DateTime::now());
    Timer::sleep(2000).await;
    return;
}

// #[tokio::test]
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

fn main() {
    println!("hello world.");
    return;
}
