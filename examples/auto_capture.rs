use log_args::params;
use log_args_runtime::{auto_capture_context, push_context};
use tracing::{info, Level};

fn init_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}

#[params(span(tenant_id, session_id))]
fn leaf() {
    info!("leaf invoked");
}

#[params(span(tenant_id, session_id))]
fn mid() {
    // Capture current context for any nested calls done indirectly
    let _g = auto_capture_context();
    leaf();
}

fn main() {
    init_subscriber();

    // Seed context manually (e.g., from middleware) and call functions
    let _guard = push_context([
        ("tenant_id".to_string(), "acme".to_string()),
        ("session_id".to_string(), "sess-123".to_string()),
    ]
    .into_iter()
    .collect());

    mid();
}
