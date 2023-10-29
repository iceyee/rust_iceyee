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

impl std::error::Error for Base64Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return None;
    }
}

impl std::fmt::Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use std::fmt::Write;
        match self {
            Self::InvalidLength(length) => {
                f.write_str("无效的长度, length=")?;
                f.write_str(length.to_string().as_str())?;
                f.write_str(".")?;
            }
            Self::UnexpectedCharacter(character) => {
                f.write_str("出现未预期字符, character=")?;
                f.write_char(*character)?;
                f.write_str(".")?;
            }
        }
        return Ok(());
    }
}

pub type HexError = Base64Error;

/// Error.
///
/// - @see [UrlEncoder]
#[derive(Debug, Clone)]
pub enum UrlError {
    InvalidFormat,
    FromUtf8Error(FromUtf8Error),
}

impl std::error::Error for UrlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match self {
            Self::InvalidFormat => None,
            Self::FromUtf8Error(e) => Some(e),
        };
    }
}

impl std::fmt::Display for UrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidFormat => {
                f.write_str("错误的格式.")?;
            }
            Self::FromUtf8Error(_) => {
                f.write_str("解码后的内容不是UTF-8编码.")?;
            }
        }
        return Ok(());
    }
}

// Trait.

/// 编码器.

pub trait Encoder {
    type Plain;
    type Cipher;
    type Error;

    /// 编码.
    fn encode(plain: Self::Plain) -> Result<Self::Cipher, Self::Error>;

    /// 解码.
    fn decode(cipher: Self::Cipher) -> Result<Self::Plain, Self::Error>;
}

// Struct.

/// Base64编码.
#[derive(Debug, Clone, Default)]
pub struct Base64Encoder;

impl Encoder for Base64Encoder {
    type Plain = Vec<u8>;
    type Cipher = String;
    type Error = Base64Error;

    /// 编码.
    ///
    /// - @exception 没有异常.
    fn encode(plain: Self::Plain) -> Result<Self::Cipher, Self::Error> {
        let plain_length: usize = plain.len();
        if plain_length == 0 {
            return Ok("".to_string());
        }
        let valid_length: usize = match plain_length % 3 {
            0 => plain_length / 3 * 4,
            1 => plain_length / 3 * 4 + 2,
            2 => plain_length / 3 * 4 + 3,
            _ => panic!(""),
        };
        const TABLE: &[u8] =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-".as_bytes();
        let mut cipher: Vec<u8> = Vec::with_capacity(plain_length * 2 + 1);
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
            v1 = (plain[y0] & 0b11111100) >> 2;
            v2 = 0;
            cipher.push(TABLE[(v1 | v2) as usize]);
            v1 = (plain[y0] & 0b00000011) << 4;
            v2 = (plain[y1] & 0b11110000) >> 4;
            cipher.push(TABLE[(v1 | v2) as usize]);
            v1 = (plain[y1] & 0b00001111) << 2;
            v2 = (plain[y2] & 0b11000000) >> 6;
            cipher.push(TABLE[(v1 | v2) as usize]);
            v1 = (plain[y2] & 0b00111111) >> 0;
            v2 = 0;
            cipher.push(TABLE[(v1 | v2) as usize]);
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
        if plain_length % 3 == 1 {
            v1 = (plain[y0] & 0b11111100) >> 2;
            v2 = 0;
            cipher.push(TABLE[(v1 | v2) as usize]);
            v1 = (plain[y0] & 0b00000011) << 4;
            v2 = 0;
            cipher.push(TABLE[(v1 | v2) as usize]);
            cipher.push(b'=');
            cipher.push(b'=');
        } else if plain_length % 3 == 2 {
            v1 = (plain[y0] & 0b11111100) >> 2;
            v2 = 0;
            cipher.push(TABLE[(v1 | v2) as usize]);
            v1 = (plain[y0] & 0b00000011) << 4;
            v2 = (plain[y1] & 0b11110000) >> 4;
            cipher.push(TABLE[(v1 | v2) as usize]);
            v1 = (plain[y1] & 0b00001111) << 2;
            v2 = 0;
            cipher.push(TABLE[(v1 | v2) as usize]);
            cipher.push(b'=');
        }
        let cipher: String = String::from_utf8(cipher).unwrap();
        return Ok(cipher);
    }

