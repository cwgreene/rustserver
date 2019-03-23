extern crate hyper;
extern crate itertools;

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use itertools::Itertools;

fn hello_world(_req: Request<Body>) -> Response<Body> {
    let headers_string = _req.headers().iter()
        .map(|(key, value)| { format!("{}: {:?}", key, value) })
        .join("\n");
    Response::new(Body::from(headers_string))
}

fn main() {
    let addr = ([127,0,0,1], 3000).into();
    let new_svc = || {
        service_fn_ok(hello_world)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));
    hyper::rt::run(server)
}
