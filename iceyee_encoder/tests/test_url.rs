// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
/* clippy: 本crate风格是每个函数显式写return. */
#![allow(clippy::needless_return, clippy::println_empty_string)]

// Use.

use iceyee_encoder::UrlEncoder;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_url_encoder() {
    println!("");
    println!("Url编码.");
    let table = [(" 1_1 ", "+1_1+"), ("hello world", "hello+world")];
    println!("测试encode功能.");
    for (x, y) in table {
        println!("{x} <encode> {y}");
        assert_eq!(UrlEncoder::encode(x), y);
    }
    let table = [
        ("%201_1%20", " 1_1 "),
        ("%201+1%20", " 1 1 "),
        ("+1+1+", " 1 1 "),
        ("%e4%b8%ad%e6%96%87", "中文"),
        ("%E4%B8%AD%E6%96%87", "中文"),
    ];
    println!("测试decode功能.");
    for (x, y) in table {
        println!("{x} <decode> {y}");
        assert_eq!(UrlEncoder::decode(x).expect("NEVER"), y);
    }
    println!("测试decode异常输入.");
    println!("%%");
    println!("%3%45");
    println!("%34%5");
    assert_eq!(
        UrlEncoder::decode("%%").map_err(|x| x.contains("错误的格式")),
        Err(true)
    );
    assert_eq!(
        UrlEncoder::decode("%3%45").map_err(|x| x.contains("错误的格式")),
        Err(true)
    );
    assert_eq!(
        UrlEncoder::decode("%34%5").map_err(|x| x.contains("错误的格式")),
        Err(true)
    );
    println!("测试保留字符集, RFC3986的unreserved是 A-Z a-z 0-9 - . _ ~.");
    let table = [
        ("~", "~"),
        ("-._~", "-._~"),
        ("$", "%24"),
        ("!", "%21"),
        ("*", "%2A"),
        ("'", "%27"),
        ("(", "%28"),
        (")", "%29"),
        ("+", "%2B"),
        ("&", "%26"),
        ("=", "%3D"),
        ("/", "%2F"),
        ("#", "%23"),
        (" ", "+"),
    ];
    for (x, y) in table {
        println!("{x:?} <encode> {y:?}");
        assert_eq!(UrlEncoder::encode(x), y);
        println!("{y:?} <decode> {x:?}");
        assert_eq!(UrlEncoder::decode(y).expect("NEVER"), x);
    }
    println!("测试往返.");
    let table = [
        "hello world",
        "中文测试",
        "a+b=c&d",
        "$-._~",
        "100%",
        "emoji 😀",
    ];
    for x in table {
        let cipher: String = UrlEncoder::encode(x);
        println!(
            "{x} <encode> {cipher} <decode> {}",
            UrlEncoder::decode(&cipher).expect("NEVER")
        );
        assert_eq!(UrlEncoder::decode(&cipher).expect("NEVER"), x);
    }
    return;
}