    /// 解码.
    ///
    /// - @exception [Base64Error::InvalidLength] 无效的长度.
    /// - @exception [Base64Error::UnexpectedCharacter] 出现未预期的字符.
    fn decode(cipher: Self::Cipher) -> Result<Self::Plain, Self::Error> {
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
        let length: usize = cipher.len();
        if length == 0 {
            return Ok(Vec::<u8>::new());
        }
        if length % 4 != 0 {
            return Err(Base64Error::InvalidLength(length));
        }
        let cipher_data: &[u8] = cipher.as_bytes();
        for x in 0..cipher_data.len() {
            let c: u8 = cipher_data[x];
            if TABLE[c as usize] != 255 || c == b'=' && cipher_data.len() <= x + 2 {
                // 正常.
            } else {
                return Err(Base64Error::UnexpectedCharacter(c as char));
            }
        }
        let new_length: usize = if cipher_data[cipher_data.len() - 2] == b'=' {
            (length - 2) / 4 * 3 + 1
        } else if cipher_data[cipher_data.len() - 1] == b'=' {
            (length - 1) / 4 * 3 + 2
        } else {
            length / 4 * 3
        };
        let mut plain: Vec<u8> = Vec::new();
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
            v1 = (TABLE[cipher_data[y0] as usize] & 0b00111111) << 2;
            v2 = (TABLE[cipher_data[y1] as usize] & 0b00110000) >> 4;
            plain.push(v1 | v2);
            v1 = (TABLE[cipher_data[y1] as usize] & 0b00001111) << 4;
            v2 = (TABLE[cipher_data[y2] as usize] & 0b00111100) >> 2;
            plain.push(v1 | v2);
            v1 = (TABLE[cipher_data[y2] as usize] & 0b00000011) << 6;
            v2 = (TABLE[cipher_data[y3] as usize] & 0b00111111) >> 0;
            plain.push(v1 | v2);
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
            v1 = (TABLE[cipher_data[y0] as usize] & 0b00111111) << 2;
            v2 = (TABLE[cipher_data[y1] as usize] & 0b00110000) >> 4;
            plain.push(v1 | v2);
        } else if new_length % 3 == 2 {
            v1 = (TABLE[cipher_data[y0] as usize] & 0b00111111) << 2;
            v2 = (TABLE[cipher_data[y1] as usize] & 0b00110000) >> 4;
            plain.push(v1 | v2);
            v1 = (TABLE[cipher_data[y1] as usize] & 0b00001111) << 4;
            v2 = (TABLE[cipher_data[y2] as usize] & 0b00111100) >> 2;
            plain.push(v1 | v2);
        }
        return Ok(plain);
    }
}

/// 十六进制编码.
#[derive(Debug, Clone, Default)]
pub struct HexEncoder;

impl Encoder for HexEncoder {
    type Plain = Vec<u8>;
    type Cipher = String;
    type Error = HexError;

    /// 编码.
    ///
    /// - @exception 没有异常.
    fn encode(plain: Self::Plain) -> Result<Self::Cipher, Self::Error> {
        static TABLE: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let mut cipher: String = String::new();
        for x in plain {
            let high: u8 = (x >> 4) & 0x0F;
            let low: u8 = (x >> 0) & 0x0F;
            cipher.push(TABLE[high as usize]);
            cipher.push(TABLE[low as usize]);
        }
        return Ok(cipher);
    }

