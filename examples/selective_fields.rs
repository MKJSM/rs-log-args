use log_args::params;
use tracing::{info, Level};

fn init_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}

// Only include selected parameters as fields
#[params(span(tenant_id, session_id))]
fn process_request(_tenant_id: String, _session_id: String, secret: String) {
    let _ = secret; // not logged
    info!("Processing request");
}

fn main() {
    init_subscriber();
    process_request(
        "acme-corp".into(),
        "client-123".into(),
        "should-not-log".into(),
    );
}
