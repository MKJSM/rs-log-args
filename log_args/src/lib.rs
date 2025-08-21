//! # log_args
//!
//! A powerful procedural macro crate providing the `#[params]` attribute for automatic parameter logging
//! and context propagation in Rust applications. Built on top of the `tracing` ecosystem, it enables
//! truly automatic context inheritance across all boundaries including async/await, spawned tasks,
//! closures, and WebSocket upgrades.
//!
//! ## ✨ Key Features
//!
//! ### 🎯 Automatic Context Inheritance
//! - **Zero Configuration**: Child functions inherit parent context with just `#[params]`
//! - **Cross-Boundary**: Works across closures, async spawns, WebSocket upgrades, and thread boundaries
//! - **Transparent**: No manual context passing or management required
//!
//! ### 🚀 Performance & Safety
//! - **Zero Runtime Overhead**: All processing happens at compile-time via macro expansion
//! - **Memory Efficient**: Only specified fields are cloned and logged
//! - **Async Safe**: Proper handling of ownership in async contexts
//! - **Thread Safe**: Context propagation uses thread-local and task-local storage
//!
//! ### 🔧 Flexible Configuration
//! - **Selective Logging**: Choose exactly which parameters to log with `fields(...)`
//! - **Custom Fields**: Add computed metadata and expressions with `custom(...)`
//! - **Span Propagation**: Automatic context inheritance with `span(...)`
//! - **Nested Access**: Support for deep field access like `user.profile.settings.theme`
//! - **Method Calls**: Log results of method calls and expressions
//!
//! ### 🔒 Security & Privacy
//! - **Secure by Default**: Sensitive parameters excluded unless explicitly specified
//! - **Fine-grained Control**: Log only what's needed for debugging
//! - **Compliance Ready**: Selective logging helps meet privacy requirements
//! - **Production Safe**: Configurable logging levels and field selection
//!
//! ## 🚀 Quick Start
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! log_args = "0.1.4"
//! log-args-runtime = { version = "0.1.2", features = ["with_context"] }
//! tracing = "0.1"
//! tracing-subscriber = { version = "0.3", features = ["json"] }
//! ```
//!
//! ### Basic Usage Examples
//!
//! ```rust, ignore
//! use log_args::params;
//! use tracing::info;
//!
//! // Default behavior: Only span propagation and function name logging
//! #[params]
//! fn process_request(user_id: String, data: String) {
//!     info!("Processing request");
//!     // Output: {"message": "Processing request", "target": "my_app::process_request"}
//! }
//!
//! // Selective parameter logging (excludes sensitive data)
//! #[params(fields(user_id, action))]
//! fn user_action(user_id: String, action: String, password: String) {
//!     info!("User performed action");
//!     // Output: {"message": "User performed action", "user_id": "123", "action": "login"}
//!     // Note: password is excluded for security
//! }
//!
//! // Span context propagation - the killer feature!
//! #[params(span(request_id, user_id))]
//! fn handle_api_request(request_id: String, user_id: String, payload: String) {
//!     info!("API request received");
//!     validate_request(payload); // Child function inherits context
//!     process_business_logic();   // This too!
//! }
//!
//! // Child functions automatically inherit request_id and user_id
//! #[params]
//! fn validate_request(payload: String) {
//!     info!("Validating request");
//!     // Output: {"request_id": "req-123", "user_id": "user-456", "message": "Validating request"}
//! }
//!
//! #[params]
//! fn process_business_logic() {
//!     info!("Processing business logic");
//!     // Output: {"request_id": "req-123", "user_id": "user-456", "message": "Processing business logic"}
//! }
//! ```
//!
//! ## 🔧 Advanced Usage
//!
//! ### Custom Fields with Expressions
//! ```rust, ignore
//! #[params(
//!     fields(user.id, user.name),
//!     custom(
//!         email_count = user.emails.len(),
//!         is_premium = user.subscription.tier == "premium",
//!         timestamp = std::time::SystemTime::now()
//!     )
//! )]
//! fn analyze_user(user: User, api_key: String) {
//!     info!("Analyzing user account");
//!     // Output: {
//!     //   "message": "Analyzing user account",
//!     //   "user_id": 42,
//!     //   "user_name": "Alice",
//!     //   "email_count": 3,
//!     //   "is_premium": true,
//!     //   "timestamp": "2024-01-01T12:00:00Z"
//!     // }
//! }
//! ```
//!
//! ### Async Task Processing
//! ```rust, ignore
//! #[params(span(job_id, user_id))]
//! async fn process_background_job(job_id: String, user_id: String, job_data: JobData) {
//!     info!("Background job started");
//!     
//!     // Spawn async tasks - they inherit context automatically
//!     let handle1 = tokio::spawn(async {
//!         validate_job_data().await;
//!     });
//!     
//!     let handle2 = tokio::spawn(async {
//!         send_notifications().await;
//!     });
//!     
//!     tokio::try_join!(handle1, handle2).unwrap();
//!     info!("Background job completed");
//! }
//!
//! #[params]
//! async fn validate_job_data() {
//!     info!("Validating job data");
//!     // Automatically includes job_id and user_id from parent context
//! }
//! ```
//!
//! ## 🔧 Setup & Configuration
//!
//! ### Tracing Subscriber Setup
//! For structured JSON logging with context fields at the top level:
//!
//! ```rust, ignore
//! fn init_logging() {
//!     tracing_subscriber::fmt()
//!         .json()
//!         .flatten_event(true)  // Required for field flattening
//!         .init();
//! }
//! ```
//!
//! ### Production Configuration
//! ```rust, ignore
//! fn init_prod_logging() {
//!     tracing_subscriber::fmt()
//!         .json()
//!         .flatten_event(true)
//!         .with_env_filter("info,my_app=debug")
//!         .with_target(false)
//!         .init();
//! }
//! ```
//!
//! ## 🔒 Security Best Practices
//!
//! **Always use selective logging in production:**
//! ```rust, ignore
//! // ✅ Good - Only logs safe fields
//! #[params(fields(user_id, operation_type))]
//! fn secure_operation(user_id: String, password: String, operation_type: String) {
//!     info!("Operation started");
//!     // password is excluded for security
//! }
//!
//! // ❌ Bad - Logs everything including sensitive data
//! #[params(all)]
//! fn insecure_operation(user_id: String, password: String) {
//!     info!("Operation started"); // This would log the password!
//! }
//! ```
//!
//! ## 📚 Attribute Reference
//!
//! - `#[params]` - Default: span propagation and function name logging only
//! - `#[params(all)]` - Log all parameters (use carefully in production)
//! - `#[params(fields(param1, param2))]` - Log only specified parameters
//! - `#[params(span(param1, param2))]` - Propagate parameters as context to child functions
//! - `#[params(custom(key = expression))]` - Add computed custom fields
//!
//! ## 🚫 Limitations
//!
//! - Array indexing like `users[0].name` is not supported (use `users.first().map(|u| &u.name)` instead)
//! - The macro redefines logging macros within function scope only
//! - Complex expressions may not parse correctly (simplify or use custom fields)
//!
//! ## 📚 Examples
//!
//! See the [workspace examples](https://github.com/MKJSM/log-args/tree/main/examples) for comprehensive demonstrations.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::{
    parenthesized, parse_quote,
    visit_mut::{self, VisitMut},
    Expr, FnArg, Ident, MetaNameValue, Pat, Token,
};

