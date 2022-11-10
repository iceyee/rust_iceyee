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
    use iceyee_error::StdError;
    use iceyee_error::StdFmtError;
    println!("");
    let e: IceyeeError = IceyeeError::new();
    println!("{e}\n");
    let e: IceyeeError = IceyeeError::from("hello world.");
    println!("{e}\n");
    let e: IceyeeError = IceyeeError::from(Box::new(StdFmtError) as Box<dyn StdError>);
    println!("{e}\n");
    return;
}
