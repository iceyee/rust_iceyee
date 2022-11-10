// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 自定义的异常.
//!
//! 重新导出标准库的异常并重命名:
//! - [std::error::Error] as [StdError]
//! - [std::fmt::Error] as [StdFmtError]
//! - [std::io::ErrorKind] as [StdIoErrorKind]
//! - [std::io::Error] as [StdIoError]

// Use.

pub use std::error::Error as StdError;
pub use std::fmt::Error as StdFmtError;
pub use std::io::Error as StdIoError;
pub use std::io::ErrorKind as StdIoErrorKind;

use std::backtrace::Backtrace;

// Enum.

// Trait.

// Struct.

/// 自定义的异常.

pub struct IceyeeError {
    message: String,
    error: Option<Box<dyn StdError>>,
    backtrace: Backtrace,
}

impl IceyeeError {
    pub fn new() -> Self {
        return IceyeeError {
            message: "".to_string(),
            error: None,
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl From<&str> for IceyeeError {
    fn from(s: &str) -> Self {
        return IceyeeError {
            message: s.to_string(),
            error: None,
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl From<&String> for IceyeeError {
    fn from(s: &String) -> Self {
        return IceyeeError {
            message: s.clone(),
            error: None,
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl From<String> for IceyeeError {
    fn from(s: String) -> Self {
        return IceyeeError {
            message: s,
            error: None,
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl From<Box<dyn StdError>> for IceyeeError {
    fn from(e: Box<dyn StdError>) -> Self {
        return IceyeeError {
            message: e.to_string(),
            error: Some(e),
            backtrace: Backtrace::force_capture(),
        };
    }
}

impl StdError for IceyeeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use std::ops::Deref;
        return match &self.error {
            Some(e) => Some(e.deref()),
            None => None,
        };
    }
}

impl std::fmt::Debug for IceyeeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("IceyeeError: ")?;
        f.write_str(self.message.as_str())?;
        f.write_str(format!("\n{:#?}", self.backtrace).as_str())?;
        return Ok(());
    }
}

impl std::fmt::Display for IceyeeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("IceyeeError: ")?;
        f.write_str(self.message.as_str())?;
        f.write_str(format!("\n{:#?}", self.backtrace).as_str())?;
        return Ok(());
    }
}

// Function.
