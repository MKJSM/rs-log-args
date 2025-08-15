# log-args-runtime

Runtime support crate for the `log_args` procedural macros. It provides:

- A lightweight context store that merges sync/async context across boundaries
- Drop-in logging macros (`info!`, `warn!`, `error!`, `debug!`, `trace!`) that enrich events with inherited context
- `log_with_context!` to forward to underlying `tracing` macros while attaching context
- Optional `with_context` feature to include a structured `context` field in events (ideal with JSON + `flatten_event(true)`)

## Install

```toml
[dependencies]
log-args-runtime = { version = "0.1", features = ["with_context"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
```

- Disable `with_context` if you do not want the context map emitted (macro instrumentation still works; fields may be logged directly by the macros you use).

## Subscriber setup

To surface context fields at the top-level of your JSON logs, initialize `tracing-subscriber` with JSON output and flattened events:

```rust
use tracing::Level;

fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}
```

## Quick example

```rust
use tracing::info;

fn main() {
    init_logging();

    // Push some request-scoped context
    log_args_runtime::push_context(|ctx| {
        ctx.insert("company_id".into(), "acme".into());
        ctx.insert("client_id".into(), "web".into());
    });

    // Logs will include the merged context when `with_context` is enabled
    log_args_runtime::info!("hello from runtime");
}

fn init_logging() {
    tracing_subscriber::fmt().json().flatten_event(true).init();
}
```

If you are using the `log_args` attribute macros, they will redefine local logging macros (e.g. `info!`) inside annotated functions so that your function and span fields are automatically included.

## Relationship to `log_args`

- `log_args` (proc-macro) rewrites your functions, capturing parameters/fields and wiring per-function logging behavior.
- `log-args-runtime` (this crate) holds the context and provides enriched logging macros. The proc-macro depends on this at expand/run time.

## Examples

This repository includes runnable examples at the workspace root (not this crate’s directory). From the repository root:

```bash
cargo run --example basic_usage
cargo run --example selective_fields
cargo run --example span_propagation
```

Ensure the `with_context` feature is enabled for this crate in your workspace if you want the `context` map included.

## License

MIT
