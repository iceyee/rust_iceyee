// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::client::HttpClient;
use iceyee_net::http::server::Context;
use iceyee_net::http::server::Filter;
use iceyee_net::http::server::HttpServer;
use iceyee_net::http::server::Level;
use iceyee_net::http::server::Work;
use iceyee_net::http::server::R;
use iceyee_net::http::Status;

// Enum.

// Trait.

// Struct.

struct WorkRedirect;
#[async_trait::async_trait]
impl Work for WorkRedirect {
    fn path(&self) -> String {
        return "/redirect".to_string();
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
    fn path(&self) -> String {
        return "/first_work".to_string();
    }

    async fn do_work(&self, context: &mut Context) -> Result<(), String> {
        R::write_ok(&mut context.response);
        context.response.body = "This is the first work.".as_bytes().to_vec();
        return Ok(());
    }
}

struct WorkError;
#[async_trait::async_trait]
impl Work for WorkError {
    fn path(&self) -> String {
        return "/error".to_string();
    }

    async fn do_work(&self, context: &mut Context) -> Result<(), String> {
        let _ = context;
        Err("100%触发异常.".to_string())?;
        return Ok(());
    }
}

use serde::Serialize;

#[derive(Serialize)]
struct T915 {
    a: String,
    b: usize,
}

struct WorkJson;
#[async_trait::async_trait]
impl Work for WorkJson {
    fn path(&self) -> String {
        return "/json".to_string();
    }

    async fn do_work(&self, context: &mut Context) -> Result<(), String> {
        use iceyee_net::http::server::ResponseObject;

        let a001 = T915 {
            a: "hello world.".to_string(),
            b: 74591870,
        };
        let r: ResponseObject<T915> = ResponseObject {
            success: false,
            message: "某些错误信息.".to_string(),
            data: a001,
        };
        R::write_json(&mut context.response, &r);
        return Ok(());
    }
}

// Function.

#[tokio::test]
pub async fn test_first_work() {
    use iceyee_net::http::server::component::FilterBasicAuth;
    use iceyee_net::http::server::component::FilterCORS;

    println!("");

    let a001 = tokio::task::spawn(async move {
        HttpServer::wait_one_second().await;
        HttpClient::new()
            .set_verbose(true)
            .set_header("Connection", "close")
            .set_url("http://localhost:10877/first_work")
            .unwrap()
            .send(None)
            .await
            .unwrap();
        HttpClient::new()
            .set_verbose(true)
            .set_header("Connection", "close")
            .set_url("http://localhost:10877/error")
            .unwrap()
            .set_header("Authorization", "Basic aWNleWVlOjc0NTkxODcw")
            .send(None)
            .await
            .unwrap();
        HttpClient::new()
            .set_verbose(true)
            .set_header("Connection", "close")
            .set_url("http://localhost:10877/json")
            .unwrap()
            .set_header("Authorization", "Basic aWNleWVlOjc0NTkxODcw")
            .send(None)
            .await
            .unwrap();
        // HttpClient::new()
        //     .set_verbose(true)
        //     .set_header("Connection", "close")
        //     .set_url("http://localhost:10877/_Shizuku_start.sh")
        //     .map_err(|_| ())
        //     .unwrap()
        //     .send(None)
        //     .await
        //     .map_err(|_| ())
        //     .unwrap();
    });

    HttpServer::new()
        .set_root("/home/ljq")
        .add_filter_before_work(FilterCORS::new().allow_origin("*").wrap())
        .add_filter_before_work(FilterBasicAuth::new("iceyee", "74591870").wrap())
        .add_work(WorkFirst.wrap())
        .add_work(WorkRedirect.wrap())
        .add_work(WorkError.wrap())
        .add_work(WorkJson.wrap())
        .start("localhost:10877", Some(Level::Debug), None, None)
        .await
        .unwrap();

    a001.await.unwrap();
    HttpServer::wait_one_second().await;

    return;
}
