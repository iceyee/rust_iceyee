
http客户端的用法.
```rust
#[tokio::test]
pub async fn test_httpclient_no_proxy() {
    use iceyee_net::http::client::HttpClient;

    HttpClient::new()
        .set_verbose(true)
        .set_url("http://ip-api.com/json/")
        .unwrap()
        .set_header("Connection", "close")
        .set_forwarded(None)
        .send(None)
        .await
        .unwrap();
}
```

http服务端的用法.
```rust
use iceyee_net::http::server::Context;
use iceyee_net::http::server::Filter;
use iceyee_net::http::server::HttpServer;
use iceyee_net::http::server::Level;
use iceyee_net::http::server::Work;
use iceyee_net::http::Status;

struct WorkRedirect;
#[async_trait::async_trait]
impl Work for WorkRedirect {
    fn path(&self) -> &'static str {
        return "/redirect";
    }

    async fn do_work(&self, context: &mut Context) -> Result<(), String> {
        R::write_status(
            &mut context.response,
            Status::MovedTemporarily("/first_work".to_string()),
        );
        return Ok(());
    }
}

struct WorkFirst;
#[async_trait::async_trait]
impl Work for WorkFirst {
    fn path(&self) -> &'static str {
        return "/first_work";
    }

    async fn do_work(&self, context: &mut Context) -> Result<(), String> {
        R::write_ok(&mut context.response);
        context.response.body = "This is the first work.".as_bytes().to_vec();
        return Ok(());
    }
}

#[tokio::test]
pub async fn test_first_work() {
    HttpServer::new()
        .set_root("/home/xxx")
        .add_work(WorkFirst.to_arc())
        .add_work(WorkRedirect.to_arc())
        .start("localhost:10877", Some(Level::Debug), None, None)
        .await;
    return;
}
```
