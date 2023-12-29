// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use iceyee_net::http::client::HttpClient;
use iceyee_net::http::client::NoProxy;
use iceyee_net::http::client::Proxy;
use iceyee_net::http::server::component::FilterBasicAuth;
use iceyee_net::http::server::component::FilterCORS;
use iceyee_net::http::server::Context;
use iceyee_net::http::server::Filter;
use iceyee_net::http::server::HttpServer;
use iceyee_net::http::server::ResponseObject;
use iceyee_net::http::server::Work;
use iceyee_net::http::server::R;
use iceyee_net::http::Status;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;

// Enum.

// Trait.

// Struct.

struct WorkRedirect;
impl Work for WorkRedirect {
    fn path(&self) -> String {
        return "/redirect".to_string();
    }

    fn do_work<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            R::write_status(
                &mut context.response,
                Status::MovedTemporarily("/first_work".to_string()),
            );
            return Ok(());
        });
    }
}

struct WorkFirst;
impl Work for WorkFirst {
    fn path(&self) -> String {
        return "/first_work".to_string();
    }

    fn do_work<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            R::write_ok(&mut context.response);
            context.response.body = "This is the first work.".as_bytes().to_vec();
            return Ok(());
        });
    }
}

struct WorkError;
impl Work for WorkError {
    fn path(&self) -> String {
        return "/error".to_string();
    }

    fn do_work<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let _ = context;
            Err("100%触发异常.".to_string())?;
            return Ok(());
        });
    }
}

use serde::Serialize;

#[derive(Serialize)]
struct T915 {
    a: String,
    b: usize,
}

struct WorkJson;
impl Work for WorkJson {
    fn path(&self) -> String {
        return "/json".to_string();
    }

    fn do_work<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
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
        });
    }
}

// Function.

#[tokio::test]
pub async fn test_first_work() {
    println!("");
    // iceyee_logger::init(iceyee_logger::Level::Debug, None, None).await;
    let stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();
    let a001 = tokio::task::spawn(async move {
        let stop = stop_clone;
        let proxy = NoProxy::new().wrap();
        HttpClient::new()
            .set_verbose(true)
            .set_url::<&str>("http://localhost:10877/first_work")
            .unwrap()
            .send(Some(proxy.clone()))
            .await
            .expect("test_http_server.rs 857");
        HttpClient::new()
            .set_verbose(true)
            .set_url::<&str>("http://localhost:10877/error")
            .unwrap()
            .set_header("Authorization", "Basic aWNleWVlOjc0NTkxODcw")
            .send(Some(proxy.clone()))
            .await
            .expect("test_http_server.rs 945");
        HttpClient::new()
            .set_verbose(true)
            .set_url::<&str>("http://localhost:10877/json")
            .unwrap()
            .set_header("Authorization", "Basic aWNleWVlOjc0NTkxODcw")
            .send(Some(proxy.clone()))
            .await
            .expect("test_http_server.rs 593");
        stop.store(true, SeqCst);
    });
    HttpServer::new()
        .set_root("/home/ljq")
        .add_filter_before_work(FilterCORS::new().allow_origin("*").wrap())
        .add_filter_before_work(FilterBasicAuth::new("iceyee", "74591870").wrap())
        .add_work(WorkFirst.wrap())
        .add_work(WorkRedirect.wrap())
        .add_work(WorkError.wrap())
        .add_work(WorkJson.wrap())
        .test("localhost:10877", stop)
        .await
        .expect("test_http_server.rs 129");
    a001.await.unwrap();

    return;
}
