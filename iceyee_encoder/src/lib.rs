// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
/* clippy: 本crate风格是每个函数显式写return; 取模判断比is_multiple_of可读. */
#![allow(clippy::manual_is_multiple_of, clippy::needless_return)]

//! 编码器.

/* Use. */

/* Base64字符表. */
const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/* Base64反向查表, 255表示非法字符. */
const BASE64_DECODE_TABLE: [u8; 0x100] = {
    let mut table: [u8; 0x100] = [255; 0x100];
    let mut x: usize = 0;
    while x < 64 {
        table[BASE64_TABLE[x] as usize] = x as u8;
        x += 1;
    }
    table
};

/* 十六进制字符表. */
const HEX_TABLE: &[u8; 16] = b"0123456789ABCDEF";

/* Enum. */

/* Trait. */

/* Struct. */

/// Base64编码.
#[derive(Debug, Clone)]
pub struct Base64Encoder;

impl Base64Encoder {
    /// 编码.
    pub fn encode(input: &[u8]) -> String {
        let length: usize = input.len();
        let full: usize = length / 3;
        let rest: usize = length % 3;
        let mut output: String = String::with_capacity(full * 4 + if rest == 0 { 0 } else { 4 });
        for x in 0..full {
            let y0: usize = x * 3;
            let y1: usize = y0 + 1;
            let y2: usize = y0 + 2;
            output.push(BASE64_TABLE[(input[y0] >> 2) as usize] as char);
            output.push(
                BASE64_TABLE[((input[y0] & 0b00000011) << 4 | input[y1] >> 4) as usize] as char,
            );
            output.push(
                BASE64_TABLE[((input[y1] & 0b00001111) << 2 | input[y2] >> 6) as usize] as char,
            );
            output.push(BASE64_TABLE[(input[y2] & 0b00111111) as usize] as char);
        }
        if rest == 1 {
            let y0: usize = full * 3;
            output.push(BASE64_TABLE[(input[y0] >> 2) as usize] as char);
            output.push(BASE64_TABLE[((input[y0] & 0b00000011) << 4) as usize] as char);
            output.push('=');
            output.push('=');
        } else if rest == 2 {
            let y0: usize = full * 3;
            let y1: usize = y0 + 1;
            output.push(BASE64_TABLE[(input[y0] >> 2) as usize] as char);
            output.push(
                BASE64_TABLE[((input[y0] & 0b00000011) << 4 | input[y1] >> 4) as usize] as char,
            );
            output.push(BASE64_TABLE[((input[y1] & 0b00001111) << 2) as usize] as char);
            output.push('=');
        }
        return output;
    }

    /// 编码为Url安全形式, 相当于RFC4648的base64url.
    ///
    /// 用`-`替代`+`, 用`_`替代`/`, 并且去掉末尾的`=`.
    pub fn encode_url(input: &[u8]) -> String {
        let standard: String = Self::encode(input);
        let mut output: String = String::with_capacity(standard.len());
        for x in standard.as_bytes() {
            match x {
                b'+' => output.push('-'),
                b'/' => output.push('_'),
                b'=' => {}
                _ => output.push(*x as char),
            }
        }
        return output;
    }

