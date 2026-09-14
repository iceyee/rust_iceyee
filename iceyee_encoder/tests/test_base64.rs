// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
/* clippy: 本crate风格是每个函数显式写return. */
#![allow(clippy::needless_return, clippy::println_empty_string)]

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
        assert_eq!(Base64Encoder::encode(x.as_bytes()), y);
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
    assert_eq!(
        Base64Encoder::decode("12345").map_err(|x| x.contains("无效的长度")),
        Err(true)
    );
    assert_eq!(
        Base64Encoder::decode("123456").map_err(|x| x.contains("无效的长度")),
        Err(true)
    );
    assert_eq!(
        Base64Encoder::decode("1234567").map_err(|x| x.contains("无效的长度")),
        Err(true)
    );
    assert_eq!(
        Base64Encoder::decode("123@").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    assert_eq!(
        Base64Encoder::decode("123#").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    assert_eq!(
        Base64Encoder::decode("@23#").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    println!("测试'='错位的输入, 应该报错, 不能静默丢掉最后一位.");
    println!("ab=c");
    println!("AB=D");
    println!("ABCDEF=H");
    println!("ABCDEFGHIJ=L");
    println!("=bcd");
    println!("====");
    println!("ab=cab=c");
    for x in [
        "ab=c",
        "AB=D",
        "ABCDEF=H",
        "ABCDEFGHIJ=L",
        "=bcd",
        "====",
        "ab=cab=c",
    ] {
        assert_eq!(
            Base64Encoder::decode(x).map_err(|e| e.contains("出现未预期的字符")),
            Err(true)
        );
    }
    println!("测试非规范补位(补位不为0)的输入, 应该报错.");
    println!("ab==");
    println!("aGVsbG8gd29ybGR=");
    for x in ["ab==", "aGVsbG8gd29ybGR=", "AB=="] {
        assert_eq!(
            Base64Encoder::decode(x).map_err(|e| e.contains("出现未预期的字符")),
            Err(true)
        );
    }
    println!("测试规范的补位, 应该通过.");
    for (x, n) in [
        ("abc=", 2usize),
        ("aGVsbG8gd29ybGQ=", 11),
        ("aGVsbG8gd29y", 9),
    ] {
        println!("{x} <decode> {n} 字节");
        assert_eq!(Base64Encoder::decode(x).expect("NEVER").len(), n);
    }
    println!("测试Url安全形式.");
    let table: [(&[u8], &str); 5] = [
        (&b"hello world."[..], "aGVsbG8gd29ybGQu"),
        (&b"hello world"[..], "aGVsbG8gd29ybGQ"),
        (&b"hello worl"[..], "aGVsbG8gd29ybA"),
        (&b"hello wor"[..], "aGVsbG8gd29y"),
        (&[0xFBu8, 0xFF][..], "-_8"),
    ];
    for (x, y) in table {
        println!("{x:?} <encode_url> {y}");
        assert_eq!(Base64Encoder::encode_url(x), y);
        println!("{y} <decode_url> {x:?}");
        assert_eq!(Base64Encoder::decode_url(y).expect("NEVER"), x.to_vec());
    }
    println!("测试Url安全形式与标准形式的互通.");
    let data: Vec<u8> = [0xFB, 0xFF, 0xBF, 0x00, 0x3E].to_vec();
    let standard: String = Base64Encoder::encode(&data);
    let url: String = Base64Encoder::encode_url(&data);
    println!("{data:?} <encode> {standard}");
    println!("{data:?} <encode_url> {url}");
    assert_eq!(Base64Encoder::decode(&standard).expect("NEVER"), data);
    assert_eq!(Base64Encoder::decode_url(&standard).expect("NEVER"), data);
    assert_eq!(Base64Encoder::decode_url(&url).expect("NEVER"), data);
    /* 标准解码不能直接吃Url安全形式. */
    assert!(Base64Encoder::decode(&url).is_err());
    println!("测试Url安全形式的异常长度.");
    println!("a");
    println!("abcde");
    for x in ["a", "abcde"] {
        assert_eq!(
            Base64Encoder::decode_url(x).map_err(|e| e.contains("无效的长度")),
            Err(true)
        );
    }
    return;
}
