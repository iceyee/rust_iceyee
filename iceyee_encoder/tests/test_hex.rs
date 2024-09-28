// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
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
        ("0123456789", 0x0123456789u64),
        ("0123456789abcdef", 0x0123456789ABCDEFu64),
        ("0123456789ABCDEF", 0x0123456789ABCDEFu64),
    ];
    println!("测试encode_number功能.");
    for (x, y) in table {
        println!("0x{y:x} <encode_number> {x}");
        assert_eq!(HexEncoder::encode_number(y), x.to_uppercase());
    }
    println!("测试decode_number功能.");
    for (x, y) in table {
        println!("{x} <decode_number> 0x{y:x}");
        assert_eq!(HexEncoder::decode_number(x).expect("NEVER"), y);
    }
    println!("测试decode_number异常输入.");
    println!("0123456789ABCDEF0");
    println!("0123456789ABCDEF01");
    println!("-123456789");
    println!("012345678z");
    assert_eq!(
        HexEncoder::decode_number("0123456789ABCDEF0").map_err(|x| x.contains("长度超过16")),
        Err(true)
    );
    assert_eq!(
        HexEncoder::decode_number("0123456789ABCDEF01").map_err(|x| x.contains("长度超过16")),
        Err(true)
    );
    assert_eq!(
        HexEncoder::decode_number("-123456789").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    assert_eq!(
        HexEncoder::decode_number("012345678z").map_err(|x| x.contains("出现未预期的字符")),
        Err(true)
    );
    return;
}
