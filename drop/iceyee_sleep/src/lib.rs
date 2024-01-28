// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

//! 延时.

use std::time::Duration;
use tokio::time::Sleep;

// Enum.

// Trait.

// Struct.

// Function.

/// 延时.
pub async fn sleep(t: usize) -> Sleep {
    return tokio::time::sleep(Duration::from_millis(t as u64));
}
