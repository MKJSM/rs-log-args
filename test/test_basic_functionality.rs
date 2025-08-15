//! Basic functionality tests for log_args
//! 
//! Tests the fundamental behavior of the #[params] macro

use log_args::params;
use tracing::{info, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::{Arc, Mutex};

// Custom test subscriber to capture log output
struct TestSubscriber {
    logs: Arc<Mutex<Vec<String>>>,
}

impl TestSubscriber {
    fn new() -> (Self, Arc<Mutex<Vec<String>>>) {
        let logs = Arc::new(Mutex::new(Vec::new()));
        (TestSubscriber { logs: logs.clone() }, logs)
    }
}

// Test basic parameter logging
#[params]
fn test_basic_function(user_id: u64, username: String) {
    info!("Basic function executed");
}

// Test function with no parameters
#[params]
fn test_no_params_function() {
    info!("No parameters function executed");
}

// Test async function
#[params]
async fn test_async_function(request_id: String, timeout: u32) {
    info!("Async function started");
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    info!("Async function completed");
}

// Test function with complex parameters
#[params]
fn test_complex_params(
    id: u64,
    name: String,
    values: Vec<i32>,
    metadata: std::collections::HashMap<String, String>,
) {
    info!("Complex parameters function executed");
}

// Test method in impl block
struct TestService {
    service_id: String,
}

impl TestService {
    #[params]
    fn test_method(&self, operation: String) {
        info!("Method executed");
    }
    
    #[params]
    async fn test_async_method(&self, data: Vec<u8>) {
        info!("Async method executed");
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    fn setup_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_test_writer()
            .try_init();
    }

    #[test]
    fn test_basic_function_compiles_and_runs() {
        setup_tracing();
        
        // Should not panic and should compile
        test_basic_function(123, "test_user".to_string());
        
        // Test with different parameters
        test_basic_function(456, "another_user".to_string());
    }

    #[test]
    fn test_no_params_function_works() {
        setup_tracing();
        
        // Should work with no parameters
        test_no_params_function();
    }

    #[tokio::test]
    async fn test_async_function_works() {
        setup_tracing();
        
        // Should work with async functions
        test_async_function("req_123".to_string(), 5000).await;
    }

    #[test]
    fn test_complex_parameters() {
        setup_tracing();
        
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());
        metadata.insert("key2".to_string(), "value2".to_string());
        
        test_complex_params(
            789,
            "complex_test".to_string(),
            vec![1, 2, 3, 4, 5],
            metadata,
        );
    }

    #[test]
    fn test_method_support() {
        setup_tracing();
        
        let service = TestService {
            service_id: "test_service_001".to_string(),
        };
        
        service.test_method("test_operation".to_string());
    }

    #[tokio::test]
    async fn test_async_method_support() {
        setup_tracing();
        
        let service = TestService {
            service_id: "async_service_001".to_string(),
        };
        
        service.test_async_method(vec![1, 2, 3, 4]).await;
    }

    #[test]
    fn test_multiple_calls_same_function() {
        setup_tracing();
        
        // Test that function can be called multiple times
        for i in 0..5 {
            test_basic_function(i, format!("user_{}", i));
        }
    }

    #[test]
    fn test_nested_function_calls() {
        setup_tracing();
        
        #[params]
        fn parent_function(parent_id: u64) {
            info!("Parent function");
            child_function(parent_id + 1);
        }
        
        #[params]
        fn child_function(child_id: u64) {
            info!("Child function");
        }
        
        parent_function(100);
    }

    #[tokio::test]
    async fn test_async_nested_calls() {
        setup_tracing();
        
        #[params]
        async fn async_parent(parent_id: u64) {
            info!("Async parent");
            async_child(parent_id + 1).await;
        }
        
        #[params]
        async fn async_child(child_id: u64) {
            info!("Async child");
        }
        
        async_parent(200).await;
    }

    #[test]
    fn test_error_handling_with_results() {
        setup_tracing();
        
        #[params]
        fn fallible_function(input: i32) -> Result<String, String> {
            info!("Fallible function");
            if input > 0 {
                Ok(format!("Success: {}", input))
            } else {
                Err("Input must be positive".to_string())
            }
        }
        
        // Test success case
        let result = fallible_function(42);
        assert!(result.is_ok());
        
        // Test error case
        let result = fallible_function(-1);
        assert!(result.is_err());
    }

    #[test]
    fn test_generic_functions() {
        setup_tracing();
        
        #[params]
        fn generic_function<T: std::fmt::Debug>(item: T, count: usize) {
            info!("Generic function executed");
        }
        
        generic_function("string".to_string(), 1);
        generic_function(42i32, 2);
        generic_function(vec![1, 2, 3], 3);
    }

    #[test]
    fn test_function_with_references() {
        setup_tracing();
        
        #[params]
        fn function_with_refs(name: &str, data: &[u8], count: &usize) {
            info!("Function with references");
        }
        
        let name = "test";
        let data = vec![1, 2, 3, 4];
        let count = 42;
        
        function_with_refs(&name, &data, &count);
    }
}
