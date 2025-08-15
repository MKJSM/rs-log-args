//! Tests for span context propagation functionality
//! 
//! Tests the span attribute and context inheritance

use log_args::params;
use tracing::{info, Level};
use tracing_subscriber;

// Test basic span propagation
#[params(span, fields(user_id, operation))]
fn test_span_parent(user_id: u64, operation: String, secret: String) {
    info!("Span parent function");
    
    // Child should inherit context
    test_span_child();
}

#[params(span)]
fn test_span_child() {
    info!("Span child function"); // Should inherit parent context
}

// Test multi-level span inheritance
#[params(span, fields(session_id))]
fn test_multi_level_parent(session_id: String) {
    info!("Multi-level parent");
    test_multi_level_middle(42);
}

#[params(span, fields(middle_value))]
fn test_multi_level_middle(middle_value: u32) {
    info!("Multi-level middle"); // Inherits session_id
    test_multi_level_child();
}

#[params(span)]
fn test_multi_level_child() {
    info!("Multi-level child"); // Inherits both session_id and middle_value
}

// Test span with custom fields propagation
#[params(span, fields(request_id), custom(service = "test-service"))]
fn test_span_with_custom(request_id: String) {
    info!("Span with custom fields");
    test_span_child_inherits_custom();
}

#[params(span, custom(component = "child"))]
fn test_span_child_inherits_custom() {
    info!("Child should inherit parent custom fields");
}

// Test async span propagation
#[params(span, fields(async_id, batch_size))]
async fn test_async_span_parent(async_id: String, batch_size: u32, token: String) {
    info!("Async span parent");
    
    // Async child should inherit context
    test_async_span_child().await;
}

#[params(span)]
async fn test_async_span_child() {
    info!("Async span child"); // Should inherit async_id and batch_size
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
}

// Test span disabled (no propagation)
#[params(fields(isolated_id))] // No span attribute
fn test_no_span_parent(isolated_id: u64, data: String) {
    info!("No span parent");
    test_no_span_child();
}

#[params] // No span
fn test_no_span_child() {
    info!("No span child - should NOT inherit context");
}

// Test mixed span and no-span
#[params(span, fields(parent_id))]
fn test_mixed_span_parent(parent_id: u64) {
    info!("Mixed span parent");
    test_mixed_no_span_child();
    test_mixed_span_child();
}

#[params] // No span - breaks inheritance chain
fn test_mixed_no_span_child() {
    info!("Mixed no-span child - no inheritance");
}

#[params(span)]
fn test_mixed_span_child() {
    info!("Mixed span child - should still inherit from parent");
}

// Test span with methods
struct TestSpanService {
    service_id: String,
}

impl TestSpanService {
    #[params(span, fields(operation, self.service_id))]
    fn span_method_parent(&self, operation: String) {
        info!("Span method parent");
        self.span_method_child();
    }
    
    #[params(span, fields(self.service_id))]
    fn span_method_child(&self) {
        info!("Span method child"); // Should inherit operation
    }
    
    #[params(span, fields(task_id))]
    async fn async_span_method(&self, task_id: String) {
        info!("Async span method");
        self.async_span_child().await;
    }
    
