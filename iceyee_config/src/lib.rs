// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 读写配置.
//!
//! ```
//! use serde::Deserialize;
//! use serde::Serialize;
//! ```

// Use.

use serde::de::Deserialize;
use serde::ser::Serialize;
use serde_json::Error as JsonError;
use serde_yaml::Error as YamlError;
use std::string::FromUtf8Error;

// Enum.

/// Error.
#[derive(Debug)]
pub enum ConfigError {
    StdIoError(std::io::Error),
    FromUtf8Error(FromUtf8Error),
    JsonError(JsonError),
    YamlError(YamlError),
    NotSupportSuffix(String),
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match self {
            ConfigError::StdIoError(e) => Some(e),
            ConfigError::FromUtf8Error(e) => Some(e),
            ConfigError::JsonError(e) => Some(e),
            ConfigError::YamlError(e) => Some(e),
            ConfigError::NotSupportSuffix(_) => None,
        };
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("ConfigError: ")?;
        match self {
            ConfigError::StdIoError(e) => f.write_str(e.to_string().as_str())?,
            ConfigError::FromUtf8Error(e) => f.write_str(e.to_string().as_str())?,
            ConfigError::JsonError(e) => f.write_str(e.to_string().as_str())?,
            ConfigError::YamlError(e) => f.write_str(e.to_string().as_str())?,
            ConfigError::NotSupportSuffix(suffix) => {
                f.write_str("不支持的后缀名: ")?;
                f.write_str(suffix.as_str())?;
            }
        };
        return Ok(());
    }
}

// Trait.

// Struct.

/// 读写配置, 只支持'json'和'yaml'格式.
#[derive(Debug, Clone, Default)]
pub struct ConfigParser;

impl ConfigParser {
    /// 读配置.
    ///
    /// - @param buffer 与返回结果的生命周期相同.
    /// - @exception [ConfigError::StdIoError] 读文件时出现IO异常.
    /// - @exception [ConfigError::FromUtf8Error] 文件内容不是Utf8字符.
    /// - @exception [ConfigError::JsonError] Json错误.
    /// - @exception [ConfigError::YamlError] Yaml错误.
    /// - @exception [ConfigError::NotSupportSuffix] 不支持的文件格式.
    pub async fn read<'a, T>(file_name: &str, buffer: &'a mut String) -> Result<T, ConfigError>
    where
        T: Deserialize<'a>,
    {
        static mut CONTENT: String = String::new();
        // # std::io::Error
        let content: Vec<u8> = tokio::fs::read(file_name)
            .await
            .map_err(|e| ConfigError::StdIoError(e))?;
        // # std::string::FromUtfError
        let content: String =
            String::from_utf8(content).map_err(|e| ConfigError::FromUtf8Error(e))?;
        if file_name.ends_with(".json") {
            *buffer = content;
            // # serde_json::Error
            return serde_json::from_str(buffer.as_str()).map_err(|e| ConfigError::JsonError(e));
        } else if file_name.ends_with(".yaml") {
            *buffer = content;
            // # serde_yaml::Error
            return serde_yaml::from_str(buffer.as_str()).map_err(|e| ConfigError::YamlError(e));
        } else {
            let suffix: String = match file_name.rfind('.') {
                Some(index) => file_name[0..index].to_string(),
                None => file_name.to_string(),
            };
            return Err(ConfigError::NotSupportSuffix(suffix));
        }
    }

    /// 写配置.
    ///
    /// - @exception [ConfigError::StdIoError] 写文件时出现IO异常.
    /// - @exception [ConfigError::JsonError] Json错误.
    /// - @exception [ConfigError::YamlError] Yaml错误.
    /// - @exception [ConfigError::NotSupportSuffix] 不支持的文件格式.
    pub async fn write<T>(file_name: &str, object: T) -> Result<(), ConfigError>
    where
        T: Serialize,
    {
        let content: String = if file_name.ends_with(".json") {
            // # serde_json::Error
            serde_json::to_string(&object).map_err(|e| ConfigError::JsonError(e))?
        } else if file_name.ends_with(".yaml") {
            // # serde_yaml::Error
            serde_yaml::to_string(&object).map_err(|e| ConfigError::YamlError(e))?
        } else {
            let suffix: String = match file_name.rfind('.') {
                Some(index) => file_name[0..index].to_string(),
                None => file_name.to_string(),
            };
            return Err(ConfigError::NotSupportSuffix(suffix));
        };
        // # std::io::Error
        tokio::fs::write(file_name, content.as_bytes())
            .await
            .map_err(|e| ConfigError::StdIoError(e))?;
        return Ok(());
    }
}

// Function.
