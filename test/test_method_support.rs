//! Tests for method support in impl blocks
//! 
//! Tests that #[params] works with methods and self parameters

use log_args::params;
use tracing::{info, Level};
use tracing_subscriber;
use std::collections::HashMap;

// Test struct for method testing
#[derive(Debug, Clone)]
struct TestMethodService {
    service_name: String,
    version: String,
    config: MethodServiceConfig,
}

#[derive(Debug, Clone)]
struct MethodServiceConfig {
    timeout: u32,
    max_connections: u32,
    debug_mode: bool,
}

impl TestMethodService {
    fn new(service_name: String, version: String) -> Self {
        Self {
            service_name,
            version,
            config: MethodServiceConfig {
                timeout: 5000,
                max_connections: 100,
                debug_mode: false,
            },
        }
    }
    
    // Basic method with selective fields
    #[params(fields(user_id, operation_type, self.service_name))]
    fn process_user(&self, user_id: u64, operation_type: String, sensitive_token: String) {
        info!("Processing user in service");
    }
    
    // Method with span propagation
    #[params(span, fields(user_id, self.config.debug_mode))]
    fn validate_user(&self, user_id: u64) {
        info!("Validating user");
        
        // Call other methods
        self.check_permissions(user_id);
    }
    
    // Method with custom fields
    #[params(fields(user_id), custom(component = "permissions", level = "security"))]
    fn check_permissions(&self, user_id: u64) {
        info!("Checking user permissions");
    }
    
    // Method with all parameters
    #[params(all, span)]
    fn admin_operation(&self, admin_id: u64, operation: String, permissions: Vec<String>) {
        info!("Administrative operation");
        self.log_admin_action(admin_id, &operation);
    }
    
    // Method logging multiple self fields
    #[params(fields(admin_id, action, self.service_name, self.version, self.config.timeout))]
    fn log_admin_action(&self, admin_id: u64, action: &str) {
        info!("Logging administrative action");
    }
    
    // Async method with fields
    #[params(span, fields(batch_id, self.config.max_connections))]
    async fn process_batch_async(&self, batch_id: String, items: Vec<String>) {
        info!("Starting async batch processing");
        
        for item in items {
            self.process_item_async(item).await;
        }
    }
    
    #[params(fields(item_id, self.config.timeout))]
    async fn process_item_async(&self, item_id: String) {
        info!("Processing item asynchronously");
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
    
    // Mutable method
    #[params(fields(new_timeout, self.service_name))]
    fn update_config(&mut self, new_timeout: u32, new_debug_mode: bool) {
        info!("Updating service configuration");
        
        self.config.timeout = new_timeout;
        self.config.debug_mode = new_debug_mode;
    }
    
    // Method with auto capture
    #[params(auto_capture, fields(operation_name, self.version))]
    fn process_with_closures(&self, operation_name: String, items: Vec<String>) {
        info!("Processing with closures");
        
        items.iter().for_each(|item| {
            info!("Processing item: {}", item);
        });
    }
    
    // Static method (associated function)
    #[params(fields(service_type, region), custom(factory = "service_factory"))]
    fn create_service(service_type: String, region: String, config: MethodServiceConfig) -> Self {
        info!("Creating new service instance");
        
        Self {
            service_name: service_type,
            version: "1.0.0".to_string(),
            config,
        }
    }
}

// Generic struct for testing generic methods
#[derive(Debug)]
struct GenericRepository<T> {
    storage: Vec<T>,
    capacity: usize,
    name: String,
}

impl<T: std::fmt::Debug> GenericRepository<T> {
    #[params(fields(item_count, self.capacity, self.name))]
    fn add_items(&mut self, items: Vec<T>) {
        info!("Adding items to repository");
        
        for item in items {
            if self.storage.len() < self.capacity {
                self.storage.push(item);
            }
        }
    }
    
    #[params(span, fields(self.name))]
    fn get_stats(&self) -> (usize, usize) {
        info!("Getting repository statistics");
        (self.storage.len(), self.capacity)
    }
    
