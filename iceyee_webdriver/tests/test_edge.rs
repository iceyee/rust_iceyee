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

#[tokio::test]
pub async fn _1() {
    println!("");
    let (driver, mut child) = iceyee_webdriver::edge(false, None, None).await.expect("");
    driver.goto("https://www.baidu.com/").await.ok();
    tokio::signal::ctrl_c().await.ok();
    driver.quit().await.ok();
    child.kill().ok();
    child.wait().ok();
    return;
}
