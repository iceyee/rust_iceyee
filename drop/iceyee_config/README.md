
# iceyee_config

读写配置.

## Supported Os

- [x] linux
- [x] macos
- [x] windows

## Example

```rust
const JSON: &str = "
{
    \"a\": 1,
    \"b\": 2
}
";
const YAML: &str = "
    a: 3
    b: 4
";

#[tokio::test]
pub async fn test_config() {
    use iceyee_config::ConfigParser;
    use serde::Deserialize;
    use serde::Serialize;
    #[derive(Debug, Serialize, Deserialize)]
    struct A {
        a: usize,
        b: usize,
    }
    println!("");
    // 写入数据.
    tokio::fs::write("/tmp/test.json", JSON.as_bytes())
        .await
        .unwrap();
    tokio::fs::write("/tmp/test.yaml", YAML.as_bytes())
        .await
        .unwrap();
    // 读配置, 验证.
    let mut buffer: String = String::new();
    let a: A = ConfigParser::read("/tmp/test.json", &mut buffer)
        .await
        .unwrap();
    assert!(a.a == 1);
    assert!(a.b == 2);
    let a: A = ConfigParser::read("/tmp/test.yaml", &mut buffer)
        .await
        .unwrap();
    assert!(a.a == 3);
    assert!(a.b == 4);
    return;
}
```
