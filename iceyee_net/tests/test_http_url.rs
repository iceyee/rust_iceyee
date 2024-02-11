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
    let url: Url = "http://api.k780.com/?app=ip.local&appkey=10003&sign=b59bc3ef6191eb9f747dd4e83c99f2a4&format=json".parse::<Url>().unwrap();
    println!("{:#?}", url);
    let url: Url = "http://ip-api.com/json/".parse::<Url>().unwrap();
    println!("{:#?}", url);
    let url: Url = "https://www.baidu.com/p?a=1&b=2&c=3#ID"
        .parse::<Url>()
        .unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 443);
    assert_eq!(url.path, "/p");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = "http://www.baidu.com/p?a=1&b=2&c=3#ID"
        .parse::<Url>()
        .unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "http:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 80);
    assert_eq!(url.path, "/p");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = "https://www.baidu.com:10443/p?a=1&b=2&c=3#ID"
        .parse::<Url>()
        .unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 10443);
    assert_eq!(url.path, "/p");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = "https://www.baidu.com?a=1&b=2&c=3#ID"
        .parse::<Url>()
        .unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 443);
    assert_eq!(url.path, "/");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = "https://www.baidu.com/p#ID".parse::<Url>().unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 443);
    assert_eq!(url.path, "/p");
    assert!(url.query.is_none());
    assert_eq!(url.fragment, Some("#ID".to_string()));
    let url: Url = "https://www.baidu.com/p?a=1&b=2&c=3"
        .parse::<Url>()
        .unwrap();
    println!("{:#?}", url);
    assert_eq!(url.protocol, "https:");
    assert_eq!(url.host, "www.baidu.com");
    assert_eq!(url.port, 443);
    assert_eq!(url.path, "/p");
    assert_eq!(url.query, Some("?a=1&b=2&c=3".to_string()));
    assert!(url.fragment.is_none());
    assert!("//www.baidu.com/p?a=1&b=2&c=3".parse::<Url>().is_err());
    assert!("https:///p?a=1&b=2&c=3".parse::<Url>().is_err());
    assert!("https://www.baidu.com:".parse::<Url>().is_err());
    assert!("https://www.baidu.com:999999".parse::<Url>().is_err());
    return;
}
