use log_args::params;
use tracing::{info, Level};

fn init_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}

struct Service {
    name: String,
}

impl Service {
    #[params(span(service_name), fields(op))]
    fn handle(&self, op: String) {
        let service_name = self.name.clone();
        info!("handling op");
    }
}

fn main() {
    init_subscriber();
    let svc = Service { name: "orders".into() };
    svc.handle("create".into());
}
