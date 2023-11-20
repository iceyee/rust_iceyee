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

// Use.

use crate::http::Request;
use crate::http::Response;
use iceyee_logger::Logger;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as StdIoError;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::net::ToSocketAddrs;
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::RwLock as TokioRwLock;

pub use crate::http::Status;
pub use iceyee_logger::Level;

// Enum.

// Trait.

/// ```
/// use iceyee_net::http::server::Context;
/// use iceyee_net::http::server::Filter;
///
/// #[async_trait::async_trait]
/// pub trait Filter: Send + Sync {
///     async fn rule(&self, _context: &mut Context) -> bool;
///     async fn do_filter(&self, _context: &mut Context) -> Result<bool, String>;
///     async fn on_error(&self, _context: &mut Context) -> bool;
/// }
/// ```
/// @see [Context]
///
/// @see [Cookies]
///
/// @see [Request]
///
/// @see [Response]
///
/// @see [Session]
///
/// @see [HttpServer]
///
/// @see [Work]
#[async_trait::async_trait]
pub trait Filter: Send + Sync {
    async fn rule(&self, _context: &mut Context) -> bool {
        return true;
    }

    async fn do_filter(&self, _context: &mut Context) -> Result<bool, String> {
        return Ok(true);
    }

    async fn on_error(&self, _context: &mut Context) -> bool {
        R::write_status(&mut _context.response, Status::InternalServerError);
        _context.response.body = _context.e_message.as_ref().unwrap().as_bytes().to_vec();
        return false;
    }

    fn to_arc(self) -> Arc<dyn Filter>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

/// ```
/// use iceyee_net::http::server::Context;
/// use iceyee_net::http::server::Work;
///
/// #[async_trait::async_trait]
/// pub trait Work: Send + Sync {
///     fn method(&self) -> &'static str;
///     fn path(&self) -> &'static str;
///     async fn do_work(&self, _context: &mut Context) -> Result<(), String>;
///     async fn on_error(&self, _context: &mut Context);
/// }
/// ```
/// @see [Context]
///
/// @see [Cookies]
///
/// @see [Request]
///
/// @see [Response]
///
/// @see [Session]
///
/// @see [HttpServer]
///
/// @see [Filter]
#[async_trait::async_trait]
pub trait Work: Send + Sync {
    fn method(&self) -> &'static str {
        return "GET";
    }

    fn path(&self) -> &'static str;

    async fn do_work(&self, _context: &mut Context) -> Result<(), String> {
        return Ok(());
    }

    async fn on_error(&self, _context: &mut Context) {
        R::write_status(&mut _context.response, Status::InternalServerError);
        return;
    }

    fn to_arc(self) -> Arc<dyn Work>
    where
        Self: Sized + 'static,
    {
        return Arc::new(self);
    }
}

// Struct.

pub type Cookies = HashMap<String, String>;

/// 会话, 以键值的方式存储用户数据.
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

    pub async fn get(&self, key: &str) -> String {
        let this = self.this.read().await;
        if this.contains_key(key) {
            return this.get(key).unwrap().clone();
        } else {
            return "".to_string();
        }
    }

    pub async fn set(&self, key: &str, value: &str) {
        let mut this = self.this.write().await;
        this.insert(key.to_string(), value.to_string());
        return;
    }
}

/// 上下文.
///
/// @see [Filter]
///
/// @see [Work]
pub struct Context {
    pub logger: Arc<Logger>,
    pub request: Request,
    pub response: Response,
    pub cookies: Cookies,
    pub session: Session,
    pub e_message: Option<String>,
}

/// 简单的用户认证.
#[derive(Clone, Debug)]
pub struct BasicAuthFilter {
    auth_string_s: HashSet<String>,
}

impl BasicAuthFilter {
    pub fn new(user: &str, password: &str) -> Self {
        let this = Self {
            auth_string_s: HashSet::new(),
        };
        return this.add(user, password);
    }

