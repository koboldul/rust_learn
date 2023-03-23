use std::collections::HashMap;

pub struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

pub struct Response {
    pub code: u32,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

type RouteCallback = Box<dyn Fn(&Request) -> Response>;
pub struct BasicRouter {
    routes: HashMap<String, RouteCallback>
}

impl BasicRouter {
    pub fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new()
        }
    }

    pub fn add_route<C>(&mut self, path: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static 
    {
        self.routes.insert(path.to_string(), Box::new(callback));
    }

    pub fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            Some(callback) => callback(request),
            None => Response {
                code: 404,
                headers: HashMap::new(),
                body: Vec::new()
            }
        }
    }
}