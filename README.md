# Log Args - Structured Logging for Rust

[![Crates.io](https://img.shields.io/crates/v/log_args.svg)](https://crates.io/crates/log_args)
[![Docs.rs](https://docs.rs/log_args/badge.svg)](https://docs.rs/log_args)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A powerful Rust workspace providing **truly automatic context inheritance** for structured logging using procedural macros and the `tracing` ecosystem.

## 🚀 Quick Start

```rust
use log_args::params;
use tracing::info;

#[params(fields(username))]
fn authenticate_user(username: String, password: String) {
    info!("User authentication attempt");
    // Individual span fields automatically appear in JSON output
}
```

**Key Features:**
- 🎯 **Truly Automatic Context Inheritance** - Child functions inherit parent context with just `#[params]`
- 🌐 **Cross-Boundary Propagation** - Works across closures, async spawns, and WebSocket upgrades
- 🚀 **Zero-overhead logging** - Compile-time macro expansion
- 🔧 **Flexible field selection** - Log only what you need with `fields(...)`
- 📊 **Structured JSON output** - Perfect for log aggregation and analysis

## 📦 Workspace Structure

This workspace contains two main crates:

### [`log_args`](./log_args/) - Procedural Macro Crate
The main procedural macro that provides the `#[params]` attribute for automatic parameter logging and context propagation.

### [`log-args-runtime`](./log-args-runtime/) - Runtime Support
Runtime support providing context storage, enriched logging macros, and cross-boundary context inheritance.

## 🛠️ Installation

Add both crates to your `Cargo.toml`:

```toml
[dependencies]
log_args = "0.1.6"
log-args-runtime = { version = "0.1.4", features = ["with_context"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
```

## 📖 Usage

### Basic Setup

```rust
use log_args::params;
use tracing::{info, Level};

fn main() {
    // Initialize JSON logging with flattened events
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();

    process_user("john_doe".to_string(), "secret123".to_string());
}

// Default: Only span propagation and function name logging
#[params]
fn process_user(username: String, password: String) {
    info!("Processing user");
    validate_credentials(username, password);
}

// Span context propagation - child functions inherit context
#[params(span(username))]
fn validate_credentials(username: String, password: String) {
    info!("Validating credentials");
    // Child functions automatically inherit username context
}
```

### Advanced Features

```rust
// Custom fields with expressions
#[params(
    fields(user.id, user.name),
    custom(
        email_count = user.emails.len(),
        is_premium = user.subscription.tier == "premium"
    )
)]
fn analyze_user(user: User, api_key: String) {
    info!("Analyzing user data");
}

// Span context propagation
#[params(span(tenant_id, session_id))]
fn business_logic(tenant_id: String, session_id: String, data: String) {
    info!("Processing business logic");
    // Child functions automatically inherit tenant_id and session_id context
}
```

## 🧪 Examples

Run the included examples to see the features in action:

```bash
# Basic usage
cargo run --example basic_usage

# Context propagation across async boundaries
cargo run --example span_propagation

# Selective field logging
cargo run --example selective_fields

# Custom fields and metadata
cargo run --example custom_fields

# Method support
cargo run --example method_support

# All parameters logging (debug contexts)
cargo run --example all_parameters

# Auto-capture for nested calls
cargo run --example auto_capture
```

## 🔧 Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Building Documentation

```bash
# Build docs for all workspace members
cargo doc --workspace --open

# Build docs for specific crate
cargo doc -p log_args --open
cargo doc -p log-args-runtime --open
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy --workspace --all-targets --all-features
```

## 🏗️ Architecture

The system works through a two-part architecture:

1. **Compile-time**: The `#[params]` macro analyzes function signatures and generates code to capture parameters and set up context propagation.

2. **Runtime**: The `log-args-runtime` crate provides a context store that merges sync/async context across boundaries and enriched logging macros.

### Context Inheritance Flow

```
Parent Function [#[params(span(user_id))]]
    ↓ (automatic context propagation)
Child Function [#[params]]
    ↓ (inherits user_id automatically)
Grandchild Function [#[params]]
    ↓ (also inherits user_id)
```

## 📊 JSON Output Example

With proper setup, your logs will include structured fields:

```json
{
  "timestamp": "2024-01-01T12:00:00Z",
  "level": "INFO",
  "fields": {
    "message": "Processing user data",
    "user_id": "12345",
    "tenant_id": "acme-corp",
    "username": "john_doe"
  },
  "target": "my_app::user_service"
}
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built on top of the excellent [`tracing`](https://github.com/tokio-rs/tracing) ecosystem
- Inspired by the need for truly automatic context propagation in distributed systems
- Thanks to the Rust community for feedback and contributions

---

**Made with ❤️ by MKJS Tech (P) Ltd**
