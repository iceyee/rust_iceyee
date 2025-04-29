// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 编码器.

// Use.

// Enum.

// Trait.

// Struct.

/// Base64编码.
#[derive(Debug, Clone)]
pub struct Base64Encoder;

impl Base64Encoder {
    /// 编码.
    pub fn encode(input: &[u8]) -> String {
        let input_length: usize = input.len();
        if input_length == 0 {
            return "".to_string();
        }
        let valid_length: usize = match input_length % 3 {
            0 => input_length / 3 * 4,
            1 => input_length / 3 * 4 + 2,
            2 => input_length / 3 * 4 + 3,
            _ => panic!(""),
        };
        const TABLE: &[u8] =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
        let mut output: Vec<u8> = Vec::with_capacity(input_length * 2 + 1);
        let m: usize = valid_length / 4;
        for x in 0..m {
            // let x0: usize = x * 4 + 0;
            // let x1: usize = x * 4 + 1;
            // let x2: usize = x * 4 + 2;
            // let x3: usize = x * 4 + 3;
            let y0: usize = x * 3 + 0;
            let y1: usize = x * 3 + 1;
            let y2: usize = x * 3 + 2;
            let mut v1: u8;
            let mut v2: u8;
            v1 = (input[y0] & 0b11111100) >> 2;
            v2 = 0;
            output.push(TABLE[(v1 | v2) as usize]);
            v1 = (input[y0] & 0b00000011) << 4;
            v2 = (input[y1] & 0b11110000) >> 4;
            output.push(TABLE[(v1 | v2) as usize]);
            v1 = (input[y1] & 0b00001111) << 2;
            v2 = (input[y2] & 0b11000000) >> 6;
            output.push(TABLE[(v1 | v2) as usize]);
            v1 = (input[y2] & 0b00111111) >> 0;
            v2 = 0;
            output.push(TABLE[(v1 | v2) as usize]);
        }
        // let x0: usize = m * 4 + 0;
        // let x1: usize = m * 4 + 1;
        // let x2: usize = m * 4 + 2;
        // let x3: usize = m * 4 + 3;
        let y0: usize = m * 3 + 0;
        let y1: usize = m * 3 + 1;
        // let y2: usize = m * 3 + 2;
        let mut v1: u8;
        let mut v2: u8;
        if input_length % 3 == 1 {
            v1 = (input[y0] & 0b11111100) >> 2;
            v2 = 0;
            output.push(TABLE[(v1 | v2) as usize]);
            v1 = (input[y0] & 0b00000011) << 4;
            v2 = 0;
            output.push(TABLE[(v1 | v2) as usize]);
            output.push(b'=');
            output.push(b'=');
        } else if input_length % 3 == 2 {
            v1 = (input[y0] & 0b11111100) >> 2;
            v2 = 0;
            output.push(TABLE[(v1 | v2) as usize]);
            v1 = (input[y0] & 0b00000011) << 4;
            v2 = (input[y1] & 0b11110000) >> 4;
            output.push(TABLE[(v1 | v2) as usize]);
            v1 = (input[y1] & 0b00001111) << 2;
            v2 = 0;
            output.push(TABLE[(v1 | v2) as usize]);
            output.push(b'=');
        }
        let output: String = String::from_utf8(output).unwrap();
        return output;
    }

