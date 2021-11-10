extern crate openfaas_runtime;

use log::debug;
use std::sync::Arc;

#[derive(Clone)]
struct Greeter {
    greet: &'static str,
}

impl Greeter {
    fn new(greet: &'static str) -> Self {
        Self { greet }
    }

    fn greet(&self, person: &str) -> String {
        format!("{} {}", self.greet, person)
    }
}

fn handler(req: serde_json::Value, greeter: Arc<Greeter>) -> String {
    debug!("Received request: {}", req);

    let response = if req["name"].is_string() {
        greeter.greet(req["name"].as_str().unwrap())
    } else {
        "Doon't know the guy".to_string()
    };
    debug!("Responding: {}", response);

    response
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let greeter = Arc::new(Greeter::new("Hello"));
    let handler = move |req: serde_json::Value| handler(req, greeter.clone());

    openfaas_runtime::run(handler).await;
}
