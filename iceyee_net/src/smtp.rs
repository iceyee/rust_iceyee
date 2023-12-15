// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! SMTP协议.

// Use.

use tokio::net::ToSocketAddrs;

// Enum.

// Trait.

// Struct.

pub struct MailAgent;

impl MailAgent {
    /// 使用smtp协议发送邮件.
    pub async fn send<A>(
        _server_address: A,
        _name: &str,
        _auth: &str,
        _to: &str,
        _title: &str,
        _body: &str,
    ) -> Result<String, String>
    where
        A: ToSocketAddrs,
    {
        unimplemented!("");
    }
}

// Function.
