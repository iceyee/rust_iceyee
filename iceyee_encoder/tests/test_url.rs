// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_encoder::UrlEncoder;
use iceyee_encoder::UrlError;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_url_encoder() {
    println!("");
    println!("Url编码.");
    let table = [(" 1_1 ", "+1%5F1+")];
    println!("测试encode功能.");
    for (x, y) in table {
        println!("{x} <encode> {y}");
        assert_eq!(UrlEncoder::encode(x), y);
    }
    let table = [
        ("%201_1%20", " 1_1 "),
        ("%201+1%20", " 1 1 "),
        ("+1+1+", " 1 1 "),
    ];
    println!("测试decode功能.");
    for (x, y) in table {
        println!("{x} <decode> {y}");
        assert_eq!(UrlEncoder::decode(x).expect("test_url.rs 417"), y);
    }
    println!("测试decode异常输入.");
    assert_eq!(
        UrlEncoder::decode("%%".to_string()),
        Err(UrlError::InvalidFormat)
    );
    assert_eq!(
        UrlEncoder::decode("%3%45".to_string()),
        Err(UrlError::InvalidFormat)
    );
    assert_eq!(
        UrlEncoder::decode("%34%5".to_string()),
        Err(UrlError::InvalidFormat)
    );
    return;
}
