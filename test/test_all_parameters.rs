//! Tests for the 'all' attribute functionality
//! 
//! Tests logging all function parameters

use log_args::params;
use tracing::{info, Level};
use tracing_subscriber;
use std::collections::{HashMap, HashSet};

// Test basic all parameters
#[params(all)]
fn test_all_basic(user_id: u64, username: String, enabled: bool) {
    info!("All basic parameters function");
}

// Test all parameters with complex types
#[params(all)]
fn test_all_complex(
    id: u64,
    name: String,
    values: Vec<i32>,
    metadata: HashMap<String, String>,
    tags: HashSet<String>,
) {
    info!("All complex parameters function");
}

// Test all parameters with span propagation
#[params(all, span)]
fn test_all_with_span(parent_id: u64, operation: String) {
    info!("All parameters with span");
    test_all_span_child();
}

#[params(all)]
fn test_all_span_child() {
    info!("All parameters span child"); // Should inherit parent context
}

// Test all parameters with custom fields
#[params(all, custom(service = "all-params-service", version = "1.0"))]
fn test_all_with_custom(request_id: String, priority: u32) {
    info!("All parameters with custom fields");
}

// Test all parameters async
#[params(all, span)]
async fn test_all_async(
    session_id: String,
    timeout: u64,
    data: Vec<u8>,
) {
    info!("All async parameters");
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
}

// Test all parameters with methods
struct TestAllService {
    service_name: String,
    config: ServiceConfig,
}

#[derive(Debug)]
struct ServiceConfig {
    max_connections: u32,
    timeout_ms: u64,
}

impl TestAllService {
    #[params(all)]
    fn all_method(&self, operation: String, data: Vec<u8>) {
        info!("All parameters method");
    }
    
    #[params(all, span)]
    fn all_method_with_span(&self, request_id: String) {
        info!("All parameters method with span");
        self.all_child_method();
    }
    
    #[params(all)]
    fn all_child_method(&self) {
        info!("All parameters child method");
    }
    
    #[params(all)]
    async fn all_async_method(&self, task_id: String, payload: String) {
        info!("All parameters async method");
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
}

// Test all parameters with references
#[params(all)]
fn test_all_with_refs(name: &str, data: &[u8], config: &HashMap<String, String>) {
    info!("All parameters with references");
}

// Test all parameters with generics
#[params(all)]
fn test_all_generic<T: std::fmt::Debug>(item: T, count: usize, metadata: String) {
    info!("All parameters generic function");
}

// Test all parameters with optional/result types
#[params(all)]
fn test_all_with_option(
    required: String,
    optional: Option<u64>,
    result_data: Result<String, String>,
) -> Result<(), String> {
    info!("All parameters with Option and Result");
    Ok(())
}

// WARNING: Security-sensitive test (for demo purposes only)
#[params(all)]
fn test_all_security_warning(
    user_id: u64,
    password: String,        // This WILL be logged!
    api_key: String,         // This WILL be logged!
    credit_card: String,     // This WILL be logged!
) {
    info!("WARNING: This logs sensitive data - for testing only!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_test_writer()
            .try_init();
    }

    #[test]
    fn test_all_basic_parameters() {
        setup_tracing();
        
        // Should log all three parameters
        test_all_basic(123, "testuser".to_string(), true);
    }

    #[test]
    fn test_all_complex_types() {
        setup_tracing();
        
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());
        metadata.insert("key2".to_string(), "value2".to_string());
        
        let mut tags = HashSet::new();
        tags.insert("tag1".to_string());
        tags.insert("tag2".to_string());
        
        // Should log all complex parameters
        test_all_complex(
            456,
            "complex_test".to_string(),
            vec![1, 2, 3, 4, 5],
            metadata,
            tags,
        );
    }

    #[test]
    fn test_all_with_span_propagation() {
        setup_tracing();
        
        // All parameters should propagate through span
        test_all_with_span(789, "span_test".to_string());
    }

    #[test]
    fn test_all_with_custom_fields() {
        setup_tracing();
        
        // Should log all parameters AND custom fields
        test_all_with_custom("req_001".to_string(), 1);
    }

    #[tokio::test]
    async fn test_all_async_parameters() {
        setup_tracing();
        
        // Should log all async function parameters
        test_all_async(
            "async_session_123".to_string(),
            5000,
            vec![1, 2, 3, 4, 5, 6],
        ).await;
    }

