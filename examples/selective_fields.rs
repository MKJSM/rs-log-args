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
#[params(span(company_id, client_id))]
fn process_request(company_id: String, client_id: String, secret: String) {
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