    /// 解码.
    ///
    /// - @exception 无效的长度.
    /// - @exception 出现未预期的字符.
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let data: &[u8] = input.as_bytes();
        let length: usize = data.len();
        if length == 0 {
            return Ok(Vec::<u8>::new());
        }
        if length % 4 != 0 {
            return Err(iceyee_error::c!("无效的长度"));
        }
        /* '='只允许出现在末尾, 最多两个. */
        let padding: usize = if data[length - 1] == b'=' {
            if data[length - 2] == b'=' {
                2
            } else {
                1
            }
        } else {
            0
        };
        /* 校验字符, 并保证'='不提前出现. */
        for x in 0..length {
            if data[x] == b'=' {
                if x < length - padding {
                    return Err(iceyee_error::c!("出现未预期的字符"));
                }
            } else if BASE64_DECODE_TABLE[data[x] as usize] == 255 {
                return Err(iceyee_error::c!("出现未预期的字符"));
            }
        }
        /* 校验补位是否为0, 非规范的补位一律拒绝. */
        if padding == 1 && BASE64_DECODE_TABLE[data[length - 2] as usize] & 0b00000011 != 0 {
            return Err(iceyee_error::c!("出现未预期的字符"));
        }
        if padding == 2 && BASE64_DECODE_TABLE[data[length - 3] as usize] & 0b00001111 != 0 {
            return Err(iceyee_error::c!("出现未预期的字符"));
        }
        let new_length: usize = match padding {
            2 => (length - 2) / 4 * 3 + 1,
            1 => (length - 1) / 4 * 3 + 2,
            _ => length / 4 * 3,
        };
        let mut output: Vec<u8> = Vec::with_capacity(new_length);
        let m: usize = new_length / 3;
        for x in 0..m {
            let y0: usize = x * 4;
            let y1: usize = y0 + 1;
            let y2: usize = y0 + 2;
            let y3: usize = y0 + 3;
            let v0: u8 = BASE64_DECODE_TABLE[data[y0] as usize];
            let v1: u8 = BASE64_DECODE_TABLE[data[y1] as usize];
            let v2: u8 = BASE64_DECODE_TABLE[data[y2] as usize];
            let v3: u8 = BASE64_DECODE_TABLE[data[y3] as usize];
            output.push((v0 & 0b00111111) << 2 | (v1 & 0b00110000) >> 4);
            output.push((v1 & 0b00001111) << 4 | (v2 & 0b00111100) >> 2);
            output.push((v2 & 0b00000011) << 6 | (v3 & 0b00111111));
        }
        if new_length % 3 == 1 {
            let y0: usize = m * 4;
            let y1: usize = y0 + 1;
            let v0: u8 = BASE64_DECODE_TABLE[data[y0] as usize];
            let v1: u8 = BASE64_DECODE_TABLE[data[y1] as usize];
            output.push((v0 & 0b00111111) << 2 | (v1 & 0b00110000) >> 4);
        } else if new_length % 3 == 2 {
            let y0: usize = m * 4;
            let y1: usize = y0 + 1;
            let y2: usize = y0 + 2;
            let v0: u8 = BASE64_DECODE_TABLE[data[y0] as usize];
            let v1: u8 = BASE64_DECODE_TABLE[data[y1] as usize];
            let v2: u8 = BASE64_DECODE_TABLE[data[y2] as usize];
            output.push((v0 & 0b00111111) << 2 | (v1 & 0b00110000) >> 4);
            output.push((v1 & 0b00001111) << 4 | (v2 & 0b00111100) >> 2);
        }
        return Ok(output);
    }

    /// 解码Url安全形式的Base64.
    ///
    /// 用`-`还原`+`, 用`_`还原`/`, 并自动补齐末尾缺失的`=`.
    ///
    /// - @exception 无效的长度.
    /// - @exception 出现未预期的字符.
    pub fn decode_url(input: &str) -> Result<Vec<u8>, String> {
        let data: &[u8] = input.as_bytes();
        let mut standard: Vec<u8> = Vec::with_capacity(data.len() + 4);
        for x in data {
            match x {
                b'-' => standard.push(b'+'),
                b'_' => standard.push(b'/'),
                _ => standard.push(*x),
            }
        }
        /* 补齐'='到4的倍数, 余1不可能由Base64编码产生. */
        match standard.len() % 4 {
            0 => {}
            2 => {
                standard.push(b'=');
                standard.push(b'=');
            }
            3 => {
                standard.push(b'=');
            }
            _ => {
                return Err(iceyee_error::c!("无效的长度"));
            }
        }
        /* 入参本身是合法UTF-8, 且只追加了ASCII, 不会失败. */
        let standard: String = String::from_utf8(standard).unwrap();
        return Self::decode(&standard);
    }
}

/// 十六进制编码.
#[derive(Debug, Clone)]
pub struct HexEncoder;

impl HexEncoder {
    /// 编码.
    pub fn encode(input: &[u8]) -> String {
        let mut output: String = String::with_capacity(input.len() * 2);
        for x in input {
            output.push(HEX_TABLE[(x >> 4) as usize] as char);
            output.push(HEX_TABLE[(x & 0x0F) as usize] as char);
        }
        return output;
    }

