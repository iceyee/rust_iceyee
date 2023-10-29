// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

//! 转换类型.

// Enum.

/// Error.
#[derive(Debug, Clone)]
pub enum ConversionError {
    TooLong(usize),
    UnexpectedCharacter(char),
}

impl std::error::Error for ConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return None;
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::TooLong(length) => {
                f.write_str("长度过长, length=")?;
                f.write_str(length.to_string().as_str())?;
                f.write_str(".")?;
            }
            Self::UnexpectedCharacter(character) => {
                f.write_str("出现未预期字符, character=")?;
                f.write_str(character.to_string().as_str())?;
                f.write_str(".")?;
            }
        }
        return Ok(());
    }
}

// Trait.

// Struct.

/// 转换类型.

#[derive(Debug, Clone, Default)]
pub struct Conversion;

impl Conversion {
    /// 字符串转数字, 十六进制.
    ///
    /// - @exception [ConversionError::TooLong] 长度超过16.
    /// - @exception [ConversionError::UnexpectedCharacter] 出现未预期的字符.
    #[deprecated = "被iceyee_encoder v1.3.0 的 HexEncoder::decode_to_number() 代替."]
    pub fn string_to_hex(s: &str) -> Result<u64, ConversionError> {
        let v1: &[u8] = s.as_bytes();
        if 16 < v1.len() {
            // 长度过长.
            return Err(ConversionError::TooLong(v1.len()));
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
                    return Err(ConversionError::UnexpectedCharacter(any as char));
                }
            }
        }
        return Ok(hex);
    }
}

// Function.
