// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::Args;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_args() {
    println!("");
    println!("new Args.");
    let mut args: Args = Args::new();
    println!("add '你好', '我好', '他好', 'k'.");
    args.add("你好", "1");
    args.add("你好", "2");
    args.add("我好", "他好");
    args.add("他好", "他好");
    args.add("k", "PrW4rLRM-K40GMA77lYUD+fvXc8=");
    println!("{}", args.to_string());
    println!("{:#?}", args);
    println!("remove '他好'");
    args.remove("他好");
    println!("{}", args.to_string());
    println!("{:#?}", args);
    println!("parse old args.to_string().");
    let args: Args = Args::parse(args.to_string().as_str());
    println!("{:#?}", args);
    return;
}
