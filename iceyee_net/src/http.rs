// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! HTTP协议.

pub mod client;
pub mod server;

use std::collections::HashMap;
use std::error::Error as StdError;
use std::io::Error as StdIoError;
use std::io::ErrorKind as StdIoErrorKind;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

// Use.

// Enum.

/// 常用的状态码.
///
/// 200 OK
///
/// 201 Created
///
/// 202 Accepted
///
/// 204 No Content
///
/// 301 Moved Permanently
///
/// 302 Moved Temporarily
///
/// 304 Not Modified
///
/// 400 Bad Request
///
/// 401 Unauthorized
///
/// 403 Forbidden
///
/// 404 Not Found
///
/// 500 Internal Server Error
///
/// 501 Not Implemented
///
/// 502 Bad Gateway
///
/// 503 Service Unavailable
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    OK(Option<String>),
    Created(String),
    Accepted(Option<String>),
    NoContent,
    MovedPermanently(String),
    MovedTemporarily(String),
    NotModified(Option<String>),
    BadRequest(Option<String>),
    Unauthorized(Option<String>),
    Forbidden(Option<String>),
    NotFound(Option<String>),
    InternalServerError(Option<String>),
    NotImplemented(Option<String>),
    BadGateway(Option<String>),
    ServiceUnavailable(Option<String>),
    UnkownStatusCode,
}

impl From<u16> for Status {
    fn from(value: u16) -> Self {
        match value {
            200 => Self::OK(None),
            201 => Self::Created("".to_string()),
            202 => Self::Accepted(None),
            204 => Self::NoContent,
            301 => Self::MovedPermanently("".to_string()),
            302 => Self::MovedTemporarily("".to_string()),
            304 => Self::NotModified(None),
            400 => Self::BadRequest(None),
            401 => Self::Unauthorized(None),
            403 => Self::Forbidden(None),
            404 => Self::NotFound(None),
            500 => Self::InternalServerError(None),
            501 => Self::NotImplemented(None),
            502 => Self::BadGateway(None),
            503 => Self::ServiceUnavailable(None),
            _ => Self::UnkownStatusCode,
        }
    }
}

