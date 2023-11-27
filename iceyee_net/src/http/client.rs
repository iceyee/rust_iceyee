// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

//! 客户端接口.

pub use crate::http::Request;
pub use crate::http::Response;

use crate::http::Args;
use crate::http::Url;
use crate::http::UrlError;
use std::io::Error as StdIoError;
use std::io::ErrorKind as StdIoErrorKind;
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

/// 抽象代理.
#[async_trait::async_trait]
pub trait Proxy: AsyncRead + AsyncWrite + Unpin {
    /// 连接目标服务器或代理服务器.
    async fn connect(&mut self) -> Result<(), StdIoError>;

    /// 关闭连接.
    fn close(&mut self);

    /// 是否已关闭.
    fn is_closed(&self) -> bool;

    /// 将自己打包.
    fn wrap(self) -> Arc<TokioMutex<Box<dyn Proxy + Send + Sync>>>
    where
        Self: Sized + Send + Sync + 'static,
    {
        return Arc::new(TokioMutex::new(Box::new(self)));
    }
}

// Struct.

/// 不使用代理.
#[derive(Debug)]
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
    async fn connect(&mut self) -> Result<(), StdIoError> {
        let plain_socket: TokioTcpStream =
            TokioTcpStream::connect((self.target_host.clone(), self.target_port)).await?;
        if !self.using_ssl {
            self.plain_socket = Some(plain_socket);
        } else {
            let connector = tokio_native_tls::native_tls::TlsConnector::new()
                .map_err(|e| StdIoError::new(StdIoErrorKind::Other, e.to_string()))?;
            let connector = tokio_native_tls::TlsConnector::from(connector);
            let ssl_socket: TlsStream<TokioTcpStream> = connector
                .connect(self.target_host.as_str(), plain_socket)
                .await
                .map_err(|e| StdIoError::new(StdIoErrorKind::Other, e.to_string()))?;
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
        return self.plain_socket.is_none() && self.ssl_socket.is_none();
    }
}

/// Http代理.
#[derive(Debug)]
pub struct HttpProxy {
    target_host: String,
    target_port: u16,
    using_ssl: bool,
    plain_socket: Option<TokioTcpStream>,
    ssl_socket: Option<TlsStream<TokioTcpStream>>,
    proxy_host: String,
    proxy_port: u16,
    proxy_auth: Option<String>,
}

impl HttpProxy {
    pub fn new(
        target_host: &str,
        target_port: u16,
        using_ssl: bool,
        proxy_host: &str,
        proxy_port: u16,
        proxy_auth: Option<&str>,
    ) -> HttpProxy {
        return HttpProxy {
            target_host: target_host.to_string(),
            target_port: target_port,
            using_ssl: using_ssl,
            plain_socket: None,
            ssl_socket: None,
            proxy_host: proxy_host.to_string(),
            proxy_port: proxy_port,
            proxy_auth: proxy_auth.map(|s| s.to_string()),
        };
    }
}

