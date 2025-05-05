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
use std::collections::BTreeMap;
use std::future::Future;
use std::net::IpAddr;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener as TokioTcpListener;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::RwLock as TokioRwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;
use tokio::sync::Semaphore;

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
/// - @see [R]
/// - @see [Work]
pub trait Filter: Send + Sync {
    /// 返回值决定是否执行do_filter(), 默认true.
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
    ///             .expect("Context::e_message None")
    ///             .clone();
    ///         let a001: ResponseObject<bool> = ResponseObject {
    ///             success: false,
    ///             message: e_message.clone(),
    ///             data: false,
    ///         };
    ///         R::write_json(&mut context.response, &a001);
    ///         iceyee_logger::error!(context.id, e_message);
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
                .expect("Context::e_message None")
                .clone();
            let a001: ResponseObject<bool> = ResponseObject {
                success: false,
                message: e_message.clone(),
                data: false,
            };
            R::write_json(&mut context.response, &a001);
            iceyee_logger::error!(context.id, e_message);
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
/// - @see [R]
pub trait Work: Send + Sync {
    /// 请求方法, 默认'GET'.
    fn method(&self) -> String {
        return "GET".to_string();
    }

    /// 路径.
    fn path(&self) -> String;

    /// 干活.
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
    ///             .expect("Context::e_message None")
    ///             .clone();
    ///         let a001: ResponseObject<bool> = ResponseObject {
    ///             success: false,
    ///             message: e_message.clone(),
    ///             data: false,
    ///         };
    ///         R::write_json(&mut context.response, &a001);
    ///         iceyee_logger::error!(self.path(), context.id, e_message);
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
                .expect("Context::e_message None")
                .clone();
            let a001: ResponseObject<bool> = ResponseObject {
                success: false,
                message: e_message.clone(),
                data: false,
            };
            R::write_json(&mut context.response, &a001);
            iceyee_logger::error!(self.path(), context.id, e_message);
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
    id: u64,
    counter: u64,
}

impl Id {
    fn new() -> Id {
        return Id {
            id: Random::next(),
            counter: 0,
        };
    }

    fn add(&mut self) {
        self.counter += 1;
        return;
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "{} {}", self.id, self.counter);
    }
}

/// cookies.
#[derive(Clone, Debug, Default)]
pub struct Cookies(pub BTreeMap<String, String>);