    /// 解码.
    ///
    /// - @exception 无效的长度.
    /// - @exception 出现未预期的字符.
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let data: &[u8] = input.as_bytes();
        let length: usize = data.len();
        if length % 2 != 0 {
            return Err(iceyee_error::c!("无效的长度"));
        }
        let mut output: Vec<u8> = Vec::with_capacity(length / 2);
        for x in 0..(length / 2) {
            /* 统一转小写后判断, 大小写都接受. */
            let high: u8 = match data[x * 2].to_ascii_lowercase() {
                c @ b'0'..=b'9' => c - b'0',
                c @ b'a'..=b'f' => c - b'a' + 10,
                _ => return Err(iceyee_error::c!("出现未预期的字符")),
            };
            let low: u8 = match data[x * 2 + 1].to_ascii_lowercase() {
                c @ b'0'..=b'9' => c - b'0',
                c @ b'a'..=b'f' => c - b'a' + 10,
                _ => return Err(iceyee_error::c!("出现未预期的字符")),
            };
            output.push((high << 4) | low);
        }
        return Ok(output);
    }

    /// 编码64位整数.
    ///
    /// 结果首位是`0`且长度大于1时去掉首位, 与decode_number对称.
    pub fn encode_number(input: u64) -> String {
        if input == 0 {
            return "0".to_string();
        }
        let mut input: u64 = input;
        let mut output: Vec<u8> = Vec::with_capacity(16);
        while input != 0 {
            output.push(HEX_TABLE[(input & 0xF) as usize]);
            input >>= 4;
            output.push(HEX_TABLE[(input & 0xF) as usize]);
            input >>= 4;
        }
        output.reverse();
        if output.len() > 1 && output[0] == b'0' {
            output.remove(0);
        }
        return String::from_utf8(output).unwrap();
    }

    /// 解码64位整数.
    ///
    /// - @exception 出现未预期的字符.
    pub fn decode_number(input: &str) -> Result<u64, String> {
        /* 去掉分隔符, 统一转小写, 再按字节截取(Vec截取不会panic). */
        let mut input: Vec<u8> = input
            .as_bytes()
            .iter()
            .copied()
            .filter(|x| *x != b' ' && *x != b'_')
            .collect();
        input.make_ascii_lowercase();
        input.truncate(16);
        let mut output: u64 = 0;
        for x in &input {
            match x {
                b'0'..=b'9' => {
                    output <<= 4;
                    output |= (x - b'0') as u64;
                }
                b'a'..=b'f' => {
                    output <<= 4;
                    output |= (x - b'a' + 10) as u64;
                }
                _ => {
                    return Err(iceyee_error::c!("出现未预期的字符"));
                }
            }
        }
        return Ok(output);
    }
}

/* Macro. */

/* 生成进制转换函数. */
macro_rules! number_to_string_function {
    ($number:ty, $name:ident, $name_separated:ident, $digits:expr, $bits:expr, $group:expr,
     $prefix:expr) => {
        /// 转成定长字符串, 高位补0.
        #[allow(clippy::unnecessary_cast)]
        pub fn $name(input: $number) -> String {
            return number_to_string(input as u64, $digits, $bits, 0, "");
        }

        /// 转成定长字符串, 高位补0, 带前缀, 每隔若干个字符插入一个下划线.
        #[allow(clippy::unnecessary_cast)]
        pub fn $name_separated(input: $number) -> String {
            return number_to_string(input as u64, $digits, $bits, $group, $prefix);
        }
    };
}

/// 进制编码.
#[derive(Clone, Debug)]
pub struct RadixEncoder;

impl RadixEncoder {
    /// 字符串转64位整数.
    ///
    /// 前缀0b/0o/0x不区分大小写, 没有前缀时按十六进制处理.
    ///
    /// - @exception 出现未预期的字符.
    pub fn string_to_u64(input: &str) -> Result<u64, String> {
        enum Radix {
            Binary,
            Octal,
            Hexadecimal,
        }
        /* 去掉分隔符, 统一转小写, 再按字节截取(Vec截取不会panic). */
        let mut input: Vec<u8> = input
            .as_bytes()
            .iter()
            .copied()
            .filter(|x| *x != b' ' && *x != b'_')
            .collect();
        input.make_ascii_lowercase();
        let radix: Radix = if input.starts_with(b"0b") {
            input.drain(..2);
            input.truncate(64);
            Radix::Binary
        } else if input.starts_with(b"0o") {
            input.drain(..2);
            input.truncate(22);
            Radix::Octal
        } else if input.starts_with(b"0x") {
            input.drain(..2);
            input.truncate(16);
            Radix::Hexadecimal
        } else {
            Radix::Hexadecimal
        };
        let mut output: u64 = 0;
        match radix {
            Radix::Binary => {
                for x in &input {
                    match x {
                        b'0'..=b'1' => {
                            output <<= 1;
                            output |= (x - b'0') as u64;
                        }
                        _ => {
                            return Err(iceyee_error::c!("出现未预期的字符"));
                        }
                    }
                }
            }
            Radix::Octal => {
                for x in &input {
                    match x {
                        b'0'..=b'7' => {
                            output <<= 3;
                            output |= (x - b'0') as u64;
                        }
                        _ => {
                            return Err(iceyee_error::c!("出现未预期的字符"));
                        }
                    }
                }
            }
            Radix::Hexadecimal => {
                for x in &input {
                    match x {
                        b'0'..=b'9' => {
                            output <<= 4;
                            output |= (x - b'0') as u64;
                        }
                        b'a'..=b'f' => {
                            output <<= 4;
                            output |= (x - b'a' + 10) as u64;
                        }
                        _ => {
                            return Err(iceyee_error::c!("出现未预期的字符"));
                        }
                    }
                }
            }
        }
        return Ok(output);
    }

