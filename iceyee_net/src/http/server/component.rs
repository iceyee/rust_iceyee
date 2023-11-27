// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//

//! 一些组件.

// Use.

use crate::http::server::Context;
use crate::http::server::Filter;
use crate::http::server::R;
use crate::http::Status;
use std::collections::HashMap;
use std::collections::HashSet;

// Enum.

// Trait.

// Struct.

// Function.

#[derive(Clone, Debug)]
pub(in crate::http::server) struct FilterHost {
    full_hosts: HashSet<String>,
    usual_hosts: HashSet<String>,
}

impl FilterHost {
    pub fn new() -> FilterHost {
        let mut host_filter = FilterHost {
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
impl Filter for FilterHost {
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
            R::write_status(&mut context.response, Status::Forbidden(None));
        }
        return Ok(auth);
    }
}

#[derive(Clone, Debug)]
pub(in crate::http::server) struct FileRouter {
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
                R::write_status(&mut context.response, Status::NotFound(None));
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
            R::write_status(
                &mut context.response,
                Status::Forbidden(Some("禁止访问上级目录".to_string())),
            );
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
            R::write_status(
                &mut context.response,
                Status::BadRequest(Some("目标路径是个目录".to_string())),
            );
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
        let body: String = context.e_message.as_ref().unwrap().clone();
        R::write_status(
            &mut context.response,
            Status::InternalServerError(Some(body)),
        );
        context
            .logger
            .error(context.e_message.as_ref().unwrap())
            .await;
        return true;
    }
}

/// 简单的用户认证.
#[derive(Clone, Debug)]
pub struct FilterBasicAuth {
    auth_string_s: HashSet<String>,
}

impl FilterBasicAuth {
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
impl Filter for FilterBasicAuth {
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
            R::write_status(&mut context.response, Status::Unauthorized(None));
            context.response.header.insert(
                "WWW-Authenticate".to_string(),
                vec!["Basic realm=\"Realm\"".to_string()],
            );
            return Ok(false);
        }
    }
}

/// CORS.
#[derive(Clone, Debug)]
pub struct FilterCORS {
    allow_origin: Option<String>,
    allow_methods: Option<String>,
    allow_headers: Option<String>,
}

impl FilterCORS {
    pub fn new() -> Self {
        return FilterCORS {
            allow_origin: None,
            allow_methods: None,
            allow_headers: None,
        };
    }

    pub fn allow_origin(mut self, origin: &str) -> Self {
        self.allow_origin = Some(origin.to_string());
        return self;
    }

    pub fn allow_methods(mut self, methods: &str) -> Self {
        self.allow_methods = Some(methods.to_string());
        return self;
    }

    pub fn allow_headers(mut self, headers: &str) -> Self {
        self.allow_headers = Some(headers.to_string());
        return self;
    }
}

#[async_trait::async_trait]
impl Filter for FilterCORS {
    async fn do_filter(&self, context: &mut Context) -> Result<bool, String> {
        if self.allow_origin.is_some() {
            context.response.header.insert(
                "Access-Control-Allow-Origin".to_string(),
                vec![self.allow_origin.as_ref().unwrap().clone()],
            );
        }
        if self.allow_methods.is_some() {
            context.response.header.insert(
                "Access-Control-Allow-Methods".to_string(),
                vec![self.allow_methods.as_ref().unwrap().clone()],
            );
        }
        if self.allow_headers.is_some() {
            context.response.header.insert(
                "Access-Control-Allow-Headers".to_string(),
                vec![self.allow_headers.as_ref().unwrap().clone()],
            );
        }
        return Ok(true);
    }
}