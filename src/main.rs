use std::fs;
use std::io;

use futures::{future, Future};
use hyper::{Body, Request, Response, Server};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::service::service_fn;

async fn hello(_: Request<Body>) -> impl Future<Output = Result<Response<Body>, hyper::http::Error>> {
    let foo = "FOO";
    future::ready(
        Response::builder()
            .header(CONTENT_TYPE, "text/plain")
            .header(CONTENT_LENGTH, foo.len())
            .body(foo.into())
    )
}

fn run() -> io::Result<()> {
    let path = "foo.sock";
    if let Err(err) = fs::remove_file(path) {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err);
        }
    }
    let srv = Server::bind_unix(path, service_fn(hello))?;
    srv.run()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("Error starting server: {}", err)
    }
}
