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
use iceyee_random::Random;
use std::process::Child;
use thirtyfour::common::capabilities::desiredcapabilities::Proxy as WebProxy;
use thirtyfour::prelude::*;
use thirtyfour::ChromeCapabilities;
use thirtyfour::EdgeCapabilities;
use tokio::io::AsyncWriteExt;

// Enum.

// Trait.

// Struct.

// Function.

pub async fn chrome(
    headless: bool,
    http_proxy: Option<String>,
    socks5_proxy: Option<String>,
) -> WebDriverResult<(WebDriver, Child)> {
    let port: u64 = Random::next() % 0x7FFF + 0xFFF;
    let child = std::process::Command::new("chromium.chromedriver")
        .arg("--log-level=WARNING")
        .arg("--port=".to_string() + port.to_string().as_str())
        .spawn()
        .expect("start chromium.chromedriver");
    iceyee_time::sleep(3_000).await;
    let mut options: ChromeCapabilities = ChromeCapabilities::new();
    options.set_ignore_certificate_errors()?;
    options.add_chrome_arg("--no-sandbox")?;
    if headless {
        options.set_headless()?;
    }
    let proxy: WebProxy = WebProxy::Manual {
        ftp_proxy: None,
        http_proxy: http_proxy.clone(),
        ssl_proxy: None,
        socks_proxy: socks5_proxy.clone(),
        socks_version: Some(5),
        socks_username: None,
        socks_password: None,
        no_proxy: None,
    };
    if http_proxy.is_some() || socks5_proxy.is_some() {
        options.insert(
            "proxy".to_string(),
            serde_json::to_value(proxy).map_err(|e| WebDriverError::Json(e))?,
        );
    }
    iceyee_logger::info!("打开浏览器");
    iceyee_logger::info_object!(&options);
    let url: String = format!("http://localhost:{port}");
    let driver: WebDriver = WebDriver::new(&url, options).await?;
    if !headless {
        if let serde_json::value::Value::Number(number) = driver
            .execute("return window.outerWidth;", vec![])
            .await?
            .json()
        {
            let width: u32 = number.as_u64().expect("NEVER") as u32;
            if let serde_json::value::Value::Number(number) = driver
                .execute("return window.outerHeight;", vec![])
                .await?
                .json()
            {
                let height: u32 = number.as_u64().expect("NEVER") as u32;
                driver.set_window_rect(0, 0, width, height * 3 / 4).await?;
            }
        }
    }
    return Ok((driver, child));
}

pub async fn edge(
    headless: bool,
    http_proxy: Option<String>,
    socks5_proxy: Option<String>,
) -> WebDriverResult<(WebDriver, Child)> {
    let _headless = if headless { "--headless" } else { " " };
    let port: u64 = Random::next() % 0x7FFF + 0xFFF;
    let child = std::process::Command::new("msedgedriver")
        .arg(_headless)
        .arg("--log-level=WARNING")
        .arg("--port=".to_string() + port.to_string().as_str())
        .spawn()
        .expect("start msedgedriver");
    iceyee_time::sleep(3_000).await;
    let mut options: EdgeCapabilities = EdgeCapabilities::new();
    let proxy: WebProxy = WebProxy::Manual {
        ftp_proxy: None,
        http_proxy: http_proxy.clone(),
        ssl_proxy: None,
        socks_proxy: socks5_proxy.clone(),
        socks_version: Some(5),
        socks_username: None,
        socks_password: None,
        no_proxy: None,
    };
    if http_proxy.is_some() || socks5_proxy.is_some() {
        options.insert(
            "proxy".to_string(),
            serde_json::to_value(proxy).map_err(|e| WebDriverError::Json(e))?,
        );
    }
    iceyee_logger::info!("打开浏览器");
    iceyee_logger::info_object!(&options);
    let url: String = format!("http://localhost:{port}");
    let driver: WebDriver = WebDriver::new(&url, options).await?;
    if let serde_json::value::Value::Number(number) = driver
        .execute("return window.outerWidth;", vec![])
        .await?
        .json()
    {
        let width: u32 = number.as_u64().expect("NEVER") as u32;
        if let serde_json::value::Value::Number(number) = driver
            .execute("return window.outerHeight;", vec![])
            .await?
            .json()
        {
            let height: u32 = number.as_u64().expect("NEVER") as u32;
            driver.set_window_rect(0, 0, width, height * 3 / 4).await?;
        }
    }
    return Ok((driver, child));
}

pub async fn wait_url(driver: &WebDriver, url: &str, equal: bool) -> WebDriverResult<()> {
    iceyee_logger::info!("wait_url", url);
    let mut stdout = tokio::io::stdout();
    for x in 0..60 {
        stdout.write_all(b"\r").await.expect("Stdout::write_all");
        stdout
            .write_all(x.to_string().as_bytes())
            .await
            .expect("Stdout::write_all");
        stdout.flush().await.expect("Stdout::flush");
        if (driver.current_url().await?.as_str() == url && equal)
            || (driver.current_url().await?.as_str() != url && !equal)
        {
            stdout.write_all(b"\r").await.expect("Stdout::write_all");
            stdout.flush().await.expect("Stdout::flush");
            iceyee_time::sleep(1_000).await;
            return Ok(());
        }
        iceyee_time::sleep(1_000).await;
    }
    return Err(WebDriverError::Timeout("".to_string()));
}

pub async fn wait_ready(driver: &WebDriver) -> WebDriverResult<()> {
    iceyee_logger::info!("wait_ready");
    let mut stdout = tokio::io::stdout();
    for x in 0..60 {
        stdout.write_all(b"\r").await.expect("Stdout::write_all");
        stdout
            .write_all(x.to_string().as_bytes())
            .await
            .expect("Stdout::write_all");
        stdout.flush().await.expect("Stdout::flush");
        if driver.status().await?.ready {
            stdout.write_all(b"\r").await.expect("Stdout::write_all");
            stdout.flush().await.expect("Stdout::flush");
            iceyee_time::sleep(1_000).await;
            return Ok(());
        }
        iceyee_time::sleep(1_000).await;
    }
    return Err(WebDriverError::Timeout("".to_string()));
}

pub async fn wait_element(driver: &WebDriver, css: &str, number: usize) -> WebDriverResult<()> {
    iceyee_logger::info!("wait_element", css, number);
    let mut stdout = tokio::io::stdout();
    for x in 0..60 {
        stdout.write_all(b"\r").await.expect("Stdout::write_all");
        stdout
            .write_all(x.to_string().as_bytes())
            .await
            .expect("Stdout::write_all");
        stdout.flush().await.expect("Stdout::flush");
        if number <= driver.find_all(By::Css(css)).await?.len() {
            stdout.write_all(b"\r").await.expect("Stdout::write_all");
            stdout.flush().await.expect("Stdout::flush");
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
    iceyee_logger::info!("add_cookie", key, value, domain);
    let mut cookie = Cookie::new(key.to_string(), value.to_string());
    cookie.set_domain(domain.to_string());
    cookie.set_path("/");
    cookie.set_same_site(Some(SameSite::None));
    return driver.add_cookie(cookie.clone()).await;
}

pub async fn set_cookie(driver: &WebDriver, cookie: &str, domain: &str) -> WebDriverResult<()> {
    iceyee_logger::info!("set_cookie", cookie, domain);
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
