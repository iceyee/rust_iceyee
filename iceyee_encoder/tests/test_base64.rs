// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_encoder::Base64Encoder;
use iceyee_encoder::Base64Error;

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
    println!("测试encode功能.");
    for (x, y) in table {
        println!("{x} <encode> {y}");
        assert_eq!(Base64Encoder::encode(&x.as_bytes().to_vec()), y);
    }
    println!("测试decode功能.");
    for (x, y) in table {
        println!("{y} <decode> {x}");
        assert_eq!(
            String::from_utf8(Base64Encoder::decode(y).expect("test_base64.rs 529"))
                .expect("test_base64.rs 889"),
            x
        );
    }
    println!("测试异常输入.");
    Base64Encoder::decode("/234").expect("test_base64.rs 033");
    assert_eq!(
        Base64Encoder::decode("12345"),
        Err(Base64Error::InvalidLength(5)),
    );
    assert_eq!(
        Base64Encoder::decode("123456"),
        Err(Base64Error::InvalidLength(6))
    );
    assert_eq!(
        Base64Encoder::decode("1234567"),
        Err(Base64Error::InvalidLength(7))
    );
    assert_eq!(
        Base64Encoder::decode("123@"),
        Err(Base64Error::UnexpectedCharacter('@'))
    );
    assert_eq!(
        Base64Encoder::decode("123#"),
        Err(Base64Error::UnexpectedCharacter('#'))
    );
    assert_eq!(
        Base64Encoder::decode("@23#"),
        Err(Base64Error::UnexpectedCharacter('@'))
    );
    return;
}
