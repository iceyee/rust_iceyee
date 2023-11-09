// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use crate::http::Args;
use crate::http::Request;
use crate::http::Response;
use crate::http::Url;
use crate::http::UrlError;
use iceyee_error::IceyeeError;
use iceyee_error::StdError;
use iceyee_error::StdIoError;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadBuf;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::sync::Mutex as TokioMutex;
use tokio_native_tls::TlsStream;

// Enum.

// Trait.

#[async_trait::async_trait]
pub trait Proxy: AsyncRead + AsyncWrite + Unpin {
    async fn connect(&mut self) -> Result<(), IceyeeError>;

    fn close(&mut self);

    fn is_closed(&self) -> bool {
        true
    }

    fn wrap(self) -> Arc<TokioMutex<Box<dyn Proxy>>>
    where
        Self: Sized + 'static,
    {
        Arc::new(TokioMutex::new(Box::new(self)))
    }
}

// Struct.

pub struct NoProxy {
    target_host: String,
    target_port: u16,
    using_ssl: bool,
    plain_socket: Option<TokioTcpStream>,
    ssl_socket: Option<TlsStream<TokioTcpStream>>,
}

impl NoProxy {
    pub fn new(target_host: &str, target_port: u16, using_ssl: bool) -> NoProxy {
        return NoProxy {
            target_host: target_host.to_string(),
            target_port: target_port,
            using_ssl: using_ssl,
            plain_socket: None,
            ssl_socket: None,
        };
    }
}

#[async_trait::async_trait]
impl Proxy for NoProxy {
    async fn connect(&mut self) -> Result<(), IceyeeError> {
        if !self.using_ssl {
            let plain_socket: TokioTcpStream =
                TokioTcpStream::connect((self.target_host.clone(), self.target_port))
                    .await
                    .map_err(|e| IceyeeError::from(Box::new(e) as Box<dyn StdError>))?;
            self.plain_socket = Some(plain_socket);
        } else {
            let plain_socket: TokioTcpStream =
                TokioTcpStream::connect((self.target_host.clone(), self.target_port))
                    .await
                    .map_err(|e| IceyeeError::from(Box::new(e) as Box<dyn StdError>))?;
            let connector = tokio_native_tls::native_tls::TlsConnector::new()
                .map_err(|e| IceyeeError::from(Box::new(e) as Box<dyn StdError>))?;
            let connector = tokio_native_tls::TlsConnector::from(connector);
            let ssl_socket: TlsStream<TokioTcpStream> = connector
                .connect(self.target_host.as_str(), plain_socket)
                .await
                .map_err(|e| IceyeeError::from(Box::new(e) as Box<dyn StdError>))?;
            self.ssl_socket = Some(ssl_socket);
        }
        Ok(())
    }

    fn close(&mut self) {
        self.plain_socket = None;
        self.ssl_socket = None;
        return;
    }

    fn is_closed(&self) -> bool {
        self.plain_socket.is_none() && self.ssl_socket.is_none()
    }
}

impl AsyncRead for NoProxy {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), StdIoError>> {
        if self.plain_socket.is_some() {
            let mut pinned = std::pin::pin!(self.plain_socket.as_mut().unwrap());
            return pinned.as_mut().poll_read(cx, buf);
        } else if self.ssl_socket.is_some() {
            let mut pinned = std::pin::pin!(self.ssl_socket.as_mut().unwrap());
            return pinned.as_mut().poll_read(cx, buf);
        } else {
            return std::task::Poll::Ready(Ok(()));
        }
    }
}