    pub fn add(mut self, user: &str, password: &str) -> Self {
        use iceyee_encoder::Base64Encoder;

        let auth: String = user.to_string() + ":" + password;
        let auth: String = Base64Encoder::encode(auth.as_bytes().to_vec());
        self.auth_string_s.insert(auth);
        return self;
    }
}

#[async_trait::async_trait]
impl Filter for BasicAuthFilter {
    async fn rule(&self, context: &mut Context) -> bool {
        if context.request.path == "/favicon.ico" {
            return false;
        } else {
            return true;
        }
    }

    async fn do_filter(&self, context: &mut Context) -> Result<bool, String> {
        // Authorization
        let auth: String = match context.request.header.get("Authorization") {
            Some(auth) => {
                if !auth.starts_with("Basic ") {
                    "".to_string()
                } else {
                    auth.to_string().split_off(6)
                }
            }
            None => "".to_string(),
        };
        if self.auth_string_s.contains(&auth) {
            return Ok(true);
        } else {
            R::write_status(&mut context.response, Status::Unauthorized);
            context.response.header.insert(
                "WWW-Authenticate".to_string(),
                vec!["Basic realm=\"Realm\"".to_string()],
            );
            return Ok(false);
        }
    }
}

#[derive(Clone, Debug)]
struct HostFilter {
    full_hosts: HashSet<String>,
    usual_hosts: HashSet<String>,
}

impl HostFilter {
    pub fn new() -> HostFilter {
        let mut host_filter = HostFilter {
            full_hosts: HashSet::new(),
            usual_hosts: HashSet::new(),
        };
        host_filter.full_hosts.insert("127.0.0.1".to_string());
        host_filter.full_hosts.insert("localhost".to_string());
        return host_filter;
    }

    pub fn add_full(&mut self, host: &str) {
        self.full_hosts.insert(host.to_string());
        return;
    }

    pub fn add_usual(&mut self, host: &str) {
        self.usual_hosts.insert(host.to_string());
        return;
    }
}

#[async_trait::async_trait]
impl Filter for HostFilter {
    async fn do_filter(&self, context: &mut Context) -> Result<bool, String> {
        // 如果有端口, 则截掉端口部分.
        let host: Option<String> = context.request.header.get("Host").map(|host| {
            if host.contains(":") {
                host.splitn(2, ":").next().unwrap().to_string()
            } else {
                host.to_string()
            }
        });
        let auth: bool = match host {
            Some(host) => {
                if self.full_hosts.contains(&host) {
                    // 如果全匹配.
                    return Ok(true);
                } else if !host.contains(".") {
                    // 如果没有二级域名.
                    false
                } else {
                    // 如果有二级域名 | 截掉前面的二级域名, 然后匹配.
                    let a001 = host.clone();
                    let mut a001 = a001.splitn(2, ".");
                    a001.next();
                    let a002 = ".".to_string() + a001.next().unwrap();
                    self.usual_hosts.contains(&a002)
                }
            }
            None => false,
        };
        if !auth {
            R::write_status(&mut context.response, Status::Forbidden);
        }
        return Ok(auth);
    }
}

#[derive(Clone, Debug)]
struct FileRouter {
    root: String,
    map: HashMap<String, String>,
}