const WITH_CONTEXT_ENABLED: bool = cfg!(feature = "with_context");

struct BlockRewriter;

impl VisitMut for BlockRewriter {
    fn visit_macro_mut(&mut self, mac: &mut syn::Macro) {
        let path = &mac.path;
        if let Some(last_segment) = path.segments.last() {
            if last_segment.ident == "info"
                || last_segment.ident == "warn"
                || last_segment.ident == "error"
                || last_segment.ident == "debug"
                || last_segment.ident == "trace"
            {
                if let Some(first_segment) = path.segments.first() {
                    if first_segment.ident == "tracing" {
                        // It's a `tracing::info!` style macro call. We need to strip `tracing::`
                        // so it becomes `info!`, which will then be resolved to our redefined macro.
                        let mut new_path = path.clone();
                        new_path.segments = new_path.segments.into_iter().skip(1).collect();
                        mac.path = new_path;
                    }
                }
            }
        }

        // Continue traversing the rest of the macro contents
        visit_mut::visit_macro_mut(self, mac);
    }
}

struct SpawnInstrumentRewriter;

impl VisitMut for SpawnInstrumentRewriter {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if let syn::Expr::Call(expr_call) = expr {
            if let syn::Expr::Path(expr_path) = &*expr_call.func {
                if expr_path.path.segments.iter().any(|s| s.ident == "spawn") {
                    if let Some(fut_arg) = expr_call.args.first_mut() {
                        let original_fut = fut_arg.clone();
                        *fut_arg = parse_quote! {
                            ::tracing::Instrument::instrument(#original_fut, ::tracing::Span::current())
                        };
                    }
                }
            }
        }

        // Continue traversing to find nested spawns
        visit_mut::visit_expr_mut(self, expr);
    }
}

// Convert snake_case to camelCase (first letter lowercase)
#[cfg(feature = "function-names-camel")]
#[allow(dead_code)]
fn to_camel_case(snake_case: &str) -> String {
    let mut camel_case = String::new();
    let mut capitalize = false;
    for c in snake_case.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            camel_case.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            camel_case.push(c);
        }
    }
    camel_case
}

// Convert snake_case to SCREAMING_SNAKE_CASE
#[cfg(feature = "function-names-screaming")]
#[allow(dead_code)]
fn to_screaming_snake_case(snake_case: &str) -> String {
    snake_case.to_ascii_uppercase()
}

// Convert snake_case to kebab-case
#[cfg(feature = "function-names-kebab")]
#[allow(dead_code)]
fn to_kebab_case(snake_case: &str) -> String {
    snake_case.replace('_', "-")
}

// Convert snake_case to PascalCase (first letter uppercase)
#[cfg(any(feature = "function-names-pascal", feature = "function-names"))]
fn to_pascal_case(snake_case: &str) -> String {
    let mut pascal_case = String::new();
    let mut capitalize = true; // Start with capital
    for c in snake_case.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            pascal_case.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            pascal_case.push(c);
        }
    }
    pascal_case
}

