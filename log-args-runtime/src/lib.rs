//! log-args-runtime
//!
//! Runtime support for the `log_args` procedural macros. This crate provides:
//! - Context storage and helpers to push/pop context across sync and async boundaries
//! - Logging macros (`info!`, `warn!`, `error!`, `debug!`, `trace!`) that automatically
//!   merge inherited context into your events
//! - `log_with_context!` which enriches an underlying `tracing` macro
//!
//! Feature flags
//! - `with_context` (off by default): When enabled, the runtime includes a `context` field
//!   (debug-formatted map) in each log when there is context available. Configure your
//!   `tracing-subscriber` JSON formatter with `.flatten_event(true)` to surface the fields
//!   at the top level in JSON output.
//!
//! Quick start
//! ```no_run
//! use tracing::{info, Level};
//!
//! fn init() {
//!     tracing_subscriber::fmt().json().flatten_event(true).with_max_level(Level::DEBUG).init();
//! }
//!
//! fn main() {
//!     init();
//!     log_args_runtime::info!("hello");
//! }
//! ```
//!
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Feature gate evaluated in this crate, not at the macro callsite.
// Downstream crates won't see unexpected cfg values.
pub const WITH_CONTEXT_ENABLED: bool = cfg!(feature = "with_context");

// Global context store for cross-boundary persistence
static GLOBAL_CONTEXT: std::sync::LazyLock<Arc<Mutex<HashMap<String, String>>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Set global context that persists across all boundaries
pub fn set_global_context(key: &str, value: &str) {
    if let Ok(mut global) = GLOBAL_CONTEXT.lock() {
        global.insert(key.to_string(), value.to_string());
    }
}

/// Get global context for cross-boundary persistence
pub fn get_global_context() -> Option<HashMap<String, String>> {
    if let Ok(global) = GLOBAL_CONTEXT.lock() {
        if !global.is_empty() {
            return Some(global.clone());
        }
    }
    None
}

// Thread-local storage for context stacks
thread_local! {
    static CONTEXT_STACK: RefCell<Vec<HashMap<String, String>>> = RefCell::new(Vec::new());
    static ASYNC_CONTEXT_STACK: RefCell<Vec<HashMap<String, String>>> = RefCell::new(Vec::new());
}

/// Guard for synchronous context that automatically pops on drop
#[doc(hidden)]
pub struct ContextGuard;

impl Drop for ContextGuard {
    fn drop(&mut self) {
        CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}

// Function to get a context value from the current span context
pub fn get_context_value(key: &str) -> Option<String> {
    // First, try async context stack
    if let Ok(stack) = ASYNC_CONTEXT_STACK.try_with(|stack| stack.borrow().clone()) {
        for context_map in stack.iter().rev() {
            if let Some(value) = context_map.get(key) {
                return Some(value.clone());
            }
        }
    }

    // Then try sync context stack
    let result = CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        for context_map in stack.iter().rev() {
            if let Some(value) = context_map.get(key) {
                return Some(value.clone());
            }
        }
        None
    });

    if result.is_some() {
        return result;
    }

    // Finally, try global context store for cross-boundary persistence
    if let Ok(global) = GLOBAL_CONTEXT.lock() {
        if let Some(value) = global.get(key) {
            return Some(value.clone());
        }
    }

    None
}

/// Get current synchronous context
#[doc(hidden)]
pub fn get_context() -> HashMap<String, String> {
    CONTEXT_STACK.with(|stack| {
        stack
            .borrow()
            .iter()
            .fold(HashMap::new(), |mut acc, context| {
                acc.extend(context.clone());
                acc
            })
    })
}

#[doc(hidden)]
pub fn get_async_context() -> HashMap<String, String> {
    ASYNC_CONTEXT_STACK
        .try_with(|stack| {
            stack
                .borrow()
                .iter()
                .fold(HashMap::new(), |mut acc, context| {
                    acc.extend(context.clone());
                    acc
                })
        })
        .unwrap_or_default()
}

#[doc(hidden)]
pub fn get_current_async_stack() -> Vec<HashMap<String, String>> {
    ASYNC_CONTEXT_STACK
        .try_with(|stack| stack.borrow().clone())
        .unwrap_or_else(|_| vec![HashMap::new()])
}

/// Push context for synchronous functions with span
#[doc(hidden)]
pub fn push_context(context: HashMap<String, String>) -> ContextGuard {
    CONTEXT_STACK.with(|stack| {
        stack.borrow_mut().push(context);
    });
    ContextGuard
}

/// Push context for asynchronous functions with span
#[doc(hidden)]
pub fn push_async_context(context: HashMap<String, String>) -> AsyncContextGuard {
    ASYNC_CONTEXT_STACK.with(|stack| {
        stack.borrow_mut().push(context);
    });
    AsyncContextGuard
}

/// Guard for async context that automatically pops on drop
pub struct AsyncContextGuard;

