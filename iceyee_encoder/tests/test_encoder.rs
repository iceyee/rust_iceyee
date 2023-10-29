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
pub fn test_base64_encoder() {
    use iceyee_encoder::Base64Encoder;
    use iceyee_encoder::Base64Error;
    use iceyee_encoder::Encoder;
    println!("");
    let table = [
        ("hello world.", "aGVsbG8gd29ybGQu"),
        ("hello world", "aGVsbG8gd29ybGQ="),
        ("hello worl", "aGVsbG8gd29ybA=="),
        ("hello wor", "aGVsbG8gd29y"),
    ];
    for (x, y) in table {
        assert!(Base64Encoder::encode(x.as_bytes().to_vec()).unwrap() == y);
        assert!(String::from_utf8(Base64Encoder::decode(y.to_string()).unwrap()).unwrap() == x);
    }
    match Base64Encoder::decode("12345".to_string()) {
        Err(Base64Error::InvalidLength(5)) => {}
        _ => assert!(false),
    };
    match Base64Encoder::decode("123456".to_string()) {
        Err(Base64Error::InvalidLength(6)) => {}
        _ => assert!(false),
    };
    match Base64Encoder::decode("1234567".to_string()) {
        Err(Base64Error::InvalidLength(7)) => {}
        _ => assert!(false),
    };
    match Base64Encoder::decode("123@".to_string()) {
        Err(Base64Error::UnexpectedCharacter('@')) => {}
        _ => assert!(false),
    };
    match Base64Encoder::decode("123#".to_string()) {
        Err(Base64Error::UnexpectedCharacter('#')) => {}
        _ => assert!(false),
    };
    match Base64Encoder::decode("@23#".to_string()) {
        Err(Base64Error::UnexpectedCharacter('@')) => {}
        _ => assert!(false),
    };
    return;
}

#[test]
pub fn test_hex_encoder() {
    use iceyee_encoder::Encoder;
    use iceyee_encoder::HexEncoder;
    use iceyee_encoder::HexError;
    let a: Vec<u8> = [0x12, 0x34, 0x56, 0xab, 0xcd].to_vec();
    let b1: String = "123456ABCD".to_string();
    let b2: String = "123456abcd".to_string();
    println!("");
    assert!(HexEncoder::encode(a.clone()).unwrap() == b1);
    assert!(HexEncoder::decode(b1.clone()).unwrap() == a);
    assert!(HexEncoder::decode(b2.clone()).unwrap() == a);
    match HexEncoder::decode("123456a".to_string()) {
        Err(HexError::InvalidLength(7)) => {}
        _ => assert!(false),
    };
    match HexEncoder::decode("123456abc".to_string()) {
        Err(HexError::InvalidLength(9)) => {}
        _ => assert!(false),
    };
    match HexEncoder::decode("12@456abcd".to_string()) {
        Err(HexError::UnexpectedCharacter('@')) => {}
        _ => assert!(false),
    };
    match HexEncoder::decode("123456a#cd".to_string()) {
        Err(HexError::UnexpectedCharacter('#')) => {}
        _ => assert!(false),
    };
    match HexEncoder::decode("123456abcg".to_string()) {
        Err(HexError::UnexpectedCharacter('g')) => {}
        _ => assert!(false),
    };
    assert!(HexEncoder::decode_to_number("0123456789".to_string()).unwrap() == 0x0123456789);
    assert!(
        HexEncoder::decode_to_number("0123456789abcdef".to_string()).unwrap() == 0x0123456789ABCDEF
    );
    assert!(
        HexEncoder::decode_to_number("0123456789ABCDEF".to_string()).unwrap() == 0x0123456789ABCDEF
    );
    match HexEncoder::decode_to_number("0123456789ABCDEF0".to_string()) {
        Err(HexError::InvalidLength(17)) => {}
        _ => assert!(false),
    };
    match HexEncoder::decode_to_number("0123456789ABCDEF01".to_string()) {
        Err(HexError::InvalidLength(18)) => {}
        _ => assert!(false),
    };
    match HexEncoder::decode_to_number("-123456789".to_string()) {
        Err(HexError::UnexpectedCharacter('-')) => {}
        _ => assert!(false),
    };
    match HexEncoder::decode_to_number("012345678z".to_string()) {
        Err(HexError::UnexpectedCharacter('z')) => {}
        _ => assert!(false),
    };
    return;
}

#[test]
pub fn test_url_encoder() {
    use iceyee_encoder::Encoder;
    use iceyee_encoder::UrlEncoder;
    use iceyee_encoder::UrlError;
    println!("");
    assert!(UrlEncoder::encode(" 1_1 ".to_string()).unwrap() == "%201_1%20");
    assert!(UrlEncoder::decode("%201_1%20".to_string()).unwrap() == " 1_1 ");
    assert!(UrlEncoder::decode("%201+1%20".to_string()).unwrap() == " 1 1 ");
    match UrlEncoder::decode("%%".to_string()) {
        Err(UrlError::InvalidFormat) => {}
        _ => assert!(false),
    };
    match UrlEncoder::decode("%3%45".to_string()) {
        Err(UrlError::InvalidFormat) => {}
        _ => assert!(false),
    };
    match UrlEncoder::decode("%34%5".to_string()) {
        Err(UrlError::InvalidFormat) => {}
        _ => assert!(false),
    };
    return;
}