impl AsyncWrite for NoProxy {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, StdIoError>> {
        if self.plain_socket.is_some() {
            let mut pinned = std::pin::pin!(self.plain_socket.as_mut().unwrap());
            return pinned.as_mut().poll_write(cx, buf);
        } else if self.ssl_socket.is_some() {
            let mut pinned = std::pin::pin!(self.ssl_socket.as_mut().unwrap());
            return pinned.as_mut().poll_write(cx, buf);
        } else {
            return std::task::Poll::Ready(Ok(0));
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), StdIoError>> {
        if self.plain_socket.is_some() {
            let mut pinned = std::pin::pin!(self.plain_socket.as_mut().unwrap());
            return pinned.as_mut().poll_flush(cx);
        } else if self.ssl_socket.is_some() {
            let mut pinned = std::pin::pin!(self.ssl_socket.as_mut().unwrap());
            return pinned.as_mut().poll_flush(cx);
        } else {
            return std::task::Poll::Ready(Ok(()));
        }
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), StdIoError>> {
        if self.plain_socket.is_some() {
            let mut pinned = std::pin::pin!(self.plain_socket.as_mut().unwrap());
            return pinned.as_mut().poll_shutdown(cx);
        } else if self.ssl_socket.is_some() {
            let mut pinned = std::pin::pin!(self.ssl_socket.as_mut().unwrap());
            return pinned.as_mut().poll_shutdown(cx);
        } else {
            return std::task::Poll::Ready(Ok(()));
        }
    }
}

/// Http客户端.
pub struct HttpClient {
    log: String,
    request: Request,
    using_ssl: bool,
    verbose: bool,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        let mut http_client: HttpClient = HttpClient {
            log: String::new(),
            request: Default::default(),
            using_ssl: false,
            verbose: false,
        };
        http_client
            .request
            .header
            .insert("User-Agent".to_string(), "iceyee/1.0".to_string());
        return http_client;
    }

    pub fn set_url(&mut self, s: &str) -> Result<&mut Self, UrlError> {
        let url: Url = Url::new(s)?;
        self.request
            .header
            .insert("Host".to_string(), url.host.clone());
        self.request.host = url.host.clone();
        self.request.port = url.port;
        if url.port == 80 && url.protocol == "http:" || url.port == 443 && url.protocol == "https:"
        {
            self.request.header.insert(
                "Referer".to_string(),
                url.protocol.clone() + "//" + url.host.as_str() + "/",
            );
        } else {
            self.request.header.insert(
                "Referer".to_string(),
                url.protocol.clone()
                    + "//"
                    + url.host.as_str()
                    + ":"
                    + url.port.to_string().as_str()
                    + "/",
            );
        }
        self.using_ssl = url.protocol == "https:";
        self.request.path = url.path;
        if url.query.is_some() {
            self.request.query = Args::parse(url.query.as_ref().unwrap());
        }
        self.request.fragment = url.fragment;
        return Ok(self);
    }

    pub fn set_method(&mut self, s: &str) -> &mut Self {
        self.request.method = s.to_string();
        return self;
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.request
            .header
            .insert(key.to_string(), value.to_string());
        return self;
    }

    pub fn remove_header(&mut self, key: &str) -> &mut Self {
        self.request.header.remove(key);
        return self;
    }

    pub fn set_body(&mut self, b: &[u8]) -> &mut Self {
        self.request.method = "POST".to_string();
        self.request.body = b.to_vec();
        self.request
            .header
            .insert("Content-Length".to_string(), b.len().to_string());
        if !self.request.header.contains_key("Content-Type") {
            self.request.header.insert(
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            );
        }
        return self;
    }

    pub fn set_forwarded(&mut self, s: Option<&str>) -> &mut Self {
        use iceyee_random::Random;

        if s.is_none() {
            let ip: String = format!(
                "{}.{}.{}.{}, {}.{}.{}.{}",
                Random::next() % 256,
                Random::next() % 256,
                Random::next() % 256,
                Random::next() % 256,
                Random::next() % 256,
                Random::next() % 256,
                Random::next() % 256,
                Random::next() % 256,
            );
            self.request
                .header
                .insert("X-Forwarded-For".to_string(), ip);
        } else {
            self.request
                .header
                .insert("X-Forwarded-For".to_string(), s.unwrap().to_string());
        }
        return self;
    }