    /// 解码.
    ///
    /// - @exception 无效的长度.
    /// - @exception 出现未预期的字符.
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let input: String = input.to_string();
        const TABLE: [u8; 0x100] = [
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 62, 255, 255, 255, 63, 52, 53, 54, 55, 56,
            57, 58, 59, 60, 61, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 255, 255, 255, 255,
            255, 255, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
            45, 46, 47, 48, 49, 50, 51, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255,
        ];
        let length: usize = input.len();
        if length == 0 {
            return Ok(Vec::<u8>::new());
        }
        if length % 4 != 0 {
            return Err(iceyee_error::a!("无效的长度"));
        }
        let input_data: &[u8] = input.as_bytes();
        for x in 0..input_data.len() {
            let c: u8 = input_data[x];
            if TABLE[c as usize] != 255 || c == b'=' && input_data.len() <= x + 2 {
                // 正常.
            } else {
                return Err(iceyee_error::a!("出现未预期的字符"));
            }
        }
        let new_length: usize = if input_data[input_data.len() - 2] == b'=' {
            (length - 2) / 4 * 3 + 1
        } else if input_data[input_data.len() - 1] == b'=' {
            (length - 1) / 4 * 3 + 2
        } else {
            length / 4 * 3
        };
        let mut output: Vec<u8> = Vec::new();
        let m: usize = new_length / 3;
        for x in 0..m {
            // let x0: usize = x * 3 + 0;
            // let x1: usize = x * 3 + 1;
            // let x2: usize = x * 3 + 2;
            let y0: usize = x * 4 + 0;
            let y1: usize = x * 4 + 1;
            let y2: usize = x * 4 + 2;
            let y3: usize = x * 4 + 3;
            let mut v1: u8;
            let mut v2: u8;
            v1 = (TABLE[input_data[y0] as usize] & 0b00111111) << 2;
            v2 = (TABLE[input_data[y1] as usize] & 0b00110000) >> 4;
            output.push(v1 | v2);
            v1 = (TABLE[input_data[y1] as usize] & 0b00001111) << 4;
            v2 = (TABLE[input_data[y2] as usize] & 0b00111100) >> 2;
            output.push(v1 | v2);
            v1 = (TABLE[input_data[y2] as usize] & 0b00000011) << 6;
            v2 = (TABLE[input_data[y3] as usize] & 0b00111111) >> 0;
            output.push(v1 | v2);
        }
        // let x0: usize = m * 3 + 0;
        // let x1: usize = m * 3 + 1;
        // let x2: usize = m * 3 + 2;
        let y0: usize = m * 4 + 0;
        let y1: usize = m * 4 + 1;
        let y2: usize = m * 4 + 2;
        // let y3: usize = m * 4 + 3;
        let mut v1: u8;
        let mut v2: u8;
        if new_length % 3 == 1 {
            v1 = (TABLE[input_data[y0] as usize] & 0b00111111) << 2;
            v2 = (TABLE[input_data[y1] as usize] & 0b00110000) >> 4;
            output.push(v1 | v2);
        } else if new_length % 3 == 2 {
            v1 = (TABLE[input_data[y0] as usize] & 0b00111111) << 2;
            v2 = (TABLE[input_data[y1] as usize] & 0b00110000) >> 4;
            output.push(v1 | v2);
            v1 = (TABLE[input_data[y1] as usize] & 0b00001111) << 4;
            v2 = (TABLE[input_data[y2] as usize] & 0b00111100) >> 2;
            output.push(v1 | v2);
        }
        return Ok(output);
    }
}

/// 十六进制编码.
#[derive(Debug, Clone)]
pub struct HexEncoder;

impl HexEncoder {
    /// 编码.
    pub fn encode(input: &[u8]) -> String {
        static TABLE: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let mut output: String = String::new();
        for x in input {
            let high: u8 = (x >> 4) & 0x0F;
            let low: u8 = (x >> 0) & 0x0F;
            output.push(TABLE[high as usize]);
            output.push(TABLE[low as usize]);
        }
        return output;
    }

