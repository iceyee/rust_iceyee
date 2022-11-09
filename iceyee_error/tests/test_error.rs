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
pub fn test_iceyee_error() {
    use iceyee_error::IceyeeError;
    use std::error::Error;
    println!("");
    let e: IceyeeError = IceyeeError::new();
    println!("{}\n", e.to_string());
    let e: IceyeeError = IceyeeError::from("hello world.");
    println!("{}\n", e.to_string());
    let e: IceyeeError = IceyeeError::from(Box::new(std::fmt::Error {}) as Box<dyn Error>);
    println!("{}\n", e.to_string());
    return;
}