    pub fn set_verbose(&mut self, v: bool) -> &mut Self {
        self.verbose = v;
        return self;
    }

    async fn send_001(
        &mut self,
        mut proxy: Option<Arc<TokioMutex<Box<dyn Proxy>>>>,
    ) -> Result<Response, IceyeeError> {
        use async_compression::tokio::bufread::GzipDecoder;
        use std::ops::DerefMut;

        if proxy.is_none() {
            proxy = Some(
                NoProxy::new(
                    self.request.host.as_str(),
                    self.request.port,
                    self.using_ssl,
                )
                .wrap(),
            );
        }
        let proxy = proxy.unwrap();
        let mut proxy = proxy.lock().await;
        self.log.push_str("\r\n---- Start ----\r\n");
        // 1 连接.
        if proxy.is_closed() {
            proxy.connect().await?;
        }
        if proxy.is_closed() {
            return Err(IceyeeError::from("连接失败."));
        }
        // 2 请求头.
        for (key, value) in [
            ("Accept", "*/*"),
            ("Accept-Encoding", "gzip"),
            ("Accept-Language", "zh"),
            ("Connection", "keep-alive"),
            ("Content-Length", "0"),
            ("User-Agent", "ICEYEE/1"),
            ("X-Requested-With", "XMLHttpRequest"),
        ] {
            if !self.request.header.contains_key(key) {
                self.request
                    .header
                    .insert(key.to_string(), value.to_string());
            }
        }
        // 3 写请求头.
        let header: String = self.request.to_string();
        self.log.push_str("\r\n---- Request ----\r\n");
        self.log.push_str(&header);
        proxy
            .write(header.as_bytes())
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
        // 4 写请求正文.
        match String::from_utf8(self.request.body.clone()) {
            Ok(s) => self.log.push_str(&s),
            _ => self.log.push_str("[body]"),
        }
        if self.request.body.len() != 0 {
            proxy
                .write(self.request.body.as_slice())
                .await
                .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
        }
        // 5 解析响应.
        self.log.push_str("\r\n---- Response ----\r\n");
        let mut response = Response::read_from(proxy.deref_mut())
            .await
            .map_err(|e| IceyeeError::from(Box::new(e) as Box<dyn StdError>))?;
        self.log.push_str(response.to_string().as_str());
        if response.header.contains_key("Content-Encoding")
            && response.header.get("Content-Encoding").unwrap()[0]
                .to_lowercase()
                .contains("gzip")
        {
            let mut body: Vec<u8> = Vec::new();
            GzipDecoder::new(response.body.as_slice())
                .read_to_end(&mut body)
                .await
                .map_err(|e| Box::new(e) as Box<dyn StdError>)?;
            response.body = body;
        }
        match String::from_utf8(response.body.clone()) {
            Ok(s) => self.log.push_str(&s),
            _ => self.log.push_str("[body]"),
        }
        // Connection.
        if response.header.contains_key("Connection")
            && response.header.get("Connection").unwrap()[0]
                .to_lowercase()
                .contains("close")
        {
            self.log.push_str("\r\n---- Connection close ----\r\n");
            proxy.close();
        }
        return Ok(response);
    }

    pub async fn send(
        &mut self,
        proxy: Option<Arc<TokioMutex<Box<dyn Proxy>>>>,
    ) -> Result<Response, IceyeeError> {
        self.log.clear();
        let r = match self.send_001(proxy.clone()).await {
            Ok(response) => Ok(response),
            Err(e) => {
                self.log.push_str("\r\n---- Exception ----\r\n");
                self.log.push_str(e.to_string().as_str());
                self.log.push_str("\r\n---- Connection close ----\r\n");
                if proxy.is_some() {
                    proxy.unwrap().lock().await.close();
                }
                Err(e)
            }
        };
        self.log.push_str("\r\n---- End ----\r\n");
        if self.verbose {
            println!("{}", &self.log);
        }
        return r;
    }
}

// Function.