// Get the formatted function name based on enabled features
fn get_formatted_function_name(function_name: &str) -> String {
    #[cfg(feature = "function-names-camel")]
    {
        return to_camel_case(function_name);
    }

    #[cfg(any(feature = "function-names-pascal", feature = "function-names"))]
    {
        return to_pascal_case(function_name);
    }

    #[cfg(feature = "function-names-screaming")]
    {
        return to_screaming_snake_case(function_name);
    }

    #[cfg(feature = "function-names-kebab")]
    {
        return to_kebab_case(function_name);
    }

    #[cfg(feature = "function-names-snake")]
    {
        return function_name.to_string();
    }

    // Default: return original snake_case function name
    #[allow(unreachable_code)]
    function_name.to_string()
}

/// A powerful procedural macro for automatic function argument logging with structured tracing.
///
/// **The `#[params]` macro enables truly automatic context inheritance across all boundaries**
/// including async/await, spawned tasks, closures, and WebSocket upgrades. By default, it provides
/// span-based context propagation and function name logging with zero configuration.
///
/// ## ✨ Key Features
///
/// - **🎯 Automatic Context Inheritance**: Child functions inherit parent context seamlessly
/// - **🚀 Zero Runtime Overhead**: All processing happens at compile-time
/// - **🔧 Selective Logging**: Choose exactly which parameters to log
/// - **🔒 Security-First**: Sensitive data excluded by default
/// - **🌐 Cross-Boundary**: Works across async/await, spawned tasks, closures
///
/// ## 🚀 Basic Usage (Default Behavior)
///
/// By default, `#[params]` only enables span propagation and function name logging:
///
/// ```rust, ignore
/// use log_args::params;
/// use tracing::info;
///
/// #[params]
/// fn process_request(user_id: String, data: String) {
///     info!("Processing request");
///     // Child functions inherit context automatically - no manual passing needed!
///     validate_request(data);
///     send_response();
/// }
///
/// #[params]
/// fn validate_request(payload: String) {
///     info!("Validating request"); // Inherits parent context automatically
/// }
///
/// #[params]
/// fn send_response() {
///     info!("Sending response"); // Also inherits parent context
/// }
/// ```
///
/// **JSON Output:**
/// ```json
/// {
///   "timestamp": "2024-01-01T12:00:00Z",
///   "level": "INFO",
///   "fields": {
///     "message": "Processing request",
///     "target": "my_app::process_request"
///   }
/// }
/// ```
///
/// ## 🔧 Selective Field Logging (Production Recommended)
///
/// For security and performance, specify exactly which fields to log:
///
/// ```rust, ignore
/// #[params(fields(user_id, action))]
/// fn user_action(user_id: String, action: String, password: String, api_key: String) {
///     info!("User performed action");
///     // Output: {"user_id": "123", "action": "login", "message": "User performed action"}
///     // Note: password and api_key are excluded for security
/// }
/// ```
///
/// ## 🔗 Span Context Propagation (The Killer Feature!)
///
/// Automatically propagate context to all child functions:
///
/// ```rust, ignore
/// // Parent function sets up context
/// #[params(span(request_id, user_id))]
/// fn handle_api_request(request_id: String, user_id: String, payload: String) {
///     info!("API request received");
///     validate_request(payload);   // Inherits request_id and user_id
///     process_business_logic();    // Also inherits context
///     audit_log();                 // This too!
/// }
///
/// // Child functions automatically inherit request_id and user_id
/// #[params]
/// fn validate_request(payload: String) {
///     info!("Validating request");
///     // Output: {"request_id": "req-123", "user_id": "user-456", "message": "Validating request"}
/// }
///
/// #[params]
/// fn process_business_logic() {
///     info!("Processing business logic");
///     // Output: {"request_id": "req-123", "user_id": "user-456", "message": "Processing business logic"}
/// }
/// ```
///
/// ## 🏷️ Custom Fields with Expressions
///
/// Add computed metadata and service information:
///
/// ```rust, ignore
/// #[params(
///     fields(user_id),
///     custom(
///         service = "user-management",
///         version = "2.1.0",
///         environment = "production"
///     )
/// )]
/// fn service_operation(user_id: u64, sensitive_data: String) {
///     info!("Service operation");
/// }
/// ```
///
/// ## All Parameters Logging
///
/// Use the `all` attribute to explicitly log all function parameters:
///
/// ```rust, ignore
/// #[params(all)]
/// fn debug_function(user_id: u64, data: String, config: Config) {
///     info!("Debug information");
/// }
/// ```
///
/// This is useful for debugging or when you want to ensure all parameters are logged
/// regardless of other attributes.
///
/// ## Span Context Propagation (Enabled by Default)
///
/// **Note: Span propagation is now enabled by default with `#[params]`.**
/// Context automatically propagates to child functions:
///
/// ```rust, ignore
/// use log_args_runtime::{info as ctx_info};
///
/// #[params(fields(user.id, transaction.amount))]
/// fn process_payment(user: User, transaction: Transaction, card_data: CardData) {
///     info!("Starting payment processing");
///     
///     validate_payment();  // Inherits context automatically
///     charge_card();       // Inherits context automatically
/// }
///
/// #[params]
/// fn validate_payment() {
///     info!("Validating payment");  // Includes parent context
/// }
/// ```
///
/// ## Function Name Logging
///
/// Enable function name logging with Cargo features:
///
/// ```toml
/// [dependencies]
/// log_args = { version = "0.1", features = ["function-names-pascal"] }
/// ```
///
/// Available casing styles:
/// - `function-names-snake` → `process_payment`
/// - `function-names-camel` → `processPayment`
/// - `function-names-pascal` → `ProcessPayment` (recommended)
/// - `function-names-screaming` → `PROCESS_PAYMENT`
/// - `function-names-kebab` → `process-payment`
///
/// ## Async Support
///
/// Works seamlessly with async functions:
///
/// ```rust, ignore
/// #[params(span, fields(user_id, operation_type))]
/// async fn async_operation(user_id: u64, operation_type: String, secret: String) {
///     info!("Starting async operation");
///     
///     tokio::time::sleep(Duration::from_millis(100)).await;
///     
///     info!("Async operation completed");
/// }
/// ```
///
/// ## Method Support
///
/// Works with methods in impl blocks:
///
/// ```rust, ignore
/// impl UserService {
///     #[params(span, fields(user.id, self.config.timeout))]
///     fn process_user(&self, user: User, sensitive_token: String) {
///         info!("Processing user in service");
///     }
/// }
/// ```
///
/// ## Security Considerations
///
/// **⚠️ Important:** Always use selective logging in production to avoid logging sensitive data:
///
/// - Passwords, tokens, API keys
/// - Personal Identifiable Information (PII)
/// - Credit card numbers, financial data
/// - Internal system keys and secrets
///
/// ## Error Handling
///
/// The macro works with Result types and error handling patterns:
///
/// ```rust, ignore
/// #[params(fields(operation_id, retry_count))]
/// fn fallible_operation(
///     operation_id: String,
///     retry_count: u32,
///     secret_key: String,  // Not logged
/// ) -> Result<String, ProcessingError> {
///     info!("Starting fallible operation");
///     
///     // Operation logic that might fail
///     Ok("success".to_string())
/// }
/// ```
///
/// ## Performance Notes
///
/// - Selective logging (`fields(...)`) is more efficient than logging all parameters
/// - Complex field expressions are evaluated at runtime - use judiciously in hot paths
/// - Span creation has overhead - use for important operations that benefit from context
///
/// For comprehensive documentation and examples, see:
/// - [USAGE.md](https://github.com/MKJSM/log-args/blob/main/USAGE.md)
/// - [Examples](https://github.com/MKJSM/log-args/tree/main/examples)
/// - [Integration Tests](https://github.com/MKJSM/log-args/tree/main/tests)
///
/// fn child_function() {
///     info!("Child task");
/// }
///
#[proc_macro_attribute]
pub fn params(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = if let Ok(item_fn) = syn::parse::<syn::ItemFn>(input.clone()) {
        FnItem::Item(item_fn)
    } else if let Ok(impl_item_fn) = syn::parse::<syn::ImplItemFn>(input.clone()) {
        FnItem::ImplItem(impl_item_fn)
    } else {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(input),
            "The #[params] attribute can only be applied to functions or methods.",
        )
        .to_compile_error()
        .into();
    };

    let allow_unused_macros_attr: syn::Attribute = syn::parse_quote! { #[allow(unused_macros)] };
    item.attrs_mut().push(allow_unused_macros_attr);

    let attrs = match Punctuated::<Attribute, Token![,]>::parse_terminated.parse(args) {
        Ok(attrs) => attrs,
        Err(e) => return e.to_compile_error().into(),
    };

    let config = AttrConfig::from_attributes(attrs);
    let context_fields = get_context_fields_quote(&item, &config);

    let is_async = item.sig().asyncness.is_some();
    let new_block_tokens = generate_new_block(&item, &config, &context_fields, is_async);
    *item.block_mut() = match syn::parse2(new_block_tokens) {
        Ok(block) => block,
        Err(e) => return e.to_compile_error().into(),
    };

    TokenStream::from(quote! { #item })
}

fn generate_new_block(
    item: &FnItem,
    config: &AttrConfig,
    context_fields: &[proc_macro2::TokenStream],
    is_async: bool,
) -> proc_macro2::TokenStream {
    let log_redefines = get_log_redefines_with_fields(context_fields, is_async);
    let original_block = item.block().clone();
    let mut transformed_block = original_block.clone();
    BlockRewriter.visit_block_mut(&mut transformed_block);
    SpawnInstrumentRewriter.visit_block_mut(&mut transformed_block);

    if config.span {
        let context_map = get_context_map_for_span(item, config);
        let auto_capture_stmt = if config.auto_capture {
            quote! { let _auto_capture_guard = ::log_args_runtime::capture_context(); }
        } else {
            quote! {}
        };
        let push_fn = if is_async {
            quote! { ::log_args_runtime::push_async_context(#context_map) }
        } else {
            quote! { ::log_args_runtime::push_context(#context_map) }
        };

        quote! {
            {
                let _context_guard = #push_fn;
                #auto_capture_stmt
                #log_redefines
                #transformed_block
            }
        }
    } else {
        quote! {
            {
                #log_redefines
                #transformed_block
            }
        }
    }
}

/// Represents the different attribute configurations available for the `#[params]` macro.
///
/// Each attribute controls how function parameters are logged and how context is propagated
/// to child functions. These attributes can be combined to create flexible logging strategies.
///
/// # Available Attributes
///
/// - `fields(...)` - Selectively log specific function parameters as individual fields
/// - `custom(...)` - Add computed fields with custom expressions and metadata
/// - `current(...)` - Log current context values (legacy/internal use)
/// - `clone_upfront` - Clone parameters before async operations to prevent move issues
/// - `span(...)` - Set up context propagation for child functions to inherit
/// - `all` - Log all function parameters (use with caution in production)
/// - `auto_capture` - Automatically capture context in closures and spawned tasks
///
/// # Security Note
///
/// By default, `#[params]` without arguments is secure and doesn't log parameters.
/// Always be explicit about what you log in production environments.
enum Attribute {
    /// **Selective Parameter Logging** - `fields(param1, param2, ...)`
    ///
    /// Logs only the specified function parameters as individual fields in the log output.
    /// This is the recommended approach for production logging as it gives you precise
    /// control over what data is logged.
    ///
    /// # Example
    /// ```rust,ignore
    /// #[params(fields(user_id, action))]
    /// fn user_action(user_id: String, action: String, password: String) {
    ///     info!("User performed action"); // Only user_id and action are logged
    /// }
    /// ```
    ///
    /// # Security
    /// - ✅ Secure: Only specified parameters are logged
    /// - ✅ Production-safe: Excludes sensitive data by default
    /// - ✅ Performance: Only processes specified fields
    Fields(Punctuated<Expr, Token![,]>),

    /// **Custom Computed Fields** - `custom(field_name = expression, ...)`
    ///
    /// Adds computed fields to log output using custom expressions. Useful for adding
    /// metadata, timestamps, or derived values that aren't direct function parameters.
    ///
    /// # Example
    /// ```rust,ignore
    /// #[params(
    ///     custom(
    ///         timestamp = std::time::SystemTime::now(),
    ///         data_size = data.len(),
    ///         is_admin = user.role == "admin"
    ///     )
    /// )]
    /// fn process_data(data: Vec<u8>, user: User) {
    ///     info!("Processing data"); // Includes computed fields
    /// }
    /// ```
    ///
    /// # Performance Note
    /// Keep expressions lightweight as they're evaluated on every log call.
    Custom(Punctuated<MetaNameValue, Token![,]>),

    /// **Current Context Values** - `current(...)`
    ///
    /// Internal attribute for logging current context values. Primarily used internally
    /// by the macro system for context management.
    ///
    /// # Usage
    /// This is typically not used directly by end users.
    Current(Punctuated<Expr, Token![,]>),

    /// **Clone Upfront** - `clone_upfront`
    ///
    /// Clones function parameters before async operations to prevent ownership issues.
    /// Useful when parameters need to be moved into async blocks or spawned tasks.
    ///
    /// # Example
    /// ```rust,ignore
    /// #[params(fields(user_id), clone_upfront)]
    /// async fn async_operation(user_id: String, data: Vec<u8>) {
    ///     tokio::spawn(async move {
    ///         // user_id was cloned upfront, so this works
    ///         process_data(data).await;
    ///     });
    /// }
    /// ```
    ///
    /// # Performance Impact
    /// Only use when necessary as it adds cloning overhead.
    CloneUpfront,

    /// **Context Propagation** - `span(param1, param2, ...)` or `span`
    ///
    /// Sets up automatic context inheritance for child functions. This is the key feature
    /// that enables truly automatic context propagation across function boundaries.
    ///
    /// # Example
    /// ```rust,ignore
    /// #[params(span(request_id, user_id))]
    /// fn handle_request(request_id: String, user_id: String, data: String) {
    ///     info!("Request received");
    ///     process_data(data); // Child function inherits request_id and user_id
    /// }
    ///
    /// #[params] // Inherits context from parent
    /// fn process_data(data: String) {
    ///     info!("Processing"); // Automatically includes request_id and user_id
    /// }
    /// ```
    ///
    /// # Cross-Boundary Support
    /// - ✅ Async/await boundaries
    /// - ✅ Spawned tasks (tokio::spawn)
    /// - ✅ Closures and iterators
    /// - ✅ Thread boundaries
    Span(Punctuated<Expr, Token![,]>),

    /// **Log All Parameters** - `all`
    ///
    /// Logs all function parameters as individual fields.
    ///
    /// # ⚠️ Security Warning
    /// Use with extreme caution in production as this logs ALL parameters,
    /// including potentially sensitive data like passwords, tokens, and personal information.
    ///
    /// # Example
    /// ```rust,ignore
    /// #[params(all)] // ⚠️ Only use in development/debugging
    /// fn debug_function(user_id: String, email: String, data: Vec<u8>) {
    ///     info!("Debug info"); // Logs ALL parameters
    /// }
    /// ```
    ///
    /// # Recommended Usage
    /// - ✅ Development and debugging
    /// - ✅ Non-production environments
    /// - ❌ Production environments
    /// - ❌ Functions with sensitive parameters
    All,

    /// **Automatic Context Capture** - `auto_capture`
    ///
    /// Automatically captures and propagates context in closures and spawned tasks.
    /// This ensures context is preserved even in complex async scenarios.
    ///
    /// # Example
    /// ```rust,ignore
    /// #[params(span(batch_id), auto_capture)]
    /// fn process_batch(batch_id: String, items: Vec<Item>) {
    ///     items.iter().for_each(|item| {
    ///         // Context automatically captured in closure
    ///         process_item(item.clone());
    ///     });
    /// }
    /// ```
    ///
    /// # Use Cases
    /// - Complex async workflows
    /// - Iterator chains with closures
    /// - Nested task spawning
    AutoCapture,
}

impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if ident == "fields" {
            let content;
            parenthesized!(content in input);
            let fields = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;
            Ok(Attribute::Fields(fields))
        } else if ident == "custom" {
            let content;
            parenthesized!(content in input);
            let custom = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(&content)?;
            Ok(Attribute::Custom(custom))
        } else if ident == "current" {
            let content;
            parenthesized!(content in input);
            let current = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;
            Ok(Attribute::Current(current))
        } else if ident == "clone_upfront" {
            Ok(Attribute::CloneUpfront)
        } else if ident == "span" {
            // Accept both `span` and `span(...)`
            if input.peek(syn::token::Paren) {
                let content;
                parenthesized!(content in input);
                let span_fields = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;
                Ok(Attribute::Span(span_fields))
            } else {
                Ok(Attribute::Span(Punctuated::new()))
            }
        } else if ident == "all" {
            Ok(Attribute::All)
        } else if ident == "auto_capture" {
            Ok(Attribute::AutoCapture)
        } else {
            Err(syn::Error::new_spanned(ident, "unknown attribute"))
        }
    }
}

struct AttrConfig {
    fields: Vec<syn::Expr>,
    custom: Vec<syn::MetaNameValue>,
    current: Vec<syn::Expr>,
    clone_upfront: bool,
    span: bool,
    span_fields: Vec<syn::Expr>,
    all_params: bool,
    auto_capture: bool, // New field for automatic closure context capture
}

impl Default for AttrConfig {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
            custom: Vec::new(),
            current: Vec::new(),
            clone_upfront: true, // Default to true for safety
            span: false,         // Disable span by default per requirement
            span_fields: Vec::new(),
            all_params: false,
            auto_capture: false, // Default to false for auto_capture
        }
    }
}

impl AttrConfig {
    fn from_attributes(attrs: Punctuated<Attribute, Token![,]>) -> Self {
        let mut config = AttrConfig::default();
        for attr in attrs {
            match attr {
                Attribute::Fields(fields) => config.fields.extend(fields),
                Attribute::Custom(custom) => config.custom.extend(custom),
                Attribute::Current(current) => config.current.extend(current),
                Attribute::CloneUpfront => config.clone_upfront = true,
                Attribute::Span(span_fields) => {
                    config.span = true;
                    config.clone_upfront = true; // Span implies clone_upfront for safety
                    config.span_fields.extend(span_fields);
                }
                Attribute::All => {
                    config.all_params = true;
                }
                Attribute::AutoCapture => {
                    config.auto_capture = true;
                }
            }
        }
        config
    }
}

fn get_context_fields_quote(item: &FnItem, config: &AttrConfig) -> Vec<proc_macro2::TokenStream> {
    let mut field_assignments = vec![];

    // Determine what to log based on configuration
    let _has_selective_attributes =
        !config.fields.is_empty() || !config.custom.is_empty() || !config.current.is_empty();

    // For span propagation, automatically inherit parent context fields
    // This ensures automatic context propagation without manual attributes
    if config.span
        && config.fields.is_empty()
        && config.custom.is_empty()
        && config.current.is_empty()
        && !config.all_params
    {
        // When only span is enabled (default behavior), inherit all parent context fields
        // This uses the runtime macro to dynamically include inherited fields
        if WITH_CONTEXT_ENABLED {
            field_assignments.push(quote! {
                context = ::log_args_runtime::get_inherited_context_string()
            });
        }
    }

    if config.all_params {
        // Log all parameters only when 'all' is explicitly specified
        let all_args = get_all_args(item);
        for ident in all_args {
            let ident_str = ident.to_string();
            // When span is enabled, use span context lookup for post-move safety
            if config.span {
                field_assignments.push(quote! {
                    #ident = ::log_args_runtime::get_context_value(&#ident_str).unwrap_or_else(|| "".to_string())
                });
            } else {
                field_assignments.push(quote! {#ident = ?#ident });
            }
        }
    }

    if !config.fields.is_empty() {
        // Log only specified fields
        for field_expr in &config.fields {
            // Convert complex expressions to string field names
            let field_name = quote! { #field_expr }.to_string();
            let field_key = field_name.replace(' ', "");

            // If clone_upfront is enabled and expression contains self.field, handle it specially
            if config.clone_upfront {
                let expr_str = quote!(#field_expr).to_string();
                if expr_str.contains("self.") {
                    // When span is enabled, use span context lookup for post-move safety
                    if config.span {
                        field_assignments.push(quote! {
                            #field_name = ::log_args_runtime::get_context_value(&#field_key).unwrap_or_else(|| "".to_string())
                        });
                    } else {
                        // No span, use cloned variable approach (similar to custom fields)
                        let mut modified_expr_str = expr_str.clone();
                        let mut start = 0;
                        while let Some(pos) = modified_expr_str[start..].find("self.") {
                            let field_start = start + pos + 5; // Skip "self."
                            let remaining = &modified_expr_str[field_start..];

                            // Find the end of the field name
                            let field_end = remaining
                                .find(|c: char| !c.is_alphanumeric() && c != '_')
                                .unwrap_or(remaining.len());

                            let field_name_part = &remaining[..field_end];
                            let replacement = format!("__{field_name_part}_for_macro");

                            // Replace self.field_name with __field_name_for_macro
                            let old_expr = format!("self.{field_name_part}");
                            modified_expr_str = modified_expr_str.replace(&old_expr, &replacement);

                            start = field_start + field_end;
                        }

                        // Parse the modified string back to a token stream
                        let modified_expr: proc_macro2::TokenStream = modified_expr_str
                            .parse()
                            .unwrap_or_else(|_| quote!(#field_expr));
                        field_assignments.push(quote! {#field_name = ?#modified_expr });
                    }
                } else {
                    field_assignments.push(quote! {#field_name = ?#field_expr });
                }
            } else {
                field_assignments.push(quote! {#field_name = ?#field_expr });
            }
        }
    }

    // If user specified span(field1, field2, ...), inject these using values from context if available
    if !config.span_fields.is_empty() {
        for field_expr in &config.span_fields {
            let field_name = quote! { #field_expr }.to_string();
            let field_key = field_name.replace(' ', "");
            // Pull value from context if present; otherwise default to empty string
            field_assignments.push(quote! {
                #field_name = ::log_args_runtime::get_context_value(&#field_key).unwrap_or_else(|| "".to_string())
            });
        }
    }
    // Default behavior: Only enable span propagation and function name logging
    // No automatic parameter logging unless explicitly requested
    // If only custom/current are specified (no fields), we don't log any parameters

    // Add custom fields (always included)
    for nv in &config.custom {
        let key = &nv.path;
        let value = &nv.value;

        // Add to logging fields
        field_assignments.push(quote! {
            #key = #value
        });
    }

    // Add current fields (only logged in current function, not propagated)
    for current_field in &config.current {
        let field_name = quote! { #current_field }.to_string();
        let field_key = field_name.replace(' ', "");

        // If clone_upfront is enabled and expression contains self.field, handle it specially
        if config.clone_upfront {
            let expr_str = quote!(#current_field).to_string();
            if expr_str.contains("self.") {
                // When span is enabled, use span context lookup for post-move safety
                if config.span {
                    field_assignments.push(quote! {
                        #field_name = ::log_args_runtime::get_context_value(&#field_key).unwrap_or_else(|| "".to_string())
                    });
                } else {
                    // No span, use cloned variable approach (similar to custom fields)
                    let mut modified_expr_str = expr_str.clone();
                    let mut start = 0;
                    while let Some(pos) = modified_expr_str[start..].find("self.") {
                        let field_start = start + pos + 5; // Skip "self."
                        let remaining = &modified_expr_str[field_start..];

                        // Find the end of the field name
                        let field_end = remaining
                            .find(|c: char| !c.is_alphanumeric() && c != '_')
                            .unwrap_or(remaining.len());

                        let field_name_part = &remaining[..field_end];
                        let replacement = format!("__{field_name_part}_for_macro");

                        // Replace self.field_name with __field_name_for_macro
                        let old_expr = format!("self.{field_name_part}");
                        modified_expr_str = modified_expr_str.replace(&old_expr, &replacement);

                        start = field_start + field_end;
                    }

                    // Parse the modified string back to a token stream
                    let modified_expr: proc_macro2::TokenStream = modified_expr_str
                        .parse()
                        .unwrap_or_else(|_| quote!(#current_field));
                    field_assignments.push(quote! {#field_name = ?#modified_expr });
                }
            } else {
                field_assignments.push(quote! {#field_name = ?#current_field });
            }
        } else {
            field_assignments.push(quote! {#field_name = ?#current_field });
        }
    }

    // Add function name if any function-names feature is enabled
    add_function_name_field(&mut field_assignments, item);

    field_assignments
}

/// Add function name field to log output when any function-names feature is enabled.
/// The function name will be formatted according to the enabled feature.
fn add_function_name_field(field_assignments: &mut Vec<proc_macro2::TokenStream>, item: &FnItem) {
    // Check if any function-names feature is enabled
    #[cfg(any(
        feature = "function-names-snake",
        feature = "function-names-camel",
        feature = "function-names-pascal",
        feature = "function-names-screaming",
        feature = "function-names-kebab",
        feature = "function-names"
    ))]
    {
        let function_name = item.sig().ident.to_string();
        let formatted_name = get_formatted_function_name(&function_name);

        field_assignments.push(quote! {
            function_name = #formatted_name
        });
    }
}

fn get_context_map_for_span(_item: &FnItem, config: &AttrConfig) -> proc_macro2::TokenStream {
    let mut fields_to_log = vec![];

    // Store all field types in span context for dynamic lookup
    // This ensures that span context lookup works for ALL field types

    // 1. Add all parameters if requested
    if config.all_params {
        let all_args = get_all_args(_item);
        for ident in all_args {
            let ident_str = ident.to_string();
            fields_to_log.push(quote! {
                new_context.insert(#ident_str.to_string(), format!("{:?}", #ident));
            });
        }
    }

    // 2. Add explicitly specified fields
    if !config.fields.is_empty() {
        for field_expr in &config.fields {
            let key_str = quote!(#field_expr).to_string().replace(' ', "");
            fields_to_log.push(quote! {
                new_context.insert(#key_str.to_string(), format!("{:?}", &#field_expr));
            });
        }
    }

    // Note: Do NOT store span(...) keys in context; they are only added to the current log call

    // 3. Add custom fields (always included)
    for nv in &config.custom {
        let key = &nv.path;
        let value = &nv.value;
        let key_str = quote!(#key).to_string().replace(' ', "");

        // For span context, use the original expression directly
        // This will be evaluated before any moves happen
        fields_to_log.push(quote! {
            new_context.insert(#key_str.to_string(), format!("{}", #value));
        });

        // Also store globally for cross-boundary persistence
        fields_to_log.push(quote! {
            ::log_args_runtime::set_global_context(&#key_str, &format!("{}", #value));
        });
    }

    // 4. Add current fields (these are also stored in context for consistency)
    for current_field in &config.current {
        let field_name = quote! { #current_field }.to_string();
        let field_key = field_name.replace(' ', "");
        fields_to_log.push(quote! {
            new_context.insert(#field_key.to_string(), format!("{:?}", #current_field));
        });
    }

    // Add function name to context if any function-names feature is enabled (always propagated)

    quote! {
        {
            let mut new_context = ::std::collections::HashMap::new();
            #(#fields_to_log)*
            new_context
        }
    }
}

fn get_all_args(item: &FnItem) -> Vec<Ident> {
    item.sig()
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pt) = arg {
                if let Pat::Ident(pi) = &*pt.pat {
                    if pi.ident != "self" {
                        return Some(pi.ident.clone());
                    }
                }
            }
            None
        })
        .collect()
}

enum FnItem {
    Item(syn::ItemFn),
    ImplItem(syn::ImplItemFn),
}

impl quote::ToTokens for FnItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FnItem::Item(i) => i.to_tokens(tokens),
            FnItem::ImplItem(i) => i.to_tokens(tokens),
        }
    }
}

impl FnItem {
    fn attrs_mut(&mut self) -> &mut Vec<syn::Attribute> {
        match self {
            FnItem::Item(item_fn) => &mut item_fn.attrs,
            FnItem::ImplItem(impl_item_fn) => &mut impl_item_fn.attrs,
        }
    }

    fn sig(&self) -> &syn::Signature {
        match self {
            FnItem::Item(i) => &i.sig,
            FnItem::ImplItem(i) => &i.sig,
        }
    }

    fn block(&self) -> &syn::Block {
        match self {
            FnItem::Item(i) => &i.block,
            FnItem::ImplItem(i) => &i.block,
        }
    }

    fn block_mut(&mut self) -> &mut syn::Block {
        match self {
            FnItem::Item(i) => &mut i.block,
            FnItem::ImplItem(i) => &mut i.block,
        }
    }
}

fn get_log_redefines_with_fields(
    context_fields: &[proc_macro2::TokenStream],
    _is_async: bool,
) -> proc_macro2::TokenStream {
    // Always redefine macros to include both local fields and inherited context
    // The context inheritance will be handled by including context fields from the runtime
    quote! {
        macro_rules! info {
            ($($t:tt)*) => {
                ::log_args_runtime::log_with_context!(::tracing::info, ::log_args_runtime::get_context(), #(#context_fields,)* $($t)*)
            };
        }
        macro_rules! warn {
            ($($t:tt)*) => {
                ::log_args_runtime::log_with_context!(::tracing::warn, ::log_args_runtime::get_context(), #(#context_fields,)* $($t)*)
            };
        }
        macro_rules! error {
            ($($t:tt)*) => {
                ::log_args_runtime::log_with_context!(::tracing::error, ::log_args_runtime::get_context(), #(#context_fields,)* $($t)*)
            };
        }
        macro_rules! debug {
            ($($t:tt)*) => {
                ::log_args_runtime::log_with_context!(::tracing::debug, ::log_args_runtime::get_context(), #(#context_fields,)* $($t)*)
            };
        }
        macro_rules! trace {
            ($($t:tt)*) => {
                ::log_args_runtime::log_with_context!(::tracing::trace, ::log_args_runtime::get_context(), #(#context_fields,)* $($t)*)
            };
        }
    }
}
