// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! SMTP协议.

// Use.

use lettre::address::Address;
use lettre::message::Message;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error as SmtpError;
use lettre::transport::smtp::SmtpTransport;
use lettre::Transport;

// Enum.

// Trait.

// Struct.

pub struct MailAgent;

impl MailAgent {
    /// 发送邮件.
    pub async fn send(
        server: &str,
        name: &str,
        auth: &str,
        to: &str,
        title: &str,
        body: &str,
    ) -> bool {
        if let Err(e) = Self::send_email(server, name, auth, to, title, body).await {
            iceyee_logger::error!(e);
            return false;
        } else {
            return true;
        }
    }

    async fn send_email(
        server: &str,
        name: &str,
        auth: &str,
        to: &str,
        title: &str,
        body: &str,
    ) -> Result<(), SmtpError> {
        let message = Message::builder()
            .sender(name.parse::<Address>().unwrap().into())
            .from(name.parse::<Address>().unwrap().into())
            .to(to.parse::<Address>().unwrap().into())
            .subject(title)
            .body(body.to_string())
            .expect("MessageBuilder::body()");
        let a001: Vec<u8> = message.formatted();
        let a002: String = String::from_utf8(a001).expect("NEVER");
        iceyee_logger::warn!("\n", a002);
        SmtpTransport::relay(server)?
            .credentials(Credentials::new(name.to_string(), auth.to_string()))
            .build()
            .send(&message)?;
        return Ok(());
    }
}

// Function.