impl Drop for AsyncContextGuard {
    fn drop(&mut self) {
        ASYNC_CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}

// Helper macro to dynamically add context fields to log statements
// This macro is now completely dynamic with no hardcoded field names
#[macro_export]
macro_rules! add_context_fields {
    ($log_macro:path, $ctx:expr, $($args:tt)*) => {
        // Completely dynamic approach - no hardcoded field names
        // Create field tokens for all context fields dynamically
        let mut field_tokens = Vec::new();

        // Add all context fields dynamically without hardcoding any field names
        for (key, value) in $ctx.iter() {
            // Create a field token for any field name
            let field_token = if key.contains('.') {
                // Handle dotted field names (like "user.id")
                format!("\"{key}\" = %{value}", key = key, value = value)
            } else {
                // Handle regular field names
                format!("{key} = %{value}", key = key, value = value)
            };
            field_tokens.push(field_token);
        }

        // Note: This approach still has Rust macro limitations
        // The field tokens can't be directly injected into the macro call
        // This is kept for potential future use or alternative implementations
    };
}

#[macro_export]
macro_rules! log_with_context {
    ($log_macro:path, $context:expr, $($args:tt)*) => {
        {
            let ctx = $context;
            // Avoid cfg in macro body; use a const from this crate instead.
            if !$crate::WITH_CONTEXT_ENABLED {
                $log_macro!($($args)*);
            } else {
                // Pass the context map as a debug-formatted field.
                // The tracing-subscriber can be configured to flatten this.
                $log_macro!(context = ?ctx, $($args)*);
            }
        }
    };
}

/// Global context-aware logging macros that inherit parent context
/// These can be used in any function to automatically include context from parent functions with span
#[macro_export]
macro_rules! info {
    ($($t:tt)*) => {
        $crate::log_with_context!(::tracing::info, $crate::get_context(), $($t)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($t:tt)*) => {
        $crate::log_with_context!(::tracing::warn, $crate::get_context(), $($t)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => {
        $crate::log_with_context!(::tracing::error, $crate::get_context(), $($t)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($t:tt)*) => {
        $crate::log_with_context!(::tracing::debug, $crate::get_context(), $($t)*)
    };
}

#[macro_export]
macro_rules! trace {
    ($($t:tt)*) => {
        $crate::log_with_context!(::tracing::trace, $crate::get_context(), $($t)*)
    };
}

/// Automatically capture and preserve current context for function execution
/// This ensures context is maintained across function boundaries without user intervention
pub fn auto_capture_context() -> ContextGuard {
    let current_context = get_context();

    // Push to both async and sync stacks to ensure maximum compatibility
    let _async_guard = push_async_context(current_context.clone());
    let _sync_guard = push_context(current_context);

    // Return the existing ContextGuard (empty struct)
    ContextGuard
}

/// Capture current context and store it globally for cross-boundary persistence
/// This function is automatically called by the macro to ensure context is preserved
pub fn capture_context() -> ContextGuard {
    // Merge async and sync contexts so we don't lose fields pushed to the async stack
    let mut current_context = get_async_context();
    let sync_ctx = get_context();
    current_context.extend(sync_ctx);

    // Store each context field globally for cross-boundary access
    for (key, value) in &current_context {
        set_global_context(key, value);
    }

    // Also push to context stacks for immediate access after capture
    let _async_guard = push_async_context(current_context.clone());
    let _sync_guard = push_context(current_context);

    // Return the existing ContextGuard (empty struct)
    ContextGuard
}

/// Get inherited context as a formatted string for automatic span propagation
/// This function retrieves all context fields from the current span context
/// and formats them as a string for logging
pub fn get_inherited_context_string() -> String {
    let mut context_parts = Vec::new();

    // First, try to get context from tracing span (most reliable for cross-boundary propagation)
    let current_span = tracing::Span::current();
    if !current_span.is_none() {
        // Try to extract fields from the current span
        // This works across async boundaries when spans are properly propagated
        // Note: Direct span field extraction is complex, so we rely on other methods
    }

    // Try async context stack (most likely to have the context)
    if let Ok(stack) = ASYNC_CONTEXT_STACK.try_with(|stack| stack.borrow().clone()) {
        // Search through all contexts in the stack, not just the most recent
        for context_map in stack.iter().rev() {
            for (key, value) in context_map {
                // Skip function name to avoid duplication
                if key != "function"
                    && !context_parts
                        .iter()
                        .any(|p: &String| p.starts_with(&format!("{key}=")))
                {
                    context_parts.push(format!("{key}={value}"));
                }
            }
        }
    }

    // Also try sync context stack and merge results
    CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        for context_map in stack.iter().rev() {
            for (key, value) in context_map {
                // Skip function name and avoid duplicates
                if key != "function"
                    && !context_parts
                        .iter()
                        .any(|p: &String| p.starts_with(&format!("{key}=")))
                {
                    context_parts.push(format!("{key}={value}"));
                }
            }
        }
    });

    // If still no context, try global context store (for cross-boundary persistence)
    if context_parts.is_empty() {
        if let Some(global_context) = get_global_context() {
            for (key, value) in global_context {
                if key != "function" {
                    context_parts.push(format!("{key}={value}"));
                }
            }
        }
    }

    if context_parts.is_empty() {
        "".to_string()
    } else {
        context_parts.join(",")
    }
}

/// Get inherited context fields as individual key-value pairs
/// This function returns a HashMap of inherited context fields for dynamic field injection
pub fn get_inherited_fields_map() -> std::collections::HashMap<String, String> {
    let mut context_map = std::collections::HashMap::new();

    // Try async context stack first
    if let Ok(stack) = ASYNC_CONTEXT_STACK.try_with(|stack| stack.borrow().clone()) {
        for stack_context in stack.iter().rev() {
            for (key, value) in stack_context {
                // Skip function name to avoid duplication
                if key != "function" {
                    context_map.insert(key.clone(), value.clone());
                }
            }
            if !context_map.is_empty() {
                return context_map; // Use the most recent context
            }
        }
    }

    // If no async context, try sync context stack
    if context_map.is_empty() {
        CONTEXT_STACK.with(|stack| {
            let stack = stack.borrow();
            for stack_context in stack.iter().rev() {
                for (key, value) in stack_context {
                    // Skip function name to avoid duplication
                    if key != "function" {
                        context_map.insert(key.clone(), value.clone());
                    }
                }
                if !context_map.is_empty() {
                    return; // Use the most recent context
                }
            }
        });
    }

    context_map
}
