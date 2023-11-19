// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::client::HttpClient;
use iceyee_net::http::server::Context;
use iceyee_net::http::server::HttpServer;
use iceyee_net::http::server::Level;
use iceyee_net::http::server::Work;
use iceyee_net::http::server::R;
use iceyee_net::http::Status;
use std::sync::Arc;

// Enum.

// Trait.

// Struct.

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

struct WorkSetA;
#[async_trait::async_trait]
impl Work for WorkSetA {
    fn path(&self) -> &'static str {
        return "/set_a";
    }

    async fn do_work(&self, context: &mut Context) -> Result<(), String> {
        R::write_ok(&mut context.response);
        let a = context.request.query.get("a")[0].to_string();
        context.session.set("a", &a).await;
        return Ok(());
    }
}

struct WorkGetA;
#[async_trait::async_trait]
impl Work for WorkGetA {
    fn path(&self) -> &'static str {
        return "/get_a";
    }

    async fn do_work(&self, context: &mut Context) -> Result<(), String> {
        R::write_ok(&mut context.response);
        let a = context.session.get("a").await;
        context.response.body = a.as_bytes().to_vec();
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

// Function.

#[tokio::test]
pub async fn test_first_work() {
    use iceyee_net::http::server::BasicAuthFilter;
    use iceyee_net::http::server::Filter;

    println!("");

    let a001 = tokio::task::spawn(async move {
        // let response = HttpClient::new()
        //     .set_verbose(true)
        //     .set_header("Connection", "close")
        //     // .set_header("Accept-Encoding", "/")
        //     .set_url("http://localhost:10877/first_work")
        //     .map_err(|_| ())
        //     .unwrap()
        //     .send(None)
        //     .await
        //     .map_err(|_| ())
        //     .unwrap();
        let response = HttpClient::new()
            .set_verbose(true)
            .set_header("Connection", "close")
            // .set_header("Accept-Encoding", "/")
            .set_url("http://localhost:10877/_Shizuku_start.sh")
            .map_err(|_| ())
            .unwrap()
            .send(None)
            .await
            .map_err(|_| ())
            .unwrap();
    });

    HttpServer::new()
        .set_root("/home/ljq")
        .add_filter_before_work(BasicAuthFilter::new("iceyee", "74591870").to_arc())
        .add_work(WorkFirst.to_arc())
        .add_work(WorkGetA.to_arc())
        .add_work(WorkSetA.to_arc())
        .add_work(WorkRedirect.to_arc())
        .start("localhost:10877", Some(Level::Debug), None, None)
        .await;

    a001.await;
    // iceyee_datetime::sleep(2_000).await;

    return;
}
