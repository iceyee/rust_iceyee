// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_encoder::Base64Encoder;

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
            String::from_utf8(Base64Encoder::decode(y).expect("NEVER")).expect("NEVER"),
            x
        );
    }
    println!("测试异常输入.");
    println!("12345");
    println!("123456");
    println!("1234567");
    println!("123@");
    println!("123#");
    println!("@23#");
    assert!(Base64Encoder::decode("12345").is_err());
    assert!(Base64Encoder::decode("123456").is_err());
    assert!(Base64Encoder::decode("1234567").is_err());
    assert!(Base64Encoder::decode("123@").is_err());
    assert!(Base64Encoder::decode("123#").is_err());
    assert!(Base64Encoder::decode("@23#").is_err());
    return;
}
