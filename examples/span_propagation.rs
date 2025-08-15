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

#[params(span(company_id, client_id))]
async fn child() {
    debug!("child running");
}

#[params(span(company_id, client_id))]
async fn parent(company_id: String, client_id: String) {
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
    parent("acme".into(), "client-123".into()).await;
}
