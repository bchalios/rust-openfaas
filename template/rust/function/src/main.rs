extern crate openfaas_runtime;
use openfaas_runtime::Error;

use log::debug;
use std::sync::Arc;

struct Greeter {
    greet: String,
}

impl Greeter {
    /// Create a new Greeter object defining
    /// the greet
    fn new(greet: &str) -> Self {
        Self {
            greet: greet.to_string(),
        }
    }

    /// Greet someone
    fn greet(&self, person: &str) -> String {
        format!("{} {}", self.greet, person)
    }
}

async fn handler(req: serde_json::Value, greeter: Arc<Greeter>) -> Result<String, String> {
    debug!("Received request: {}", req);

    let response = if req["name"].is_string() {
        greeter.greet(req["name"].as_str().unwrap())
    } else {
        "Doon't know the guy".to_string()
    };
    debug!("Responding: {}", response);

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    // Create the object that will be shared across invocation calls
    let greeter = Arc::new(Greeter::new("Hello"));

    // Define our handler closure.
    //
    // The runtime expects an `FnOnce(serde_json::Value) -> Future<Output=Result<Resp, Into<Error>>>
    // where Resp: Serialize`.
    //
    // So what we do here is create a closure with this type
    // which captures the a reference to the Greeter object
    // and calls the actual handler passing the reference as
    // an argument.
    let handler = move |req: serde_json::Value| handler(req, greeter.clone());

    // Invoke the runtime
    openfaas_runtime::run(handler).await?;

    Ok(())
}
