// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn a() {
    println!("");
    let a001 = iceyee_error::a!();
    let a002 = iceyee_error::a!("hello world");
    let a003 = iceyee_error::a!("hello", "world");
    let a004 = iceyee_error::b!(&a003, "how", "are", "you");
    let a005 = iceyee_error::b!(&a004, "thank", "you");
    println!("创建默认异常");
    println!("{}", a001);
    println!("创建异常, 参数'hello world'");
    println!("{}", a002);
    println!("创建异常, 参数'hello', 'world'");
    println!("{}", a003);
    println!("继承异常, 参数'how', 'are', 'you'");
    println!("{}", a004);
    println!("继承异常, 参数'thank', 'you'");
    println!("{}", a005);
    return;
}
