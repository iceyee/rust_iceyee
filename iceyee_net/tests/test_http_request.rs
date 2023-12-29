// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::Request;

// Enum.

// Trait.

// Struct.

// Function.

#[tokio::test]
pub async fn test_request() {
    println!("");
    println!("默的的Request.");
    println!("加入协议头, 'Header1', 'Header2', 'Header3'.");
    let mut request: Request = Request::default();
    request
        .header
        .insert("Header1".to_string(), "Value1".to_string());
    request
        .header
        .insert("Header3".to_string(), "Value3".to_string());
    request
        .header
        .insert("Header2".to_string(), "Value2".to_string());
    println!("加入请求参数, 'k', 'q'.");
    request.query.add("k", "PrW4rLRM-K40GMA77lYUD+fvXc8=");
    request.query.add("q", "PrW4rLRM-K40GMA77lYUD+fvXc8=");
    println!("{}", request.to_string());
    println!("");
    println!("Request::read_from().");
    let s: &str = "\
GET / HTTP/1.1\r\n\
Host: www.baidu.com\r\n\
Accept: */*\r\n\
User-Agent: ICEYEE/1\r\n\
Content-Length: 5\r\n\
\r\n\
hello\
";
    let a001 = Request::read_from(s.as_bytes(), None).await.unwrap();
    println!(
        "{}{}",
        a001.to_string(),
        String::from_utf8(a001.body.clone()).expect("test_http_request.rs 985")
    );
}
