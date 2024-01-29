// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! - @see [iceyee_logger](../iceyee_logger/index.html)
//! - @see [iceyee_time](../iceyee_time/index.html)
//! - @see [thirtyfour](../thirtyfour/index.html)
//! - @see [tokio](../tokio/index.html)

// Use.

use cookie::SameSite;
use thirtyfour::error::WebDriverError;
use thirtyfour::error::WebDriverResult;
use thirtyfour::By;
use thirtyfour::ChromeCapabilities;
use thirtyfour::Cookie;
use thirtyfour::WebDriver;
use thirtyfour::WebElement;
use tokio::io::AsyncWriteExt;

// Enum.

// Trait.

// Struct.

// Function.

pub async fn chrome(driver_url: Option<&str>, headless: bool) -> WebDriverResult<WebDriver> {
    let mut options: ChromeCapabilities = ChromeCapabilities::new();
    options.set_ignore_certificate_errors()?;
    options.add_chrome_arg("--no-sandbox")?;
    if headless {
        options.set_headless()?;
    }
    iceyee_logger::info("打开浏览器.").await;
    iceyee_logger::info_object("", &options).await;
    let driver: WebDriver =
        WebDriver::new(driver_url.unwrap_or("http://localhost:9515"), options).await?;
    return Ok(driver);
}

pub async fn wait_url(driver: &WebDriver, url: &str, equal: bool) -> WebDriverResult<()> {
    iceyee_logger::info_2("wait_url()", url).await;
    let mut stdout = tokio::io::stdout();
    for x in 0..60 {
        stdout
            .write_all(b"\r")
            .await
            .expect("iceyee_webdriver/lib.rs 081");
        stdout
            .write_all(x.to_string().as_bytes())
            .await
            .expect("iceyee_webdriver/lib.rs 049");
        stdout.flush().await.expect("iceyee_webdriver/lib.rs 177");
        if (driver.current_url().await?.as_str() == url && equal)
            || (driver.current_url().await?.as_str() != url && !equal)
        {
            stdout
                .write_all(b"\r")
                .await
                .expect("iceyee_webdriver/lib.rs 081");
            stdout.flush().await.expect("iceyee_webdriver/lib.rs 713");
            iceyee_time::sleep(1_000).await;
            return Ok(());
        }
        iceyee_time::sleep(1_000).await;
    }
    return Err(WebDriverError::Timeout("".to_string()));
}

pub async fn wait_ready(driver: &WebDriver) -> WebDriverResult<()> {
    iceyee_logger::info("wait_ready()").await;
    let mut stdout = tokio::io::stdout();
    for x in 0..60 {
        stdout
            .write_all(b"\r")
            .await
            .expect("iceyee_webdriver/lib.rs 081");
        stdout
            .write_all(x.to_string().as_bytes())
            .await
            .expect("iceyee_webdriver/lib.rs 049");
        stdout.flush().await.expect("iceyee_webdriver/lib.rs 177");
        if driver.status().await?.ready {
            stdout
                .write_all(b"\r")
                .await
                .expect("iceyee_webdriver/lib.rs 081");
            stdout.flush().await.expect("iceyee_webdriver/lib.rs 713");
            iceyee_time::sleep(1_000).await;
            return Ok(());
        }
        iceyee_time::sleep(1_000).await;
    }
    return Err(WebDriverError::Timeout("".to_string()));
}

pub async fn wait_element(driver: &WebDriver, css: &str, number: usize) -> WebDriverResult<()> {
    iceyee_logger::info_3("wait_element()", css, number.to_string()).await;
    let mut stdout = tokio::io::stdout();
    for x in 0..60 {
        stdout
            .write_all(b"\r")
            .await
            .expect("iceyee_webdriver/lib.rs 081");
        stdout
            .write_all(x.to_string().as_bytes())
            .await
            .expect("iceyee_webdriver/lib.rs 049");
        stdout.flush().await.expect("iceyee_webdriver/lib.rs 177");
        if number <= driver.find_all(By::Css(css)).await?.len() {
            stdout
                .write_all(b"\r")
                .await
                .expect("iceyee_webdriver/lib.rs 081");
            stdout.flush().await.expect("iceyee_webdriver/lib.rs 713");
            return Ok(());
        }
        iceyee_time::sleep(1_000).await;
    }
    return Err(WebDriverError::Timeout("".to_string()));
}

pub async fn get_element(
    driver: &WebDriver,
    css: &str,
    index: usize,
) -> WebDriverResult<WebElement> {
    return Ok(driver.find_all(By::Css(css)).await?.remove(index));
}

pub async fn add_cookie(
    driver: &WebDriver,
    key: &str,
    value: &str,
    domain: &str,
) -> WebDriverResult<()> {
    iceyee_logger::info_4("add_cookie()", key, value, domain).await;
    let mut cookie = Cookie::new(key.to_string(), value.to_string());
    cookie.set_domain(domain.to_string());
    cookie.set_path("/");
    cookie.set_same_site(Some(SameSite::None));
    return driver.add_cookie(cookie.clone()).await;
}

pub async fn set_cookie(driver: &WebDriver, cookie: &str, domain: &str) -> WebDriverResult<()> {
    iceyee_logger::info_3("set_cookie()", cookie, domain).await;
    driver.delete_all_cookies().await?;
    for x in cookie.split(";") {
        let mut y = x.split("=");
        if let Some(key) = y.next() {
            let key = key.trim().to_string();
            if let Some(value) = y.next() {
                let value = value.trim().to_string();
                let mut cookie = Cookie::new(key, value);
                cookie.set_domain(domain.to_string());
                cookie.set_path("/");
                cookie.set_same_site(Some(SameSite::None));
                driver.add_cookie(cookie.clone()).await?;
            }
        }
    }
    return Ok(());
}

pub async fn get_cookie(driver: &WebDriver) -> WebDriverResult<(String, String)> {
    let mut output_1: String = String::new();
    let mut output_2: String = String::new();
    for cookie in driver.get_all_cookies().await? {
        output_1.push_str(format!("{}={}; ", cookie.name(), cookie.value()).as_str());
        output_2.push_str(
            format!(
                "\r\ndocument.cookie='{}={}; path=/;' ",
                cookie.name(),
                cookie.value()
            )
            .as_str(),
        );
    }
    return Ok((output_1, output_2));
}
