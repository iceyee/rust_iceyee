
# iceyee_convert

转换类型.

- string_to_hex

## Supported Os

- [x] linux
- [x] macos
- [x] windows

## Example

```rust
#[test]
pub fn test_string_to_hex() {
    use iceyee_convert::Conversion;
    println!("");
    assert!(Conversion::string_to_hex("FFF").unwrap() == 0xFFF);
    assert!(Conversion::string_to_hex("1af").unwrap() == 0x1AF);
    assert!(Conversion::string_to_hex("FFFG").is_err());
    assert!(Conversion::string_to_hex(" Z").is_err());
    return;
}
```
