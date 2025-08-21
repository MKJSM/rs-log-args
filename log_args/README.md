# log_args

[![Crates.io](https://img.shields.io/crates/v/log_args.svg)](https://crates.io/crates/log_args)
[![Docs.rs](https://docs.rs/log_args/badge.svg)](https://docs.rs/log_args)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MKJSM/log_args/blob/main/LICENSE)

Procedural macro crate providing the `#[params]` attribute for automatic parameter logging and context propagation in Rust applications.

> **Note**: This is part of the [log-args workspace](https://github.com/MKJSM/rs-log-args/blob/main/README.md). For complete setup and examples, see the [main documentation](https://github.com/MKJSM/rs-log-args/blob/main/README.md).

## ✨ Key Features

- **Automatic Context Inheritance**: Child functions inherit parent context with `#[params(span(...))]`
- **Cross-Boundary Propagation**: Works across async/await, spawned tasks, and closures
- **Selective Logging**: Choose specific parameters with `fields(...)`
- **Custom Fields**: Add computed metadata with `custom(...)`
- **Secure by Default**: Sensitive parameters excluded unless explicitly specified
- **Zero Runtime Overhead**: Compile-time macro expansion

## 🚀 Getting Started

### Step 1: Installation

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
log_args = "0.1.6"
log-args-runtime = { version = "0.1.4", features = ["with_context"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
```

### Step 2: Basic Setup

Set up structured JSON logging in your `main.rs`:

```rust
use log_args::params;
use tracing::{info, Level};

fn main() {
    // Initialize JSON logging with flattened events
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)  // Important: flattens fields to top level
        .init();

    // Your application code
    handle_user_request("req-123".to_string(), "user-456".to_string());
}
```

### Step 3: Start Simple

Begin with the secure default behavior:

```rust
#[params]  // Secure: no parameters logged
fn handle_user_request(request_id: String, user_id: String) {
    info!("Processing user request");
    // Output: {"message": "Processing user request", "target": "my_app::handle_user_request"}
}
```

### Step 4: Add Context Propagation

Enable automatic context inheritance:

```rust
#[params(span(request_id, user_id))]  // Set up context
fn handle_user_request(request_id: String, user_id: String) {
    info!("Processing user request");
    validate_request();  // Child function inherits context
}

#[params]  // Inherits request_id and user_id automatically
fn validate_request() {
    info!("Validating request");
    // Output: {"request_id": "req-123", "user_id": "user-456", "message": "Validating request"}
}
```

### Step 5: Add Selective Logging (Production-Ready)

Log only specific parameters:

```rust
#[params(fields(user_id, action))]  // Only log safe parameters
fn user_action(user_id: String, action: String, password: String) {
    info!("User performed action");
    // Output: {"user_id": "user-456", "action": "login", "message": "User performed action"}
    // Note: password is NOT logged - secure!
}
```

🎉 **Congratulations!** You now have automatic context propagation and secure parameter logging set up.

## 🚀 Quick Start

```rust
use log_args::params;
use tracing::{info, Level};

fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .json()
        .flatten_event(true)
        .init();
}

// Default behavior - context propagation only
#[params]
fn greet(name: String) {
    info!("Greeting user");
}

// Span context propagation
#[params(span(tenant_id, session_id))]
fn handle_request(tenant_id: String, session_id: String, data: String) {
    info!("Handling request");
    process_data(data); // Child inherits context
}

#[params]
fn process_data(data: String) {
    info!("Processing data"); // Includes tenant_id and session_id
}

fn main() {
    init_logging();
    greet("Ada".to_string());
}

## 📖 Complete Attribute Reference

The `#[params]` macro supports multiple attributes that can be combined to create flexible logging strategies. Here's a comprehensive guide to all available attributes:

### `#[params]` - Default (Secure) Behavior

**Purpose**: Enables context inheritance without logging any parameters.

```rust
#[params]
fn authenticate_user(username: String, password: String) {
    info!("Authentication attempt"); // No parameters logged - secure!
}
```

**When to use**:
- ✅ Production functions with sensitive data
- ✅ When you only need context inheritance
- ✅ Default choice for security

---

### `#[params(fields(...))]` - Selective Parameter Logging

**Purpose**: Log only specific function parameters as individual fields.

```rust
#[params(fields(user_id, action))]
fn user_action(user_id: String, action: String, password: String) {
    info!("User performed action");
    // Output: {"user_id": "123", "action": "login", "message": "User performed action"}
    // Note: password is NOT logged
}
```

**When to use**:
- ✅ Production logging with specific data needs
- ✅ When you need precise control over logged data
- ✅ Compliance and security requirements

---

### `#[params(span(...))]` - Context Propagation

**Purpose**: Set up automatic context inheritance for child functions.

```rust
#[params(span(request_id, user_id))]
fn handle_api_request(request_id: String, user_id: String, payload: String) {
    info!("API request received");
    validate_payload(payload); // Child inherits request_id and user_id
    process_business_logic();   // This too!
}

#[params]
fn validate_payload(payload: String) {
    info!("Validating payload");
    // Output includes: {"request_id": "req-123", "user_id": "user-456", ...}
}
```

**Cross-boundary support**:
- ✅ Async/await boundaries
- ✅ Spawned tasks (`tokio::spawn`)
- ✅ Closures and iterators
- ✅ Thread boundaries

**When to use**:
- ✅ Distributed tracing
- ✅ Request/session tracking
- ✅ Microservices architecture

---

### `#[params(custom(...))]` - Computed Fields

**Purpose**: Add computed fields using custom expressions.

```rust
#[params(
    custom(
        timestamp = std::time::SystemTime::now(),
        data_size = data.len(),
        is_admin = user.role == "admin",
        service_version = env!("CARGO_PKG_VERSION")
    )
)]
fn process_request(data: Vec<u8>, user: User) {
    info!("Processing request");
    // Output includes computed fields with their values
}
```

**When to use**:
- ✅ Adding metadata and timestamps
- ✅ Computed values and flags
- ✅ Environment information

**Performance tip**: Keep expressions lightweight as they're evaluated on every log call.

---

### `#[params(all)]` - Log All Parameters

**Purpose**: Log all function parameters as individual fields.

```rust
#[params(all)] // ⚠️ Use with caution!
fn debug_function(user_id: String, email: String, data: Vec<u8>) {
    info!("Debug information");
    // Output: {"user_id": "123", "email": "user@example.com", "data": [1,2,3], ...}
}
```

**⚠️ Security Warning**: This logs ALL parameters, including sensitive data!

**When to use**:
- ✅ Development and debugging
- ✅ Non-production environments
- ❌ Production environments
- ❌ Functions with sensitive parameters

---

### `#[params(clone_upfront)]` - Async-Safe Parameter Cloning

**Purpose**: Clone parameters before async operations to prevent ownership issues.

```rust
#[params(fields(user_id), clone_upfront)]
async fn async_operation(user_id: String, data: Vec<u8>) {
    tokio::spawn(async move {
        // user_id was cloned upfront, so this works without ownership issues
        process_data(data).await;
    });
}
```

**When to use**:
- ✅ Async functions with spawned tasks
- ✅ When parameters need to be moved into closures
- ✅ Complex async workflows

**Performance note**: Only use when necessary as it adds cloning overhead.

---

### `#[params(auto_capture)]` - Automatic Context Capture

**Purpose**: Automatically capture context in closures and spawned tasks.

```rust
#[params(span(batch_id), auto_capture)]
fn process_batch(batch_id: String, items: Vec<Item>) {
    items.iter().for_each(|item| {
        // Context automatically captured in closure
        process_item(item.clone());
    });
}
```

**When to use**:
- ✅ Complex async workflows
- ✅ Iterator chains with closures
- ✅ Nested task spawning

---

## 🔧 Combining Attributes

You can combine multiple attributes for powerful logging strategies:

```rust
#[params(
    fields(user_id, action),           // Log specific parameters
    custom(timestamp = now()),         // Add computed fields
    span(request_id),                  // Set up context propagation
    clone_upfront                      // Handle async ownership
)]
async fn complex_operation(request_id: String, user_id: String, action: String, secret: String) {
    info!("Complex operation started");
    // Logs: user_id, action, timestamp, request_id (but NOT secret)
    // Child functions inherit request_id context
}
```

## 🔧 Troubleshooting

### Fields Not Appearing in Logs?

**Problem**: Your fields aren't showing up in the JSON output.

**Solution**: Ensure your tracing subscriber is configured correctly:

```rust
// ✅ Correct setup
tracing_subscriber::fmt()
    .json()
    .flatten_event(true)  // This is crucial!
    .init();

// ❌ Wrong - fields will be nested
tracing_subscriber::fmt()
    .json()
    // Missing .flatten_event(true)
    .init();
```

### Context Not Propagating?

**Problem**: Child functions aren't inheriting context from parent functions.

**Solution**: Use `span(...)` for propagation, not `fields(...)`:

```rust
// ❌ Wrong - fields() doesn't propagate context
#[params(fields(user_id))]
fn parent(user_id: String) {
    child(); // Won't inherit user_id
}

// ✅ Correct - span() propagates context
#[params(span(user_id))]
fn parent(user_id: String) {
    child(); // Will inherit user_id
}
```

### Compilation Errors?

**Problem**: Field names don't match parameter names.

```rust
// ❌ Error - field name doesn't exist
#[params(fields(nonexistent_field))]
fn my_function(user_id: String) {} // user_id ≠ nonexistent_field

// ✅ Correct - field names match parameters
#[params(fields(user_id))]
fn my_function(user_id: String) {}
```

### Quick Debug Tips

1. **Start simple**: Begin with `#[params]` and add complexity gradually
2. **Check field names**: Ensure they exactly match parameter names
3. **Verify setup**: Make sure `flatten_event(true)` is enabled
4. **Use examples**: Run the workspace examples to see working code

---

## 📚 Examples

See the [workspace examples](../examples/) for runnable demonstrations:

```bash
cargo run --example basic_usage
cargo run --example span_propagation
cargo run --example selective_fields
cargo run --example custom_fields
cargo run --example method_support
cargo run --example all_parameters
cargo run --example auto_capture
```

## License

MIT - See [LICENSE](../LICENSE) for details.
