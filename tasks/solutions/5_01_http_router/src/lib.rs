use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

struct Route {
    method: String,
    segments: Vec<Segment>,
    handler: Box<dyn Fn(&Request) -> Response>,
}

enum Segment {
    Literal(String),
    Param(String),
}

pub struct Router {
    routes: Vec<Route>,
    middlewares: Vec<Box<dyn Fn(&Request) -> Option<Response>>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, mw: Box<dyn Fn(&Request) -> Option<Response>>) {
        self.middlewares.push(mw);
    }

    pub fn add_route(
        &mut self,
        method: &str,
        path: &str,
        handler: Box<dyn Fn(&Request) -> Response>,
    ) {
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                if let Some(name) = s.strip_prefix(':') {
                    Segment::Param(name.to_string())
                } else {
                    Segment::Literal(s.to_string())
                }
            })
            .collect();
        self.routes.push(Route {
            method: method.to_uppercase(),
            segments,
            handler,
        });
    }

    pub fn route(&self, method: &str, path: &str) -> Response {
        let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let method_upper = method.to_uppercase();

        for route in &self.routes {
            if route.method != method_upper {
                continue;
            }
            if route.segments.len() != path_parts.len() {
                continue;
            }
            let mut params = HashMap::new();
            let mut matched = true;
            for (seg, part) in route.segments.iter().zip(path_parts.iter()) {
                match seg {
                    Segment::Literal(lit) => {
                        if lit != part {
                            matched = false;
                            break;
                        }
                    }
                    Segment::Param(name) => {
                        params.insert(name.clone(), part.to_string());
                    }
                }
            }
            if !matched {
                continue;
            }

            let req = Request {
                method: method_upper.clone(),
                path: path.to_string(),
                params,
                headers: HashMap::new(),
                body: String::new(),
            };

            for mw in &self.middlewares {
                if let Some(resp) = mw(&req) {
                    return resp;
                }
            }

            return (route.handler)(&req);
        }

        Response {
            status: 404,
            body: "Not Found".into(),
        }
    }
}
