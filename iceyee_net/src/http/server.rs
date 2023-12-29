// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 服务端接口.
//!
//! ```
//! Filter    do_filter(), 返回值,
//!   |         true表示通行,
//!   v         false表示拦截, 中止.
//! Work
//!   |
//!   v
//! Filter
//! ```

pub mod component;

// Use.

pub use crate::http::Request;
pub use crate::http::Response;
pub use crate::http::Status;

use crate::http::server::component::FileRouter;
use crate::http::server::component::FilterHost;
use async_compression::tokio::bufread::GzipEncoder;
use iceyee_encoder::HexEncoder;
use iceyee_random::Random;
use serde::Serialize;
use std::collections::HashMap;
use std::future::Future;
use std::io::Error as StdIoError;
use std::io::ErrorKind as StdIoErrorKind;
use std::net::IpAddr;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener as TokioTcpListener;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::net::ToSocketAddrs;
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::RwLock as TokioRwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

// Enum.

// Trait.

/// 过滤器.
///
/// # Use
/// ```
/// use iceyee_net::http::server::Context;
/// use iceyee_net::http::server::Filter;
/// use iceyee_net::http::server::R;
/// use iceyee_net::http::server::ResponseObject;
/// use std::future::Future;
/// use std::pin::Pin;
/// ```
/// - @see [Context]
/// - @see [HttpServer]
/// - @see [Work]
pub trait Filter: Send + Sync {
    /// 返回值决定是否执行do_filter(), 默认true.
    ///
    /// # Example
    /// ```
    /// fn rule<'a, 'b>(
    ///     &'a self,
    ///     context: &'b mut Context,
    /// ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    /// where
    ///     'a: 'b,
    /// {
    ///     return Box::pin(async {
    ///         let _ = context;
    ///         return true;
    ///     });
    /// }
    /// ```
    fn rule<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let _ = context;
            return true;
        });
    }

    /// 干活, 返回值决定是否放行.
    ///
    /// # Example
    /// ```
    /// fn do_filter<'a, 'b>(
    ///     &'a self,
    ///     context: &'b mut Context,
    /// ) -> Pin<Box<dyn Future<Output = Result<bool, String>> + Send + 'b>>
    /// where
    ///     'a: 'b,
    /// {
    ///     return Box::pin(async {
    ///         let _ = context;
    ///         println!("hello world.");
    ///         return Ok(true);
    ///     });
    /// }
    /// ```
    fn do_filter<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = Result<bool, String>> + Send + 'b>>
    where
        'a: 'b;

    /// 当执行do_filter()时抛出异常, 则执行on_error(), 返回值同do_filter().
    ///
    /// # Example
    /// ```
    /// fn on_error<'a, 'b>(
    ///     &'a self,
    ///     context: &'b mut Context,
    /// ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    /// where
    ///     'a: 'b,
    /// {
    ///     return Box::pin(async {
    ///         let e_message: String = context
    ///             .e_message
    ///             .as_ref()
    ///             .expect("Context::e_message is none")
    ///             .clone();
    ///         let a001: ResponseObject<bool> = ResponseObject {
    ///             success: false,
    ///             message: e_message.clone(),
    ///             data: false,
    ///         };
    ///         R::write_json(&mut context.response, &a001);
    ///         iceyee_logger::error_2(context.id.to_string(), e_message.clone()).await;
    ///         return false;
    ///     });
    /// }
    /// ```
    fn on_error<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let e_message: String = context
                .e_message
                .as_ref()
                .expect("Context::e_message is none")
                .clone();
            let a001: ResponseObject<bool> = ResponseObject {
                success: false,
                message: e_message.clone(),
                data: false,
            };
            R::write_json(&mut context.response, &a001);
            iceyee_logger::error_2(context.id.to_string(), e_message.clone()).await;
            return false;
        });
    }

    fn wrap(self) -> Arc<dyn Filter>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

