// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_encoder::Base64Encoder;
use iceyee_encoder::Encoder;
use iceyee_encoder::UrlEncoder;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_base64_encoder() {
    println!("");
    let table = [
        ("hello world.", "aGVsbG8gd29ybGQu"),
        ("hello world", "aGVsbG8gd29ybGQ="),
        ("hello worl", "aGVsbG8gd29ybA=="),
        ("hello wor", "aGVsbG8gd29y"),
    ];
    for (x, y) in table {
        assert!(Base64Encoder::encode(x.as_bytes().to_vec()).unwrap() == y);
        assert!(String::from_utf8(Base64Encoder::decode(y.to_string()).unwrap()).unwrap() == x);
    }
    return;
}

#[test]
pub fn test_url_encoder() {
    println!("");
    assert!(UrlEncoder::encode(" 1_1 ".to_string()).unwrap() == "%201_1%20");
    assert!(UrlEncoder::decode("%201_1%20".to_string()).unwrap() == " 1_1 ");
    assert!(UrlEncoder::decode("%201+1%20".to_string()).unwrap() == " 1 1 ");
    return;
}