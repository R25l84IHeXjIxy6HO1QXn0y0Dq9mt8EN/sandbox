use std::error::Error;
use std::fs;
use std::io;

use futures::{future, Future};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyperlocal::UnixServerExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let path = "foo.sock";
    if let Err(err) = fs::remove_file(path) {
        if err.kind() != io::ErrorKind::NotFound {
            panic!("FAILED TO REMOVE EXISTING SOCKET");
        }
    }
    let srv = Server::bind_unix(path)?
        .serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(|_req| async {
                Ok::<_, hyper::Error>(Response::new(Body::from("FOO")))
            }))
        }))
        .await?;
    Ok(())
}
