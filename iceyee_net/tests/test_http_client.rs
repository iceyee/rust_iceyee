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
pub async fn test_httpclient_no_proxy() {
    use iceyee_net::http::client::HttpClient;
    use iceyee_net::http::Response;

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