/// 干活的.
///
/// # Use
/// ```
/// use iceyee_net::http::server::Context;
/// use iceyee_net::http::server::R;
/// use iceyee_net::http::server::ResponseObject;
/// use iceyee_net::http::server::Work;
/// use std::future::Future;
/// use std::pin::Pin;
/// ```
/// - @see [Context]
/// - @see [Filter]
/// - @see [HttpServer]
pub trait Work: Send + Sync {
    /// 请求方法, 默认'GET'.
    ///
    /// # Example
    /// ```
    /// fn method(&self) -> String {
    ///     return "GET".to_string();
    /// }
    /// ```
    fn method(&self) -> String {
        return "GET".to_string();
    }

    /// 路径.
    ///
    /// # Example
    /// ```
    /// fn path(&self) -> String {
    ///     return "/hello".to_string();
    /// }
    /// ```
    fn path(&self) -> String;

    /// 干活.
    ///
    /// # Example
    /// ```
    /// fn do_work<'a, 'b>(
    ///     &'a self,
    ///     context: &'b mut Context,
    /// ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'b>>
    /// where
    ///     'a: 'b,
    /// {
    ///     return Box::pin(async {
    ///         println!("hello world.");
    ///         return;
    ///     });
    /// }
    /// ```
    fn do_work<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'b>>
    where
        'a: 'b;

    /// do_work()抛出异常的时候执行on_error().
    ///
    /// # Example
    /// ```
    /// fn on_error<'a, 'b>(
    ///     &'a self,
    ///     context: &'b mut Context,
    /// ) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    /// where
    ///     'a: 'b,
    /// {
    ///     return Box::pin(async {
    ///         let e_message: String = context
    ///             .e_message
    ///             .as_ref()
    ///             .expect("Context::e_message is none")
    ///             .clone();
    ///         let a001: ResponseObject<bool> = ResponseObject {
    ///             success: false,
    ///             message: e_message.clone(),
    ///             data: false,
    ///         };
    ///         R::write_json(&mut context.response, &a001);
    ///         iceyee_logger::error_2(context.id.to_string(), e_message.clone()).await;
    ///         return;
    ///     });
    /// }
    /// ```
    fn on_error<'a, 'b>(
        &'a self,
        context: &'b mut Context,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
    where
        'a: 'b,
    {
        return Box::pin(async {
            let e_message: String = context
                .e_message
                .as_ref()
                .expect("Context::e_message is none")
                .clone();
            let a001: ResponseObject<bool> = ResponseObject {
                success: false,
                message: e_message.clone(),
                data: false,
            };
            R::write_json(&mut context.response, &a001);
            iceyee_logger::error_2(context.id.to_string(), e_message.clone()).await;
            return;
        });
    }

    fn wrap(self) -> Arc<dyn Work>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

// Struct.

/// 一般用于[Response]返回的json对象.
#[derive(Clone, Debug, Serialize)]
pub struct ResponseObject<T>
where
    T: Serialize,
{
    pub success: bool,
    pub message: String,
    pub data: T,
}

impl<T> std::default::Default for ResponseObject<T>
where
    T: Serialize + Default,
{
    fn default() -> Self {
        return ResponseObject {
            success: true,
            message: "OK".to_string(),
            data: Default::default(),
        };
    }
}

/// 服务器分配给请求的一个id.
#[derive(Clone, Debug)]
pub struct Id {
    id: usize,
    counter: usize,
}

impl Id {
    fn new() -> Id {
        return Id {
            id: Random::next(),
            counter: 0,
        };
    }

    fn add(&mut self) -> &mut Self {
        self.counter += 1;
        return self;
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "{} {}", self.id, self.counter);
    }
}

/// cookies.
#[derive(Clone, Debug)]
pub struct Cookies {
    this: HashMap<String, String>,
}

impl std::ops::Deref for Cookies {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        return &self.this;
    }
}

impl std::ops::DerefMut for Cookies {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.this;
    }
}

impl Cookies {
    pub fn new() -> Self {
        return Cookies {
            this: HashMap::new(),
        };
    }
}

/// 会话, 以键值的方式存储用户数据, 内部包含有读写锁.
#[derive(Clone)]
pub struct Session {
    this: Arc<TokioRwLock<HashMap<String, String>>>,
}

