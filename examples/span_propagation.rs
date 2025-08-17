use log_args::params;
use tracing::{debug, info, Level};
use tokio::task;

fn init_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}

#[params(span(tenant_id, session_id))]
async fn child() {
    debug!("child running");
}

#[params(span(tenant_id, session_id))]
async fn parent(tenant_id: String, session_id: String) {
    info!("parent start");

    // Same-task propagation
    child().await;

    // Cross-task propagation
    let h = task::spawn(async move {
        child().await;
    });
    h.await.unwrap();

    info!("parent end");
}

#[tokio::main]
async fn main() {
    init_subscriber();
    parent("acme".into(), "sess-123".into()).await;
}