    /// 解码.
    ///
    /// - @exception [HexError::InvalidLength] 无效的长度.
    /// - @exception [HexError::UnexpectedCharacter] 出现未预期的字符.
    fn decode(cipher: Self::Cipher) -> Result<Self::Plain, Self::Error> {
        let length: usize = cipher.len();
        if length % 2 != 0 {
            return Err(HexError::InvalidLength(length));
        }
        let mut plain: Vec<u8> = Vec::new();
        let cipher: &[u8] = cipher.as_bytes();
        let mut high: u8;
        let mut low: u8;
        for x in 0..(length / 2) {
            match cipher[x * 2] {
                b'0'..=b'9' => {
                    high = cipher[x * 2] - b'0';
                }
                b'a'..=b'f' => {
                    high = cipher[x * 2] - b'a' + 10;
                }
                b'A'..=b'F' => {
                    high = cipher[x * 2] - b'A' + 10;
                }
                _ => return Err(HexError::UnexpectedCharacter(cipher[x * 2] as char)),
            }
            match cipher[x * 2 + 1] {
                b'0'..=b'9' => {
                    low = cipher[x * 2 + 1] - b'0';
                }
                b'a'..=b'f' => {
                    low = cipher[x * 2 + 1] - b'a' + 10;
                }
                b'A'..=b'F' => {
                    low = cipher[x * 2 + 1] - b'A' + 10;
                }
                _ => return Err(HexError::UnexpectedCharacter(cipher[x * 2 + 1] as char)),
            }
            let b: u8 = (high << 4) | (low << 0);
            plain.push(b);
        }
        return Ok(plain);
    }
}

impl HexEncoder {
    /// 解码到64位整数.
    ///
    /// - @exception [HexError::InvalidLength] 长度超过16.
    /// - @exception [HexError::UnexpectedCharacter] 出现未预期的字符.
    pub fn decode_to_number(cipher: String) -> Result<u64, HexError> {
        let v1: &[u8] = cipher.as_bytes();
        if 16 < v1.len() {
            // 长度过长.
            return Err(HexError::InvalidLength(v1.len()));
        }
        let mut hex: u64 = 0;
        for x in 0..v1.len() {
            match v1[x] {
                b'0'..=b'9' => {
                    hex <<= 4;
                    hex |= (v1[x] - b'0') as u64;
                }
                b'A'..=b'F' => {
                    hex <<= 4;
                    hex |= (v1[x] - b'A' + 10) as u64;
                }
                b'a'..=b'f' => {
                    hex <<= 4;
                    hex |= (v1[x] - b'a' + 10) as u64;
                }
                any => {
                    return Err(HexError::UnexpectedCharacter(any as char));
                }
            }
        }
        return Ok(hex);
    }
}

/// Url编码.

#[derive(Debug, Clone, Default)]
pub struct UrlEncoder;

impl Encoder for UrlEncoder {
    type Plain = String;
    type Cipher = String;
    type Error = UrlError;

    /// 编码.
    ///
    /// - @exception 没有异常.
    fn encode(plain: Self::Plain) -> Result<Self::Cipher, Self::Error> {
        static TABLE: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let mut cipher: String = String::new();
        let plain: &[u8] = plain.as_bytes();
        for x in plain {
            if x.is_ascii_alphanumeric() || *x == b'_' || *x == b'-' {
                cipher.push(*x as char);
            } else {
                let high: u8 = (*x >> 4) & 0x0F;
                let low: u8 = (*x >> 0) & 0x0F;
                cipher.push('%');
                cipher.push(TABLE[high as usize]);
                cipher.push(TABLE[low as usize]);
            }
        }
        return Ok(cipher);
    }

    /// 解码.
    ///
    /// - @exception [UrlError::InvalidFormat] 错误的格式.
    /// - @exception [UrlError::FromUtf8Error] 解码后的内容不是UTF-8编码.
    fn decode(cipher: Self::Cipher) -> Result<Self::Plain, Self::Error> {
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
                Status::Normal => {
                    if *x == b'%' {
                        status = Status::High;
                    }
                    match *x {
                        b'%' => status = Status::High,
                        b'+' => plain.push(b' '),
                        _ => plain.push(*x),
                    }
                }
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