#[async_trait::async_trait]
impl Proxy for HttpProxy {
    async fn connect(&mut self) -> Result<(), StdIoError> {
        use iceyee_encoder::Base64Encoder;

        // 1 连接代理.
        let mut plain_socket: TokioTcpStream =
            TokioTcpStream::connect((self.proxy_host.clone(), self.proxy_port)).await?;
        // 2 CONNECT.
        let mut request: Request = Request::new();
        request.method = "CONNECT".to_string();
        request.path = self.target_host.clone() + ":" + self.target_port.to_string().as_str();
        request
            .header
            .insert("Host".to_string(), request.path.clone());
        request
            .header
            .insert("Proxy-Connection".to_string(), "keep-alive".to_string());
        if self.proxy_auth.is_some() {
            let auth: String =
                Base64Encoder::encode(self.proxy_auth.as_ref().unwrap().as_bytes().to_vec());
            let auth: String = "Basic ".to_string() + auth.trim();
            request
                .header
                .insert("Authorization".to_string(), auth.clone());
            request
                .header
                .insert("Proxy-Authenticate".to_string(), auth.clone());
            request
                .header
                .insert("Proxy-Authorization".to_string(), auth.clone());
        }
        plain_socket.write(request.to_string().as_bytes()).await?;
        // CONNECT响应.
        let response: Response = Response::read_from(&mut plain_socket).await?;
        if 200 <= response.status_code && response.status_code < 300 {
            // 请求代理连接成功.
        } else {
            // 请求代理连接失败.
            let message: String = format!(
                "请求代理连接失败 @proxy='{}:{}'",
                &self.proxy_host, self.proxy_port
            );
            Err(StdIoError::new(StdIoErrorKind::ConnectionRefused, message))?;
        }
        // 3 tls握手.
        if !self.using_ssl {
            self.plain_socket = Some(plain_socket);
        } else {
            let connector = tokio_native_tls::native_tls::TlsConnector::new()
                .map_err(|e| StdIoError::new(StdIoErrorKind::Other, e.to_string()))?;
            let connector = tokio_native_tls::TlsConnector::from(connector);
            let ssl_socket: TlsStream<TokioTcpStream> = connector
                .connect(self.target_host.as_str(), plain_socket)
                .await
                .map_err(|e| StdIoError::new(StdIoErrorKind::Other, e.to_string()))?;
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
        return self.plain_socket.is_none() && self.ssl_socket.is_none();
    }
}

/// Socks5代理.
#[derive(Debug)]
pub struct Socks5Proxy {
    target_host: String,
    target_port: u16,
    using_ssl: bool,
    plain_socket: Option<TokioTcpStream>,
    ssl_socket: Option<TlsStream<TokioTcpStream>>,
    proxy_host: String,
    proxy_port: u16,
    proxy_auth: Option<String>,
}

impl Socks5Proxy {
    pub fn new(
        target_host: &str,
        target_port: u16,
        using_ssl: bool,
        proxy_host: &str,
        proxy_port: u16,
        proxy_auth: Option<&str>,
    ) -> Socks5Proxy {
        return Socks5Proxy {
            target_host: target_host.to_string(),
            target_port: target_port,
            using_ssl: using_ssl,
            plain_socket: None,
            ssl_socket: None,
            proxy_host: proxy_host.to_string(),
            proxy_port: proxy_port,
            proxy_auth: proxy_auth.map(|s| s.to_string()),
        };
    }
}

#[async_trait::async_trait]
impl Proxy for Socks5Proxy {
    async fn connect(&mut self) -> Result<(), StdIoError> {
        // 1 连接代理.
        let mut plain_socket: TokioTcpStream =
            TokioTcpStream::connect((self.proxy_host.clone(), self.proxy_port)).await?;
        // 2 认证.
        // client:
        // +----+----------+----------+
        // |VER | NMETHODS | METHODS  |
        // +----+----------+----------+
        // | 1  |    1     | 1 to 255 |
        // +----+----------+----------+
        // server
        // +----+--------+
        // |VER | METHOD |
        // +----+--------+
        // | 1  |    1   |
        // +----+--------+
        // The values currently defined for METHOD are:
        //   o X’00’ NO AUTHENTICATION REQUIRED
        //   o X’01’ GSSAPI
        //   o X’02’ USERNAME/PASSWORD
        //   o X’03’ to X’7F’ IANA ASSIGNED
        //   o X’80’ to X’FE’ RESERVED FOR PRIVATE METHODS
        //   o X’FF’ NO ACCEPTABLE METHODS
        plain_socket.write(&[0x05u8, 0x02, 0x00, 0x02]).await?;
        let mut buffer: [u8; 0xFFF] = [0; 0xFFF];
        let length: usize = plain_socket.read(&mut buffer).await?;
        if length != 2 {
            Err(StdIoError::new(StdIoErrorKind::Other, "非预期."))?;
        }
        if buffer[1] == 0xFF {
            // 认证被拒绝.
            Err(StdIoError::new(
                StdIoErrorKind::ConnectionRefused,
                "代理认证被拒绝.",
            ))?;
        } else if buffer[1] == 0x02 {
            // USERNAME/PASSWORD.
            // client:
            // +----+------+----------+------+----------+
            // |VER | ULEN |  UNAME   | PLEN |  PASSWD  |
            // +----+------+----------+------+----------+
            // | 1  |  1   | 1 to 255 |  1   | 1 to 255 |
            // +----+------+----------+------+----------+
            // VER: 0x01
            if self.proxy_auth.is_some() && self.proxy_auth.as_ref().unwrap().contains(":") {
                let auth: String = self.proxy_auth.as_ref().unwrap().clone();
                let mut a001 = auth.splitn(2, ":");
                let username: String = a001.next().unwrap().to_string();
                let username: &[u8] = username.as_bytes();
                let password: String = a001.next().unwrap().to_string();
                let password: &[u8] = password.as_bytes();
                let mut auth: Vec<u8> = Vec::new();
                auth.push(0x01);
                auth.push(username.len() as u8);
                auth.extend_from_slice(username);
                auth.push(password.len() as u8);
                auth.extend_from_slice(password);
                plain_socket.write(&auth).await?;
                // server:
                // +----+--------+
                // |VER | STATUS |
                // +----+--------+
                // | 1  |   1    |
                // +----+--------+
                // 返回STATUS, 0x00表示成功.
                let length: usize = plain_socket.read(&mut buffer).await?;
                if length != 2 {
                    Err(StdIoError::new(StdIoErrorKind::Other, "非预期."))?;
                } else if buffer[1] != 0x00 {
                    Err(StdIoError::new(
                        StdIoErrorKind::ConnectionRefused,
                        "代理认证失败.",
                    ))?;
                }
            }
        }
        // 3 CONNECT.
        // client:
        // +----+-----+-------+------+----------+----------+
        // |VER | CMD |  RSV  | ATYP | DST.ADDR | DST.PORT |
        // +----+-----+-------+------+----------+----------+
        // | 1  |  1  | X’00’ |  1   | Variable |     2    |
        // +----+-----+-------+------+----------+----------+
        // o VER protocol version: X’05’
        // o CMD
        //   o CONNECT X’01’
        //   o BIND X’02’
        //   o UDP ASSOCIATE X’03’
        // o RSV RESERVED
        // o ATYP address type of following address
        //   o IP V4 address: X’01’
        //   o DOMAINNAME: X’03’    1*(长度) + *(地址)
        //   o IP V6 address: X’04’
        // o DST.ADDR desired destination address
        // o DST.PORT desired destination port in network octet order 大端序
        let mut request: Vec<u8> = Vec::new();
        request.push(0x05);
        request.push(0x01);
        request.push(0x00);
        request.push(0x03);
        request.push(self.target_host.len() as u8);
        request.extend_from_slice(self.target_host.as_bytes());
        request.push(((self.target_port >> 8) & 0xFF) as u8);
        request.push(((self.target_port >> 0) & 0xFF) as u8);
        plain_socket.write(&request).await?;
        // server:
        // +----+-----+-------+------+----------+----------+
        // |VER | REP |  RSV  | ATYP | BND.ADDR | BND.PORT |
        // +----+-----+-------+------+----------+----------+
        // | 1  |  1  | X’00’ |  1   | Variable |    2     |
        // +----+-----+-------+------+----------+----------+
        // o VER protocol version: X’05’
        // o REP Reply field:
        //   o X’00’ succeeded
        //   o X’01’ general SOCKS server failure
        //   o X’02’ connection not allowed by ruleset
        //   o X’03’ Network unreachable
        //   o X’04’ Host unreachable
        //   o X’05’ Connection refused
        //   o X’06’ TTL expired
        //   o X’07’ Command not supported
        //   o X’08’ Address type not supported
        //   o X’09’ to X’FF’ unassigned
        // o RSV RESERVED
        // o ATYP address type of following address
        //   o IP V4 address: X’01’
        //   o DOMAINNAME: X’03’
        //   o IP V6 address: X’04’
        // o BND.ADDR server bound address
        // o BND.PORT server bound port in network octet order
        let length: usize = plain_socket.read(&mut buffer).await?;
        if length < 4 {
            Err(StdIoError::new(StdIoErrorKind::Other, "非预期."))?;
        }
        if buffer[1] == 0x00 {
            // succeeded.
        } else if buffer[1] == 0x01 {
            Err(StdIoError::new(
                StdIoErrorKind::Other,
                "general SOCKS server failure",
            ))?;
        } else if buffer[1] == 0x02 {
            Err(StdIoError::new(
                StdIoErrorKind::ConnectionRefused,
                "connection not allowed by ruleset",
            ))?;
        } else if buffer[1] == 0x03 {
            Err(StdIoError::new(
                StdIoErrorKind::NetworkUnreachable,
                "Network unreachable",
            ))?;
        } else if buffer[1] == 0x04 {
            Err(StdIoError::new(
                StdIoErrorKind::HostUnreachable,
                "Host unreachable",
            ))?;
        } else if buffer[1] == 0x05 {
            Err(StdIoError::new(
                StdIoErrorKind::ConnectionRefused,
                "Connection refused",
            ))?;
        } else if buffer[1] == 0x06 {
            Err(StdIoError::new(StdIoErrorKind::TimedOut, "TTL expired"))?;
        } else if buffer[1] == 0x07 {
            Err(StdIoError::new(
                StdIoErrorKind::InvalidInput,
                "Command not supported",
            ))?;
        } else if buffer[1] == 0x08 {
            Err(StdIoError::new(
                StdIoErrorKind::Unsupported,
                "Address type not supported",
            ))?;
        } else if buffer[1] == 0x09 {
            Err(StdIoError::new(
                StdIoErrorKind::Other,
                "to X’FF’ unassigned",
            ))?;
        }
        // 4 tls握手.
        if !self.using_ssl {
            self.plain_socket = Some(plain_socket);
        } else {
            let connector = tokio_native_tls::native_tls::TlsConnector::new()
                .map_err(|e| StdIoError::new(StdIoErrorKind::Other, e.to_string()))?;
            let connector = tokio_native_tls::TlsConnector::from(connector);
            let ssl_socket: TlsStream<TokioTcpStream> = connector
                .connect(self.target_host.as_str(), plain_socket)
                .await
                .map_err(|e| StdIoError::new(StdIoErrorKind::Other, e.to_string()))?;
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
        return self.plain_socket.is_none() && self.ssl_socket.is_none();
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

impl AsyncRead for HttpProxy {
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

impl AsyncRead for Socks5Proxy {
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

impl AsyncWrite for HttpProxy {
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

impl AsyncWrite for Socks5Proxy {
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
        return HttpClient {
            log: String::new(),
            request: Request::new(),
            using_ssl: false,
            verbose: false,
        };
    }

    pub fn set_url(mut self, s: &str) -> Result<Self, UrlError> {
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

    pub fn set_method(mut self, s: &str) -> Self {
        self.request.method = s.to_string();
        return self;
    }

    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        self.request
            .header
            .insert(key.to_string(), value.to_string());
        return self;
    }

    pub fn remove_header(mut self, key: &str) -> Self {
        self.request.header.remove(key);
        return self;
    }

    pub fn set_body(mut self, b: &[u8]) -> Self {
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

    pub fn set_forwarded(mut self, s: Option<&str>) -> Self {
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

    pub fn set_verbose(mut self, v: bool) -> Self {
        self.verbose = v;
        return self;
    }

    async fn send_001(
        &mut self,
        proxy: &mut tokio::sync::MutexGuard<'_, Box<dyn Proxy + Send + Sync>>,
    ) -> Result<Response, StdIoError> {
        use async_compression::tokio::bufread::GzipDecoder;
        use std::ops::DerefMut;

        self.log.push_str("\r\n---- Start ----\r\n");
        // 1 连接.
        if proxy.is_closed() {
            proxy.connect().await?;
        }
        if proxy.is_closed() {
            Err(StdIoError::new(
                StdIoErrorKind::ConnectionReset,
                "连接失败.",
            ))?;
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
        proxy.write(header.as_bytes()).await?;
        // 4 写请求正文.
        match String::from_utf8(self.request.body.clone()) {
            Ok(s) => self.log.push_str(&s),
            _ => self.log.push_str("[body is not utf-8.]"),
        }
        if self.request.body.len() != 0 {
            proxy.write(self.request.body.as_slice()).await?;
        }
        // 5 解析响应.
        self.log.push_str("\r\n---- Response ----\r\n");
        let mut response = Response::read_from(proxy.deref_mut()).await?;
        self.log.push_str(response.to_string().as_str());
        if response.header.contains_key("Content-Encoding")
            && response.header.get("Content-Encoding").unwrap()[0]
                .to_lowercase()
                .contains("gzip")
        {
            let mut body: Vec<u8> = Vec::new();
            GzipDecoder::new(response.body.as_slice())
                .read_to_end(&mut body)
                .await?;
            response.body = body;
        }
        match String::from_utf8(response.body.clone()) {
            Ok(s) => self.log.push_str(&s),
            _ => self.log.push_str("[body is not utf-8.]"),
        }
        // Connection.
        if response
            .header
            .get("Connection")
            .unwrap_or(&vec![String::new()])[0]
            .to_lowercase()
            .contains("close")
        {
            self.log.push_str("\r\n---- Connection close ----\r\n");
            proxy.close();
        }
        return Ok(response);
    }

    pub async fn send(
        mut self,
        mut proxy: Option<Arc<TokioMutex<Box<dyn Proxy + Send + Sync>>>>,
    ) -> Result<Response, StdIoError> {
        if self.request.port == 0 {
            Err(StdIoError::new(StdIoErrorKind::Other, "未设置url."))?;
        }
        self.log.clear();
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
        let r = match self.send_001(&mut proxy).await {
            Ok(response) => Ok(response),
            Err(e) => {
                self.log.push_str("\r\n---- Exception ----\r\n");
                self.log.push_str(e.to_string().as_str());
                self.log.push_str("\r\n---- Connection close ----\r\n");
                proxy.close();
                Err(e)
            }
        };
        self.log.push_str("\r\n---- End ----\r\n");
        if self.verbose {
            println!("{}", self.log);
        }
        return r;
    }
}

// Function.
