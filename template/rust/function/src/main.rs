use openfaas_runtime::{Error, Request, Response};

extern crate openfaas_runtime;

async fn handle() -> Result<Response<Body>, Error> {}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
