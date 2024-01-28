// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use thirtyfour::error::WebDriverError;
use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use tokio::io::AsyncWriteExt;

// Enum.

// Trait.

// Struct.

// Function.

#[tokio::test]
pub async fn _1() {
    println!("");
    let driver = iceyee_webdriver::chrome(None, false)
        .await
        .expect("test_1.rs 017");
    iceyee_time::sleep(5_000).await;
    if let Err(e) = _2(&driver).await {
        println!("{}", e.to_string());
    }
    // iceyee_time::sleep(20_000).await;
    // let _ = driver.quit().await;
    return;
}

pub async fn _2(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://www.baidu.com/").await?;
    iceyee_webdriver::wait_ready(driver).await?;
    iceyee_webdriver::wait_element(driver, "#kw", 1).await?;
    iceyee_webdriver::get_element(driver, "#kw", 0)
        .await?
        .send_keys("hello world")
        .await?;
    iceyee_webdriver::wait_element(driver, ".s_btn_wr input", 1).await?;
    iceyee_webdriver::get_element(driver, ".s_btn_wr input", 0)
        .await?
        .click()
        .await?;
    iceyee_webdriver::set_cookie(driver, "a=b;c=d;  e= f", "baidu.com").await?;
    let cookie = iceyee_webdriver::get_cookie(driver).await?;
    println!("{}", cookie.0);
    println!("{}", cookie.1);
    return Ok(());
}