impl Into<u16> for Status {
    fn into(self) -> u16 {
        return match self {
            Self::OK(_) => 200,
            Self::Created(_) => 201,
            Self::Accepted(_) => 202,
            Self::NoContent => 204,
            Self::MovedPermanently(_) => 301,
            Self::MovedTemporarily(_) => 302,
            Self::NotModified(_) => 304,
            Self::BadRequest(_) => 400,
            Self::Unauthorized(_) => 401,
            Self::Forbidden(_) => 403,
            Self::NotFound(_) => 404,
            Self::InternalServerError(_) => 500,
            Self::NotImplemented(_) => 501,
            Self::BadGateway(_) => 502,
            Self::ServiceUnavailable(_) => 503,
            Self::UnkownStatusCode => 0,
        };
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        let default_string: String = self.default_string();
        return match self {
            Self::OK(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::Created(s) => s.clone(),
            Self::Accepted(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::NoContent => "".to_string(),
            Self::NotModified(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::BadRequest(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::Unauthorized(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::Forbidden(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::NotFound(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::InternalServerError(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::NotImplemented(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::BadGateway(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::ServiceUnavailable(s) => s.as_ref().unwrap_or(&default_string).clone(),
            Self::MovedPermanently(s) => s.clone(),
            Self::MovedTemporarily(s) => s.clone(),
            Self::UnkownStatusCode => "Unkown Status Code".to_string(),
        };
    }
}

impl Status {
    pub fn default_string(&self) -> String {
        return match self {
            Self::OK(_) => "OK".to_string(),
            Self::Created(_) => "Created".to_string(),
            Self::Accepted(_) => "Accepted".to_string(),
            Self::NoContent => "No Content".to_string(),
            Self::MovedPermanently(_) => "Moved Permanently".to_string(),
            Self::MovedTemporarily(_) => "Moved Temporarily".to_string(),
            Self::NotModified(_) => "Not Modified".to_string(),
            Self::BadRequest(_) => "Bad Request".to_string(),
            Self::Unauthorized(_) => "Unauthorized".to_string(),
            Self::Forbidden(_) => "Forbidden".to_string(),
            Self::NotFound(_) => "Not Found".to_string(),
            Self::InternalServerError(_) => "Internal Server Error".to_string(),
            Self::NotImplemented(_) => "Not Implemented".to_string(),
            Self::BadGateway(_) => "Bad Gateway".to_string(),
            Self::ServiceUnavailable(_) => "Service Unavailable".to_string(),
            Self::UnkownStatusCode => "Unkown Status Code".to_string(),
        };
    }
}

// Trait.

// Struct.

/// Url参数.
#[derive(Clone, Debug, Default)]
pub struct Args {
    hm: HashMap<String, Vec<String>>,
    empty_vec: Vec<String>,
}

impl ToString for Args {
    /// 转字符串, 如'?a=1&b=2&b=3', 包含url编码.
    fn to_string(&self) -> String {
        use iceyee_encoder::UrlEncoder;

        let mut s: String = String::new();
        let mut keys = Vec::from_iter(self.hm.keys());
        keys.sort();
        for key in keys {
            for value in self.hm.get(key).unwrap() {
                if s.len() == 0 {
                    s.push_str("?");
                } else {
                    s.push_str("&");
                }
                s.push_str(UrlEncoder::encode(key.clone()).as_str());
                s.push_str("=");
                s.push_str(UrlEncoder::encode(value.clone()).as_str());
            }
        }
        return s;
    }
}

impl Args {
    pub fn add(&mut self, key: &str, value: &str) {
        if !self.hm.contains_key(key) {
            self.hm.insert(key.to_string(), Vec::new());
        }
        self.hm.get_mut(key).unwrap().push(value.to_string());
        return;
    }

    pub fn remove(&mut self, key: &str) {
        self.hm.remove(key);
        return;
    }

    pub fn get<'a>(&'a self, key: &str) -> &'a Vec<String> {
        return self.hm.get(key).unwrap_or(&self.empty_vec);
    }

    /// 解析参数, 例如'?a=1&a=2&b=3'解析得到\[(a,1),(a,2),(b,3)\].
    ///
    /// 解析包括URL解码.
    pub fn parse(s: &str) -> Args {
        use iceyee_encoder::UrlEncoder;

        let mut args: Args = Args::default();
        for x in s.split(['?', '&']) {
            if !x.contains('=') {
                continue;
            }
            let mut a001 = x.splitn(2, '=');
            let key: String = a001.next().unwrap().to_string();
            let key: String = UrlEncoder::decode(key).unwrap_or("".to_string());
            let value: String = a001.next().unwrap().to_string();
            let value: String = UrlEncoder::decode(value).unwrap_or("".to_string());
            if !args.hm.contains_key(&key) {
                args.hm.insert(key.clone(), Vec::new());
            }
            args.hm.get_mut(&key).unwrap().push(value);
        }
        return args;
    }
}

// /// 通一资源定位符, Uniform Resource Identifiers.
// type Uri = Url;

#[derive(Clone, Debug)]
enum State {
    Protocol,
    Host,
    Port,
    Path,
    Query,
    Fragment,
}

/// Error.
#[derive(Clone, Debug)]
pub struct UrlError {
    link: String,
    state: State,
    index: usize,
    message: String,
}

impl UrlError {
    fn new(link: String, state: State, index: usize, message: String) -> Self {
        return Self {
            link: link,
            state: state,
            index: index,
            message: message,
        };
    }
}

impl std::fmt::Display for UrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(
            f,
            "UrlError, 错误的格式, @link={}, @state={:?}, @index={}, @message={}",
            self.link, self.state, self.index, self.message
        );
    }
}

impl StdError for UrlError {}

/// 统一资源定位器, Uniform Resource Locator.
///
/// http_URL = "http:" "//" host \[ ":" port \] \[ abs_path \]
#[derive(Clone, Debug, Default)]
pub struct Url {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Url {
    pub fn new<S>(value: S) -> Result<Self, UrlError>
    where
        S: AsRef<str>,
    {
        let value = value.as_ref().to_string();
        let link: Arc<String> = Arc::new(value.to_string());
        let mut state: State = State::Protocol;
        let value: &[u8] = value.as_bytes();
        let mut index: usize = 0;
        let length: usize = value.len();
        let mut buffer: Vec<u8> = Vec::new();
        let mut url: Url = Url::default();
        let mut _state: Arc<State> = Arc::new(state.clone());
        while index < length {
            _state = Arc::new(state.clone());
            match state {
                State::Protocol => {
                    if value[index] == b'/' {
                        let protocol: String = String::from_utf8(buffer.to_vec()).map_err(|e| {
                            UrlError::new(
                                link.to_string(),
                                _state.as_ref().clone(),
                                index,
                                e.to_string(),
                            )
                        })?;
                        if !protocol.ends_with(":") {
                            Err(UrlError::new(
                                link.to_string(),
                                state,
                                index,
                                "".to_string(),
                            ))?;
                        }
                        url.protocol = protocol;
                        buffer.clear();
                        state = State::Host;
                        index += 2;
                    } else {
                        buffer.push(value[index]);
                        index += 1;
                    }
                }
                State::Host => match value[index] {
                    b':' | b'/' | b'?' | b'#' => {
                        let host: String = String::from_utf8(buffer.to_vec()).map_err(|e| {
                            UrlError::new(
                                link.to_string(),
                                _state.as_ref().clone(),
                                index,
                                e.to_string(),
                            )
                        })?;
                        if host.len() == 0 {
                            Err(UrlError::new(
                                link.to_string(),
                                state,
                                index,
                                "".to_string(),
                            ))?;
                        }
                        url.host = host;
                        buffer.clear();
                        match value[index] {
                            b':' => {
                                state = State::Port;
                                index += 1;
                            }
                            b'/' => {
                                state = State::Path;
                                index += 0;
                            }
                            b'?' => {
                                state = State::Query;
                                index += 0;
                            }
                            b'#' => {
                                state = State::Fragment;
                                index += 0;
                            }
                            _ => {
                                panic!("不可能到达.");
                            }
                        }
                    }
                    c => {
                        buffer.push(c);
                        index += 1;
                    }
                },
                State::Port => match value[index] {
                    b'/' | b'?' | b'#' => {
                        let port: u16 = String::from_utf8(buffer.to_vec())
                            .map_err(|e| {
                                UrlError::new(
                                    link.to_string(),
                                    _state.as_ref().clone(),
                                    index,
                                    e.to_string(),
                                )
                            })?
                            .parse::<u16>()
                            .map_err(|e| {
                                UrlError::new(
                                    link.to_string(),
                                    _state.as_ref().clone(),
                                    index,
                                    e.to_string(),
                                )
                            })?;
                        if port == 0 {
                            Err(UrlError::new(
                                link.to_string(),
                                state,
                                index,
                                "".to_string(),
                            ))?;
                        }
                        url.port = port;
                        buffer.clear();
                        match value[index] {
                            b'/' => {
                                state = State::Path;
                                index += 0;
                            }
                            b'?' => {
                                state = State::Query;
                                index += 0;
                            }
                            b'#' => {
                                state = State::Fragment;
                                index += 0;
                            }
                            _ => {
                                panic!("不可能到达.");
                            }
                        }
                    }
                    c => {
                        buffer.push(c);
                        index += 1;
                    }
                },
                State::Path => match value[index] {
                    b'?' | b'#' => {
                        let path: String = String::from_utf8(buffer.to_vec()).map_err(|e| {
                            UrlError::new(
                                link.to_string(),
                                _state.as_ref().clone(),
                                index,
                                e.to_string(),
                            )
                        })?;
                        url.path = path;
                        buffer.clear();
                        match value[index] {
                            b'?' => {
                                state = State::Query;
                                index += 0;
                            }
                            b'#' => {
                                state = State::Fragment;
                                index += 0;
                            }
                            _ => {
                                panic!("不可能到达.");
                            }
                        }
                    }
                    c => {
                        buffer.push(c);
                        index += 1;
                    }
                },
                State::Query => match value[index] {
                    b'#' => {
                        let query: String = String::from_utf8(buffer.to_vec()).map_err(|e| {
                            UrlError::new(
                                link.to_string(),
                                _state.as_ref().clone(),
                                index,
                                e.to_string(),
                            )
                        })?;
                        url.query = Some(query);
                        buffer.clear();
                        state = State::Fragment;
                        index += 0;
                    }
                    c => {
                        buffer.push(c);
                        index += 1;
                    }
                },
                State::Fragment => {
                    buffer.push(value[index]);
                    index += 1;
                }
            }
        } // while index < length
        match state {
            State::Protocol => {
                Err(UrlError::new(
                    link.to_string(),
                    state,
                    index,
                    "".to_string(),
                ))?;
            }
            State::Host => {
                let host: String = String::from_utf8(buffer.to_vec()).map_err(|e| {
                    UrlError::new(
                        link.to_string(),
                        _state.as_ref().clone(),
                        index,
                        e.to_string(),
                    )
                })?;
                if host.len() == 0 {
                    Err(UrlError::new(
                        link.to_string(),
                        state,
                        index,
                        "".to_string(),
                    ))?;
                }
                url.host = host;
            }
            State::Port => {
                let port: u16 = String::from_utf8(buffer.to_vec())
                    .map_err(|e| {
                        UrlError::new(
                            link.to_string(),
                            _state.as_ref().clone(),
                            index,
                            e.to_string(),
                        )
                    })?
                    .parse::<u16>()
                    .map_err(|e| {
                        UrlError::new(
                            link.to_string(),
                            _state.as_ref().clone(),
                            index,
                            e.to_string(),
                        )
                    })?;
                if port == 0 {
                    Err(UrlError::new(
                        link.to_string(),
                        state,
                        index,
                        "".to_string(),
                    ))?;
                }
                url.port = port;
            }
            State::Path => {
                let path: String = String::from_utf8(buffer.to_vec()).map_err(|e| {
                    UrlError::new(
                        link.to_string(),
                        _state.as_ref().clone(),
                        index,
                        e.to_string(),
                    )
                })?;
                url.path = path;
            }
            State::Query => {
                let query: String = String::from_utf8(buffer.to_vec()).map_err(|e| {
                    UrlError::new(
                        link.to_string(),
                        _state.as_ref().clone(),
                        index,
                        e.to_string(),
                    )
                })?;
                url.query = Some(query);
            }
            State::Fragment => {
                let fragment: String = String::from_utf8(buffer.to_vec()).map_err(|e| {
                    UrlError::new(
                        link.to_string(),
                        _state.as_ref().clone(),
                        index,
                        e.to_string(),
                    )
                })?;
                url.fragment = Some(fragment);
            }
        }
        if url.port == 0 {
            if url.protocol == "http:" {
                url.port = 80;
            } else if url.protocol == "https:" {
                url.port = 443;
            }
        }
        if url.path.len() == 0 {
            url.path = "/".to_string();
        }
        return Ok(url);
    }
}

#[derive(Clone, Debug)]
struct Buffer {
    pub block: [u8; 0xFFFF],
    pub length: usize,
}

impl Buffer {
    pub fn new() -> Buffer {
        return Buffer {
            block: [0; 0xFFFF],
            length: 0,
        };
    }

    pub fn extend(&mut self, target: &[u8], length: usize) {
        if 0xFFFF < self.length + length {
            return;
        }
        for x in 0..length {
            self.block[self.length + x] = target[x];
        }
        self.length += length;
        return;
    }

    pub fn roll(&mut self, offset: usize) {
        if self.length < offset {
            self.length = 0;
            return;
        }
        for x in 0..(self.length - offset) {
            self.block[x] = self.block[offset + x];
        }
        self.length -= offset;
        return;
    }
}

/// 请求.
///
/// GET / HTTP/1.1 \r\n
///
/// Header1: Value1 \r\n
///
/// Header2: Value1 \r\n
///
/// Header3: Value1 \r\n
///
/// \r\n
///
/// \[body\]
#[derive(Clone, Debug)]
pub struct Request {
    pub method: String,
    pub version: String,
    pub path: String,
    pub query: Args,
    pub fragment: Option<String>,
    pub header: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl std::default::Default for Request {
    fn default() -> Self {
        return Request {
            method: "GET".to_string(),
            path: "/".to_string(),
            query: Args::default(),
            fragment: None,
            version: "HTTP/1.1".to_string(),
            header: HashMap::new(),
            body: Vec::new(),
        };
    }
}

/// 转成报文, 但不包含请求正文.
impl ToString for Request {
    fn to_string(&self) -> String {
        let mut s: String = String::new();
        s.push_str(&self.method);
        s.push_str(" ");
        s.push_str(&self.path);
        s.push_str(&self.query.to_string());
        self.fragment.as_ref().filter(|t| {
            s.push_str(t);
            false
        });
        s.push_str(" ");
        s.push_str(&self.version);
        s.push_str("\r\n");
        let mut keys = Vec::from_iter(self.header.keys());
        keys.sort();
        for key in keys {
            s.push_str(key);
            s.push_str(": ");
            s.push_str(self.header.get(key).unwrap());
            s.push_str("\r\n");
        }
        s.push_str("\r\n");
        return s;
    }
}

impl Request {
    /// 解析数据.
    ///
    /// - @param timeout 超时, 可选, 默认1分钟.
    pub async fn read_from<R>(mut input: R, timeout: Option<usize>) -> Result<Request, StdIoError>
    where
        R: AsyncRead + Unpin,
    {
        enum State {
            MethodSpace,
            Method,
            PathSpace,
            Path,
            VersionSpace,
            Version,
            Header,
            BodySpace,
            Body,
        }
        let mut request: Request = Request::default();
        let mut state: State = State::MethodSpace;
        let mut buffer: Buffer = Buffer::new();
        let mut bytes: Vec<u8> = Vec::new();
        let mut needed: usize = 1;
        let timeout: usize = timeout.unwrap_or(60_000);
        'A: while 0 < needed {
            while needed <= 0xFFF && buffer.length < needed
                || 0xFFF <= needed && buffer.length < 0xFFF
            {
                let mut buf: [u8; 0xFFF] = [0; 0xFFF];
                let length: usize = match tokio::time::timeout(
                    Duration::from_millis(timeout as u64),
                    input.read(&mut buf),
                )
                .await
                {
                    Ok(length) => length?,
                    Err(_) => {
                        Err(StdIoError::new(StdIoErrorKind::TimedOut, "TimedOut"))?;
                        0
                    }
                };
                if length == 0 {
                    Err(StdIoError::new(
                        StdIoErrorKind::UnexpectedEof,
                        "UnexpectedEof",
                    ))?;
                }
                buffer.extend(&buf, length);
            }
            let mut x: usize = 0;
            match state {
                State::MethodSpace | State::PathSpace | State::VersionSpace => {
                    while x < buffer.length {
                        if !buffer.block[x].is_ascii_whitespace() {
                            match state {
                                State::MethodSpace => {
                                    state = State::Method;
                                    break;
                                }
                                State::PathSpace => {
                                    state = State::Path;
                                    break;
                                }
                                State::VersionSpace => {
                                    state = State::Version;
                                    break;
                                }
                                _ => {
                                    panic!("never.");
                                }
                            }
                        } else {
                            x += 1;
                        }
                    }
                }
                State::Method => {
                    while x < buffer.length {
                        if buffer.block[x].is_ascii_whitespace() {
                            request.method = String::from_utf8(bytes.clone()).map_err(|_| {
                                StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "String::from_utf8().",
                                )
                            })?;
                            bytes.clear();
                            state = State::PathSpace;
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                            if 0xFF < bytes.len() {
                                Err(StdIoError::new(std::io::ErrorKind::Other, "大小非预期."))?;
                            }
                        }
                    }
                }
                State::Path => {
                    while x < buffer.length {
                        if buffer.block[x].is_ascii_whitespace() {
                            request.path = String::from_utf8(bytes.clone()).map_err(|_| {
                                StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "String::from_utf8().",
                                )
                            })?;
                            bytes.clear();
                            state = State::VersionSpace;
                            if request.path.contains("?") {
                                let mut a001 = request.path.splitn(2, '?');
                                let a002: String = a001.next().unwrap().to_string();
                                let a003: String = a001.next().unwrap().to_string();
                                request.path = a002;
                                request.query = Args::parse(&a003);
                            }
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                            if 0xFFF < bytes.len() {
                                Err(StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "大小非预期.",
                                ))?;
                            }
                        }
                    }
                }
                State::Version => {
                    while x < buffer.length {
                        if buffer.block[x].is_ascii_whitespace() {
                            request.version = String::from_utf8(bytes.clone()).map_err(|_| {
                                StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "String::from_utf8().",
                                )
                            })?;
                            bytes.clear();
                            state = State::Header;
                            needed = 4;
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                            if 0xFF < bytes.len() {
                                Err(StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "大小非预期.",
                                ))?;
                            }
                        }
                    }
                }
                State::Header => {
                    while x + 3 < buffer.length {
                        if buffer.block[x + 0] == b'\r'
                            && buffer.block[x + 1] == b'\n'
                            && buffer.block[x + 2] == b'\r'
                            && buffer.block[x + 3] == b'\n'
                        {
                            let header: String =
                                String::from_utf8(bytes.clone()).map_err(|_| {
                                    StdIoError::new(
                                        std::io::ErrorKind::InvalidData,
                                        "String::from_utf8().",
                                    )
                                })?;
                            bytes.clear();
                            state = State::BodySpace;
                            for line in header.split("\r\n") {
                                if line.contains(":") {
                                    let mut a001 = line.splitn(2, ':');
                                    let key: String = a001.next().unwrap().trim().to_string();
                                    let value: String = a001.next().unwrap().trim().to_string();
                                    request.header.insert(key, value);
                                }
                            }
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                            if 0xFFFF < bytes.len() {
                                Err(StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "大小非预期.",
                                ))?;
                            }
                        }
                    }
                }
                State::BodySpace => {
                    state = State::Body;
                    x += 4;
                    let _ = x;
                    needed = if !request.header.contains_key("Content-Length") {
                        0
                    } else {
                        request
                            .header
                            .get("Content-Length")
                            .unwrap()
                            .trim()
                            .parse::<usize>()
                            .map_err(|_| {
                                StdIoError::new(std::io::ErrorKind::InvalidData, "String::parse().")
                            })?
                    };
                    if 0x3FFFFFFF < needed {
                        Err(StdIoError::new(std::io::ErrorKind::Other, "大小非预期."))?;
                    }
                }
                State::Body => {
                    if needed <= buffer.length {
                        for x in 0..needed {
                            bytes.push(buffer.block[x]);
                        }
                        request.body = bytes.clone();
                        bytes.clear();
                        buffer.roll(needed);
                        // needed = 0;
                        break 'A;
                    } else {
                        for x in 0..buffer.length {
                            bytes.push(buffer.block[x]);
                        }
                        needed -= buffer.length;
                        buffer.roll(buffer.length);
                    }
                }
            }
            buffer.roll(x);
        }
        return Ok(request);
    }
}

/// 响应.
///
/// HTTP/1.1 200 OK \r\n
///
/// Header1: Value1 \r\n
///
/// Header2: Value1 \r\n
///
/// Header3: Value1 \r\n
///
/// ...
///
/// \r\n
///
/// \[body\]
#[derive(Clone, Debug)]
pub struct Response {
    pub version: String,
    pub status_code: u16,
    pub status: String,
    pub header: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
}

impl std::default::Default for Response {
    fn default() -> Self {
        return Response {
            version: "HTTP/1.1".to_string(),
            status_code: 200,
            status: "OK".to_string(),
            header: HashMap::new(),
            body: Vec::new(),
        };
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut s: String = String::new();
        s.push_str(self.version.as_str());
        s.push_str(" ");
        s.push_str(self.status_code.to_string().as_str());
        s.push_str(" ");
        s.push_str(self.status.as_str());
        s.push_str("\r\n");
        let mut keys = Vec::from_iter(self.header.keys());
        keys.sort();
        for key in keys {
            for value in self.header.get(key).unwrap() {
                s.push_str(key);
                s.push_str(": ");
                s.push_str(value);
                s.push_str("\r\n");
            }
        }
        s.push_str("\r\n");
        return s;
    }
}

impl Response {
    /// 解析数据.
    ///
    /// - @param timeout 超时, 可选, 默认1分钟.
    pub async fn read_from<R>(mut input: R, timeout: Option<usize>) -> Result<Response, StdIoError>
    where
        R: AsyncRead + Unpin,
    {
        enum State {
            VersionSpace,
            Version,
            StatusCodeSpace,
            StatusCode,
            StatusSpace,
            Status,
            Header,
            BodySpace,
            Body,
            ChunkSize,
            ChunkData,
            ChunkSpace,
            ChunkEnd,
        }
        let mut response: Response = Response::default();
        let mut state: State = State::VersionSpace;
        let mut buffer: Buffer = Buffer::new();
        let mut bytes: Vec<u8> = Vec::new();
        let mut needed: usize = 1;
        let timeout: usize = timeout.unwrap_or(60_000);
        'A: loop {
            while needed <= 0xFFF && buffer.length < needed
                || 0xFFF <= needed && buffer.length < 0xFFF
            {
                let mut buf: [u8; 0xFFF] = [0; 0xFFF];
                let length: usize = match tokio::time::timeout(
                    Duration::from_millis(timeout as u64),
                    input.read(&mut buf),
                )
                .await
                {
                    Ok(length) => length?,
                    Err(_) => {
                        Err(StdIoError::new(StdIoErrorKind::TimedOut, "TimedOut"))?;
                        0
                    }
                };
                if length == 0 {
                    Err(StdIoError::new(
                        StdIoErrorKind::UnexpectedEof,
                        "UnexpectedEof",
                    ))?;
                }
                buffer.extend(&buf, length);
            }
            let mut x: usize = 0;
            match state {
                State::VersionSpace => {
                    while x < buffer.length {
                        if buffer.block[x].is_ascii_whitespace() {
                            x += 1;
                        } else {
                            state = State::Version;
                            break;
                        }
                    }
                }
                State::Version => {
                    while x < buffer.length {
                        if buffer.block[x].is_ascii_whitespace() {
                            response.version = String::from_utf8(bytes.clone()).map_err(|_| {
                                StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "String::from_utf8().",
                                )
                            })?;
                            bytes.clear();
                            state = State::StatusCodeSpace;
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                        }
                    }
                }
                State::StatusCodeSpace => {
                    while x < buffer.length {
                        if buffer.block[x].is_ascii_whitespace() {
                            x += 1;
                        } else {
                            state = State::StatusCode;
                            break;
                        }
                    }
                }
                State::StatusCode => {
                    while x < buffer.length {
                        if buffer.block[x].is_ascii_whitespace() {
                            response.status_code = String::from_utf8(bytes.clone())
                                .map_err(|_| {
                                    StdIoError::new(
                                        std::io::ErrorKind::Other,
                                        "String::from_utf8().",
                                    )
                                })?
                                .parse::<u16>()
                                .map_err(|_| {
                                    StdIoError::new(
                                        std::io::ErrorKind::InvalidData,
                                        "String::parse().",
                                    )
                                })?;
                            bytes.clear();
                            state = State::StatusSpace;
                            needed = 2;
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                        }
                    }
                }
                State::StatusSpace => {
                    while x + 1 < buffer.length {
                        if !buffer.block[x].is_ascii_whitespace() {
                            state = State::Status;
                            break;
                        } else if buffer.block[x] == b'\r' && buffer.block[x + 1] == b'\n' {
                            response.status = "".to_string();
                            state = State::Header;
                            needed = 4;
                            break;
                        } else {
                            x += 1;
                        }
                    }
                }
                State::Status => {
                    while x + 1 < buffer.length {
                        if buffer.block[x] == b'\r' && buffer.block[x + 1] == b'\n' {
                            response.status = String::from_utf8(bytes.clone()).map_err(|_| {
                                StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "String::from_utf8().",
                                )
                            })?;
                            bytes.clear();
                            state = State::Header;
                            needed = 4;
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                        }
                    }
                }
                State::Header => {
                    while x + 3 < buffer.length {
                        if buffer.block[x + 0] == b'\r'
                            && buffer.block[x + 1] == b'\n'
                            && buffer.block[x + 2] == b'\r'
                            && buffer.block[x + 3] == b'\n'
                        {
                            let a001: String = String::from_utf8(bytes.clone()).map_err(|_| {
                                StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "String::from_utf8().",
                                )
                            })?;
                            bytes.clear();
                            for line in a001.split("\r\n") {
                                if line.contains(":") {
                                    let mut a002 = line.splitn(2, ":");
                                    let key: String = a002.next().unwrap().trim().to_string();
                                    let value: String = a002.next().unwrap().trim().to_string();
                                    if !response.header.contains_key(&key) {
                                        response.header.insert(key.clone(), Vec::new());
                                    }
                                    response.header.get_mut(&key).unwrap().push(value);
                                }
                            }
                            state = State::BodySpace;
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                        }
                    }
                }
                State::BodySpace => {
                    x += 4;
                    if response.header.contains_key("Content-Length") {
                        needed = response.header.get("Content-Length").unwrap().as_slice()[0]
                            .parse::<usize>()
                            .map_err(|_| {
                                StdIoError::new(std::io::ErrorKind::InvalidData, "String::parse().")
                            })?;
                        state = State::Body;
                    } else if response.header.contains_key("Transfer-Encoding") {
                        needed = 2;
                        state = State::ChunkSize;
                    } else {
                        needed = 0;
                        state = State::Body;
                    }
                }
                State::Body => {
                    if needed <= buffer.length {
                        for x in 0..needed {
                            response.body.push(buffer.block[x]);
                        }
                        buffer.roll(needed);
                        needed = 0;
                        _ = needed;
                        break 'A;
                    } else {
                        for x in 0..buffer.length {
                            response.body.push(buffer.block[x]);
                        }
                        needed -= buffer.length;
                        buffer.roll(buffer.length);
                    }
                }
                State::ChunkSize => {
                    while x + 1 < buffer.length {
                        if buffer.block[x] == b'\r' && buffer.block[x + 1] == b'\n' {
                            let a001: String = String::from_utf8(bytes.clone()).map_err(|_| {
                                StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "String::from_utf8().",
                                )
                            })?;
                            needed = usize::from_str_radix(&a001, 16).map_err(|_| {
                                StdIoError::new(
                                    std::io::ErrorKind::InvalidData,
                                    "usize::from_str_radix().",
                                )
                            })?;
                            bytes.clear();
                            if needed == 0 {
                                needed = 2;
                                x += 2;
                                state = State::ChunkEnd;
                            } else {
                                x += 2;
                                state = State::ChunkData;
                            }
                            break;
                        } else {
                            bytes.push(buffer.block[x]);
                            x += 1;
                        }
                    }
                }
                State::ChunkData => {
                    if needed <= buffer.length {
                        for x in 0..needed {
                            response.body.push(buffer.block[x]);
                        }
                        buffer.roll(needed);
                        needed = 2;
                        state = State::ChunkSpace;
                    } else {
                        for x in 0..buffer.length {
                            response.body.push(buffer.block[x]);
                        }
                        needed -= buffer.length;
                        buffer.roll(buffer.length);
                    }
                }
                State::ChunkSpace => {
                    if buffer.block[0] != b'\r' || buffer.block[1] != b'\n' {
                        return Err(StdIoError::new(
                            std::io::ErrorKind::InvalidData,
                            "非预期的格式.",
                        ));
                    }
                    bytes.clear();
                    x += 2;
                    needed = 2;
                    state = State::ChunkSize;
                }
                State::ChunkEnd => {
                    if buffer.block[0] != b'\r' || buffer.block[1] != b'\n' {
                        return Err(StdIoError::new(
                            std::io::ErrorKind::InvalidData,
                            "非预期的格式.",
                        ));
                    }
                    buffer.roll(2);
                    break 'A;
                }
            } // match state{}
            buffer.roll(x);
        }
        return Ok(response);
    }
}

// Function.