    number_to_string_function!(u64, u64_to_bin, u64_to_bin_, 64, 1, 8, "0b");
    number_to_string_function!(u64, u64_to_oct, u64_to_oct_, 22, 3, 0, "0o");
    number_to_string_function!(u64, u64_to_hex, u64_to_hex_, 16, 4, 4, "0x");
    number_to_string_function!(u32, u32_to_bin, u32_to_bin_, 32, 1, 8, "0b");
    number_to_string_function!(u32, u32_to_oct, u32_to_oct_, 11, 3, 0, "0o");
    number_to_string_function!(u32, u32_to_hex, u32_to_hex_, 8, 4, 4, "0x");
    number_to_string_function!(u16, u16_to_bin, u16_to_bin_, 16, 1, 8, "0b");
    number_to_string_function!(u16, u16_to_oct, u16_to_oct_, 6, 3, 0, "0o");
    number_to_string_function!(u16, u16_to_hex, u16_to_hex_, 4, 4, 4, "0x");
    number_to_string_function!(u8, u8_to_bin, u8_to_bin_, 8, 1, 8, "0b");
    number_to_string_function!(u8, u8_to_oct, u8_to_oct_, 3, 3, 0, "0o");
    number_to_string_function!(u8, u8_to_hex, u8_to_hex_, 2, 4, 4, "0x");
}

/// Url编码.

#[derive(Debug, Clone)]
pub struct UrlEncoder;

impl UrlEncoder {
    /// 编码.
    ///
    /// 空格编码成`+`; 不编码 `A-Z a-z 0-9 - . _ ~`; 其它字节编码成`%XX`.
    pub fn encode(input: &str) -> String {
        let data: &[u8] = input.as_bytes();
        /* 最坏情况每个字节都编码成%XX, 一次分配到位. */
        let mut output: String = String::with_capacity(data.len() * 3);
        for x in data {
            if *x == b' ' {
                output.push('+');
            } else if x.is_ascii_alphanumeric() || b"-._~".contains(x) {
                output.push(*x as char);
            } else {
                output.push('%');
                output.push(HEX_TABLE[(x >> 4) as usize] as char);
                output.push(HEX_TABLE[(x & 0x0F) as usize] as char);
            }
        }
        return output;
    }

    /// 解码.
    ///
    /// - @exception 错误的格式.
    /// - @exception 内容不是UTF-8编码.
    pub fn decode(cipher: &str) -> Result<String, String> {
        enum Status {
            Normal,
            High,
            Low,
        }
        let data: &[u8] = cipher.as_bytes();
        let mut plain: Vec<u8> = Vec::with_capacity(data.len());
        let mut status: Status = Status::Normal;
        let mut high: u8 = 0;
        for x in data {
            match status {
                Status::Normal => match *x {
                    b'%' => {
                        status = Status::High;
                    }
                    b'+' => {
                        plain.push(b' ');
                    }
                    _ => {
                        plain.push(*x);
                    }
                },
                /* 统一转小写后判断, 大小写都接受. */
                Status::High => match x.to_ascii_lowercase() {
                    c @ b'0'..=b'9' => {
                        status = Status::Low;
                        high = c - b'0';
                    }
                    c @ b'a'..=b'f' => {
                        status = Status::Low;
                        high = c - b'a' + 10;
                    }
                    _ => {
                        return Err(iceyee_error::c!("错误的格式"));
                    }
                },
                Status::Low => match x.to_ascii_lowercase() {
                    c @ b'0'..=b'9' => {
                        status = Status::Normal;
                        plain.push((high << 4) | (c - b'0'));
                    }
                    c @ b'a'..=b'f' => {
                        status = Status::Normal;
                        plain.push((high << 4) | (c - b'a' + 10));
                    }
                    _ => {
                        return Err(iceyee_error::c!("错误的格式"));
                    }
                },
            }
        }
        match status {
            Status::Normal => {}
            Status::High | Status::Low => {
                return Err(iceyee_error::c!("错误的格式"));
            }
        }
        let plain: String =
            String::from_utf8(plain).map_err(|_| iceyee_error::c!("内容不是UTF-8编码"))?;
        return Ok(plain);
    }
}

/* Function. */

/* 进制转换. */
///
/// - input: 数值.
/// - digits: 字符个数, 高位补0.
/// - bits: 每个字符占用的二进制位数.
/// - group: 每隔多少个字符插入一个下划线, 0表示不插入.
/// - prefix: 前缀.
fn number_to_string(input: u64, digits: usize, bits: usize, group: usize, prefix: &str) -> String {
    let mask: u64 = (1u64 << bits) - 1;
    let separator: usize = (digits - 1).checked_div(group).unwrap_or(0);
    let mut output: String = String::with_capacity(prefix.len() + digits + separator);
    output.push_str(prefix);
    for x in 0..digits {
        if group != 0 && x != 0 && x % group == 0 {
            output.push('_');
        }
        let shift: u32 = ((digits - 1 - x) * bits) as u32;
        output.push(HEX_TABLE[((input >> shift) & mask) as usize] as char);
    }
    return output;
}