    /// 解码.
    ///
    /// - @exception 无效的长度.
    /// - @exception 出现未预期的字符.
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let input: String = input.to_string();
        let length: usize = input.len();
        if length % 2 != 0 {
            return Err(iceyee_error::a!("无效的长度"));
        }
        let mut output: Vec<u8> = Vec::new();
        let input: &[u8] = input.as_bytes();
        let mut high: u8;
        let mut low: u8;
        for x in 0..(length / 2) {
            match input[x * 2] {
                b'0'..=b'9' => {
                    high = input[x * 2] - b'0';
                }
                b'a'..=b'f' => {
                    high = input[x * 2] - b'a' + 10;
                }
                b'A'..=b'F' => {
                    high = input[x * 2] - b'A' + 10;
                }
                _ => return Err(iceyee_error::a!("出现未预期的字符")),
            }
            match input[x * 2 + 1] {
                b'0'..=b'9' => {
                    low = input[x * 2 + 1] - b'0';
                }
                b'a'..=b'f' => {
                    low = input[x * 2 + 1] - b'a' + 10;
                }
                b'A'..=b'F' => {
                    low = input[x * 2 + 1] - b'A' + 10;
                }
                _ => return Err(iceyee_error::a!("出现未预期的字符")),
            }
            let b: u8 = (high << 4) | (low << 0);
            output.push(b);
        }
        return Ok(output);
    }

    /// 编码64位整数.
    pub fn encode_number(mut input: u64) -> String {
        static TABLE: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        if input == 0 {
            return "0".to_string();
        }
        let mut output: Vec<u8> = Vec::new();
        while input != 0 {
            let digit: usize = (input & 0xF) as usize;
            output.push(TABLE[digit] as u8);
            input = input >> 4;
            let digit: usize = (input & 0xF) as usize;
            output.push(TABLE[digit] as u8);
            input = input >> 4;
        }
        output.reverse();
        return String::from_utf8(output).unwrap();
    }

    /// 解码64位整数.
    ///
    /// - @exception 出现未预期的字符.
    pub fn decode_number(input: &str) -> Result<u64, String> {
        let mut input: String = input.to_string().replace(" ", "").replace("_", "");
        input.truncate(16);
        let mut output: u64 = 0;
        for x in input.as_bytes().iter() {
            match x {
                b'0'..=b'9' => {
                    output <<= 4;
                    output |= (x - b'0') as u64;
                }
                b'a'..=b'f' => {
                    output <<= 4;
                    output |= (x - b'a' + 10) as u64;
                }
                b'A'..=b'F' => {
                    output <<= 4;
                    output |= (x - b'A' + 10) as u64;
                }
                _ => {
                    return Err(iceyee_error::a!("出现未预期的字符"));
                }
            }
        }
        return Ok(output);
    }
}

/// 进制编码.
#[derive(Clone, Debug)]
pub struct RadixEncoder;

impl RadixEncoder {
    /// 字符串转64位整数.
    ///
    /// - @exception 出现未预期的字符.
    pub fn string_to_u64(input: &str) -> Result<u64, String> {
        enum Radix {
            Binary,
            Octal,
            Hexadecimal,
        }
        let mut input: String = input.to_string().replace(" ", "").replace("_", "");
        let radix: Radix = if input.starts_with("0b") {
            input = input[2..].to_string();
            input.truncate(64);
            Radix::Binary
        } else if input.starts_with("0o") {
            input = input[2..].to_string();
            input.truncate(22);
            Radix::Octal
        } else if input.starts_with("0x") {
            input = input[2..].to_string();
            input.truncate(16);
            Radix::Hexadecimal
        } else {
            Radix::Hexadecimal
        };
        let mut output: u64 = 0;
        match radix {
            Radix::Binary => {
                for x in input.as_bytes().iter() {
                    match x {
                        b'0'..=b'1' => {
                            output <<= 1;
                            output |= (x - b'0') as u64;
                        }
                        _ => {
                            return Err(iceyee_error::a!("出现未预期的字符"));
                        }
                    }
                }
            }
            Radix::Octal => {
                for x in input.as_bytes().iter() {
                    match x {
                        b'0'..=b'7' => {
                            output <<= 3;
                            output |= (x - b'0') as u64;
                        }
                        _ => {
                            return Err(iceyee_error::a!("出现未预期的字符"));
                        }
                    }
                }
            }
            Radix::Hexadecimal => {
                for x in input.as_bytes().iter() {
                    match x {
                        b'0'..=b'9' => {
                            output <<= 4;
                            output |= (x - b'0') as u64;
                        }
                        b'a'..=b'f' => {
                            output <<= 4;
                            output |= (x - b'a' + 10) as u64;
                        }
                        b'A'..=b'F' => {
                            output <<= 4;
                            output |= (x - b'A' + 10) as u64;
                        }
                        _ => {
                            return Err(iceyee_error::a!("出现未预期的字符"));
                        }
                    }
                }
            }
        }
        return Ok(output);
    }

