pub trait Request {
    fn get_path(&self) -> &String;
    fn get_method(&self) -> &Method;

    fn get_user_agent(&self) -> &str {
        "github-activity"
    }
}

pub struct EventRequest {
    path: String,
    method: Method,
}

impl EventRequest {
    pub fn new(user: &str) -> Self {
        EventRequest {
            path: format!("/users/{}/events", user),
            method: Method::Get
        }
    }
}

impl Request for EventRequest {
    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_method(&self) -> &Method {
        &self.method
    }
}

pub enum Method {
    Get, Head, Post, Put, Delete, Connect, Options, Trace, Patch
}

impl Method {
    pub fn raw_value(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH"
        }
    }
}