// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::client::HttpClient;
use iceyee_net::http::client::Proxy;
use iceyee_net::http::client::Socks5Proxy;

// Enum.

// Trait.

// Struct.

// Function.

#[tokio::test]
pub async fn test_httpclient_socks5_proxy() {
    println!("");
    let proxy = Socks5Proxy::new("localhost", 1082, None).wrap();
    let proxy = Socks5Proxy::new("vpn.iceyee.cn", 10002, Some("iceyee:74591870")).wrap();
    let url: &str = "http://ip-api.com/json/";
    let url: &str = "http://www.baidu.com/";
    let url: &str = "https://www.baidu.com/";
    let url: &str = "https://www.c5game.com/";
    let url: &str = "https://buff.163.com/";
    let _ = HttpClient::new()
        .set_verbose(true)
        .set_url::<&str>(url)
        .expect("test_http_client_no_proxy.rs 449")
        .set_header("Connection", "close")
        .set_forwarded(None)
        .send(Some(proxy))
        .await;
}
