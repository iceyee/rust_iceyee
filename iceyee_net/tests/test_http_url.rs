// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::Url;

// Enum.

// Trait.

// Struct.

// Function.

#[test]
pub fn test_url() {
    println!("");
    let url: Url = Url::new("http://api.k780.com/?app=ip.local&appkey=10003&sign=b59bc3ef6191eb9f747dd4e83c99f2a4&format=json").unwrap();
    println!("{:#?}", url);
    let url: Url = Url::new("http://ip-api.com/json/").unwrap();
    println!("{:#?}", url);
    let url: Url = Url::new("https://www.baidu.com/p?a=1&b=2&c=3#ID").unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 443);
    assert_eq!(url.path, "/p");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = Url::new("http://www.baidu.com/p?a=1&b=2&c=3#ID").unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "http:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 80);
    assert_eq!(url.path, "/p");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = Url::new("https://www.baidu.com:10443/p?a=1&b=2&c=3#ID").unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 10443);
    assert_eq!(url.path, "/p");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = Url::new("https://www.baidu.com?a=1&b=2&c=3#ID").unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 443);
    assert_eq!(url.path, "/");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = Url::new("https://www.baidu.com/p#ID").unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 443);
    assert_eq!(url.path, "/p");
    assert!(url.query.is_none());
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = Url::new("https://www.baidu.com/p?a=1&b=2&c=3").unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 443);
    assert_eq!(url.path, "/p");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert!(url.fragment.is_none());
    assert!(Url::new("//www.baidu.com/p?a=1&b=2&c=3").is_err());
    assert!(Url::new("https:///p?a=1&b=2&c=3").is_err());
    assert!(Url::new("https://www.baidu.com:").is_err());
    assert!(Url::new("https://www.baidu.com:999999").is_err());
    return;
}
