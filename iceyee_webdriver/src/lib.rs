// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

/* Use. */

use iceyee_random::Random;
use std::process::Child;
use thirtyfour::common::capabilities::desiredcapabilities::DesiredCapabilities;
use thirtyfour::common::capabilities::desiredcapabilities::Proxy as WebProxy;
use thirtyfour::prelude::*;
use tokio::io::AsyncWriteExt;

/* Enum. */

/* Trait. */

/* Struct. */

/// 子进程的RAII包装, drop时自动kill并回收, 避免僵尸进程.
struct DropChild(Option<Child>);

impl DropChild {
    fn new(child: Child) -> Self {
        return DropChild(Some(child));
    }

    /// 取出子进程句柄, 之后由调用者负责其生命周期.
    fn take(&mut self) -> Option<Child> {
        return self.0.take();
    }
}

impl Drop for DropChild {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            child.kill().ok();
            child.wait().ok();
        }
    }
}

/* Function. */

/// 启动chrome浏览器，主要用于linux.
pub async fn chrome(
    headless: bool,
    http_proxy: Option<String>,
    socks5_proxy: Option<String>,
) -> WebDriverResult<(WebDriver, Child)> {
    let port = Random::next() % 0x7FFF + 0xFFF;
    let child = std::process::Command::new("chromium.chromedriver")
        .arg("--log-level=WARNING")
        .arg("--port=".to_string() + port.to_string().as_str())
        .spawn()
        .expect("start chromium.chromedriver");
    let mut child = DropChild::new(child);
    iceyee_time::sleep(3_000).await;
    let mut options = DesiredCapabilities::chrome();
    options.set_ignore_certificate_errors()?;
    options.set_no_sandbox()?;
    if headless {
        options.set_headless()?;
    }
    if http_proxy.is_none() && socks5_proxy.is_none() {
        options.set_proxy(WebProxy::Direct)?;
    } else {
        let proxy: WebProxy = WebProxy::Manual {
            ftp_proxy: None,
            http_proxy: http_proxy.clone(),
            ssl_proxy: http_proxy.clone(),
            socks_proxy: socks5_proxy.clone(),
            socks_version: Some(5),
            socks_username: None,
            socks_password: None,
            no_proxy: None,
        };
        options.set_proxy(proxy)?;
    }
    iceyee_logger::info!("打开浏览器");
    iceyee_logger::info_object!(&options);
    let url: String = format!("http://localhost:{port}");
    let driver: WebDriver = WebDriver::new(&url, options).await?;
    if !headless {
        driver.fullscreen_window().await?;
        let rect = driver.get_window_rect().await?;
        driver
            .set_window_rect(0, 0, rect.width as u32, (rect.height * 3 / 4) as u32)
            .await?;
    }
    return Ok((driver, child.take().expect("NEVER")));
}

/// 启动edge浏览器，主要用于windows.
pub async fn edge(
    headless: bool,
    http_proxy: Option<String>,
    socks5_proxy: Option<String>,
) -> WebDriverResult<(WebDriver, Child)> {
    let port: u64 = Random::next() % 0x7FFF + 0xFFF;
    let child = std::process::Command::new("msedgedriver")
        .arg("--log-level=WARNING")
        .arg("--port=".to_string() + port.to_string().as_str())
        .spawn()
        .expect("start msedgedriver");
    let mut child = DropChild::new(child);
    iceyee_time::sleep(3_000).await;
    let mut options = DesiredCapabilities::edge();
    options.set_ignore_certificate_errors()?;
    options.set_no_sandbox()?;
    if headless {
        options.set_headless()?;
    }
    if http_proxy.is_none() && socks5_proxy.is_none() {
        options.set_proxy(WebProxy::Direct)?;
    } else {
        let proxy: WebProxy = WebProxy::Manual {
            ftp_proxy: None,
            http_proxy: http_proxy.clone(),
            ssl_proxy: http_proxy.clone(),
            socks_proxy: socks5_proxy.clone(),
            socks_version: Some(5),
            socks_username: None,
            socks_password: None,
            no_proxy: None,
        };
        options.set_proxy(proxy)?;
    }
    iceyee_logger::info!("打开浏览器");
    iceyee_logger::info_object!(&options);
    let url: String = format!("http://localhost:{port}");
    let driver: WebDriver = WebDriver::new(&url, options).await?;
    if !headless {
        driver.fullscreen_window().await?;
        let rect = driver.get_window_rect().await?;
        driver
            .set_window_rect(0, 0, rect.width as u32, (rect.height * 3 / 4) as u32)
            .await?;
    }
    return Ok((driver, child.take().expect("NEVER")));
}

