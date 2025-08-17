use log_args::params;
use tracing::{info, Level};

fn init_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}

// Logs all parameters. Use only in safe/debug contexts.
#[params(all)]
fn store_payment(card_number: String, amount: f64) {
    info!("storing payment");
}

fn main() {
    init_subscriber();
    store_payment("4111111111111111".into(), 99.50);
}