    pub fn u64_to_bin(input: u64) -> String {
        let mut input: u64 = input;
        let mut output: String = String::new();
        for _x in 0..64 {
            let y: u8 = input as u8 & 0b1;
            input >>= 1;
            output.push((y + b'0') as char);
        }
        return output.chars().rev().collect();
    }

    pub fn u64_to_bin_(input: u64) -> String {
        let mut input: u64 = input;
        let mut output: String = String::new();
        for x in 0..64 {
            if x != 0 && x % 8 == 0 {
                output.push('_');
            }
            let y: u8 = input as u8 & 0b1;
            input >>= 1;
            output.push((y + b'0') as char);
        }
        output.push('b');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u32_to_bin(input: u32) -> String {
        let mut input: u32 = input;
        let mut output: String = String::new();
        for _x in 0..32 {
            let y: u8 = input as u8 & 0b1;
            input >>= 1;
            output.push((y + b'0') as char);
        }
        return output.chars().rev().collect();
    }

    pub fn u32_to_bin_(input: u32) -> String {
        let mut input: u32 = input;
        let mut output: String = String::new();
        for x in 0..32 {
            if x != 0 && x % 8 == 0 {
                output.push('_');
            }
            let y: u8 = input as u8 & 0b1;
            input >>= 1;
            output.push((y + b'0') as char);
        }
        output.push('b');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u16_to_bin(input: u16) -> String {
        let mut input: u16 = input;
        let mut output: String = String::new();
        for _x in 0..16 {
            let y: u8 = input as u8 & 0b1;
            input >>= 1;
            output.push((y + b'0') as char);
        }
        return output.chars().rev().collect();
    }

    pub fn u16_to_bin_(input: u16) -> String {
        let mut input: u16 = input;
        let mut output: String = String::new();
        for x in 0..16 {
            if x != 0 && x % 8 == 0 {
                output.push('_');
            }
            let y: u8 = input as u8 & 0b1;
            input >>= 1;
            output.push((y + b'0') as char);
        }
        output.push('b');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u8_to_bin(input: u8) -> String {
        let mut input: u8 = input;
        let mut output: String = String::new();
        for _x in 0..8 {
            let y: u8 = input as u8 & 0b1;
            input >>= 1;
            output.push((y + b'0') as char);
        }
        return output.chars().rev().collect();
    }

    pub fn u8_to_bin_(input: u8) -> String {
        let mut input: u8 = input;
        let mut output: String = String::new();
        for x in 0..8 {
            if x != 0 && x % 8 == 0 {
                output.push('_');
            }
            let y: u8 = input as u8 & 0b1;
            input >>= 1;
            output.push((y + b'0') as char);
        }
        output.push('b');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u64_to_oct(input: u64) -> String {
        let mut input: u64 = input;
        let mut output: String = String::new();
        for _x in 0..22 {
            let y: u8 = input as u8 & 0b111;
            input >>= 3;
            output.push((y + b'0') as char);
        }
        return output.chars().rev().collect();
    }

    pub fn u64_to_oct_(input: u64) -> String {
        let mut input: u64 = input;
        let mut output: String = String::new();
        for x in 0..22 {
            if x != 0 && false {
                output.push('_');
            }
            let y: u8 = input as u8 & 0b111;
            input >>= 3;
            output.push((y + b'0') as char);
        }
        output.push('o');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u32_to_oct(input: u32) -> String {
        let mut input: u32 = input;
        let mut output: String = String::new();
        for _x in 0..11 {
            let y: u8 = input as u8 & 0b111;
            input >>= 3;
            output.push((y + b'0') as char);
        }
        return output.chars().rev().collect();
    }

    pub fn u32_to_oct_(input: u32) -> String {
        let mut input: u32 = input;
        let mut output: String = String::new();
        for x in 0..11 {
            if x != 0 && false {
                output.push('_');
            }
            let y: u8 = input as u8 & 0b111;
            input >>= 3;
            output.push((y + b'0') as char);
        }
        output.push('o');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u16_to_oct(input: u16) -> String {
        let mut input: u16 = input;
        let mut output: String = String::new();
        for _x in 0..6 {
            let y: u8 = input as u8 & 0b111;
            input >>= 3;
            output.push((y + b'0') as char);
        }
        return output.chars().rev().collect();
    }

    pub fn u16_to_oct_(input: u16) -> String {
        let mut input: u16 = input;
        let mut output: String = String::new();
        for x in 0..6 {
            if x != 0 && false {
                output.push('_');
            }
            let y: u8 = input as u8 & 0b111;
            input >>= 3;
            output.push((y + b'0') as char);
        }
        output.push('o');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u8_to_oct(input: u8) -> String {
        let mut input: u8 = input;
        let mut output: String = String::new();
        for _x in 0..3 {
            let y: u8 = input as u8 & 0b111;
            input >>= 3;
            output.push((y + b'0') as char);
        }
        return output.chars().rev().collect();
    }

    pub fn u8_to_oct_(input: u8) -> String {
        let mut input: u8 = input;
        let mut output: String = String::new();
        for x in 0..3 {
            if x != 0 && false {
                output.push('_');
            }
            let y: u8 = input as u8 & 0b111;
            input >>= 3;
            output.push((y + b'0') as char);
        }
        output.push('o');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u64_to_hex(input: u64) -> String {
        let mut input: u64 = input;
        let mut output: String = String::new();
        for _x in 0..16 {
            let y: u8 = input as u8 & 0xF;
            input >>= 4;
            if y < 10 {
                output.push((y + b'0') as char);
            } else {
                output.push((y - 10 + b'A') as char);
            }
        }
        return output.chars().rev().collect();
    }

    pub fn u64_to_hex_(input: u64) -> String {
        let mut input: u64 = input;
        let mut output: String = String::new();
        for x in 0..16 {
            if x != 0 && x % 4 == 0 {
                output.push('_');
            }
            let y: u8 = input as u8 & 0xF;
            input >>= 4;
            if y < 10 {
                output.push((y + b'0') as char);
            } else {
                output.push((y - 10 + b'A') as char);
            }
        }
        output.push('x');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u32_to_hex(input: u32) -> String {
        let mut input: u32 = input;
        let mut output: String = String::new();
        for _x in 0..8 {
            let y: u8 = input as u8 & 0xF;
            input >>= 4;
            if y < 10 {
                output.push((y + b'0') as char);
            } else {
                output.push((y - 10 + b'A') as char);
            }
        }
        return output.chars().rev().collect();
    }

    pub fn u32_to_hex_(input: u32) -> String {
        let mut input: u32 = input;
        let mut output: String = String::new();
        for x in 0..8 {
            if x != 0 && x % 4 == 0 {
                output.push('_');
            }
            let y: u8 = input as u8 & 0xF;
            input >>= 4;
            if y < 10 {
                output.push((y + b'0') as char);
            } else {
                output.push((y - 10 + b'A') as char);
            }
        }
        output.push('x');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u16_to_hex(input: u16) -> String {
        let mut input: u16 = input;
        let mut output: String = String::new();
        for _x in 0..4 {
            let y: u8 = input as u8 & 0xF;
            input >>= 4;
            if y < 10 {
                output.push((y + b'0') as char);
            } else {
                output.push((y - 10 + b'A') as char);
            }
        }
        return output.chars().rev().collect();
    }

    pub fn u16_to_hex_(input: u16) -> String {
        let mut input: u16 = input;
        let mut output: String = String::new();
        for x in 0..4 {
            if x != 0 && x % 4 == 0 {
                output.push('_');
            }
            let y: u8 = input as u8 & 0xF;
            input >>= 4;
            if y < 10 {
                output.push((y + b'0') as char);
            } else {
                output.push((y - 10 + b'A') as char);
            }
        }
        output.push('x');
        output.push('0');
        return output.chars().rev().collect();
    }

    pub fn u8_to_hex(input: u8) -> String {
        let mut input: u8 = input;
        let mut output: String = String::new();
        for _x in 0..2 {
            let y: u8 = input as u8 & 0xF;
            input >>= 4;
            if y < 10 {
                output.push((y + b'0') as char);
            } else {
                output.push((y - 10 + b'A') as char);
            }
        }
        return output.chars().rev().collect();
    }

    pub fn u8_to_hex_(input: u8) -> String {
        let mut input: u8 = input;
        let mut output: String = String::new();
        for x in 0..2 {
            if x != 0 && x % 4 == 0 {
                output.push('_');
            }
            let y: u8 = input as u8 & 0xF;
            input >>= 4;
            if y < 10 {
                output.push((y + b'0') as char);
            } else {
                output.push((y - 10 + b'A') as char);
            }
        }
        output.push('x');
        output.push('0');
        return output.chars().rev().collect();
    }
}

/// Url编码.

#[derive(Debug, Clone)]
pub struct UrlEncoder;

impl UrlEncoder {
    /// 编码.
    pub fn encode(input: &str) -> String {
        let input: String = input.to_string();
        static TABLE: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let mut output: String = String::new();
        let input: &[u8] = input.as_bytes();
        for x in input {
            if *x == b' ' {
                output.push('+');
            } else if x.is_ascii_alphanumeric() || "$-_.".contains(*x as char) {
                output.push(*x as char);
            } else {
                let high: u8 = (*x >> 4) & 0x0F;
                let low: u8 = (*x >> 0) & 0x0F;
                output.push('%');
                output.push(TABLE[high as usize]);
                output.push(TABLE[low as usize]);
            }
        }
        return output;
    }

    /// 解码.
    ///
    /// - @exception 错误的格式.
    /// - @exception 内容不是UTF-8编码.
    pub fn decode(cipher: &str) -> Result<String, String> {
        let cipher: String = cipher.to_string();
        enum Status {
            Normal,
            High,
            Low,
        }
        let mut plain: Vec<u8> = Vec::new();
        let cipher: &[u8] = cipher.as_bytes();
        let mut status: Status = Status::Normal;
        let mut high: u8 = 0;
        let mut low: u8;
        for x in cipher {
            match status {
                Status::Normal => match *x {
                    b'%' => status = Status::High,
                    b'+' => plain.push(b' '),
                    _ => plain.push(*x),
                },
                Status::High => match *x {
                    b'0'..=b'9' => {
                        status = Status::Low;
                        high = *x - b'0';
                    }
                    b'a'..=b'f' => {
                        status = Status::Low;
                        high = *x - b'a' + 10;
                    }
                    b'A'..=b'F' => {
                        status = Status::Low;
                        high = *x - b'A' + 10;
                    }
                    _ => return Err(iceyee_error::a!("错误的格式")),
                },
                Status::Low => match *x {
                    b'0'..=b'9' => {
                        status = Status::Normal;
                        low = *x - b'0';
                        plain.push((high << 4) | (low << 0));
                    }
                    b'a'..=b'f' => {
                        status = Status::Normal;
                        low = *x - b'a' + 10;
                        plain.push((high << 4) | (low << 0));
                    }
                    b'A'..=b'F' => {
                        status = Status::Normal;
                        low = *x - b'A' + 10;
                        plain.push((high << 4) | (low << 0));
                    }
                    _ => return Err(iceyee_error::a!("错误的格式")),
                },
            }
        }
        match status {
            Status::Normal => {}
            _ => return Err(iceyee_error::a!("错误的格式")),
        }
        let plain: String =
            String::from_utf8(plain).map_err(|_| iceyee_error::a!("内容不是UTF-8编码"))?;
        return Ok(plain);
    }
}

// Function.
