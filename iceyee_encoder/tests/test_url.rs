// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
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
    let table = [(" 1_1 ", "+1_1+")];
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
        assert_eq!(UrlEncoder::decode(x).expect("NEVER"), y);
    }
    println!("测试decode异常输入.");
    println!("%%");
    println!("%3%45");
    println!("%34%5");
    assert!(UrlEncoder::decode("%%").is_err());
    assert!(UrlEncoder::decode("%3%45").is_err());
    assert!(UrlEncoder::decode("%34%5").is_err());
    return;
}
