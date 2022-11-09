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

const JSON: &str = "
{
    \"a\": 1
}
";
const YAML: &str = "
    a: 2
";

#[tokio::test]
pub async fn test_config() {
    use iceyee_config::ConfigParser;
    use serde::Deserialize;
    use serde::Serialize;
    println!("");
    // 写入数据.
    tokio::fs::write("/tmp/test.json", JSON.as_bytes())
        .await
        .unwrap();
    tokio::fs::write("/tmp/test.yaml", YAML.as_bytes())
        .await
        .unwrap();
    #[derive(Debug, Serialize, Deserialize)]
    struct A {
        a: usize,
    }
    let mut buffer: String = String::new();
    let a: A = ConfigParser::read("/tmp/test.json", &mut buffer)
        .await
        .unwrap();
    assert!(a.a == 1);
    let a: A = ConfigParser::read("/tmp/test.yaml", &mut buffer)
        .await
        .unwrap();
    assert!(a.a == 2);
    return;
}
