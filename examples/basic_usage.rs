use log_args::params;
use tracing::{info, Level};

fn init_subscriber() {
    // JSON formatter with flattened event fields. The optional `context` map
    // will only be included if the runtime feature `with_context` is enabled.
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}

#[params]
fn greet(name: String) {
    info!("Greeting user");
}

fn main() {
    init_subscriber();
    greet("Ada".to_string());
}