/// 等待URL变化.
///
/// - @param url 用于与当前页面url比较.
/// - @param equal
///   - true 当`current_url==url`时返回
///   - false 当`current_url!=url`时返回
/// - @param t 等待超时，单位:秒.
pub async fn wait_url(driver: &WebDriver, url: &str, equal: bool, t: usize) -> WebDriverResult<()> {
    iceyee_logger::info!("wait_url", url, t);
    let mut stdout = tokio::io::stdout();
    for x in 0..t {
        if x != 0 {
            iceyee_time::sleep(1_000).await;
        }
        stdout.write_all(b"\r").await.expect("Stdout::write_all");
        stdout
            .write_all(x.to_string().as_bytes())
            .await
            .expect("Stdout::write_all");
        stdout.flush().await.expect("Stdout::flush");
        let current = driver.current_url().await?;
        if (current.as_str() == url && equal) || (current.as_str() != url && !equal) {
            stdout.write_all(b"\r").await.expect("Stdout::write_all");
            stdout.flush().await.expect("Stdout::flush");
            return Ok(());
        }
    }
    return Err(WebDriverError::Timeout("".to_string()));
}

/// 等待页面加载完毕.
///
/// - @param t 等待超时，单位:秒.
pub async fn wait_ready(driver: &WebDriver, t: usize) -> WebDriverResult<()> {
    iceyee_logger::info!("wait_ready", t);
    let mut stdout = tokio::io::stdout();
    for x in 0..t {
        if x != 0 {
            iceyee_time::sleep(1_000).await;
        }
        stdout.write_all(b"\r").await.expect("Stdout::write_all");
        stdout
            .write_all(x.to_string().as_bytes())
            .await
            .expect("Stdout::write_all");
        stdout.flush().await.expect("Stdout::flush");
        if driver.status().await?.ready {
            stdout.write_all(b"\r").await.expect("Stdout::write_all");
            stdout.flush().await.expect("Stdout::flush");
            return Ok(());
        }
    }
    return Err(WebDriverError::Timeout("".to_string()));
}

/// 等待元素出现.
///
/// - @param css css选择器.
/// - @param number 数量不低于number时返回.
/// - @param t 等待超时，单位:秒.
pub async fn wait_element(
    driver: &WebDriver,
    css: &str,
    number: usize,
    t: usize,
) -> WebDriverResult<()> {
    iceyee_logger::info!("wait_element", css, number, t);
    let mut stdout = tokio::io::stdout();
    for x in 0..t {
        if x != 0 {
            iceyee_time::sleep(1_000).await;
        }
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
    }
    return Err(WebDriverError::Timeout("".to_string()));
}

/// 判断css选择器匹配的元素是否存在.
pub async fn has_element(driver: &WebDriver, css: &str) -> WebDriverResult<bool> {
    return Ok(!driver.find_all(By::Css(css)).await?.is_empty());
}

/// 使用css选择器取得单个元素.
pub async fn get_element(
    driver: &WebDriver,
    css: &str,
    index: usize,
) -> WebDriverResult<WebElement> {
    let elements = driver.find_all(By::Css(css)).await?;
    let len = elements.len();
    return elements.get(index).cloned().ok_or_else(|| {
        thirtyfour::error::no_such_element(format!("索引越界: index={}, 元素数={}", index, len))
    });
}

/// 添加单个cookie.
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
    cookie.set_same_site(SameSite::None);
    return driver.add_cookie(cookie).await;
}

/// 先清空所有旧cookie，然后添加新的cookie.
///
/// - @param cookie 格式同http请求头的cookie.
/// - @param domain
pub async fn set_cookie(driver: &WebDriver, cookie: &str, domain: &str) -> WebDriverResult<()> {
    iceyee_logger::info!("set_cookie", cookie, domain);
    driver.delete_all_cookies().await?;
    for x in cookie.split(";") {
        let mut y = x.splitn(2, "=");
        if let Some(key) = y.next() {
            let key = key.trim().to_string();
            if let Some(value) = y.next() {
                let value = value.trim().to_string();
                let mut cookie = Cookie::new(key, value);
                cookie.set_domain(domain.to_string());
                cookie.set_path("/");
                cookie.set_same_site(SameSite::None);
                driver.add_cookie(cookie).await?;
            }
        }
    }
    return Ok(());
}

/// @return $0用于http请求头cookie, $1用浏览器控制台直接写入cookie.
pub async fn get_cookie(driver: &WebDriver) -> WebDriverResult<(String, String)> {
    use std::fmt::Write as _;
    let mut output_1: String = String::new();
    let mut output_2: String = String::new();
    for cookie in driver.get_all_cookies().await? {
        write!(output_1, "{}={}; ", cookie.name, cookie.value).ok();
        let name = cookie.name.replace('\\', "\\\\").replace('\'', "\\'");
        let value = cookie.value.replace('\\', "\\\\").replace('\'', "\\'");
        write!(
            output_2,
            "\r\ndocument.cookie='{}={}; path=/;' ",
            name, value
        )
        .ok();
    }
    return Ok((output_1, output_2));
}
