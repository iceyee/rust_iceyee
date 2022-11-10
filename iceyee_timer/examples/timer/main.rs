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

    // timer
    //     .schedule_pattern("*/2 * * * * *", || async {
    //         println!("{}", DateTime::new().to_string());
    //     })
    //     .unwrap();
    // Timer::sleep(6666).await;

    // timer
    //     .schedule_pattern("1-50/2,59 * * * * *", || async {
    //         println!("{}", DateTime::new().to_string());
    //     })
    //     .unwrap();
    // Timer::sleep(66666).await;

    timer.stop().await;
    println!("{}", DateTime::now());
    return;
}

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

fn main() {
    println!("hello world.");
    return;
}
