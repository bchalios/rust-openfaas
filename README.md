# OpenFaas Rust Template

A Rust template for OpenFaas using the `of-watchdog`.

This is a Rust template for OpenFaas. It is based on the existing
[OpenFaas Rust template](https://github.com/openfaas-incubator/rust-http-template)
and the AWS lambda Rust [runtime](https://github.com/awslabs/aws-lambda-rust-runtime).

The motivation for this template is that the existing
[template](https://github.com/openfaas-incubator/rust-http-template)
does not allow the creation of functions with state that can be shared
across invocations.

The template uses `openfaas-runtime`, a lib crate that wraps `hyper`
and exposes a public function `openfaas_runtime::run` for launching the
service that will handle user requests.

```sh
$ faas template pull https://github.com/bchalios/rust-openfaas
$ faas new --list
Languages available as templates:
- rust
```

## rust-openfaas

This function handler supported by this template receives requests with
a `Body` that can be deserialized from a valid JSON object and returns an
arbitrary value that can be serialized.

For example,

```Rust
use serde::Deserialize

/// The kind of requests we are handling
#[derive(Deserialize)]
struct Person {
	name: String,
}

struct Greeter {
	greet: String,
}

impl Greeter {
	fn new(greet: &str) -> Self {
		Self {
			greet.to_string(),
		}
	}

	fn greet(&self, person: &str) -> String {
		format("{} {}", self.greet, person)
	}
}

/// Our function handler. It receives a `Person` and a shared
/// reference to a `Greeter` object. It returns a proper greet.
fn handler(who: Person, greeter: Arc<Greeter>) -> String {
	greeter.greet(&who.name)
}

#[tokio::main]
async fn main() {
	// We only create a single `Greeter` object once
	let greeter = Arc::new(Greeter::new("Hello"));
	
	// And we pass a shared reference to it to the function handler
	let handler = move |req: Person| handler(req, greeter.clone());

        // Invoke the runtime to handle incoming requests using our
        // function handler
	openfaas_runtime::run(handler).await;
}
```
