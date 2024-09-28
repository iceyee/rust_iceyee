// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 创建异常和继承异常, [a], [b].
//!
//! # Example
//! ```
//! #[test]
//! pub fn a() {
//!     println!("");
//!     let a001 = iceyee_error::a!();
//!     let a002 = iceyee_error::a!("hello world");
//!     let a003 = iceyee_error::a!("hello", "world");
//!     let a004 = iceyee_error::b!(&a003, "how", "are", "you");
//!     let a005 = iceyee_error::b!(&a004, "thank", "you");
//!     let a006 = iceyee_error::b!(&a005);
//!     println!("创建默认异常");
//!     println!("{}", a001);
//!     println!("创建异常, 参数'hello world'");
//!     println!("{}", a002);
//!     println!("创建异常, 参数'hello', 'world'");
//!     println!("{}", a003);
//!     println!("继承异常, 参数'how', 'are', 'you'");
//!     println!("{}", a004);
//!     println!("继承异常, 参数'thank', 'you'");
//!     println!("{}", a005);
//!     println!("继承异常, 无参数");
//!     println!("{}", a006);
//!     return;
//! }
//! ```
//!
//! # Output
//! ```text
//! 创建默认异常
//! Create error at iceyee_error/tests/test_a.rs:20:16
//! 创建异常, 参数'hello world'
//! Create error at iceyee_error/tests/test_a.rs:21:16, hello world
//! 创建异常, 参数'hello', 'world'
//! Create error at iceyee_error/tests/test_a.rs:22:16, hello, world
//! 继承异常, 参数'how', 'are', 'you'
//! Create error at iceyee_error/tests/test_a.rs:22:16, hello, world
//! Inherit error at iceyee_error/tests/test_a.rs:23:16, how, are, you
//! 继承异常, 参数'thank', 'you'
//! Create error at iceyee_error/tests/test_a.rs:22:16, hello, world
//! Inherit error at iceyee_error/tests/test_a.rs:23:16, how, are, you
//! Inherit error at iceyee_error/tests/test_a.rs:24:16, thank, you
//! 继承异常, 无参数
//! Create error at iceyee_error/tests/test_a.rs:22:16, hello, world
//! Inherit error at iceyee_error/tests/test_a.rs:23:16, how, are, you
//! Inherit error at iceyee_error/tests/test_a.rs:24:16, thank, you
//! Inherit error at iceyee_error/tests/test_a.rs:25:16
//! ```

// Use.

// Enum.

// Trait.

// Struct.

// Function.

/// 创建异常.
///
/// @return [String]
#[macro_export]
macro_rules! a {
    ($($x:expr),* $(,)?) => {
        {
            let mut message = format!(
                    "Create error at {}:{}:{}",
                    file!(),
                    line!(),
                    column!());
            $(
                message.push_str(", ");
                message.push_str($x.to_string().as_str());
            )*
            message
        }
    };
}

/// 继承异常.
///
/// @return [String]
#[macro_export]
macro_rules! b {
    ($e:expr) => {
        {
            let message = format!(
                    "{}\nInherit error at {}:{}:{}",
                    $e,
                    file!(),
                    line!(),
                    column!());
            message
        }
    };
    ($e:expr, $($x:expr),* $(,)?) => {
        {
            let mut message = format!(
                    "{}\nInherit error at {}:{}:{}",
                    $e,
                    file!(),
                    line!(),
                    column!());
            $(
                message.push_str(", ");
                message.push_str($x.to_string().as_str());
            )*
            message
        }
    };
}
