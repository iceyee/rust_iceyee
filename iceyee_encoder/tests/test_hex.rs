// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_encoder::HexEncoder;
use iceyee_encoder::HexError;

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
    assert_eq!(HexEncoder::decode(&b1).expect("test_hex.rs 769"), a);
    println!("{b2:?} <decode> {a:?}");
    assert_eq!(HexEncoder::decode(&b2).expect("test_hex.rs 297"), a);
    println!("测试异常输入.");
    assert_eq!(
        HexEncoder::decode("123456a"),
        Err(HexError::InvalidLength(7))
    );
    assert_eq!(
        HexEncoder::decode("123456abc"),
        Err(HexError::InvalidLength(9))
    );
    assert_eq!(
        HexEncoder::decode("12@456abcd"),
        Err(HexError::UnexpectedCharacter('@'))
    );
    assert_eq!(
        HexEncoder::decode("123456a#cd"),
        Err(HexError::UnexpectedCharacter('#'))
    );
    assert_eq!(
        HexEncoder::decode("123456abcg"),
        Err(HexError::UnexpectedCharacter('g'))
    );
    let table = [
        ("0123456789", 0x0123456789),
        ("0123456789abcdef", 0x0123456789ABCDEF),
        ("0123456789ABCDEF", 0x0123456789ABCDEF),
    ];
    println!("测试encode_number功能.");
    for (x, y) in table {
        println!("0x{y:x} <encode_number> {x}");
        assert_eq!(HexEncoder::encode_number(y), x.to_uppercase());
    }
    println!("测试decode_number功能.");
    for (x, y) in table {
        println!("{x} <decode_number> 0x{y:x}");
        assert_eq!(HexEncoder::decode_number(x).expect("test_hex.rs 401"), y);
    }
    println!("测试decode_number异常输入.");
    assert_eq!(
        HexEncoder::decode_number("0123456789ABCDEF0"),
        Err(HexError::InvalidLength(17))
    );
    assert_eq!(
        HexEncoder::decode_number("0123456789ABCDEF01"),
        Err(HexError::InvalidLength(18))
    );
    assert_eq!(
        HexEncoder::decode_number("-123456789"),
        Err(HexError::UnexpectedCharacter('-'))
    );
    assert_eq!(
        HexEncoder::decode_number("012345678z"),
        Err(HexError::UnexpectedCharacter('z'))
    );
    return;
}