    #[params(all)]
    fn debug_repository(&self, debug_level: u32) {
        info!("Debug repository information");
    }
}

// Trait implementation testing
trait ProcessorTrait {
    fn process_data(&self, data: String) -> String;
}

struct TraitTestService {
    processor_id: String,
}

impl ProcessorTrait for TraitTestService {
    #[params(fields(data.len(), self.processor_id))]
    fn process_data(&self, data: String) -> String {
        info!("Processing data via trait implementation");
        format!("processed_{}", data)
    }
}

impl TraitTestService {
    #[params(fields(input_size, self.processor_id))]
    fn additional_processing(&self, input_size: usize, extra_data: Vec<u8>) {
        info!("Additional processing method");
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
    fn test_basic_method_functionality() {
        setup_tracing();
        
        let service = TestMethodService::new(
            "test-service".to_string(),
            "1.0.0".to_string(),
        );
        
        // Should log user_id, operation_type, self.service_name
        // Should NOT log sensitive_token
        service.process_user(
            123,
            "profile_update".to_string(),
            "secret_token_123".to_string(),
        );
    }

    #[test]
    fn test_method_span_propagation() {
        setup_tracing();
        
        let service = TestMethodService::new(
            "span-service".to_string(),
            "1.1.0".to_string(),
        );
        
        // Should propagate context to child methods
        service.validate_user(456);
    }

    #[test]
    fn test_method_all_parameters() {
        setup_tracing();
        
        let service = TestMethodService::new(
            "admin-service".to_string(),
            "2.0.0".to_string(),
        );
        
        // Should log all method parameters including self fields
        service.admin_operation(
            999,
            "delete_user".to_string(),
            vec!["admin".to_string(), "delete".to_string()],
        );
    }

    #[tokio::test]
    async fn test_async_method_functionality() {
        setup_tracing();
        
        let service = TestMethodService::new(
            "async-service".to_string(),
            "3.0.0".to_string(),
        );
        
        // Should work with async methods and span propagation
        service.process_batch_async(
            "batch_001".to_string(),
            vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
        ).await;
    }

    #[test]
    fn test_mutable_method() {
        setup_tracing();
        
        let mut service = TestMethodService::new(
            "mutable-service".to_string(),
            "1.5.0".to_string(),
        );
        
        // Should work with mutable self
        service.update_config(10000, true);
        
        // Verify config was updated
        assert_eq!(service.config.timeout, 10000);
        assert_eq!(service.config.debug_mode, true);
    }

    #[test]
    fn test_method_auto_capture() {
        setup_tracing();
        
        let service = TestMethodService::new(
            "auto-capture-service".to_string(),
            "4.0.0".to_string(),
        );
        
        // Should preserve context in closures
        service.process_with_closures(
            "closure_test".to_string(),
            vec!["item1".to_string(), "item2".to_string()],
        );
    }

    #[test]
    fn test_static_method() {
        setup_tracing();
        
        let config = MethodServiceConfig {
            timeout: 3000,
            max_connections: 50,
            debug_mode: true,
        };
        
        // Should work with static methods (associated functions)
        let _service = TestMethodService::create_service(
            "static-service".to_string(),
            "us-west-2".to_string(),
            config,
        );
    }

    #[test]
    fn test_generic_method_functionality() {
        setup_tracing();
        
        let mut repo: GenericRepository<String> = GenericRepository {
            storage: Vec::new(),
            capacity: 10,
            name: "string-repo".to_string(),
        };
        
        // Should work with generic methods
        repo.add_items(vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
        ]);
        
        let (count, capacity) = repo.get_stats();
        assert_eq!(count, 3);
        assert_eq!(capacity, 10);
        
        repo.debug_repository(5);
    }

    #[test]
    fn test_different_generic_types() {
        setup_tracing();
        
        // Test with different generic types
        let mut int_repo: GenericRepository<i32> = GenericRepository {
            storage: Vec::new(),
            capacity: 5,
            name: "int-repo".to_string(),
        };
        
        int_repo.add_items(vec![1, 2, 3, 4]);
        int_repo.debug_repository(3);
        
        // Test with complex types
        let mut vec_repo: GenericRepository<Vec<String>> = GenericRepository {
            storage: Vec::new(),
            capacity: 3,
            name: "vec-repo".to_string(),
        };
        
        vec_repo.add_items(vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ]);
        vec_repo.debug_repository(1);
    }

    #[test]
    fn test_trait_implementation_methods() {
        setup_tracing();
        
        let processor = TraitTestService {
            processor_id: "trait_proc_001".to_string(),
        };
        
        // Should work with trait implementation methods
        let result = processor.process_data("test_data".to_string());
        assert_eq!(result, "processed_test_data");
        
        // Should work with regular impl block methods
        processor.additional_processing(100, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_method_with_references() {
        setup_tracing();
        
        // Test methods with reference parameters
        impl TestMethodService {
            #[params(fields(name, self.service_name))]
            fn process_ref_data(&self, name: &str, data: &[u8], metadata: &HashMap<String, String>) {
                info!("Processing reference data");
            }
        }
        
        let service = TestMethodService::new(
            "ref-service".to_string(),
            "1.0.0".to_string(),
        );
        
        let name = "test_name";
        let data = vec![1, 2, 3, 4, 5];
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());
        
        service.process_ref_data(&name, &data, &metadata);
    }

    #[test]
    fn test_method_chaining_context() {
        setup_tracing();
        
        // Test that context propagates through method chains
        impl TestMethodService {
            #[params(span, fields(chain_id, self.service_name))]
            fn chain_step1(&self, chain_id: u64) {
                info!("Chain step 1");
                self.chain_step2();
            }
            
            #[params(span)]
            fn chain_step2(&self) {
                info!("Chain step 2");
                self.chain_step3();
            }
            
            #[params(span)]
            fn chain_step3(&self) {
                info!("Chain step 3");
            }
        }
        
        let service = TestMethodService::new(
            "chain-service".to_string(),
            "1.0.0".to_string(),
        );
        
        service.chain_step1(777);
    }

    #[test]
    fn test_method_error_handling() {
        setup_tracing();
        
        // Test methods that return Results
        impl TestMethodService {
            #[params(fields(operation_id, self.service_name))]
            fn fallible_method(&self, operation_id: u64, should_fail: bool) -> Result<String, String> {
                info!("Fallible method execution");
                
                if should_fail {
                    Err("Operation failed".to_string())
                } else {
                    Ok("Operation succeeded".to_string())
                }
            }
        }
        
        let service = TestMethodService::new(
            "error-service".to_string(),
            "1.0.0".to_string(),
        );
        
        // Test success case
        let result = service.fallible_method(111, false);
        assert!(result.is_ok());
        
        // Test error case
        let result = service.fallible_method(222, true);
        assert!(result.is_err());
    }
}
