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
pub async fn _1() {
    println!("");
    let (driver, mut _child) = iceyee_webdriver::edge().await.expect("test_1.rs 249");
    let _ = tokio::signal::ctrl_c().await;
    driver.quit().await.expect("Driver quit.");
    _child.kill().expect("child kill.");
    return;
}