impl std::ops::Deref for Cookies {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl std::ops::DerefMut for Cookies {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl std::str::FromStr for Cookies {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cookies: Cookies = Cookies::default();
        for x in s.split(";") {
            if x.contains("=") {
                let mut y = x.splitn(2, "=");
                let key: String = y.next().unwrap().trim().to_string();
                let value: String = y.next().unwrap().trim().to_string();
                cookies.insert(key, value);
            }
        }
        return Ok(cookies);
    }
}

impl Cookies {
    pub fn new() -> Self {
        return Cookies(BTreeMap::new());
    }
}

/// 会话, 以键值的方式存储用户数据, 内部包含有读写锁.
#[derive(Clone)]
pub struct Session(pub Arc<TokioRwLock<BTreeMap<String, String>>>);

impl Session {
    pub fn new() -> Session {
        return Session(Arc::new(TokioRwLock::new(BTreeMap::new())));
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, BTreeMap<String, String>> {
        return self.0.read().await;
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, BTreeMap<String, String>> {
        return self.0.write().await;
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
    connection_timeout: u64,
    session_timeout: u64,
    sessions: Arc<TokioMutex<BTreeMap<String, Session>>>,
    global_session: Session,
    filters_before_work: Vec<Arc<dyn Filter>>,
    works: BTreeMap<String, Arc<dyn Work>>,
    filters_after_work: Vec<Arc<dyn Filter>>,
    filter_host: FilterHost,
    file_router: Option<FileRouter>,
}

// unsafe impl Send for HttpServer {}
// unsafe impl Sync for HttpServer {}

impl HttpServer {
    pub fn new() -> Self {
        let server = HttpServer {
            connection_timeout: 1_000 * 60,
            session_timeout: 1_000 * 60 * 60,
            sessions: Arc::new(TokioMutex::new(BTreeMap::new())),
            global_session: Session::new(),
            filters_before_work: Vec::new(),
            works: BTreeMap::new(),
            filters_after_work: Vec::new(),
            filter_host: FilterHost::new(),
            file_router: None,
        };
        return server;
    }

    /// 设置连接超时, 单位:毫秒.
    pub fn set_connection_timeout(mut self, t: u64) -> Self {
        self.connection_timeout = t;
        return self;
    }

    /// 设置会话超时, 会话不活跃超过一定时间会被释放, 单位:分钟, 默认一小时.
    pub fn set_session_timeout(mut self, t: u64) -> Self {
        self.session_timeout = 1_000 * 60 * t;
        return self;
    }

    /// 静态文件根目录.
    ///
    /// # Panic
    ///
    /// 无效根目录.
    pub fn set_root(mut self, root: &str) -> Self {
        let mut root = root.to_string();
        while root.ends_with("/") {
            root.pop();
        }
        if root.len() == 0 {
            panic!("无效根目录.");
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
        let key: String = method.clone() + " " + &path;
        self.works.insert(key, work);
        return self;
    }
}

impl HttpServer {
    /// 启动服务器.
    ///
    /// - @return 改变状态, 使得服务器停止.
    pub async fn test(mut self, address: &str, port: u16) -> Result<Arc<AtomicBool>, String> {
        iceyee_logger::warn!("HTTPSERVER START AT", address, port);
        self.filters_before_work
            .push(self.filter_host.clone().wrap());
        let listener: TokioTcpListener = TokioTcpListener::bind((address, port))
            .await
            .map_err(|e| iceyee_error::a!(e))?;
        let address = listener.local_addr().map_err(|e| iceyee_error::a!(e))?;
        let server = Arc::new(self);
        let _server = server.clone();
        let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(0));
        let _semaphore = semaphore.clone();
        let stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let _stop = stop.clone();
        tokio::task::spawn(async move {
            _semaphore.add_permits(1);
            let mut clean_t: i64 = iceyee_time::now();
            while !_stop.load(SeqCst) {
                let server = _server.clone();
                match listener.accept().await {
                    Ok((mut tcp, address)) => {
                        if 1_000 * 60 * 60 < iceyee_time::now() - clean_t {
                            Self::clean_expired_session(server.clone()).await;
                            clean_t = iceyee_time::now();
                        }
                        if _stop.load(SeqCst) {
                            break;
                        }
                        let stop = _stop.clone();
                        tokio::task::spawn(async move {
                            let mut id: Id = Id::new();
                            let ip: String = match address.ip() {
                                IpAddr::V4(ip) => ipv4_to_string(ip),
                                IpAddr::V6(ip) => ipv6_to_string(ip),
                            };
                            iceyee_logger::debug!("建立连接", ip, id);
                            while !stop.load(SeqCst) {
                                id.add();
                                let server = server.clone();
                                let request: Request = match Request::read_from(
                                    &mut tcp,
                                    Some(server.connection_timeout),
                                )
                                .await
                                .map_err(|e| iceyee_error::b!(e, "read request"))
                                {
                                    Ok(r) => r,
                                    Err(e) => {
                                        if e.contains("TimedOut") {
                                            iceyee_logger::debug!("超时异常断开连接", ip, id);
                                            break;
                                        } else {
                                            iceyee_logger::debug!("输入异常断开连接", ip, id);
                                            iceyee_logger::error!(e);
                                            break;
                                        }
                                    }
                                };
                                let mut context: Context =
                                    Self::build_context(server.clone(), request, id.clone()).await;
                                let close: bool = Self::process(server, &mut context).await;
                                if let Err(e) = Self::write_to_tcp(&mut tcp, &context)
                                    .await
                                    .map_err(|e| iceyee_error::b!(e, "write to tcp"))
                                {
                                    iceyee_logger::debug!("输出异常断开连接", ip, id);
                                    iceyee_logger::error!(e);
                                    break;
                                }
                                if close {
                                    iceyee_logger::debug!("正常断开连接", ip, id);
                                    break;
                                }
                            }
                            // 关闭连接.
                            {
                                if let Err(e) = tcp.shutdown().await {
                                    iceyee_logger::error!(e);
                                }
                            }
                        });
                    }
                    Err(e) => {
                        iceyee_logger::error!("监听tcp异常", e);
                        break;
                    }
                }
            }
            _semaphore.add_permits(1);
        });
        semaphore
            .acquire()
            .await
            .expect("Semaphore::acquire()")
            .forget();
        let _stop = stop.clone();
        tokio::task::spawn(async move {
            while !_stop.load(SeqCst) {
                iceyee_time::sleep(100).await;
            }
            if let Ok(mut tcp) = TokioTcpStream::connect(address).await {
                let _ = tcp.shutdown().await;
            }
        });
        return Ok(stop);
    }

    /// 启动服务器.
    pub async fn start(self, address: &str, port: u16) -> Result<(), String> {
        let mut stop = Self::test(self, address, port)
            .await
            .map_err(|e| iceyee_error::b!(e, ""))?;
        println!("---- 输入[Ctrl+C]停止. ----");
        tokio::signal::ctrl_c().await.expect("");
        println!("---- 退出服务端. ----");
        stop.store(true, SeqCst);
        println!("---- 等待所有TCP处理完毕. ----");
        for _ in 0..600 {
            if Arc::get_mut(&mut stop).is_none() {
                iceyee_time::sleep(100).await;
            }
        }
        println!("---- DONE. ----");
        return Ok(());
    }

    async fn build_context(server: Arc<HttpServer>, request: Request, id: Id) -> Context {
        let mut response: Response = Response::default();
        R::write_ok(&mut response);
        let (session_id, mut cookies) = if request.header.contains_key("Cookie") {
            let cookies: Cookies = request
                .header
                .get("Cookie")
                .expect("NEVER")
                .parse::<Cookies>()
                .expect("NEVER");
            if cookies.contains_key("session_id") {
                (cookies.get("session_id").expect("NEVER").clone(), cookies)
            } else {
                (new_session_id(), cookies)
            }
        } else {
            (new_session_id(), Cookies::new())
        };
        cookies.insert("session_id".to_string(), session_id.clone());
        response.header.insert(
            "Set-Cookie".to_string(),
            vec!["session_id=".to_string() + &session_id + ";"],
        );
        let session: Session = {
            let mut sessions = server.sessions.lock().await;
            if !sessions.contains_key(&session_id) {
                sessions.insert(session_id.clone(), Session::new());
            }
            sessions.get(&session_id).expect("NEVER").clone()
        };
        let expired_time: i64 = iceyee_time::now() + server.session_timeout as i64;
        session
            .write()
            .await
            .insert("expired_time".to_string(), expired_time.to_string());
        let context: Context = Context {
            id: id.clone(),
            request: request,
            response: response,
            cookies: cookies,
            session: session,
            global_session: server.global_session.clone(),
            e_message: None,
        };
        return context;
    }

    async fn process(server: Arc<HttpServer>, context: &mut Context) -> bool {
        iceyee_logger::debug!(
            "\n",
            context.id,
            "\n",
            ">>>\n",
            context.request.to_string_with_body(),
        );
        iceyee_logger::info!(
            ">>>",
            context.id,
            context.request.method,
            context.request.path
        );
        let mut stop = false;
        for filter in &server.filters_before_work {
            if stop {
                break;
            }
            if filter.rule(context).await {
                match filter.do_filter(context).await {
                    Ok(true) => continue,
                    Ok(false) => {
                        stop = true;
                        break;
                    }
                    Err(e) => {
                        context.e_message = Some(e);
                        if !filter.on_error(context).await {
                            stop = true;
                            break;
                        }
                    }
                }
            }
        }
        let mut done = false;
        if !stop {
            let method: String = context.request.method.clone();
            let path: String = context.request.path.clone();
            let key: String = method.clone() + " " + &path;
            if server.works.contains_key(&key) {
                done = true;
                let work = server.works.get(&key).expect("NEVER");
                match work.do_work(context).await {
                    Ok(()) => {}
                    Err(e) => {
                        context.e_message = Some(e);
                        work.on_error(context).await;
                    }
                };
            }
        }
        if !stop && !done && server.file_router.is_some() {
            // Work不匹配, 则找本地文件.
            let file_router = server.file_router.as_ref().expect("NEVER");
            if file_router.rule(context).await {
                match file_router.do_filter(context).await {
                    Ok(_) => {}
                    Err(e) => {
                        context.e_message = Some(e);
                        file_router.on_error(context).await;
                    }
                }
            }
        }
        for filter in &server.filters_after_work {
            if stop {
                break;
            }
            if filter.rule(context).await {
                match filter.do_filter(context).await {
                    Ok(true) => continue,
                    Ok(false) => {
                        // stop = true;
                        break;
                    }
                    Err(e) => {
                        context.e_message = Some(e);
                        if !filter.on_error(context).await {
                            // stop = true;
                            break;
                        }
                    }
                }
            }
        }
        // Connection: close.
        let close: bool = if context.request.header.contains_key("Connection")
            && context.request.header.get("Connection").expect("NEVER") == "close"
        {
            context
                .response
                .header
                .insert("Connection".to_string(), vec!["close".to_string()]);
            true
        } else {
            context
                .response
                .header
                .insert("Connection".to_string(), vec!["keep-alive".to_string()]);
            false
        };
        // 输出.
        iceyee_logger::debug!(
            "\n",
            context.id,
            "\n",
            "<<<\n",
            context.response.to_string_with_body(),
        );
        iceyee_logger::info!(
            "<<<",
            context.id,
            context.response.status_code,
            context.response.status
        );
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
                let mut buffer = Vec::new();
                if let Err(e) = GzipEncoder::new(context.response.body.as_slice())
                    .read_to_end(&mut buffer)
                    .await
                {
                    iceyee_logger::error!(e);
                }
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
        return close;
    }

    async fn write_to_tcp(tcp: &mut TokioTcpStream, context: &Context) -> Result<(), String> {
        tcp.write_all(context.response.to_string().as_bytes())
            .await
            .map_err(|e| iceyee_error::a!(e))?;
        tcp.write_all(context.response.body.as_slice())
            .await
            .map_err(|e| iceyee_error::a!(e))?;
        return Ok(());
    }

    async fn clean_expired_session(server: Arc<HttpServer>) {
        let now: i64 = iceyee_time::now();
        let mut sessions = server.sessions.lock().await;
        let mut expired_session_id: Vec<String> = Vec::new();
        for (id, session) in sessions.iter() {
            let expired_time: i64 = session
                .read()
                .await
                .get("expired_time")
                .expect("NEVER")
                .parse::<i64>()
                .expect("NEVER");
            if expired_time < now {
                expired_session_id.push(id.clone());
            }
        }
        for id in &expired_session_id {
            sessions.remove(id);
        }
        iceyee_logger::info!(
            "清理不活跃会话",
            expired_session_id.len(),
            "个, 剩余会话",
            sessions.len(),
            "个."
        );
        drop(sessions);
        return;
    }
}

// Function.

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
    return String::from_utf8(buffer).expect("NEVER");
}

fn new_session_id() -> String {
    let a001 = HexEncoder::encode_number(Random::next());
    let a002 = HexEncoder::encode_number(Random::next());
    return a001 + &a002;
}
