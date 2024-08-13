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

#[tokio::test]
pub async fn test_mail() {
    println!("");
    iceyee_net::smtp::MailAgent::send(
        "smtp.qq.com",
        "709565591@qq.com",
        "vyhzmgkkunzrbcci",
        "709565591@qq.com",
        "1",
        "2",
    )
    .await;
    return;
}
