//! Tests for custom static fields functionality
//! 
//! Tests the custom() attribute for adding static metadata

use log_args::params;
use tracing::{info, Level};
use tracing_subscriber;

// Test basic custom fields
#[params(custom(service = "test-service", version = "1.0.0"))]
fn test_basic_custom(operation: String) {
    info!("Basic custom fields function");
}

// Test custom fields with parameter fields
#[params(
    fields(user_id, operation_type),
    custom(service = "user-service", environment = "test")
)]
fn test_mixed_fields(user_id: u64, operation_type: String, secret: String) {
    info!("Mixed fields function");
}

// Test multiple custom fields
#[params(custom(
    service = "payment-service",
    version = "2.1.0",
    environment = "production",
    region = "us-east-1",
    team = "backend"
))]
fn test_multiple_custom(transaction_id: String, amount: f64) {
    info!("Multiple custom fields function");
}

// Test custom fields with span propagation
#[params(
    span,
    fields(request_id),
    custom(component = "orchestrator", subsystem = "workflow")
)]
fn test_custom_with_span(request_id: String, workflow_data: Vec<String>) {
    info!("Custom fields with span");
    
    // Child should inherit custom fields
    child_with_custom();
}

#[params(custom(step = "validation"))]
fn child_with_custom() {
    info!("Child with custom fields"); // Should inherit parent custom fields
}

// Test async with custom fields
#[params(
    fields(session_id, task_count),
    custom(service = "async-processor", queue_type = "priority")
)]
async fn test_async_custom(session_id: String, task_count: u32, auth_token: String) {
    info!("Async custom fields function");
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
}

// Test method with custom fields
struct TestProcessor {
    processor_id: String,
}

impl TestProcessor {
    #[params(
        fields(batch_id, self.processor_id),
        custom(component = "batch-processor", version = "3.0")
    )]
    fn process_batch(&self, batch_id: String, data: Vec<u8>) {
        info!("Processing batch with custom fields");
    }
    
    #[params(custom(operation = "cleanup", priority = "high"))]
    async fn cleanup_async(&self, cleanup_data: String) {
        info!("Async cleanup with custom fields");
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
}

// Test custom fields with complex values
#[params(custom(
    config_json = "{\"timeout\": 5000, \"retries\": 3}",
    feature_flags = "flag1,flag2,flag3",
    deployment_timestamp = "2023-10-15T10:30:00Z"
))]
fn test_complex_custom_values(operation_id: String) {
    info!("Complex custom values function");
}

// Test custom fields with special characters
#[params(custom(
    "service-name" = "special-chars-service",
    "api.version" = "v2.1",
    "trace_id" = "trace-123-abc-def"
))]
fn test_custom_special_chars(request_data: String) {
    info!("Custom fields with special characters");
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
    fn test_basic_custom_fields() {
        setup_tracing();
        
        // Should include service and version in logs
        test_basic_custom("test_operation".to_string());
    }

    #[test]
    fn test_mixed_parameter_and_custom_fields() {
        setup_tracing();
        
        // Should include user_id, operation_type from parameters
        // AND service, environment from custom fields
        // Should NOT include secret
        test_mixed_fields(
            123,
            "profile_update".to_string(),
            "secret_value".to_string(),
        );
    }

    #[test]
    fn test_multiple_custom_fields() {
        setup_tracing();
        
        // Should include all custom metadata
        test_multiple_custom("txn_456".to_string(), 99.99);
    }

    #[test]
    fn test_custom_fields_with_span_propagation() {
        setup_tracing();
        
        // Parent custom fields should propagate to child
        test_custom_with_span(
            "req_789".to_string(),
            vec!["step1".to_string(), "step2".to_string()],
        );
    }

    #[tokio::test]
    async fn test_async_with_custom_fields() {
        setup_tracing();
        
        // Async function should include custom fields
        test_async_custom(
            "sess_001".to_string(),
            5,
            "bearer_token_secret".to_string(),
        ).await;
    }

    #[test]
    fn test_method_with_custom_fields() {
        setup_tracing();
        
        let processor = TestProcessor {
            processor_id: "proc_123".to_string(),
        };
        
        // Method should include both parameter fields and custom fields
        processor.process_batch(
            "batch_001".to_string(),
            vec![1, 2, 3, 4, 5],
        );
    }

    #[tokio::test]
    async fn test_async_method_with_custom_fields() {
        setup_tracing();
        
        let processor = TestProcessor {
            processor_id: "async_proc_456".to_string(),
        };
        
        // Async method should include custom fields
        processor.cleanup_async("cleanup_data_xyz".to_string()).await;
    }

    #[test]
    fn test_complex_custom_values() {
        setup_tracing();
        
        // Should handle complex string values in custom fields
        test_complex_custom_values("op_complex_001".to_string());
    }

    #[test]
    fn test_custom_fields_with_special_characters() {
        setup_tracing();
        
        // Should handle custom field names and values with special characters
        test_custom_special_chars("special_request_data".to_string());
    }

    #[test]
    fn test_empty_custom_fields() {
        setup_tracing();
        
        // Test function with empty custom fields
        #[params(custom())]
        fn empty_custom_function(param: String) {
            info!("Empty custom function");
        }
        
        empty_custom_function("test".to_string());
    }

    #[test]
    fn test_single_custom_field() {
        setup_tracing();
        
        // Test function with single custom field
        #[params(custom(service = "single-field-service"))]
        fn single_custom_function(data: u64) {
            info!("Single custom field function");
        }
        
        single_custom_function(42);
    }

    #[test]
    fn test_custom_fields_inheritance() {
        setup_tracing();
        
        #[params(span, custom(parent_service = "inheritance-test"))]
        fn parent_with_inheritance(parent_id: u64) {
            info!("Parent with custom fields");
            child_inherits_custom(parent_id + 1);
        }
        
        #[params(custom(child_component = "inherited"))]
        fn child_inherits_custom(child_id: u64) {
            info!("Child should inherit parent custom fields");
        }
        
        parent_with_inheritance(100);
    }

    #[test]
    fn test_custom_fields_override() {
        setup_tracing();
        
        #[params(span, custom(service = "parent-service", version = "1.0"))]
        fn parent_override_test(id: u64) {
            info!("Parent with custom fields");
            child_override_test();
        }
        
        #[params(custom(service = "child-service", component = "child"))]
        fn child_override_test() {
            info!("Child with potentially overriding custom fields");
        }
        
        parent_override_test(200);
    }

    #[test]
    fn test_numeric_and_boolean_custom_values() {
        setup_tracing();
        
        // Test with numeric and boolean-like string values
        #[params(custom(
            port = "8080",
            timeout_seconds = "30",
            debug_enabled = "true",
            max_connections = "100"
        ))]
        fn numeric_custom_function(operation: String) {
            info!("Numeric custom values function");
        }
        
        numeric_custom_function("numeric_test".to_string());
    }
}