    #[test]
    fn test_all_method_parameters() {
        setup_tracing();
        
        let service = TestAllService {
            service_name: "all-test-service".to_string(),
            config: ServiceConfig {
                max_connections: 100,
                timeout_ms: 5000,
            },
        };
        
        // Should log all method parameters including self fields
        service.all_method(
            "test_operation".to_string(),
            vec![10, 20, 30],
        );
    }

    #[test]
    fn test_all_method_with_span() {
        setup_tracing();
        
        let service = TestAllService {
            service_name: "span-all-service".to_string(),
            config: ServiceConfig {
                max_connections: 50,
                timeout_ms: 3000,
            },
        };
        
        // All method parameters should propagate through span
        service.all_method_with_span("span_req_001".to_string());
    }

    #[tokio::test]
    async fn test_all_async_method() {
        setup_tracing();
        
        let service = TestAllService {
            service_name: "async-all-service".to_string(),
            config: ServiceConfig {
                max_connections: 75,
                timeout_ms: 4000,
            },
        };
        
        // Should log all async method parameters
        service.all_async_method(
            "async_task_001".to_string(),
            "test_payload".to_string(),
        ).await;
    }

    #[test]
    fn test_all_with_references() {
        setup_tracing();
        
        let name = "ref_test";
        let data = vec![1, 2, 3, 4];
        let mut config = HashMap::new();
        config.insert("timeout".to_string(), "5000".to_string());
        
        // Should log all reference parameters
        test_all_with_refs(&name, &data, &config);
    }

    #[test]
    fn test_all_generic_function() {
        setup_tracing();
        
        // Test with different generic types
        test_all_generic("string_item".to_string(), 1, "metadata1".to_string());
        test_all_generic(42i32, 2, "metadata2".to_string());
        test_all_generic(vec![1, 2, 3], 3, "metadata3".to_string());
    }

    #[test]
    fn test_all_with_option_result() {
        setup_tracing();
        
        // Test with Some and Ok
        let result = test_all_with_option(
            "required_value".to_string(),
            Some(123),
            Ok("success".to_string()),
        );
        assert!(result.is_ok());
        
        // Test with None and Err
        let result = test_all_with_option(
            "required_value_2".to_string(),
            None,
            Err("error".to_string()),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_all_zero_parameters() {
        setup_tracing();
        
        // Test function with no parameters
        #[params(all)]
        fn no_params_all() {
            info!("All attribute with no parameters");
        }
        
        no_params_all();
    }

    #[test]
    fn test_all_single_parameter() {
        setup_tracing();
        
        // Test function with single parameter
        #[params(all)]
        fn single_param_all(value: String) {
            info!("All attribute with single parameter");
        }
        
        single_param_all("single_value".to_string());
    }

    #[test]
    fn test_all_many_parameters() {
        setup_tracing();
        
        // Test function with many parameters
        #[params(all)]
        fn many_params_all(
            p1: String, p2: u64, p3: bool, p4: f64, p5: i32,
            p6: Vec<String>, p7: HashMap<String, u64>, p8: Option<String>,
        ) {
            info!("All attribute with many parameters");
        }
        
        let mut map = HashMap::new();
        map.insert("key".to_string(), 42u64);
        
        many_params_all(
            "param1".to_string(),
            123,
            true,
            3.14,
            -456,
            vec!["item1".to_string(), "item2".to_string()],
            map,
            Some("optional".to_string()),
        );
    }

    #[test]
    #[should_panic] // This test demonstrates the security risk
    fn test_security_warning_demo() {
        setup_tracing();
        
        // ⚠️  WARNING: This test demonstrates why 'all' is dangerous
        // In real scenarios, this would expose sensitive data in logs
        test_all_security_warning(
            999,
            "user_password_123".to_string(),
            "sk_live_api_key_secret".to_string(),
            "4111-1111-1111-1111".to_string(),
        );
        
        // This panic is intentional to highlight the security risk
        panic!("This test demonstrates security risks of 'all' attribute!");
    }

    #[test]
    fn test_all_vs_selective_comparison() {
        setup_tracing();
        
        // Compare 'all' vs selective logging
        #[params(all)]
        fn function_with_all(safe: String, sensitive: String) {
            info!("Function with all - logs everything!");
        }
        
        #[params(fields(safe))]
        fn function_with_selective(safe: String, sensitive: String) {
            info!("Function with selective - logs only safe fields");
        }
        
        let safe_data = "public_data".to_string();
        let sensitive_data = "secret_password".to_string();
        
        // All logs everything (including sensitive data)
        function_with_all(safe_data.clone(), sensitive_data.clone());
        
        // Selective logs only specified fields
        function_with_selective(safe_data, sensitive_data);
    }
}