impl FileRouter {
    pub fn new(root: &str) -> FileRouter {
        let mut this = FileRouter {
            root: root.to_string(),
            map: HashMap::new(),
        };
        for (key, value) in [
            ("", "text/plain"),
            (".3gp", "audio/3gpp"),
            (".7z", "application/x-7z-compressed"),
            (".ar", "application/x-archive"),
            (".asp", "application/x-asp"),
            (".avi", "video/avi"),
            (".avi", "video/x-msvideo"),
            (".bmp", "image/bmp"),
            (".css", "text/css"),
            (".doc", "application/msword"),
            (".exe", "application/x-ms-dos-executable"),
            (".gif", "image/gif"),
            (".html", "text/html"),
            (".ico", "image/ico"),
            (".img", "application/x-raw-disk-image"),
            (".ini", "text/plain"),
            (".iso", "application/x-cd-image"),
            (".jar", "application/x-java-archive"),
            (".java", "text/x-java,text/x-java-source"),
            (".jpeg", "image/jpeg"),
            (".jpg", "image/jpeg"),
            (".js", "application/javascript"),
            (".json", "application/json"),
            (".json", "application/json"),
            (".log", "text/plain"),
            (".lua", "text/x-lua"),
            (".m3u8", "audio/m3u"),
            (".mp3", "audio/mpeg"),
            (".mp4", "video/mp4"),
            (".pdf", "application/pdf"),
            (".png", "image/png"),
            (".ppt", "application/vnd.ms-powerpoint"),
            (".py", "text/x-python"),
            (".rar", "application/x-rar-compressed"),
            (".sh", "text/x-sh"),
            (".sql", "text/x-sql"),
            (".svg", "image/svg"),
            (".tar", "application/x-tar"),
            (".txt", "text/plain"),
            (".wav", "audio/wav"),
            (".xls", "application/vnd.ms-excel"),
            (".zip", "application/zip"),
        ] {
            this.map.insert(key.to_string(), value.to_string());
        }
        return this;
    }

    fn map_suffix_to_type(&self, s: &str) -> String {
        if self.map.contains_key(s) {
            return self.map.get(s).unwrap().to_string();
        } else {
            return "text/plain".to_string();
        }
    }
}

#[async_trait::async_trait]
impl Filter for FileRouter {
    async fn rule(&self, context: &mut Context) -> bool {
        let path: String = if context.request.path == "/" {
            self.root.clone() + "/index.html"
        } else {
            self.root.clone() + &context.request.path
        };
        match tokio::fs::metadata(&path).await {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
    }

    async fn do_filter(&self, context: &mut Context) -> Result<bool, String> {
        let mut path: String = if context.request.path == "/" {
            self.root.clone() + "/index.html"
        } else {
            self.root.clone() + &context.request.path
        };
        if path.contains("..") {
            R::write_status(&mut context.response, Status::Forbidden);
            return Ok(true);
        }
        let metadata = tokio::fs::metadata(&path)
            .await
            .map_err(|e| e.to_string())?;
        if metadata.is_symlink() {
            path = tokio::fs::read_link(&path)
                .await
                .map_err(|e| e.to_string())?
                .to_str()
                .unwrap()
                .to_string();
        }
        if metadata.is_dir() {
            R::write_status(&mut context.response, Status::BadRequest);
        } else {
            context.response.status_code = 200;
            context.response.status = "OK".to_string();
            context.response.body = tokio::fs::read(&path).await.map_err(|e| e.to_string())?;
            let suffix: String = match path.rfind(".") {
                Some(index) => path.clone().split_off(index),
                None => "".to_string(),
            };
            context.response.header.insert(
                "Content-Type".to_string(),
                vec![self.map_suffix_to_type(&suffix)],
            );
        }
        return Ok(true);
    }

    async fn on_error(&self, context: &mut Context) -> bool {
        context
            .logger
            .error(context.e_message.as_ref().unwrap())
            .await;
        return true;
    }
}

/// Http服务端.
///
/// @see [Filter]
///
/// @see [Work]
#[derive(Clone)]
pub struct HttpServer {
    stop_server: Arc<AtomicBool>,
    filters_before_work: Vec<Arc<dyn Filter>>,
    works: HashMap<String, Vec<(String, Arc<dyn Work>)>>,
    filters_after_work: Vec<Arc<dyn Filter>>,
    sessions: Arc<TokioMutex<HashMap<String, Session>>>,
    logger: Option<Arc<Logger>>,
    host_filter: HostFilter,
    file_router: Option<FileRouter>,
}

unsafe impl Send for HttpServer {}
unsafe impl Sync for HttpServer {}

impl HttpServer {
    pub fn new() -> Self {
        let mut server = HttpServer {
            stop_server: Arc::new(AtomicBool::new(false)),
            filters_before_work: Vec::new(),
            works: HashMap::new(),
            filters_after_work: Vec::new(),
            sessions: Arc::new(TokioMutex::new(HashMap::new())),
            logger: None,
            host_filter: HostFilter::new(),
            file_router: None,
        };
        server
            .filters_before_work
            .push(server.host_filter.clone().to_arc());
        return server;
    }

