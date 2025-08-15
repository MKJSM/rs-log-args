
# log_args

[![Crates.io](https://img.shields.io/crates/v/log-args.svg)](https://crates.io/crates/log-args)
[![Docs.rs](https://docs.rs/log-args/badge.svg)](https://docs.rs/log_args)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MKJSM/log-args/blob/main/LICENSE)
[![Build Status](https://github.com/MKJSM/log-args/actions/workflows/publish.yml/badge.svg)](https://github.com/MKJSM/log-args/actions)

A procedural macro for structured logging of function arguments using the `tracing` crate, with **truly automatic context inheritance** across all boundaries.

This crate provides a procedural macro attribute `#[params]` that can be applied to functions to automatically log their arguments and propagate context. It seamlessly integrates with the `tracing` ecosystem for structured logging, offering enhanced flexibility for both synchronous and asynchronous functions.

- 🎯 **Truly Automatic Context Inheritance** - Child functions inherit parent context with just `#[params]`
- 🌐 **Cross-Boundary Propagation** - Works across closures, async spawns, and WebSocket upgrades
- 🚀 **Zero-overhead logging** - Compile-time macro expansion
- 🔧 **Flexible field selection** - Log only what you need with `fields(...)`
- 🏷️ **Custom metadata** - Add static fields with `custom(...)`
- 🔗 **Span context propagation** - Automatic context inheritance with `span`
- ⚡ **Full async support** - Works seamlessly with async/await
- 🎯 **Function name logging** - Configurable casing styles via Cargo features
- 🔒 **Security-conscious** - Selective logging to exclude sensitive data
- 📊 **Structured JSON output** - Perfect for log aggregation and analysis
- 🛠️ **Production-ready** - Comprehensive examples and robust error handling

## 🚀 Quick Start

### Basic Usage

**By default, `#[params]` only enables span propagation and function name logging:**

```rust
use log_args::params;
use tracing::info;

// Default behavior: Only span propagation and function name logging
#[params]
fn authenticate_user(username: String, password: String) {
    info!("User authentication attempt");
    // Your function logic here
}

// Selective parameter logging with fields()
#[params(fields(username))]
fn login_user(username: String, password: String) {
    info!("Login attempt");
}

// All parameter logging with 'all' attribute
#[params(all)]
fn debug_function(data: String, count: u32) {
    info!("Debug information");
}
```

**Output:**
```json
{
  "timestamp": "2024-01-01T12:00:00Z",
  "level": "INFO",
  "fields": {
    "message": "User authentication attempt"
  },
  "target": "my_app"
}
```

### Key Benefits

✅ **Zero Code Changes**: Child functions need only `#[params]` - no manual context handling  
✅ **Cross-Boundary**: Works across closures, async spawns, WebSocket upgrades, and more  
✅ **Automatic**: Context propagation happens transparently in the library  
✅ **Robust**: No more `context=""` in your logs  
✅ **Production-Ready**: Handles complex async scenarios seamlessly  

## 📖 Usage Guide

### 1. Basic Parameter Logging

The `#[params]` attribute automatically logs all function parameters:

```rust
use log_args::params;
use tracing::{info, warn, error};

#[derive(Debug, Clone)]

### Logging Options

The `#[params]` macro redefines the following `tracing` macros within the function body:
- `info!`
- `warn!`
- `error!`
- `debug!`
- `trace!`

Use these macros as you normally would - the function arguments will be automatically included in the output.

---

## 📋 Examples

The `examples/` directory contains runnable programs demonstrating each feature. Run them with `cargo run --example <name>`.

### Core Examples
- `basic_usage.rs` — minimal `#[params]` usage
- `selective_fields.rs` — log only selected parameters
- `custom_fields.rs` — attach static metadata

### Context Propagation
- `span_propagation.rs` — inherit context across async/await and spawned tasks

### Advanced/Other
- `all_parameters.rs` — log all params (debug/safe contexts only)
- `auto_capture.rs` — auto-capture current context for nested/indirect calls
- `method_support.rs` — using `#[params]` on impl methods

### Running Examples
```bash
cargo run --example basic_usage
cargo run --example selective_fields
cargo run --example custom_fields
cargo run --example span_propagation
cargo run --example all_parameters
cargo run --example auto_capture
cargo run --example method_support
```

---

## 🚀 Advanced Field Expressions

The macro supports **unlimited nesting depth** and **complex expressions** in field specifications:

### Unlimited Nesting
```rust
#[params(fields(
    user.profile.settings.theme.color,
    user.profile.settings.notifications.email,
    org.company.department.manager.contact.email
))]
fn process_user_settings(user: User, org: Organization, secret: String) {
    info!("Processing user settings"); // Only logs specified nested fields
}
```

### Method Calls and Complex Expressions
```rust
#[params(fields(
    user.name.len(),
    user.emails.is_empty(),
    user.tags.first(),
    config.settings.keys().count()
))]
fn analyze_user_data(user: User, config: Config, password: String) {
    info!("Analyzing user data"); // Evaluates expressions and logs results
}
```

### Custom Fields with Expressions
```rust
#[params(
    fields(user.id, user.name),
    custom(
        total_emails = user.emails.len(),
        is_premium = user.subscription.tier == "premium",
        account_age_days = (now() - user.created_at).num_days()
    )
)]
fn process_account(user: User, now: DateTime, api_key: String) {
    info!("Processing account"); // Combines fields and computed custom values
}
```

### Selective Logging Benefits
- **Security**: Exclude sensitive data (passwords, tokens, keys)
- **Privacy**: Log only necessary fields for debugging
- **Performance**: Reduce log volume and processing overhead
- **Compliance**: Fine-grained control over logged data

---

## 🔍 How It Works

The `#[params]` macro:

1. Analyzes the function signature to find available arguments
2. Processes attribute options like `fields(...)` and `custom(...)`
3. Redefines tracing macros within the function scope to automatically include the specified fields
4. With `clone_upfront`, ensures values are safely cloned to prevent ownership issues in async contexts

The macro does not add overhead beyond the normal cost of logging and cloning when needed.

---

## 🧪 Testing and Integration

### Running Tests
```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

### Integration with Tracing Ecosystem

The macro works seamlessly with the entire `tracing` ecosystem:

```rust
use tracing_subscriber::{fmt, EnvFilter};
use log_args::params;

// Set up structured JSON logging
fn init_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

#[params(fields(user_id, action))]
fn user_action(user_id: u64, action: String, sensitive_data: String) {
    tracing::info!("User performed action");
    // Output: {"timestamp":"...","level":"INFO","fields":{"message":"User performed action","user_id":"123","action":"login"}}
}
```

### Log Aggregation and Monitoring

The flattened JSON structure works perfectly with log aggregation tools:

```json
{
  "timestamp": "2025-01-15T10:30:00Z",
  "level": "INFO",
  "target": "my_app",
  "fields": {
    "message": "Processing payment",
    "user_id": "12345",
    "amount": "99.99",
    "service": "payment",
    "version": "2.0"
  }
}
```

### Performance Considerations

- **Zero Runtime Overhead**: Field selection happens at compile time
- **Minimal Memory Impact**: Only specified fields are cloned/logged
- **Async Safe**: `clone_upfront` prevents ownership issues in async contexts
- **Span Efficiency**: Context propagation uses thread-local/task-local storage

### Production Deployment

```rust
// Configure for production
tracing_subscriber::fmt()
    .json()
    .with_env_filter("info,my_app=debug")
    .with_target(false)
    .init();

#[params(fields(user_id, operation), custom(service = "api", version = "1.0"))]
fn api_endpoint(user_id: u64, operation: String, api_key: String) {
    tracing::info!("API call processed");
    // Logs user_id, operation, service, version - excludes api_key
}
```

---

## ⚠️ Limitations

- The `#[params]` macro redefines tracing macros within function scope, which may generate unused macro warnings if not all redefined macros are used (these are suppressed internally)
- When using `clone_upfront`, fields must implement `Clone`
- Array indexing syntax (e.g., `users[0].name`) is not supported; use iterator methods or access collections as whole fields instead

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🧵 Example: End-to-End

```rust
use log_args::log_args;
use tracing_subscriber;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

#[log_args(fields(user.id, user.name), custom("service" = "auth"))]
fn login(user: User) {
    info!("Login started");
    warn!("Invalid password");
}
```

### Logs:

```
INFO login: user_id=42 user_name="Alice" service="auth" Login started
WARN login: user_id=42 user_name="Alice" service="auth" Invalid password
```

---

**❌ Incorrect usage:**
```rust
#[params]
fn foo() {
    tracing::debug!("debug message"); // will NOT be enriched
}
```

**Always import the macros you use:**
```rust
use tracing::{debug, info, warn, error};
```

---

## 🔮 Future Enhancements

* `#[log_args(span = true)]`: Optional span-based logging for subfunction support
* `#[log_args(log_return)]`: Auto-log return values
* Integration with `opentelemetry` and structured span hierarchy

---

## ✅ License

MIT or Apache 2.0 — your choice.

---

## 🙌 Contributions

PRs, issues, and feedback are welcome. Let’s make logging in Rust ergonomic and powerful, together.

---

## 📫 Contact

Maintained by \[MKJS Tech](mailto:mkjsm57@gmail.com) • Feel free to reach out via [mail](mailto:mkjsm57@gmail.com) or [GitHub Issues](https://github.com/MKJSM/log-args/issues).
