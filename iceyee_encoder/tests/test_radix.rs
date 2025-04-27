// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_encoder::RadixEncoder;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_radix_encoder() {
    println!("");

    println!("0xFF00 u64_to_bin 0000000000000000000000000000000000000000000000001111111100000000");
    assert_eq!(
        RadixEncoder::u64_to_bin(0xFF00),
        "0000000000000000000000000000000000000000000000001111111100000000"
    );
    println!("0xFF00 u64_to_bin_ 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000");
    assert_eq!(
        RadixEncoder::u64_to_bin_(0xFF00),
        "0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000"
    );
    println!("0xFF00 u32_to_bin 00000000000000001111111100000000");
    assert_eq!(
        RadixEncoder::u32_to_bin(0xFF00),
        "00000000000000001111111100000000"
    );
    println!("0xFF00 u32_to_bin_ 0b00000000_00000000_11111111_00000000");
    assert_eq!(
        RadixEncoder::u32_to_bin_(0xFF00),
        "0b00000000_00000000_11111111_00000000"
    );
    println!("0xFF00 u16_to_bin 1111111100000000");
    assert_eq!(RadixEncoder::u16_to_bin(0xFF00), "1111111100000000");
    println!("0xFF00 u16_to_bin_ 0b11111111_00000000");
    assert_eq!(RadixEncoder::u16_to_bin_(0xFF00), "0b11111111_00000000");
    println!("0xF7 u8_to_bin 11110111");
    assert_eq!(RadixEncoder::u8_to_bin(0xF7), "11110111");
    println!("0xF7 u8_to_bin_ 0b11110111");
    assert_eq!(RadixEncoder::u8_to_bin_(0xF7), "0b11110111");

    println!("0o7700 u64_to_oct 0000000000000000007700");
    assert_eq!(RadixEncoder::u64_to_oct(0o7700), "0000000000000000007700");
    println!("0o7700 u64_to_oct_ 0o0000000000000000007700");
    assert_eq!(
        RadixEncoder::u64_to_oct_(0o7700),
        "0o0000000000000000007700"
    );
    println!("0o7700 u32_to_oct 00000007700");
    assert_eq!(RadixEncoder::u32_to_oct(0o7700), "00000007700");
    println!("0o7700 u32_to_oct_ 0o00000007700");
    assert_eq!(RadixEncoder::u32_to_oct_(0o7700), "0o00000007700");
    println!("0o7700 u16_to_oct 007700");
    assert_eq!(RadixEncoder::u16_to_oct(0o7700), "007700");
    println!("0o7700 u16_to_oct_ 0o007700");
    assert_eq!(RadixEncoder::u16_to_oct_(0o7700), "0o007700");
    println!("0o70 u8_to_oct 070");
    assert_eq!(RadixEncoder::u8_to_oct(0o70), "070");
    println!("0o70 u8_to_oct_ 0o070");
    assert_eq!(RadixEncoder::u8_to_oct_(0o70), "0o070");

    println!("0xFF00 u64_to_hex 000000000000FF00");
    assert_eq!(RadixEncoder::u64_to_hex(0xFF00), "000000000000FF00");
    println!("0xFF00 u64_to_hex_ 0x0000_0000_0000_FF00");
    assert_eq!(RadixEncoder::u64_to_hex_(0xFF00), "0x0000_0000_0000_FF00");
    println!("0xFF00 u32_to_hex 0000FF00");
    assert_eq!(RadixEncoder::u32_to_hex(0xFF00), "0000FF00");
    println!("0xFF00 u32_to_hex_ 0x0000_FF00");
    assert_eq!(RadixEncoder::u32_to_hex_(0xFF00), "0x0000_FF00");
    println!("0xFF00 u16_to_hex FF00");
    assert_eq!(RadixEncoder::u16_to_hex(0xFF00), "FF00");
    println!("0xFF00 u16_to_hex_ 0xFF00");
    assert_eq!(RadixEncoder::u16_to_hex_(0xFF00), "0xFF00");
    println!("0xFA u8_to_hex FA");
    assert_eq!(RadixEncoder::u8_to_hex(0xFA), "FA");
    println!("0xFA u8_to_hex_ 0xFA");
    assert_eq!(RadixEncoder::u8_to_hex_(0xFA), "0xFA");
    return;
}
