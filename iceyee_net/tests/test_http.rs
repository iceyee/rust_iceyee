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

#[test]
pub fn test_status() {
    use iceyee_net::http::Status;

    println!("");
    for x in [
        200, 201, 202, 204, 301, 302, 304, 400, 401, 403, 404, 500, 501, 502, 503, 119,
    ] {
        let status: Status = Status::from(x);
        println!("{} {}", status.default_string(), Into::<u64>::into(status));
    }
    return;
}

#[test]
pub fn test_url_and_args() {
    use iceyee_net::http::Args;
    use iceyee_net::http::Url;

    println!("");
    let url: Url = Url::new("http://api.k780.com/?app=ip.local&appkey=10003&sign=b59bc3ef6191eb9f747dd4e83c99f2a4&format=json").unwrap();
    println!("{:#?}", url);
    let url: Url = Url::new("http://ip-api.com/json/").unwrap();
    println!("{:#?}", url);
    let url: Url = Url::new("https://www.baidu.com/p?a=1&b=2&c=3#ID").unwrap();
    println!("{:#?}", url);
    assert!(url.protocol == "https:");
    assert!(url.host == "www.baidu.com");
    assert!(url.port == 443);
    assert!(url.path == "/p");
    assert!(url.query.is_some() && url.query.as_ref().unwrap() == "?a=1&b=2&c=3");
    assert!(url.fragment.is_some() && url.fragment.as_ref().unwrap() == "#ID");
    let url: Url = Url::new("http://www.baidu.com/p?a=1&b=2&c=3#ID").unwrap();
    println!("{:#?}", url);
    assert!(url.protocol == "http:");
    assert!(url.host == "www.baidu.com");
    assert!(url.port == 80);
    assert!(url.path == "/p");
    assert!(url.query.is_some() && url.query.as_ref().unwrap() == "?a=1&b=2&c=3");
    assert!(url.fragment.is_some() && url.fragment.as_ref().unwrap() == "#ID");
    let url: Url = Url::new("https://www.baidu.com:10443/p?a=1&b=2&c=3#ID").unwrap();
    println!("{:#?}", url);
    assert!(url.protocol == "https:");
    assert!(url.host == "www.baidu.com");
    assert!(url.port == 10443);
    assert!(url.path == "/p");
    assert!(url.query.is_some() && url.query.as_ref().unwrap() == "?a=1&b=2&c=3");
    assert!(url.fragment.is_some() && url.fragment.as_ref().unwrap() == "#ID");
    let url: Url = Url::new("https://www.baidu.com?a=1&b=2&c=3#ID").unwrap();
    println!("{:#?}", url);
    assert!(url.protocol == "https:");
    assert!(url.host == "www.baidu.com");
    assert!(url.port == 443);
    assert!(url.path == "/");
    assert!(url.query.is_some() && url.query.as_ref().unwrap() == "?a=1&b=2&c=3");
    assert!(url.fragment.is_some() && url.fragment.as_ref().unwrap() == "#ID");
    let url: Url = Url::new("https://www.baidu.com/p#ID").unwrap();
    println!("{:#?}", url);
    assert!(url.protocol == "https:");
    assert!(url.host == "www.baidu.com");
    assert!(url.port == 443);
    assert!(url.path == "/p");
    assert!(url.query.is_none());
    assert!(url.fragment.is_some() && url.fragment.as_ref().unwrap() == "#ID");
    let url: Url = Url::new("https://www.baidu.com/p?a=1&b=2&c=3").unwrap();
    println!("{:#?}", url);
    assert!(url.protocol == "https:");
    assert!(url.host == "www.baidu.com");
    assert!(url.port == 443);
    assert!(url.path == "/p");
    assert!(url.query.is_some() && url.query.as_ref().unwrap() == "?a=1&b=2&c=3");
    assert!(url.fragment.is_none());

    assert!(Url::new("//www.baidu.com/p?a=1&b=2&c=3").is_err());
    assert!(Url::new("https:///p?a=1&b=2&c=3").is_err());
    assert!(Url::new("https://www.baidu.com:").is_err());
    assert!(Url::new("https://www.baidu.com:999999").is_err());

    let mut args: Args = Args::new();
    args.add("你好", "1");
    args.add("你好", "2");
    args.add("我好", "他好");
    args.add("k", "PrW4rLRM-K40GMA77lYUD+fvXc8=");
    println!("{}", args.to_string());
    let args: Args = Args::parse(args.to_string().as_str());
    println!("{:#?}", args);
    return;
}

#[tokio::test]
pub async fn test_request() {
    use iceyee_net::http::Request;

    println!("");
    let mut request: Request = Request::new();
    request
        .header
        .insert("Header1".to_string(), "Value1".to_string());
    request
        .header
        .insert("Header3".to_string(), "Value3".to_string());
    request
        .header
        .insert("Header2".to_string(), "Value2".to_string());
    request.query.add("k", "PrW4rLRM-K40GMA77lYUD+fvXc8=");
    request.query.add("q", "PrW4rLRM-K40GMA77lYUD+fvXc8=");
    println!("{}", request.to_string());
    let s: &str = "\
GET / HTTP/1.1\r\n\
Host: www.baidu.com\r\n\
Accept: */*\r\n\
User-Agent: ICEYEE/1\r\n\
Content-Length: 5\r\n\
\r\n\
hello\
";
    let a001 = Request::read_from(s.as_bytes()).await.unwrap();
    println!("{}{}", a001.to_string(), unsafe {
        String::from_utf8_unchecked(a001.body.clone())
    });
}

#[tokio::test]
pub async fn test_response() {
    use iceyee_net::http::Response;

    println!("");
    let s: &str = "\
HTTP/1.1 403 Forbidden\r\n\
Content-Length: 5\r\n\
Header2: Value2\r\n\
Header1: Value1\r\n\
Header3: Value3\r\n\
\r\n\
hello\
";
    let a001 = Response::read_from(s.as_bytes()).await.unwrap();
    println!("{}{}", a001.to_string(), unsafe {
        String::from_utf8_unchecked(a001.body.clone())
    });
    let s: &str = "\
HTTP/1.1 403 Forbidden\r\n\
Transfer-Encoding: chunk\r\n\
Header2: Value2\r\n\
Header1: Value1\r\n\
Header3: Value3\r\n\
\r\n\
5\r\n\
hello\r\n\
1\r\n \
\r\n\
5\r\n\
world\r\n\
0\r\n\
\r\n\
";
    let a001 = Response::read_from(s.as_bytes()).await.unwrap();
    println!("{}{}", a001.to_string(), unsafe {
        String::from_utf8_unchecked(a001.body.clone())
    });
}
