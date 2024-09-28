// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::client::HttpClient;
use iceyee_net::http::client::NoProxy;
use iceyee_net::http::client::Proxy;

// Enum.

// Trait.

// Struct.

// Function.

// #[tokio::test]
#[allow(unused_variables)]
pub async fn test_httpclient_no_proxy() {
    println!("");
    let url: &str = "http://www.baidu.com/";
    let url: &str = "https://www.baidu.com/";
    let url: &str = "https://www.c5game.com/";
    let url: &str = "http://ip-api.com/json/";
    let url: &str = "https://buff.163.com/";
    // println!("{}", HttpClient::get_expect_string(true, url, "").await);
    let _ = HttpClient::new()
        .set_verbose(true)
        .set_url(url)
        .expect("")
        .set_header("Connection", "close")
        .set_forwarded(None)
        .send(None)
        .await;
}

// #[tokio::test]
#[allow(unused_variables)]
pub async fn test_httpclient_repeated_use() {
    println!("");
    let proxy = NoProxy::new().wrap();
    let url: &str = "https://www.c5game.com/";
    let url: &str = "https://buff.163.com/";
    let url: &str = "http://www.baidu.com/";
    let url: &str = "https://www.baidu.com/";
    let url: &str = "http://ip-api.com/json/";
    HttpClient::new()
        .set_verbose(true)
        .set_url(url)
        .expect("073")
        .set_forwarded(None)
        .send(Some(proxy.clone()))
        .await
        .expect("081");
    HttpClient::new()
        .set_verbose(true)
        .set_url(url)
        .expect("049")
        .set_forwarded(None)
        .send(Some(proxy.clone()))
        .await
        .expect("177");
    HttpClient::new()
        .set_verbose(true)
        .set_url(url)
        .expect("665")
        .set_forwarded(None)
        .send(Some(proxy.clone()))
        .await
        .expect("433");
}
