use log_args::params;
use tracing::{info, Level};

fn init_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}

// Add custom static fields alongside parameters
#[params(custom(service = "orders", version = "v1"))]
fn create_order(_order_id: String, _user_id: String) {
    info!("Created order");
}

fn main() {
    init_subscriber();
    create_order("ord_001".into(), "user_42".into());
}
