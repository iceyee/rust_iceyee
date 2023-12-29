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

#[tokio::test]
pub async fn test_httpclient_no_proxy() {
    println!("");
    let url: &str = "http://www.baidu.com/";
    let url: &str = "https://www.baidu.com/";
    let url: &str = "https://buff.163.com/";
    let url: &str = "http://ip-api.com/json/";
    let url: &str = "https://www.c5game.com/";
    HttpClient::new()
        .set_verbose(true)
        .set_url::<&str>(url)
        .expect("test_http_client_no_proxy.rs 513")
        .set_header("Connection", "close")
        .set_forwarded(None)
        .send(None)
        .await
        .expect("test_http_client_no_proxy.rs 321");
}

// #[tokio::test]
pub async fn test_httpclient_repeated_use() {
    let proxy = NoProxy::new().wrap();
    let url: &str = "http://www.baidu.com/";
    let url: &str = "https://www.baidu.com/";
    let url: &str = "http://ip-api.com/json/";
    let url: &str = "https://www.c5game.com/";
    let url: &str = "https://buff.163.com/";
    HttpClient::new()
        .set_verbose(true)
        .set_url::<&str>(url)
        .expect("test_http_client_no_proxy.rs 513")
        .set_forwarded(None)
        .send(Some(proxy.clone()))
        .await
        .expect("test_http_client_no_proxy.rs 321");
    HttpClient::new()
        .set_verbose(true)
        .set_url::<&str>(url)
        .expect("test_http_client_no_proxy.rs 513")
        .set_forwarded(None)
        .send(Some(proxy.clone()))
        .await
        .expect("test_http_client_no_proxy.rs 321");
    HttpClient::new()
        .set_verbose(true)
        .set_url::<&str>(url)
        .expect("test_http_client_no_proxy.rs 513")
        .set_forwarded(None)
        .send(Some(proxy.clone()))
        .await
        .expect("test_http_client_no_proxy.rs 321");
}
