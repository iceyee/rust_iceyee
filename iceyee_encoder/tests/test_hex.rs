// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
/* clippy: 本crate风格是每个函数显式写return. */
#![allow(clippy::needless_return, clippy::println_empty_string)]

// Use.

use iceyee_encoder::HexEncoder;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
fn test_hex_encoder() {
    println!("");
    println!("测试Hex编码.");
    let a: Vec<u8> = [0x12, 0x34, 0x56, 0xab, 0xcd].to_vec();
    let b1: String = "123456ABCD".to_string();
    let b2: String = "123456abcd".to_string();
    println!("{a:?} <encode> {b1:?}");
    assert_eq!(HexEncoder::encode(&a), b1);
    println!("{b1:?} <decode> {a:?}");
    assert_eq!(HexEncoder::decode(&b1).expect("NEVER"), a);
    println!("{b2:?} <decode> {a:?}");
    assert_eq!(HexEncoder::decode(&b2).expect("NEVER"), a);
    println!("测试异常输入.");
    println!("123456a");
    println!("123456abc");
    println!("12@456abcd");
    println!("123456a#cd");
    println!("123456abcg");
    assert_eq!(
        HexEncoder::decode("123456a").map_err(|x| x.contains("无效的长度")),
        Err(true)
    );
    assert_eq!(
        HexEncoder::decode("123456abc").map_err(|x| x.contains("无效的长度")),
        Err(true)
    );
    assert_eq!(
        HexEncoder::decode("12@456abcd").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    assert_eq!(
        HexEncoder::decode("123456a#cd").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    assert_eq!(
        HexEncoder::decode("123456abcg").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    let table = [
        ("0123456789", 0x0123456789),
        ("012_345_678_9", 0x0123456789),
        ("012_345_678 9", 0x0123456789),
        ("1FFF_FFFF_FFFF_FFFF_FFFF", 0x1FFFFFFFFFFFFFFF),
        ("0123456789abcdef", 0x0123456789ABCDEF),
        ("0123456789ABCDEF", 0x0123456789ABCDEF),
    ];
    println!("测试encode_number功能.");
    for (x, y) in table {
        println!("0x{y:x} <encode_number> {x}");
        let mut x = x.to_uppercase().replace(" ", "").replace("_", "");
        x.truncate(16);
        /* encode_number 会去掉多余的首位'0', 与 decode_number 对称. */
        if x.len() > 1 && x.starts_with('0') {
            x.remove(0);
        }
        assert_eq!(HexEncoder::encode_number(y), x);
    }
    println!("测试decode_number功能.");
    for (x, y) in table {
        println!("{x} <decode_number> 0x{y:x}");
        assert_eq!(HexEncoder::decode_number(x).expect("NEVER"), y);
    }
    println!("测试decode_number异常输入.");
    println!("-123456789");
    println!("012345678z");
    assert_eq!(
        HexEncoder::decode_number("-123456789").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    assert_eq!(
        HexEncoder::decode_number("012345678z").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    println!("测试decode_number的非ASCII输入, 应该报错而不是panic.");
    assert_eq!(
        HexEncoder::decode_number(&"你".repeat(10)).map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    println!("测试encode_number与decode_number对称.");
    for x in [
        0u64,
        1,
        0xF,
        0x10,
        0xFF,
        0x100,
        0x0F0F,
        0x0123_4567_89AB_CDEF,
    ] {
        let text: String = HexEncoder::encode_number(x);
        println!(
            "0x{x:x} <encode_number> {text} <decode_number> 0x{:x}",
            HexEncoder::decode_number(&text).expect("NEVER")
        );
        assert_eq!(HexEncoder::decode_number(&text).expect("NEVER"), x);
        /* 结果要么是"0", 要么首位不是'0'. */
        assert!(text == "0" || !text.starts_with('0'));
    }
    return;
}
