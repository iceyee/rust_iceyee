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
pub async fn test_httpclient_no_proxy() {
    use iceyee_net::http::client::HttpClient;

    println!("");
    let mut http_client: HttpClient = HttpClient::new();
    http_client
        .set_verbose(true)
        .set_header("Connection", "close");
    http_client
        .set_url("http://ip-api.com/json/")
        .unwrap()
        .set_forwarded(None)
        .send(None)
        .await
        .unwrap();
    // http_client
    //     .set_url("http://ip-api.com/json/")
    //     .unwrap()
    //     .remove_header("X-Forwarded-For")
    //     .send(None)
    //     .await
    //     .unwrap();
    // http_client
    //     .set_url("http://www.baidu.com/")
    //     .unwrap()
    //     .send(None)
    //     .await
    //     .unwrap();
    // http_client
    //     .set_url("https://www.baidu.com/")
    //     .unwrap()
    //     .send(None)
    //     .await
    //     .unwrap();
}

// #[tokio::test]
pub async fn test_httpclient_http_proxy() {
    use iceyee_net::http::client::HttpClient;
    use iceyee_net::http::client::HttpProxy;
    use iceyee_net::http::client::Proxy;

    println!("");
    let proxy = HttpProxy::new("ip-api.com", 80, false, "localhost", 1081, None).wrap();
    let mut http_client: HttpClient = HttpClient::new();
    http_client
        .set_verbose(true)
        .set_url("http://ip-api.com/json/")
        .unwrap()
        .set_header("Connection", "close")
        .set_forwarded(None)
        .send(Some(proxy))
        .await;
    // let proxy = HttpProxy::new(
    //     "ip-api.com",
    //     80,
    //     false,
    //     "vpn.iceyee.cn",
    //     10001,
    //     Some("iceyee:74591870"),
    // )
    // .wrap();
    // let mut http_client: HttpClient = HttpClient::new();
    // http_client
    //     .set_verbose(true)
    //     .set_url("http://ip-api.com/json/")
    //     .unwrap()
    //     .set_header("Connection", "close")
    //     .set_forwarded(None)
    //     .send(Some(proxy))
    //     .await;
    // let proxy = HttpProxy::new(
    //     "www.c5game.com",
    //     443,
    //     true,
    //     "vpn.iceyee.cn",
    //     10001,
    //     Some("iceyee:74591870"),
    // )
    // .wrap();
    // let mut http_client: HttpClient = HttpClient::new();
    // http_client
    //     .set_verbose(true)
    //     .set_url("http://www.c5game.com/")
    //     .unwrap()
    //     .set_header("Connection", "close")
    //     .set_forwarded(None)
    //     .send(Some(proxy))
    //     .await;
    // let proxy = HttpProxy::new("www.baidu.com", 80, false, "localhost", 1081, None).wrap();
    // let mut http_client: HttpClient = HttpClient::new();
    // http_client
    //     .set_verbose(true)
    //     .set_url("http://www.baidu.com/")
    //     .unwrap()
    //     .set_header("Connection", "close")
    //     .set_forwarded(None)
    //     .send(Some(proxy))
    //     .await;
    // let proxy = HttpProxy::new("www.baidu.com", 443, true, "localhost", 1081, None).wrap();
    // let mut http_client: HttpClient = HttpClient::new();
    // http_client
    //     .set_verbose(true)
    //     .set_url("https://www.baidu.com/")
    //     .unwrap()
    //     .set_header("Connection", "close")
    //     .set_forwarded(None)
    //     .send(Some(proxy))
    //     .await;
}
