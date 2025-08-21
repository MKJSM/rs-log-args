# log-args-runtime

[![Crates.io](https://img.shields.io/crates/v/log-args-runtime.svg)](https://crates.io/crates/log-args-runtime)
[![Docs.rs](https://docs.rs/log-args-runtime/badge.svg)](https://docs.rs/log-args-runtime)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MKJSM/log_args/blob/main/LICENSE)

Runtime support crate for the [`log_args`](../log_args/) procedural macros, providing context storage and enriched logging macros.

> **Note**: This is part of the [log-args workspace](../README.md). For complete setup and usage examples, see the [main documentation](../README.md).

## Features

- **Context Storage**: Lightweight store that merges sync/async context across boundaries
- **Enriched Logging**: Drop-in macros (`info!`, `warn!`, `error!`, `debug!`, `trace!`) with automatic context
- **Cross-Boundary Propagation**: Context inheritance across closures, async spawns, and WebSocket upgrades
- **Structured Output**: Optional `with_context` feature for JSON logging with flattened events

## Installation

```toml
[dependencies]
log-args-runtime = { version = "0.1.4", features = ["with_context"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
```

> **Note**: The `with_context` feature includes a structured `context` field in log events. Disable it if you only want direct field logging.

## Setup

For structured JSON logging with context fields at the top level:

```rust
fn init_logging() {
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .init();
}
```

## Usage

```rust
use tracing::info;

fn main() {
    init_logging();

    // Push request-scoped context
    log_args_runtime::push_context(|ctx| {
        ctx.insert("tenant_id".into(), "acme".into());
        ctx.insert("session_id".into(), "web".into());
    });

    // Logs include merged context when `with_context` is enabled
    log_args_runtime::info!("Processing request");
}
```

> **Integration**: When using `log_args` procedural macros, they automatically redefine logging macros within annotated functions to include function parameters and span context.

## Architecture

This crate works in tandem with the `log_args` procedural macro:

- **`log_args`**: Compile-time macro that rewrites functions to capture parameters and set up context propagation
- **`log-args-runtime`**: Runtime support providing context storage and enriched logging macros

## Examples

See the [workspace examples](../examples/) for complete demonstrations:

```bash
cargo run --example basic_usage
cargo run --example span_propagation
cargo run --example selective_fields
```

## License

MIT - See [LICENSE](../LICENSE) for details.
