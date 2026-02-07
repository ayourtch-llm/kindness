use solution::*;

#[test]
fn basic_route_matching() {
    use std::collections::HashMap;

    let mut router = Router::new();
    router.add_route("GET", "/hello", Box::new(|_req| Response {
        status: 200,
        body: "Hello, World!".into(),
    }));

    let resp = router.route("GET", "/hello");
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, "Hello, World!");
}

#[test]
fn path_params_extraction() {
    use std::collections::HashMap;

    let mut router = Router::new();
    router.add_route("GET", "/users/:id", Box::new(|req| Response {
        status: 200,
        body: format!("User {}", req.params.get("id").unwrap()),
    }));

    let resp = router.route("GET", "/users/42");
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, "User 42");
}

#[test]
fn multiple_path_params() {
    use std::collections::HashMap;

    let mut router = Router::new();
    router.add_route("GET", "/users/:uid/posts/:pid", Box::new(|req| Response {
        status: 200,
        body: format!("u={},p={}", req.params.get("uid").unwrap(), req.params.get("pid").unwrap()),
    }));

    let resp = router.route("GET", "/users/5/posts/99");
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, "u=5,p=99");
}

#[test]
fn method_not_matched() {
    use std::collections::HashMap;

    let mut router = Router::new();
    router.add_route("POST", "/data", Box::new(|_req| Response {
        status: 201,
        body: "Created".into(),
    }));

    let resp = router.route("GET", "/data");
    assert_eq!(resp.status, 404);
    assert_eq!(resp.body, "Not Found");
}

#[test]
fn not_found() {
    use std::collections::HashMap;

    let mut router = Router::new();
    let resp = router.route("GET", "/nonexistent");
    assert_eq!(resp.status, 404);
    assert_eq!(resp.body, "Not Found");
}

#[test]
fn middleware_short_circuit() {
    use std::collections::HashMap;

    let mut router = Router::new();
    router.add_middleware(Box::new(|req| {
        if req.path == "/blocked" {
            Some(Response { status: 403, body: "Forbidden".into() })
        } else {
            None
        }
    }));
    router.add_route("GET", "/blocked", Box::new(|_req| Response {
        status: 200,
        body: "Should not reach".into(),
    }));
    router.add_route("GET", "/allowed", Box::new(|_req| Response {
        status: 200,
        body: "OK".into(),
    }));

    let resp = router.route("GET", "/blocked");
    assert_eq!(resp.status, 403);
    assert_eq!(resp.body, "Forbidden");

    let resp2 = router.route("GET", "/allowed");
    assert_eq!(resp2.status, 200);
    assert_eq!(resp2.body, "OK");
}

#[test]
fn case_insensitive_method() {
    use std::collections::HashMap;

    let mut router = Router::new();
    router.add_route("get", "/test", Box::new(|_req| Response {
        status: 200,
        body: "ok".into(),
    }));

    let resp = router.route("GET", "/test");
    assert_eq!(resp.status, 200);
}
