// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 自定义的异常.

// Use.

use std::backtrace::Backtrace;
use std::error::Error as StdError;

// Enum.

// Trait.

// Struct.

/// 自定义的异常, 能输出堆栈信息.

#[derive(Debug)]
pub struct IceyeeError {
    message: String,
    backtrace: Backtrace,
}

impl IceyeeError {
    pub fn new(s: &str) -> Self {
        return IceyeeError {
            message: s.to_string(),
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl From<&str> for IceyeeError {
    fn from(s: &str) -> Self {
        return IceyeeError {
            message: s.to_string(),
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl From<String> for IceyeeError {
    fn from(s: String) -> Self {
        return IceyeeError {
            message: s,
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl From<Box<dyn StdError>> for IceyeeError {
    fn from(e: Box<dyn StdError>) -> Self {
        return IceyeeError {
            message: format!("{e}"),
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl std::fmt::Display for IceyeeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(format!("{}\n{:#?}", &self.message, self.backtrace).as_str())?;
        return Ok(());
    }
}

impl std::error::Error for IceyeeError {}

/// 封装的异常, 主要用于线程间传递.
///
/// 一般情况下, rust编译器不允许[std::error::Error]在线程间传递,
/// 所以才会有这个, 把[std::error::Error]包装到WrapError中即可在线程间传递.
#[derive(Debug)]
pub struct WrapError<T>
where
    T: StdError,
{
    e: T,
}

unsafe impl<T> Send for WrapError<T> where T: StdError {}
unsafe impl<T> Sync for WrapError<T> where T: StdError {}

impl<T> WrapError<T>
where
    T: StdError,
{
    pub fn new(e: T) -> Self {
        return Self { e: e };
    }

    pub fn unwrap(self) -> T {
        return self.e;
    }
}

impl<T> std::ops::Deref for WrapError<T>
where
    T: StdError,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.e;
    }
}

impl<T> std::ops::DerefMut for WrapError<T>
where
    T: StdError,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.e;
    }
}

// impl<T> From<T> for WrapError<T> {
//     fn from(e: T) -> Self {
//         return Self { e: e };
//     }
// }
//
// impl<T> Into<T> for WrapError<T>
// where
//     T: StdError,
// {
//     fn into(self) -> T {
//         return self.e;
//     }
// }

impl<T> std::fmt::Display for WrapError<T>
where
    T: StdError,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(format!("{}", &self.e).as_str())?;
        return Ok(());
    }
}

impl<T> std::error::Error for WrapError<T> where T: StdError {}

// Function.
