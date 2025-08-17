# log_args

[![Crates.io](https://img.shields.io/crates/v/log_args.svg)](https://crates.io/crates/log_args)
[![Docs.rs](https://docs.rs/log_args/badge.svg)](https://docs.rs/log_args)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MKJSM/log_args/blob/main/LICENSE)

A powerful procedural macro crate providing the `#[params]` attribute for automatic parameter logging and context propagation in Rust applications. Built on top of the `tracing` ecosystem, it enables truly automatic context inheritance across all boundaries including async/await, spawned tasks, closures, and WebSocket upgrades.

> **Note**: This is part of the [log-args workspace](../README.md). For complete workspace setup, see the [main documentation](../README.md).

## ✨ Key Features

### 🎯 Automatic Context Inheritance
- **Zero Configuration**: Child functions inherit parent context with just `#[params]`
- **Cross-Boundary**: Works across closures, async spawns, WebSocket upgrades, and thread boundaries
- **Transparent**: No manual context passing or management required

### 🚀 Performance & Safety
- **Zero Runtime Overhead**: All processing happens at compile-time via macro expansion
- **Memory Efficient**: Only specified fields are cloned and logged
- **Async Safe**: Proper handling of ownership in async contexts with `clone_upfront`
- **Thread Safe**: Context propagation uses thread-local and task-local storage

### 🔧 Flexible Configuration
- **Selective Logging**: Choose exactly which parameters to log with `fields(...)`
- **Custom Fields**: Add computed metadata and expressions with `custom(...)`
- **Span Propagation**: Automatic context inheritance with `span(...)`
- **Nested Access**: Support for deep field access like `user.profile.settings.theme`
- **Method Calls**: Log results of method calls and expressions

### 🔒 Security & Privacy
- **Secure by Default**: Sensitive parameters excluded unless explicitly specified
- **Fine-grained Control**: Log only what's needed for debugging
- **Compliance Ready**: Selective logging helps meet privacy requirements
- **Production Safe**: Configurable logging levels and field selection

## Installation

```toml
[dependencies]
log_args = "0.1.4"
log-args-runtime = { version = "0.1.2", features = ["with_context"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
```

## 🚀 Quick Start

### Basic Setup

```rust
use log_args::params;
use tracing::{info, warn, error};

// Initialize structured JSON logging
fn init_logging() {
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .init();
}

fn main() {
    init_logging();
    authenticate_user("john_doe".to_string(), "secret123".to_string());
}
```

### Usage Examples

#### 1. Default Behavior (Recommended)
```rust
// Default: Only span propagation and function name logging
#[params]
fn process_request(user_id: String, data: String) {
    info!("Processing request");
    // Output: {"message": "Processing request", "target": "my_app::process_request"}
}
```

#### 2. Selective Parameter Logging
```rust
// Log only specific parameters (excludes sensitive data)
#[params(fields(user_id, action))]
fn user_action(user_id: String, action: String, password: String) {
    info!("User performed action");
    // Output: {"message": "User performed action", "user_id": "123", "action": "login"}
    // Note: password is excluded for security
}
```

#### 3. Span Context Propagation
```rust
// Parent function sets up context
#[params(span(tenant_id, session_id))]
fn handle_request(tenant_id: String, session_id: String, data: String) {
    info!("Handling request");
    process_data(data); // Child function inherits context
}

// Child function automatically inherits tenant_id and session_id
#[params]
fn process_data(data: String) {
    info!("Processing data");
    // Output includes: {"tenant_id": "acme", "session_id": "web", "message": "Processing data"}
}
```

#### 4. Custom Fields with Expressions
```rust
#[params(
    fields(user.id, user.name),
    custom(
        email_count = user.emails.len(),
        is_premium = user.subscription.tier == "premium",
        account_age = user.created_at.elapsed().as_secs() / 86400
    )
)]
fn analyze_user(user: User, api_key: String) {
    info!("Analyzing user account");
    // Output: {
    //   "message": "Analyzing user account",
    //   "user_id": 42,
    //   "user_name": "Alice",
    //   "email_count": 3,
    //   "is_premium": true,
    //   "account_age": 365
    // }
}
```  

## 📚 Comprehensive Attribute Reference

### Basic Attributes

#### `#[params]` - Default Behavior
```rust
#[params]
fn my_function(user_id: String, data: String) {
    info!("Processing");
}
```
**Behavior**: 
- ✅ Enables span propagation (child functions inherit context)
- ✅ Includes function name in logs
- ❌ Does NOT log function parameters (secure by default)
- ✅ Zero overhead when no logging macros are used

#### `#[params(all)]` - Log All Parameters
```rust
#[params(all)]
fn debug_function(user_id: String, email: String, data: Vec<u8>) {
    info!("Debug info");
    // Output: {"user_id": "123", "email": "user@example.com", "data": [1,2,3], "message": "Debug info"}
}
```
**⚠️ Warning**: Use carefully in production - logs ALL parameters including potentially sensitive data.

#### `#[params(fields(...))]` - Selective Logging
```rust
#[params(fields(user_id, action))]
fn secure_function(user_id: String, action: String, password: String, api_key: String) {
    info!("User action");
    // Output: {"user_id": "123", "action": "login", "message": "User action"}
    // password and api_key are excluded for security
}
```
**Benefits**: 
- 🔒 Security-conscious - exclude sensitive parameters
- 🎨 Performance - only specified fields are processed
- 📈 Compliance - fine-grained control over logged data

### Advanced Attributes

#### `#[params(span(...))]` - Context Propagation
```rust
// Parent function
#[params(span(request_id, user_id))]
fn handle_api_request(request_id: String, user_id: String, payload: String) {
    info!("API request received");
    validate_request(payload);
    process_business_logic();
}

// Child functions automatically inherit request_id and user_id
#[params]
fn validate_request(payload: String) {
    info!("Validating request");
    // Output: {"request_id": "req-123", "user_id": "user-456", "message": "Validating request"}
}

#[params]
fn process_business_logic() {
    info!("Processing business logic");
    // Output: {"request_id": "req-123", "user_id": "user-456", "message": "Processing business logic"}
}
```
**Key Benefits**:
- 🎯 **Automatic Inheritance**: Child functions get context without any code changes
- 🌐 **Cross-Boundary**: Works across async/await, spawned tasks, closures
- 🔗 **Distributed Tracing**: Perfect for microservices and request tracking

#### `#[params(custom(...))]` - Computed Fields
```rust
#[params(
    custom(
        timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        service_version = env!("CARGO_PKG_VERSION"),
        environment = std::env::var("ENVIRONMENT").unwrap_or("development".to_string())
    )
)]
fn service_endpoint(request: Request) {
    info!("Service call");
    // Output: {
    //   "timestamp": 1642694400,
    //   "service_version": "1.2.3",
    //   "environment": "production",
    //   "message": "Service call"
    // }
}
```

#### Nested Field Access
```rust
#[derive(Debug)]
struct User {
    profile: Profile,
    settings: Settings,
}

#[derive(Debug)]
struct Profile {
    name: String,
    contact: Contact,
}

#[params(fields(
    user.profile.name,
    user.profile.contact.email,
    user.settings.theme,
    config.database.host
))]
fn complex_nested_access(user: User, config: Config, secret: String) {
    info!("Processing user data");
    // Output: {
    //   "user_profile_name": "Alice",
    //   "user_profile_contact_email": "alice@example.com",
    //   "user_settings_theme": "dark",
    //   "config_database_host": "localhost",
    //   "message": "Processing user data"
    // }
}
```

#### Method Calls and Expressions
```rust
#[params(fields(
    users.len(),
    users.is_empty(),
    config.get_timeout().as_secs(),
    data.iter().count()
))]
fn analyze_data(users: Vec<User>, config: Config, data: Vec<String>) {
    info!("Data analysis");
    // Output: {
    //   "users_len": 42,
    //   "users_is_empty": false,
    //   "config_get_timeout_as_secs": 30,
    //   "data_iter_count": 100,
    //   "message": "Data analysis"
    // }
}
```

### Combined Usage Patterns

#### Production API Endpoint
```rust
#[params(
    fields(request.user_id, request.endpoint),
    custom(
        service = "user-api",
        version = env!("CARGO_PKG_VERSION"),
        request_size = request.body.len()
    ),
    span(request_id, trace_id)
)]
fn api_handler(request_id: String, trace_id: String, request: ApiRequest, auth_token: String) {
    info!("API request processed");
    // Output: {
    //   "request_user_id": "user-123",
    //   "request_endpoint": "/api/users",
    //   "service": "user-api",
    //   "version": "1.0.0",
    //   "request_size": 256,
    //   "request_id": "req-456",
    //   "trace_id": "trace-789",
    //   "message": "API request processed"
    // }
    // Note: auth_token is excluded for security
}
```

#### Async Task Processing
```rust
#[params(span(job_id, user_id))]
async fn process_background_job(job_id: String, user_id: String, job_data: JobData) {
    info!("Background job started");
    
    // Spawn async tasks - they inherit context automatically
    let handle1 = tokio::spawn(async {
        validate_job_data().await;
    });
    
    let handle2 = tokio::spawn(async {
        send_notifications().await;
    });
    
    tokio::try_join!(handle1, handle2).unwrap();
    info!("Background job completed");
}

#[params]
async fn validate_job_data() {
    info!("Validating job data");
    // Automatically includes job_id and user_id from parent context
}

#[params]
async fn send_notifications() {
    info!("Sending notifications");
    // Automatically includes job_id and user_id from parent context
}
```

## 📚 Advanced Features

### Cross-Boundary Context Propagation

The `#[params]` macro enables automatic context inheritance across various Rust boundaries:

#### Async/Await Boundaries
```rust
#[params(span(request_id))]
async fn handle_request(request_id: String) {
    info!("Request started");
    
    // Context automatically propagates across .await points
    let result = async_operation().await;
    process_result(result).await;
}

#[params]
async fn async_operation() -> String {
    info!("Async operation"); // Includes request_id automatically
    "result".to_string()
}
```

#### Spawned Tasks
```rust
#[params(span(user_id, session_id))]
fn handle_user_session(user_id: String, session_id: String) {
    info!("Session started");
    
    // Spawn background task - inherits context
    tokio::spawn(async {
        background_cleanup().await;
    });
    
    // Spawn blocking task - also inherits context
    tokio::task::spawn_blocking(|| {
        cpu_intensive_work();
    });
}

#[params]
async fn background_cleanup() {
    info!("Cleanup started"); // Includes user_id and session_id
}

#[params]
fn cpu_intensive_work() {
    info!("CPU work started"); // Includes user_id and session_id
}
```

#### Closures and Higher-Order Functions
```rust
#[params(span(batch_id))]
fn process_batch(batch_id: String, items: Vec<Item>) {
    info!("Batch processing started");
    
    // Context propagates into closures
    items.iter().for_each(|item| {
        process_item(item.clone());
    });
    
    // Also works with map, filter, etc.
    let results: Vec<_> = items.into_iter()
        .map(|item| transform_item(item))
        .collect();
}

#[params]
fn process_item(item: Item) {
    info!("Processing item"); // Includes batch_id
}
```

### Method Support

```rust
struct UserService {
    db: Database,
}

impl UserService {
    #[params(fields(user_id))]
    fn get_user(&self, user_id: String) -> Option<User> {
        info!("Fetching user from database");
        self.db.find_user(&user_id)
    }
    
    #[params(span(user_id), fields(user.email))]
    fn update_user(&mut self, user_id: String, user: User) -> Result<(), Error> {
        info!("Updating user");
        self.validate_user(&user)?;
        self.db.update_user(&user_id, user)
    }
    
    #[params]
    fn validate_user(&self, user: &User) -> Result<(), Error> {
        info!("Validating user data"); // Inherits user_id from parent
        // validation logic
        Ok(())
    }
}
```

### Error Handling and Logging

```rust
#[params(span(operation_id), fields(input.len()))]
fn risky_operation(operation_id: String, input: Vec<u8>) -> Result<String, Error> {
    info!("Starting risky operation");
    
    match perform_operation(&input) {
        Ok(result) => {
            info!("Operation succeeded");
            Ok(result)
        }
        Err(e) => {
            error!("Operation failed: {}", e); // Includes operation_id and input_len
            handle_error(e)
        }
    }
}

#[params]
fn handle_error(error: Error) -> Result<String, Error> {
    warn!("Handling error"); // Inherits operation_id and input_len
    // error recovery logic
    Err(error)
}
```

## 🎨 Best Practices

### Security Guidelines

1. **Default to Secure**: Use `#[params]` without arguments for most functions
2. **Explicit Field Selection**: Use `fields(...)` to log only necessary data
3. **Avoid Sensitive Data**: Never log passwords, tokens, or personal information
4. **Review Production Logs**: Regularly audit what data is being logged

```rust
// ✅ Good: Secure by default
#[params]
fn authenticate(username: String, password: String) {
    info!("Authentication attempt"); // password not logged
}

// ✅ Good: Explicit field selection
#[params(fields(username))]
fn login(username: String, password: String, remember_me: bool) {
    info!("Login attempt"); // Only username logged
}

// ❌ Bad: Logs sensitive data
#[params(all)]
fn process_payment(card_number: String, cvv: String, amount: f64) {
    info!("Payment processing"); // Logs sensitive card data!
}
```

### Performance Optimization

1. **Selective Logging**: Only log fields you actually need
2. **Avoid Heavy Computations**: Be careful with custom field expressions
3. **Use Appropriate Log Levels**: Don't log debug info at info level

```rust
// ✅ Good: Minimal field selection
#[params(fields(user_id, action))]
fn user_action(user_id: String, action: String, large_payload: Vec<u8>) {
    info!("User action"); // large_payload not processed
}

// ❌ Bad: Expensive computation in custom field
#[params(custom(hash = compute_expensive_hash(&data)))]
fn process_data(data: Vec<u8>) {
    info!("Processing"); // Expensive hash computed on every log
}

// ✅ Good: Lightweight custom fields
#[params(custom(data_size = data.len(), timestamp = std::time::SystemTime::now()))]
fn process_data(data: Vec<u8>) {
    info!("Processing"); // Lightweight operations
}
```

### Context Propagation Patterns

1. **Request-Scoped Context**: Use `span(...)` for request/session identifiers
2. **Hierarchical Context**: Build context layers for complex operations
3. **Service Boundaries**: Propagate context across service calls

```rust
// Request-scoped context
#[params(span(request_id, user_id))]
fn handle_api_request(request_id: String, user_id: String, request: ApiRequest) {
    info!("API request received");
    
    // All child operations inherit request_id and user_id
    validate_request(&request);
    process_business_logic(&request);
    send_response();
}

// Service boundary context
#[params(span(trace_id, service_name))]
fn service_call(trace_id: String, service_name: String, payload: Payload) {
    info!("Service call initiated");
    
    // Context propagates to external service calls
    let response = external_service_client.call(payload);
    process_response(response);
}
```

## 🔧 Configuration and Setup

### Tracing Subscriber Configuration

```rust
use tracing::Level;
use tracing_subscriber::{fmt, EnvFilter};

// Development setup
fn init_dev_logging() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .pretty()
        .init();
}

// Production setup with JSON logging
fn init_prod_logging() {
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();
}

// Custom field formatting
fn init_custom_logging() {
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_env_filter("info,my_app=debug")
        .with_span_events(fmt::format::FmtSpan::CLOSE)
        .init();
}
```

### Environment-Based Configuration

```rust
fn init_logging() {
    match std::env::var("LOG_FORMAT").as_deref() {
        Ok("json") => {
            tracing_subscriber::fmt()
                .json()
                .flatten_event(true)
                .init();
        }
        _ => {
            tracing_subscriber::fmt()
                .pretty()
                .init();
        }
    }
}
```

## 🚫 Limitations and Considerations

### Current Limitations

1. **Array Indexing**: Direct array access like `users[0].name` is not supported
   ```rust
   // ❌ Not supported
   #[params(fields(users[0].name))]
   
   // ✅ Use instead
   #[params(fields(users.first().map(|u| &u.name)))]
   ```

2. **Complex Expressions**: Very complex expressions may not parse correctly
   ```rust
   // ❌ Might not work
   #[params(custom(result = complex_nested_function_call().unwrap().field))]
   
   // ✅ Better approach
   #[params]
   fn my_function(data: Data) {
       let result = complex_nested_function_call().unwrap().field;
       info!(result = ?result, "Processing data");
   }
   ```

3. **Macro Scope**: The macro redefines logging macros within function scope only
   ```rust
   #[params(fields(user_id))]
   fn my_function(user_id: String) {
       info!("This includes user_id"); // ✅ Works
       
       let closure = || {
           info!("This also includes user_id"); // ✅ Works
       };
       
       // But calling tracing::info! directly bypasses the macro
       tracing::info!("This does NOT include user_id"); // ❌ Bypassed
   }
   ```

### Performance Considerations

- **Clone Overhead**: Fields are cloned when logged, consider this for large data structures
- **Compilation Time**: Heavy macro usage may increase compile times
- **Runtime Overhead**: Minimal when logging is disabled, but field evaluation still occurs

## 🔍 Troubleshooting

### Common Issues

#### Context Not Propagating
```rust
// Problem: Child function doesn't inherit context
#[params(fields(user_id))] // ❌ Wrong: fields() doesn't propagate
fn parent(user_id: String) {
    child();
}

// Solution: Use span() for propagation
#[params(span(user_id))] // ✅ Correct: span() propagates
fn parent(user_id: String) {
    child();
}
```

#### Fields Not Appearing in Logs
```rust
// Check 1: Ensure tracing subscriber is configured for JSON with flatten_event
tracing_subscriber::fmt()
    .json()
    .flatten_event(true) // ✅ Required for field flattening
    .init();

// Check 2: Verify you're using the right logging macro
#[params(fields(user_id))]
fn my_function(user_id: String) {
    info!("Message"); // ✅ Correct - uses redefined macro
    tracing::info!("Message"); // ❌ Wrong - bypasses macro
}
```

#### Compilation Errors
```rust
// Error: Field doesn't exist
#[params(fields(nonexistent_field))]
fn my_function(user_id: String) {} // ❌ user_id != nonexistent_field

// Error: Complex expression parsing
#[params(fields(data.very.deeply.nested.field.that.might.not.exist))]
// Solution: Simplify or use custom fields with proper error handling
```

### Debug Tips

1. **Enable Debug Logging**: Set `RUST_LOG=debug` to see more detailed logs
2. **Check Macro Expansion**: Use `cargo expand` to see generated code
3. **Verify Field Names**: Ensure field names match parameter names exactly
4. **Test Incrementally**: Start with simple `#[params]` and add complexity gradually

## 📚 Examples

See the [workspace examples](../examples/) directory for runnable demonstrations:

```bash
# Basic usage patterns
cargo run --example basic_usage
cargo run --example selective_fields
cargo run --example custom_fields

# Advanced features
cargo run --example span_propagation
cargo run --example method_support
cargo run --example auto_capture

# Production patterns
cargo run --example all_parameters  # Debug contexts only
```

## Development

```bash
# Run tests
cargo test

# Build documentation
cargo doc --open
```

## License

MIT - See [LICENSE](../LICENSE) for details.
