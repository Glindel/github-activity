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
    Get,
    // Not used for now
    // Head, Post, Put, Delete, Connect, Options, Trace, Patch
}