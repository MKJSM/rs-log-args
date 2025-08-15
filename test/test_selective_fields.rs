//! Tests for selective field logging functionality
//! 
//! Tests the fields() attribute and security aspects

use log_args::params;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Debug, Clone)]
struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
struct Config {
    pub timeout: u32,
    pub debug_mode: bool,
    pub api_key: String,
}

// Test basic selective field logging
#[params(fields(user_id, operation))]
fn test_selective_basic(user_id: u64, operation: String, secret: String) {
    info!("Selective basic function");
}

// Test nested field access
#[params(fields(user.id, user.username, config.timeout))]
fn test_nested_fields(user: User, config: Config, sensitive_token: String) {
    info!("Nested fields function");
}

// Test multiple selective fields
#[params(fields(id, name, count, enabled))]
fn test_multiple_fields(
    id: u64,
    name: String,
    secret_key: String,  // Not logged
    count: i32,
    token: String,       // Not logged
    enabled: bool,
) {
    info!("Multiple fields function");
}

// Test with complex expressions
#[params(fields(user.username, config.debug_mode, items.len()))]
fn test_complex_expressions(
    user: User,
    config: Config,
    items: Vec<String>,
    private_data: std::collections::HashMap<String, String>,
) {
    info!("Complex expressions function");
}

// Test async with selective fields
#[params(fields(request_id, user.id))]
async fn test_async_selective(
    request_id: String,
    user: User,
    api_secret: String,  // Not logged
) {
    info!("Async selective function");
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
}

// Test method with selective fields
struct TestService {
    service_name: String,
    version: String,
}

impl TestService {
    #[params(fields(operation, self.service_name))]
    fn test_method_selective(&self, operation: String, private_key: String) {
        info!("Method with selective fields");
    }
    
    #[params(fields(user_id, self.version, data.len()))]
    fn test_method_complex(&self, user_id: u64, data: Vec<u8>, auth_token: String) {
        info!("Method with complex selective fields");
    }
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
    fn test_selective_basic_fields() {
        setup_tracing();
        
        // Should only log user_id and operation, not secret
        test_selective_basic(
            123,
            "test_operation".to_string(),
            "super_secret_value".to_string(),
        );
    }

    #[test]
    fn test_nested_field_access() {
        setup_tracing();
        
        let user = User {
            id: 456,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "secret_hash".to_string(),
        };
        
        let config = Config {
            timeout: 5000,
            debug_mode: true,
            api_key: "secret_api_key".to_string(),
        };
        
        // Should log user.id, user.username, config.timeout
        // Should NOT log user.email, user.password_hash, config.api_key, sensitive_token
        test_nested_fields(user, config, "sensitive_token_value".to_string());
    }

    #[test]
    fn test_multiple_selective_fields() {
        setup_tracing();
        
        // Should log id, name, count, enabled
        // Should NOT log secret_key, token
        test_multiple_fields(
            789,
            "test_name".to_string(),
            "secret_key_123".to_string(),
            42,
            "bearer_token_456".to_string(),
            true,
        );
    }

    #[test]
    fn test_complex_field_expressions() {
        setup_tracing();
        
        let user = User {
            id: 999,
            username: "complex_user".to_string(),
            email: "complex@example.com".to_string(),
            password_hash: "complex_hash".to_string(),
        };
        
        let config = Config {
            timeout: 3000,
            debug_mode: false,
            api_key: "complex_api_key".to_string(),
        };
        
        let items = vec!["item1".to_string(), "item2".to_string(), "item3".to_string()];
        
        let mut private_data = std::collections::HashMap::new();
        private_data.insert("secret1".to_string(), "value1".to_string());
        private_data.insert("secret2".to_string(), "value2".to_string());
        
        // Should log user.username, config.debug_mode, items.len()
        // Should NOT log user.id, user.email, config.timeout, private_data
        test_complex_expressions(user, config, items, private_data);
    }

    #[tokio::test]
    async fn test_async_with_selective_fields() {
        setup_tracing();
        
        let user = User {
            id: 555,
            username: "async_user".to_string(),
            email: "async@example.com".to_string(),
            password_hash: "async_hash".to_string(),
        };
        
        // Should log request_id, user.id
        // Should NOT log user.username, user.email, api_secret
        test_async_selective(
            "async_req_001".to_string(),
            user,
            "super_secret_api_key".to_string(),
        ).await;
    }

    #[test]
    fn test_method_selective_fields() {
        setup_tracing();
        
        let service = TestService {
            service_name: "test_service".to_string(),
            version: "1.0.0".to_string(),
        };
        
        // Should log operation, self.service_name
        // Should NOT log private_key
        service.test_method_selective(
            "test_op".to_string(),
            "private_key_secret".to_string(),
        );
    }

    #[test]
    fn test_method_complex_selective() {
        setup_tracing();
        
        let service = TestService {
            service_name: "complex_service".to_string(),
            version: "2.0.0".to_string(),
        };
        
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        // Should log user_id, self.version, data.len()
        // Should NOT log auth_token
        service.test_method_complex(
            777,
            data,
            "auth_token_secret".to_string(),
        );
    }

    #[test]
    fn test_empty_fields_list() {
        setup_tracing();
        
        // Test function with empty fields list - should log nothing
        #[params(fields())]
        fn empty_fields_function(param1: String, param2: u64) {
            info!("Empty fields function");
        }
        
        empty_fields_function("test".to_string(), 123);
    }

    #[test]
    fn test_single_field() {
        setup_tracing();
        
        // Test function with single field
        #[params(fields(important_param))]
        fn single_field_function(important_param: String, ignored_param: String) {
            info!("Single field function");
        }
        
        single_field_function("important".to_string(), "ignored".to_string());
    }

    #[test]
    fn test_field_order_independence() {
        setup_tracing();
        
        // Fields should be logged regardless of order in fields() list
        #[params(fields(param3, param1, param2))]
        fn unordered_fields(param1: String, param2: u64, param3: bool, param4: f64) {
            info!("Unordered fields function");
        }
        
        unordered_fields("first".to_string(), 42, true, 3.14);
    }

    #[test]
    fn test_reference_parameters_selective() {
        setup_tracing();
        
        #[params(fields(name, count))]
        fn ref_params_selective(name: &str, data: &[u8], count: &usize) {
            info!("Reference parameters selective");
        }
        
        let name = "test_name";
        let data = vec![1, 2, 3, 4];
        let count = 100;
        
        // Should log name, count; should NOT log data
        ref_params_selective(&name, &data, &count);
    }
}
