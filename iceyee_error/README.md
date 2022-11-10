
# iceyee_error

自定义的异常, 包含有堆栈信息.

## Example

```rust
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
```
