// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::Response;
use iceyee_net::http::Status;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_status() {
    println!("");
    for x in [
        200, 201, 202, 204, 301, 302, 304, 400, 401, 403, 404, 500, 501, 502, 503, 119,
    ] {
        let status: Status = Status::from(x);
        println!("{} {}", status.default_string(), Into::<u16>::into(status));
    }
    return;
}

#[tokio::test]
pub async fn test_response() {
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
    let a001 = Response::read_from(s.as_bytes(), None).await.unwrap();
    println!(
        "{}{}",
        a001.to_string(),
        String::from_utf8(a001.body.clone()).expect("String::from_utf8()"),
    );
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
    let a001 = Response::read_from(s.as_bytes(), None).await.unwrap();
    println!(
        "{}{}",
        a001.to_string(),
        String::from_utf8(a001.body.clone()).expect("String::from_utf8()"),
    );
}