    #[params(span)]
    async fn async_span_child(&self) {
        info!("Async span child method"); // Should inherit task_id
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
}

// Test concurrent span propagation
#[params(span, fields(workflow_id))]
async fn test_concurrent_spans(workflow_id: String, tasks: Vec<String>) {
    info!("Concurrent spans parent");
    
    // Multiple concurrent operations should each inherit the span context
    let handles: Vec<_> = tasks.into_iter().map(|task| {
        tokio::spawn(async move {
            concurrent_span_task(task).await;
        })
    }).collect();
    
    // Wait for all tasks
    for handle in handles {
        let _ = handle.await;
    }
}

#[params(span, fields(task_name))]
async fn concurrent_span_task(task_name: String) {
    info!("Concurrent task"); // Should inherit workflow_id
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
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
    fn test_basic_span_propagation() {
        setup_tracing();
        
        // Child should inherit user_id and operation from parent
        test_span_parent(
            123,
            "test_operation".to_string(),
            "secret_data".to_string(),
        );
    }

    #[test]
    fn test_multi_level_inheritance() {
        setup_tracing();
        
        // Should propagate context through multiple levels
        test_multi_level_parent("session_abc123".to_string());
    }

    #[test]
    fn test_span_with_custom_fields() {
        setup_tracing();
        
        // Custom fields should also propagate through spans
        test_span_with_custom("req_456".to_string());
    }

    #[tokio::test]
    async fn test_async_span_propagation() {
        setup_tracing();
        
        // Async functions should inherit span context
        test_async_span_parent(
            "async_789".to_string(),
            100,
            "bearer_token".to_string(),
        ).await;
    }

    #[test]
    fn test_no_span_isolation() {
        setup_tracing();
        
        // Without span attribute, context should NOT propagate
        test_no_span_parent(999, "isolated_data".to_string());
    }

    #[test]
    fn test_mixed_span_behavior() {
        setup_tracing();
        
        // Mix of span and no-span functions
        test_mixed_span_parent(555);
    }

    #[test]
    fn test_span_method_propagation() {
        setup_tracing();
        
        let service = TestSpanService {
            service_id: "test_service_001".to_string(),
        };
        
        // Method span propagation
        service.span_method_parent("method_test".to_string());
    }

    #[tokio::test]
    async fn test_async_method_span_propagation() {
        setup_tracing();
        
        let service = TestSpanService {
            service_id: "async_service_001".to_string(),
        };
        
        // Async method span propagation
        service.async_span_method("async_task_001".to_string()).await;
    }

    #[tokio::test]
    async fn test_concurrent_span_contexts() {
        setup_tracing();
        
        let tasks = vec![
            "task1".to_string(),
            "task2".to_string(),
            "task3".to_string(),
        ];
        
        // Concurrent tasks should each inherit the parent span context
        test_concurrent_spans("workflow_concurrent".to_string(), tasks).await;
    }

    #[test]
    fn test_span_context_boundaries() {
        setup_tracing();
        
        #[params(span, fields(boundary_id))]
        fn span_boundary_parent(boundary_id: u64) {
            info!("Span boundary parent");
            
            // Regular function call - should inherit
            span_boundary_child_1();
            
            // Function without span - breaks chain
            no_span_boundary();
            
            // Another span function - should inherit from parent, not no_span_boundary
            span_boundary_child_2();
        }
        
        #[params(span)]
        fn span_boundary_child_1() {
            info!("Span boundary child 1");
        }
        
        #[params] // No span
        fn no_span_boundary() {
            info!("No span boundary - breaks inheritance");
        }
        
        #[params(span)]
        fn span_boundary_child_2() {
            info!("Span boundary child 2");
        }
        
        span_boundary_parent(777);
    }

    #[test]
    fn test_span_explicit_vs_implicit() {
        setup_tracing();
        
        // Test that explicit span attribute works
        #[params(span, fields(explicit_id))]
        fn explicit_span_function(explicit_id: u64) {
            info!("Explicit span function");
            implicit_span_child();
        }
        
        // Test default behavior (span should be enabled by default)
        #[params(fields(implicit_id))]
        fn implicit_span_child() {
            info!("Function without explicit span attribute");
        }
        
        explicit_span_function(888);
    }

    #[tokio::test]
    async fn test_mixed_sync_async_span_propagation() {
        setup_tracing();
        
        #[params(span, fields(mixed_id))]
        fn sync_parent(mixed_id: u64) {
            info!("Sync parent with span");
            
            // Call async child from sync parent
            tokio::runtime::Handle::current().block_on(async {
                async_child_from_sync().await;
            });
        }
        
        #[params(span)]
        async fn async_child_from_sync() {
            info!("Async child called from sync parent");
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
        
        sync_parent(111);
    }
}