    /// 文件根目录.
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
            self.host_filter.add_usual(host);
        } else {
            self.host_filter.add_full(host);
        }
        return self;
    }

    pub fn add_filter_before_work(mut self, filter: Arc<dyn Filter>) -> Self {
        self.filters_before_work.push(filter);
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

    pub fn add_filter_after_work(mut self, filter: Arc<dyn Filter>) -> Self {
        self.filters_after_work.push(filter);
        return self;
    }

    /// 启动服务器.
    pub async fn start<A>(
        mut self,
        address_and_port: A,
        level: Option<Level>,
        project_name: Option<&str>,
        target_directory: Option<&str>,
    ) -> Result<(), StdIoError>
    where
        A: ToSocketAddrs,
    {
        use iceyee_random::Random;
        use std::io::ErrorKind as StdIoErrorKind;
        use std::net::IpAddr;
        use tokio::net::TcpListener as TokioTcpListener;

        self.logger = Some(Arc::new(
            Logger::new(level, project_name, target_directory).await,
        ));
        let listener: TokioTcpListener = TokioTcpListener::bind(address_and_port).await?;
        let address = listener.local_addr()?;
        let server = Arc::new(self);
        let logger = server.logger.as_ref().unwrap().clone();
        let server_clone = server.clone();
        let logger_clone = logger.clone();
        let listener_future = tokio::task::spawn(async move {
            while !server_clone.stop_server.load(SeqCst) {
                let server = server_clone.clone();
                let logger = logger_clone.clone();
                match listener.accept().await {
                    Ok((tcp, address)) => {
                        let id = Random::next();
                        let mut counter: usize = 0;
                        if server.stop_server.load(SeqCst) {
                            break;
                        }
                        tokio::task::spawn(async move {
                            let ip = match address.ip() {
                                IpAddr::V4(ip) => ipv4_to_string(ip),
                                IpAddr::V6(ip) => ipv6_to_string(ip),
                            };
                            logger
                                .debug(format!("建立连接, {ip}, {id}.").as_str())
                                .await;
                            let tcp = Arc::new(tcp);
                            loop {
                                counter += 1;
                                let server = server.clone();
                                let tcp = tcp.clone();
                                match process_tcp(server, tcp, id, &mut counter).await {
                                    Ok(true) => continue,
                                    Ok(false) => {
                                        logger
                                            .debug(format!("正常断开连接, {ip}, {id}.").as_str())
                                            .await;
                                        break;
                                    }
                                    Err(e) => {
                                        match e.kind() {
                                            StdIoErrorKind::TimedOut => {
                                                logger
                                                    .debug(
                                                        format!("超时断开连接, {ip}, {id}.")
                                                            .as_str(),
                                                    )
                                                    .await
                                            }
                                            _ => {
                                                logger
                                                    .debug(
                                                        format!("异常断开连接, {ip}, {id}.")
                                                            .as_str(),
                                                    )
                                                    .await;
                                                logger.error(e.to_string().as_str()).await;
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        logger.error(e.to_string().as_str()).await;
                        break;
                    }
                }
            }
        });
        println!("---- 输入[Ctrl+C]停止. ----");
        tokio::signal::ctrl_c().await.unwrap();
        server.stop_server.store(true, SeqCst);
        TokioTcpStream::connect(address).await?;
        listener_future.await.unwrap();
        println!("---- 退出服务端. ----");
        return Ok(());
    }

    pub async fn wait_60_secs() {
        use std::io::Write;

        for x in 1..61 {
            iceyee_datetime::sleep(1_000).await;
            print!("{x}.");
            std::io::stdout().flush().unwrap();
        }
    }
}

/// 一些针对[Response]的接口.
pub struct R;

impl R {
    pub fn write_ok(response: &mut Response) {
        Self::write_status(response, Status::OK);
        return;
    }

    pub fn write_status(response: &mut Response, status: Status) {
        response.status_code = status.clone().into();
        response.status = status.clone().to_string();
        response.body = status.clone().to_string().as_bytes().to_vec();
        response
            .header
            .insert("Content-Type".to_string(), vec!["text/plain".to_string()]);
        match status {
            Status::MovedPermanently(link) | Status::MovedTemporarily(link) => {
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

// Function.

fn ipv4_to_string(ip: std::net::Ipv4Addr) -> String {
    let ip = ip.octets();
    return format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
}

fn ipv6_to_string(ip: std::net::Ipv6Addr) -> String {
    use iceyee_encoder::HexEncoder;

    let ip = ip.octets();
    let ip: String = HexEncoder::encode(ip.to_vec());
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
    use iceyee_encoder::HexEncoder;
    use iceyee_random::Random;

    let a001 = HexEncoder::encode_number(Random::next() as u64);
    let a002 = HexEncoder::encode_number(Random::next() as u64);
    return a001 + &a002;
}

async fn process_tcp(
    server: Arc<HttpServer>,
    mut tcp: Arc<TokioTcpStream>,
    id: usize,
    counter: &mut usize,
) -> Result<bool, StdIoError> {
    use tokio::io::AsyncWriteExt;

    let mut close = false;
    let request: Request = Request::read_from(unsafe { Arc::get_mut_unchecked(&mut tcp) }).await?;
    let a001 = format!("\n{id} {counter}\n>>>\n{}", request.to_string());
    server.logger.as_ref().unwrap().debug(&a001).await;
    let mut response: Response = Response::new();
    let status: Status = Status::NotFound;
    response.status_code = status.clone().into();
    response.status = status.clone().to_string();
    response.body = status.clone().to_string().as_bytes().to_vec();
    response
        .header
        .insert("Content-Type".to_string(), vec!["text/plain".to_string()]);
    let mut cookies: Cookies = Cookies::new();
    let session_id: String = if request.header.contains_key("Cookie") {
        let s: &String = request.header.get("Cookie").unwrap();
        cookies = parse_cookie(s);
        if cookies.contains_key("session_id") {
            cookies.get("session_id").unwrap().to_string()
        } else {
            new_session_id()
        }
    } else {
        new_session_id()
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
    let mut context: Context = Context {
        logger: server.logger.as_ref().unwrap().clone(),
        request: request,
        response: response,
        cookies: cookies,
        session: session,
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
                    close = true;
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
                            context
                                .logger
                                .debug(
                                    ("\n".to_string() + context.e_message.as_ref().unwrap())
                                        .as_str(),
                                )
                                .await;
                            close = true;
                            context.e_message = Some(e);
                            work.on_error(&mut context).await;
                        }
                    };
                }
            }
        }
    }
    if !stop && !done && server.file_router.is_some() {
        // 不匹配.
        let file_router = server.file_router.as_ref().unwrap();
        if file_router.rule(&mut context).await {
            match file_router.do_filter(&mut context).await {
                Ok(_) => {}
                Err(e) => {
                    context
                        .logger
                        .debug(("\n".to_string() + context.e_message.as_ref().unwrap()).as_str())
                        .await;
                    close = true;
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
                        context
                            .logger
                            .debug(
                                ("\n".to_string() + context.e_message.as_ref().unwrap()).as_str(),
                            )
                            .await;
                        close = true;
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
            use async_compression::tokio::bufread::GzipEncoder;
            use iceyee_encoder::HexEncoder;
            use tokio::io::AsyncReadExt;

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
    let a001 = format!("\n{id} {counter}\n<<<\n{}", context.response.to_string());
    server.logger.as_ref().unwrap().debug(&a001).await;
    unsafe { Arc::get_mut_unchecked(&mut tcp) }
        .write_all(context.response.to_string().as_bytes())
        .await?;
    unsafe { Arc::get_mut_unchecked(&mut tcp) }
        .write_all(context.response.body.as_slice())
        .await?;
    return Ok(!close);
}