impl Session {
    pub fn new() -> Session {
        return Session {
            this: Arc::new(TokioRwLock::new(HashMap::new())),
        };
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, HashMap<String, String>> {
        return self.this.read().await;
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, HashMap<String, String>> {
        return self.this.write().await;
    }
}

/// 上下文.
///
/// @see [Filter]
///
/// @see [Work]
pub struct Context {
    pub id: Id,
    pub request: Request,
    pub response: Response,
    pub cookies: Cookies,
    pub session: Session,
    pub global_session: Session,
    pub e_message: Option<String>,
}

/// 一些针对[Response]的接口.
pub struct R;

impl R {
    pub fn write_ok(response: &mut Response) {
        Self::write_status(response, Status::OK(None));
        return;
    }

    pub fn write_status(response: &mut Response, status: Status) {
        response.status_code = status.clone().into();
        response.status = status.default_string();
        response.body = status.clone().to_string().as_bytes().to_vec();
        response
            .header
            .insert("Content-Type".to_string(), vec!["text/plain".to_string()]);
        match status {
            Status::Created(link)
            | Status::MovedPermanently(link)
            | Status::MovedTemporarily(link) => {
                response.header.insert("Location".to_string(), vec![link]);
            }
            _ => {}
        }
        return;
    }

    pub fn write_json<O>(response: &mut Response, object: O)
    where
        O: serde::ser::Serialize,
    {
        response.body = match serde_json::to_string(&object) {
            Ok(s) => s.as_bytes().to_vec(),
            Err(_) => Vec::new(),
        };
        response.header.insert(
            "Content-Type".to_string(),
            vec!["application/json".to_string()],
        );
        return;
    }
}

/// Http服务端.
///
/// - @see [Context]
/// - @see [Filter]
/// - @see [Work]
#[derive(Clone)]
pub struct HttpServer {
    stop_server: Arc<AtomicBool>,
    session_timeout: usize,
    connection_timeout: usize,
    sessions: Arc<TokioMutex<HashMap<String, Session>>>,
    global_session: Session,
    filters_before_work: Vec<Arc<dyn Filter>>,
    works: HashMap<String, Vec<(String, Arc<dyn Work>)>>,
    filters_after_work: Vec<Arc<dyn Filter>>,
    filter_host: FilterHost,
    file_router: Option<FileRouter>,
}

unsafe impl Send for HttpServer {}
unsafe impl Sync for HttpServer {}

impl HttpServer {
    pub fn new() -> Self {
        let server = HttpServer {
            stop_server: Arc::new(AtomicBool::new(false)),
            session_timeout: 1_000 * 60 * 60,
            connection_timeout: 1_000 * 60,
            sessions: Arc::new(TokioMutex::new(HashMap::new())),
            global_session: Session::new(),
            filters_before_work: Vec::new(),
            works: HashMap::new(),
            filters_after_work: Vec::new(),
            filter_host: FilterHost::new(),
            file_router: None,
        };
        return server;
    }

    /// 设置会话超时, 会话不活跃超过一定时间会被释放, 单位:分钟, 默认一小时.
    pub fn set_session_timeout(mut self, t: usize) -> Self {
        self.session_timeout = 1_000 * 60 * t;
        return self;
    }

    /// 设置连接超时, 单位:毫秒.
    pub fn set_connection_timeout(mut self, t: usize) -> Self {
        self.connection_timeout = t;
        return self;
    }

    /// 静态文件根目录.
    ///
    /// # Panic
    ///
    /// root是'/'.
    pub fn set_root(mut self, root: &str) -> Self {
        let mut root = root.to_string();
        while 1 < root.len() && root.ends_with("/") {
            root.pop();
        }
        if root == "/" {
            panic!("无效的根目录.");
        }
        self.file_router = Some(FileRouter::new(&root));
        return self;
    }

    /// 添加Host白名单.
    ///
    /// 以'.'开头则表示匹配二级域名, 否则表示完全匹配.
    ///
    /// 如, '.iceyee.cn'表示通配'*.iceyee.cn'.
    ///
    /// 'iceyee.cn'则需要完全匹配iceyee.cn.
    pub fn add_host(mut self, host: &str) -> Self {
        if host.starts_with(".") {
            self.filter_host.add_usual(host);
        } else {
            self.filter_host.add_full(host);
        }
        return self;
    }

    pub fn add_filter_before_work(mut self, filter: Arc<dyn Filter>) -> Self {
        self.filters_before_work.push(filter);
        return self;
    }

    pub fn add_filter_after_work(mut self, filter: Arc<dyn Filter>) -> Self {
        self.filters_after_work.push(filter);
        return self;
    }

    pub fn add_work(mut self, work: Arc<dyn Work>) -> Self {
        let method: String = work.method().to_string();
        let path: String = work.path().to_string();
        if !self.works.contains_key(&path) {
            self.works.insert(path.clone(), Vec::new());
        }
        self.works.get_mut(&path).unwrap().push((method, work));
        return self;
    }

    /// 启动服务器.
    pub async fn start<A>(mut self, address_and_port: A) -> Result<(), StdIoError>
    where
        A: ToSocketAddrs,
    {
        self.filters_before_work
            .push(self.filter_host.clone().wrap());
        let listener: TokioTcpListener = TokioTcpListener::bind(address_and_port).await?;
        let address = listener.local_addr()?;
        let server = Arc::new(self);
        tokio::task::spawn(Self::clean_expired_session(server.clone()));
        let server_clone = server.clone();
        let exit_counter: Arc<usize> = Arc::new(0);
        let exit_counter_clone = exit_counter.clone();
        let listener_future = tokio::task::spawn(async move {
            while !server_clone.stop_server.load(SeqCst) {
                let server = server_clone.clone();
                let exit_counter = exit_counter_clone.clone();
                match listener.accept().await {
                    Ok((tcp, address)) => {
                        if server.stop_server.load(SeqCst) {
                            break;
                        }
                        tokio::task::spawn(async move {
                            #[allow(unused_variables)]
                            let exit_counter = exit_counter;
                            let mut id: Id = Id::new();
                            let ip = match address.ip() {
                                IpAddr::V4(ip) => Self::ipv4_to_string(ip),
                                IpAddr::V6(ip) => Self::ipv6_to_string(ip),
                            };
                            let message: String = format!("建立连接, {ip}, {id}.");
                            iceyee_logger::debug(&message).await;
                            let mut tcp = Arc::new(tcp);
                            loop {
                                id.add();
                                let server = server.clone();
                                let tcp = tcp.clone();
                                match Self::process_tcp(server, tcp, id.clone()).await {
                                    Ok(true) => continue,
                                    Ok(false) => {
                                        let message: String = format!("正常断开连接, {ip}, {id}.");
                                        iceyee_logger::debug(&message).await;
                                        break;
                                    }
                                    Err(e) => {
                                        match e.kind() {
                                            StdIoErrorKind::TimedOut => {
                                                let message: String =
                                                    format!("超时断开连接, {ip}, {id}.");
                                                iceyee_logger::debug(&message).await;
                                            }
                                            _ => {
                                                let message: String =
                                                    format!("异常断开连接, {ip}, {id}.");
                                                iceyee_logger::debug(&message).await;
                                                iceyee_logger::error(e.to_string().as_str()).await;
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                            // 关闭连接.
                            {
                                match Arc::get_mut(&mut tcp).unwrap().shutdown().await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        iceyee_logger::error(e.to_string().as_str()).await;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        iceyee_logger::error(e.to_string().as_str()).await;
                        break;
                    }
                }
            }
        });
        println!("---- 输入[Ctrl+C]停止. ----");
        tokio::signal::ctrl_c().await.unwrap();
        println!("---- 退出服务端. ----");
        server.stop_server.store(true, SeqCst);
        TokioTcpStream::connect(address).await?.shutdown().await?;
        listener_future.await.unwrap();
        println!("---- 等待所有TCP处理完毕. ----");
        while 1 < Arc::strong_count(&exit_counter) {
            iceyee_time::sleep(200).await;
        }
        println!("---- DONE. ----");
        return Ok(());
    }

    /// 逻辑同start(), 但是stop由外部的信号决定.
    pub async fn test<A>(
        mut self,
        address_and_port: A,
        stop: Arc<AtomicBool>,
    ) -> Result<(), StdIoError>
    where
        A: ToSocketAddrs,
    {
        self.connection_timeout = 2_000;
        self.stop_server = stop;
        self.filters_before_work
            .push(self.filter_host.clone().wrap());
        let listener: TokioTcpListener = TokioTcpListener::bind(address_and_port).await?;
        let address = listener.local_addr()?;
        let server = Arc::new(self);
        tokio::task::spawn(Self::clean_expired_session(server.clone()));
        let server_clone = server.clone();
        let exit_counter: Arc<usize> = Arc::new(0);
        let exit_counter_clone = exit_counter.clone();
        let listener_future = tokio::task::spawn(async move {
            while !server_clone.stop_server.load(SeqCst) {
                let server = server_clone.clone();
                let exit_counter = exit_counter_clone.clone();
                match listener.accept().await {
                    Ok((tcp, address)) => {
                        if server.stop_server.load(SeqCst) {
                            break;
                        }
                        tokio::task::spawn(async move {
                            #[allow(unused_variables)]
                            let exit_counter = exit_counter;
                            let mut id: Id = Id::new();
                            let ip = match address.ip() {
                                IpAddr::V4(ip) => Self::ipv4_to_string(ip),
                                IpAddr::V6(ip) => Self::ipv6_to_string(ip),
                            };
                            let message: String = format!("建立连接, {ip}, {id}.");
                            iceyee_logger::debug(&message).await;
                            let mut tcp = Arc::new(tcp);
                            loop {
                                id.add();
                                let server = server.clone();
                                let tcp = tcp.clone();
                                match Self::process_tcp(server, tcp, id.clone()).await {
                                    Ok(true) => continue,
                                    Ok(false) => {
                                        let message: String = format!("正常断开连接, {ip}, {id}.");
                                        iceyee_logger::debug(&message).await;
                                        break;
                                    }
                                    Err(e) => {
                                        match e.kind() {
                                            StdIoErrorKind::TimedOut => {
                                                let message: String =
                                                    format!("超时断开连接, {ip}, {id}.");
                                                iceyee_logger::debug(&message).await;
                                            }
                                            _ => {
                                                let message: String =
                                                    format!("异常断开连接, {ip}, {id}.");
                                                iceyee_logger::debug(&message).await;
                                                iceyee_logger::error(e.to_string().as_str()).await;
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                            // 关闭连接.
                            {
                                match Arc::get_mut(&mut tcp).unwrap().shutdown().await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        iceyee_logger::error(e.to_string().as_str()).await;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        iceyee_logger::error(e.to_string().as_str()).await;
                        break;
                    }
                }
            }
        });
        while !server.stop_server.load(SeqCst) {
            iceyee_time::sleep(200).await;
        }
        println!("---- 退出服务端. ----");
        server.stop_server.store(true, SeqCst);
        TokioTcpStream::connect(address).await?.shutdown().await?;
        listener_future.await.unwrap();
        println!("---- 等待所有TCP处理完毕. ----");
        while 1 < Arc::strong_count(&exit_counter) {
            iceyee_time::sleep(200).await;
        }
        println!("---- DONE. ----");
        return Ok(());
    }

    async fn clean_expired_session(server: Arc<HttpServer>) {
        while !server.stop_server.load(SeqCst) {
            iceyee_time::sleep(1_000).await;
            let sleep = tokio::task::spawn(iceyee_time::sleep(1_000 * 60 * 60));
            let now: i64 = iceyee_time::now();
            let mut sessions = server.sessions.lock().await;
            let mut expired_session_id: Vec<String> = Vec::new();
            for (id, session) in sessions.iter() {
                let expired_time: i64 = session
                    .read()
                    .await
                    .get("expired_time")
                    .expect("iceyee_net/http/server.rs 937")
                    .parse::<i64>()
                    .unwrap_or(0);
                if expired_time < now {
                    expired_session_id.push(id.clone());
                }
            }
            for id in &expired_session_id {
                sessions.remove(id);
            }
            let message: String = format!(
                "清理不活跃会话{}个, 剩余会话{}个.",
                expired_session_id.len(),
                sessions.len()
            );
            iceyee_logger::info(&message).await;
            drop(sessions);
            while !server.stop_server.load(SeqCst) && !sleep.is_finished() {
                iceyee_time::sleep(200).await;
            }
        }
        return;
    }

    fn parse_cookie(s: &str) -> Cookies {
        let mut cookies: Cookies = Cookies::new();
        for x in s.split(";") {
            if x.contains("=") {
                let mut y = x.splitn(2, "=");
                let key: String = y.next().unwrap().trim().to_string();
                let value: String = y.next().unwrap().trim().to_string();
                cookies.insert(key, value);
            }
        }
        return cookies;
    }

    fn new_session_id() -> String {
        let a001 = HexEncoder::encode_number(Random::next() as u64);
        let a002 = HexEncoder::encode_number(Random::next() as u64);
        return a001 + &a002;
    }

    fn ipv4_to_string(ip: std::net::Ipv4Addr) -> String {
        let ip = ip.octets();
        return format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
    }

    fn ipv6_to_string(ip: std::net::Ipv6Addr) -> String {
        let ip = ip.octets();
        let ip: String = HexEncoder::encode(ip.to_vec().as_slice());
        let ip: &[u8] = ip.as_bytes();
        let mut buffer: Vec<u8> = Vec::new();
        for x in 0..32 {
            buffer.push(ip[x]);
            if x % 4 == 3 {
                buffer.push(b'.');
            }
        }
        buffer.pop();
        return String::from_utf8(buffer).unwrap();
    }

    async fn process_tcp(
        server: Arc<HttpServer>,
        mut tcp: Arc<TokioTcpStream>,
        id: Id,
    ) -> Result<bool, StdIoError> {
        let mut close = false;
        let request: Request = Request::read_from(
            unsafe { Arc::get_mut_unchecked(&mut tcp) },
            Some(server.connection_timeout),
        )
        .await?;
        let message = format!(">>> {}, {}, {}", id, request.method, request.path);
        iceyee_logger::info(&message).await;
        let mut message = format!("\n{id}\n>>>\n{}", request.to_string());
        match String::from_utf8(request.body.clone()) {
            Ok(s) => message.push_str(&s),
            Err(_) => message.push_str("[not utf-8.]"),
        }
        iceyee_logger::debug(&message).await;
        let mut response: Response = Response::default();
        R::write_ok(&mut response);
        response
            .header
            .insert("Content-Type".to_string(), vec!["text/plain".to_string()]);
        let mut cookies: Cookies = Cookies::new();
        let session_id: String = if request.header.contains_key("Cookie") {
            let s: &String = request.header.get("Cookie").unwrap();
            cookies = Self::parse_cookie(s);
            if cookies.contains_key("session_id") {
                cookies.get("session_id").unwrap().to_string()
            } else {
                Self::new_session_id()
            }
        } else {
            Self::new_session_id()
        };
        cookies.insert("session_id".to_string(), session_id.clone());
        response.header.insert(
            "Set-Cookie".to_string(),
            vec!["session_id=".to_string() + &session_id + ";"],
        );
        let mut sessions = server.sessions.lock().await;
        let session: Session = if sessions.contains_key(&session_id) {
            sessions.get(&session_id).unwrap().clone()
        } else {
            sessions.insert(session_id.clone(), Session::new());
            sessions.get(&session_id).unwrap().clone()
        };
        drop(sessions);
        let expired_time: i64 = iceyee_time::now() + server.session_timeout as i64;
        session
            .write()
            .await
            .insert("expired_time".to_string(), expired_time.to_string());
        let mut context: Context = Context {
            id: id.clone(),
            request: request,
            response: response,
            cookies: cookies,
            session: session,
            global_session: server.global_session.clone(),
            e_message: None,
        };
        let mut stop = false;
        for filter in &server.filters_before_work {
            if filter.rule(&mut context).await {
                match filter.do_filter(&mut context).await {
                    Ok(true) => continue,
                    Ok(false) => {
                        stop = true;
                        break;
                    }
                    Err(e) => {
                        context.e_message = Some(e);
                        if !filter.on_error(&mut context).await {
                            stop = true;
                            break;
                        }
                    }
                }
            }
        }
        let mut done = false;
        if !stop {
            if server.works.contains_key(&context.request.path) {
                let works = server.works.get(&context.request.path).unwrap();
                for (method, work) in works {
                    if method == &context.request.method {
                        done = true;
                        match work.do_work(&mut context).await {
                            Ok(()) => {}
                            Err(e) => {
                                context.e_message = Some(e);
                                work.on_error(&mut context).await;
                            }
                        };
                    }
                }
            }
        }
        if !stop && !done && server.file_router.is_some() {
            // Work不匹配.
            let file_router = server.file_router.as_ref().unwrap();
            if file_router.rule(&mut context).await {
                match file_router.do_filter(&mut context).await {
                    Ok(_) => {}
                    Err(e) => {
                        context.e_message = Some(e);
                        file_router.on_error(&mut context).await;
                    }
                }
            }
        }
        if !stop {
            for filter in &server.filters_after_work {
                if filter.rule(&mut context).await {
                    match filter.do_filter(&mut context).await {
                        Ok(true) => continue,
                        Ok(false) => break,
                        Err(e) => {
                            context.e_message = Some(e);
                            if !filter.on_error(&mut context).await {
                                break;
                            }
                        }
                    }
                }
            }
        }
        // Connection: close.
        if context.request.header.contains_key("Connection") {
            if context.request.header.get("Connection").unwrap() == "close" {
                context
                    .response
                    .header
                    .insert("Connection".to_string(), vec!["close".to_string()]);
                close = true;
            } else {
                context
                    .response
                    .header
                    .insert("Connection".to_string(), vec!["keep-alive".to_string()]);
            }
        }
        // Content-Length.
        // chunk.
        match context
            .request
            .header
            .get("Accept-Encoding")
            .map(|a| a.trim().contains("gzip") || a.trim().contains("*"))
        {
            Some(false) | None => {
                context.response.header.insert(
                    "Content-Length".to_string(),
                    vec![context.response.body.len().to_string()],
                );
            }
            Some(true) => {
                context
                    .response
                    .header
                    .insert("Content-Encoding".to_string(), vec!["gzip".to_string()]);
                context
                    .response
                    .header
                    .insert("Transfer-Encoding".to_string(), vec!["chunked".to_string()]);
                let mut gzip = GzipEncoder::new(context.response.body.as_slice());
                let mut buffer = Vec::new();
                gzip.read_to_end(&mut buffer).await?;
                drop(gzip);
                context.response.body.clear();
                let length: String = HexEncoder::encode_number(buffer.len() as u64);
                context
                    .response
                    .body
                    .append(&mut length.as_bytes().to_vec());
                context.response.body.push(b'\r');
                context.response.body.push(b'\n');
                context.response.body.append(&mut buffer.to_vec());
                context.response.body.push(b'\r');
                context.response.body.push(b'\n');
                context.response.body.push(b'0');
                context.response.body.push(b'\r');
                context.response.body.push(b'\n');
                context.response.body.push(b'\r');
                context.response.body.push(b'\n');
            }
        }
        // 输出.
        let message = format!(
            "<<< {}, {}, {}",
            id, context.response.status_code, context.response.status
        );
        iceyee_logger::info(&message).await;
        let message = format!("\n{id}\n<<<\n{}", context.response.to_string());
        iceyee_logger::debug(&message).await;
        unsafe { Arc::get_mut_unchecked(&mut tcp) }
            .write_all(context.response.to_string().as_bytes())
            .await?;
        unsafe { Arc::get_mut_unchecked(&mut tcp) }
            .write_all(context.response.body.as_slice())
            .await?;
        return Ok(!close);
    }
}

// Function.
