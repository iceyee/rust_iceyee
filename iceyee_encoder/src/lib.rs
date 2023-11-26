// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 编码器.

// Use.

use std::string::FromUtf8Error;

// Enum.

/// Error.
///
/// - @see [Base64Encoder]
#[derive(Debug, Clone)]
pub enum Base64Error {
    InvalidLength(usize),
    UnexpectedCharacter(char),
}

impl std::fmt::Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLength(length) => {
                f.write_str(format!("Base64Error, 无效的长度, length={}.", length).as_str())?;
            }
            Self::UnexpectedCharacter(character) => {
                f.write_str(
                    format!("Base64Error, 出现未预期字符, character={}.", character).as_str(),
                )?;
            }
        }
        return Ok(());
    }
}

impl std::error::Error for Base64Error {}

/// Error.
///
/// - @see [HexEncoder]
#[derive(Debug, Clone)]
pub enum HexError {
    InvalidLength(usize),
    UnexpectedCharacter(char),
}

impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLength(length) => {
                f.write_str(format!("HexError, 无效的长度, length={}.", length).as_str())?;
            }
            Self::UnexpectedCharacter(character) => {
                f.write_str(
                    format!("HexError, 出现未预期字符, character={}.", character).as_str(),
                )?;
            }
        }
        return Ok(());
    }
}

impl std::error::Error for HexError {}

/// Error.
///
/// - @see [UrlEncoder]
#[derive(Debug, Clone)]
pub enum UrlError {
    InvalidFormat,
    FromUtf8Error(FromUtf8Error),
}

impl std::fmt::Display for UrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidFormat => {
                f.write_str("UrlError, 错误的格式.")?;
            }
            Self::FromUtf8Error(_) => {
                f.write_str("UrlError, 解码后的内容不是UTF-8编码.")?;
            }
        }
        return Ok(());
    }
}

impl std::error::Error for UrlError {}

// Trait.

// Struct.

/// Base64编码.
#[derive(Debug, Clone)]
pub struct Base64Encoder;

impl Base64Encoder {
    /// 编码.
    pub fn encode(input: Vec<u8>) -> String {
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
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-".as_bytes();
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
    /// - @exception [Base64Error::InvalidLength] 无效的长度.
    /// - @exception [Base64Error::UnexpectedCharacter] 出现未预期的字符.
    pub fn decode(input: String) -> Result<Vec<u8>, Base64Error> {
        const TABLE: [u8; 0x100] = [
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 62, 255, 63, 255, 255, 52, 53, 54, 55, 56,
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
            return Err(Base64Error::InvalidLength(length));
        }
        let input_data: &[u8] = input.as_bytes();
        for x in 0..input_data.len() {
            let c: u8 = input_data[x];
            if TABLE[c as usize] != 255 || c == b'=' && input_data.len() <= x + 2 {
                // 正常.
            } else {
                return Err(Base64Error::UnexpectedCharacter(c as char));
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
    pub fn encode(input: Vec<u8>) -> String {
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
    /// - @exception [HexError::InvalidLength] 无效的长度.
    /// - @exception [HexError::UnexpectedCharacter] 出现未预期的字符.
    pub fn decode(input: String) -> Result<Vec<u8>, HexError> {
        let length: usize = input.len();
        if length % 2 != 0 {
            return Err(HexError::InvalidLength(length));
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
                _ => return Err(HexError::UnexpectedCharacter(input[x * 2] as char)),
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
                _ => return Err(HexError::UnexpectedCharacter(input[x * 2 + 1] as char)),
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
    /// - @exception [HexError::InvalidLength] 长度超过16.
    /// - @exception [HexError::UnexpectedCharacter] 出现未预期的字符.
    pub fn decode_number(input: String) -> Result<u64, HexError> {
        let v1: &[u8] = input.as_bytes();
        if 16 < v1.len() {
            // 长度过长.
            return Err(HexError::InvalidLength(v1.len()));
        }
        let mut output: u64 = 0;
        for x in 0..v1.len() {
            match v1[x] {
                b'0'..=b'9' => {
                    output <<= 4;
                    output |= (v1[x] - b'0') as u64;
                }
                b'A'..=b'F' => {
                    output <<= 4;
                    output |= (v1[x] - b'A' + 10) as u64;
                }
                b'a'..=b'f' => {
                    output <<= 4;
                    output |= (v1[x] - b'a' + 10) as u64;
                }
                any => {
                    return Err(HexError::UnexpectedCharacter(any as char));
                }
            }
        }
        return Ok(output);
    }
}

/// Url编码.

#[derive(Debug, Clone)]
pub struct UrlEncoder;

impl UrlEncoder {
    /// 编码.
    ///
    /// - @exception 没有异常.
    pub fn encode(input: String) -> String {
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
    /// - @exception [UrlError::InvalidFormat] 错误的格式.
    /// - @exception [UrlError::FromUtf8Error] 解码后的内容不是UTF-8编码.
    pub fn decode(cipher: String) -> Result<String, UrlError> {
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
                    _ => return Err(UrlError::InvalidFormat),
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
                    _ => return Err(UrlError::InvalidFormat),
                },
            }
        }
        match status {
            Status::Normal => {}
            _ => return Err(UrlError::InvalidFormat),
        }
        // # std::string::FromUtf8Error
        let plain: String = String::from_utf8(plain).map_err(|e| UrlError::FromUtf8Error(e))?;
        return Ok(plain);
    }
}

// Function.
