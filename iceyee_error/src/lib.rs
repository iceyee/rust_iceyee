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
//!     return;
//! }
//! ```
//!
//! 输出结果如下:
//! ```text
//! 创建默认异常
//! Create error at iceyee_error/tests/test_a.rs:20:16 test_a
//! 创建异常, 参数'hello world'
//! Create error at iceyee_error/tests/test_a.rs:21:16 test_a, hello world
//! 创建异常, 参数'hello', 'world'
//! Create error at iceyee_error/tests/test_a.rs:22:16 test_a, hello, world
//! 继承异常, 参数'how', 'are', 'you'
//! Create error at iceyee_error/tests/test_a.rs:22:16 test_a, hello, world
//! Inherit error at iceyee_error/tests/test_a.rs:23:16 test_a, how, are, you
//! 继承异常, 参数'thank', 'you'
//! Create error at iceyee_error/tests/test_a.rs:22:16 test_a, hello, world
//! Inherit error at iceyee_error/tests/test_a.rs:23:16 test_a, how, are, you
//! Inherit error at iceyee_error/tests/test_a.rs:24:16 test_a, thank, you
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
            let mut message = format!("Create error at {}:{}:{} {}", file!(), line!(), column!(), module_path!());
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
    ($e:expr, $($x:expr),* $(,)?) => {
        {
            let mut message = format!("{}\nInherit error at {}:{}:{} {}", $e, file!(), line!(), column!(), module_path!());
            $(
                message.push_str(", ");
                message.push_str($x.to_string().as_str());
            )*
            message
        }
    };
}
