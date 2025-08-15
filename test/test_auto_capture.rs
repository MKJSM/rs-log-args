//! Tests for auto_capture functionality
//! 
//! Tests automatic context capture for closures and boundaries

use log_args::params;
use tracing::{info, Level};
use tracing_subscriber;
use std::collections::HashMap;

// Test basic auto capture
#[params(auto_capture, fields(user_id, operation))]
fn test_auto_capture_basic(user_id: u64, operation: String, secret: String) {
    info!("Auto capture basic function");
    
    // Context should be preserved in closures
    let closure = || {
        info!("Inside closure with preserved context");
    };
    closure();
}

// Test auto capture with complex closures
#[params(auto_capture, fields(batch_id, count))]
fn test_auto_capture_complex(batch_id: String, count: u32, items: Vec<String>) {
    info!("Auto capture complex function");
    
    // Multiple closure types should preserve context
    items.iter().for_each(|item| {
        info!("Processing item: {}", item);
    });
    
    let results: Vec<String> = items.iter().map(|item| {
        info!("Mapping item: {}", item);
        format!("processed_{}", item)
    }).collect();
    
    info!("Processed {} items", results.len());
}

// Test auto capture with nested closures
#[params(auto_capture, fields(workflow_id))]
fn test_auto_capture_nested(workflow_id: String, data: HashMap<String, Vec<String>>) {
    info!("Auto capture nested closures");
    
    data.iter().for_each(|(phase, steps)| {
        info!("Processing phase: {}", phase);
        
        steps.iter().for_each(|step| {
            info!("Processing step: {}", step);
            
            // Even deeper nesting
            let sub_tasks = vec!["validate", "execute", "verify"];
            sub_tasks.iter().for_each(|task| {
                info!("Sub-task: {} for step: {}", task, step);
            });
        });
    });
}

// Test auto capture with async closures
#[params(auto_capture, fields(session_id, task_count))]
async fn test_auto_capture_async(session_id: String, task_count: u32, tasks: Vec<String>) {
    info!("Auto capture async function");
    
    // Async closures should preserve context
    let futures: Vec<_> = tasks.into_iter().map(|task| {
        tokio::spawn(async move {
            info!("Async processing task: {}", task);
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            format!("completed_{}", task)
        })
    }).collect();
    
    // Wait for all tasks
    for future in futures {
        let _ = future.await;
    }
}

// Test auto capture with error handling
#[params(auto_capture, fields(operation_id, retry_count))]
fn test_auto_capture_error_handling(operation_id: String, retry_count: u32) -> Result<String, String> {
    info!("Auto capture error handling");
    
    let result = (0..retry_count).find_map(|attempt| {
        info!("Attempt {} for operation", attempt + 1);
        
        if attempt < 1 {
            info!("Attempt failed, retrying...");
            None
        } else {
            info!("Operation succeeded on attempt {}", attempt + 1);
            Some("success".to_string())
        }
    });
    
    match result {
        Some(success) => {
            info!("Operation completed successfully");
            Ok(success)
        }
        None => {
            info!("Operation failed after all retries");
            Err("All attempts failed".to_string())
        }
    }
}

// Test auto capture with methods
struct TestAutoService {
    service_name: String,
    config: AutoServiceConfig,
}

#[derive(Debug, Clone)]
struct AutoServiceConfig {
    timeout: u32,
    max_retries: u8,
}

impl TestAutoService {
    #[params(auto_capture, fields(operation, self.service_name))]
    fn auto_method(&self, operation: String, items: Vec<String>) {
        info!("Auto capture method");
        
        // Method closures should preserve context
        items.iter().for_each(|item| {
            info!("Method processing item: {}", item);
        });
    }
    
    #[params(auto_capture, fields(batch_id, self.config.timeout))]
    async fn auto_async_method(&self, batch_id: String, data: Vec<u8>) {
        info!("Auto capture async method");
        
        // Async method closures should preserve context
        let chunks: Vec<_> = data.chunks(2).enumerate().map(|(i, chunk)| {
            tokio::spawn(async move {
                info!("Processing chunk {}: {:?}", i, chunk);
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            })
        }).collect();
        
        for chunk_future in chunks {
            let _ = chunk_future.await;
        }
    }
}

// Test auto capture with custom fields
#[params(auto_capture, fields(request_id), custom(service = "auto-service"))]
fn test_auto_capture_with_custom(request_id: String, processors: Vec<String>) {
    info!("Auto capture with custom fields");
    
    processors.into_iter().for_each(|processor| {
        info!("Using processor: {}", processor);
        
        // Nested operations should preserve both parameter and custom fields
        let operations = vec!["init", "process", "cleanup"];
        operations.into_iter().for_each(|op| {
            info!("Processor {} executing operation: {}", processor, op);
        });
    });
}

// Test auto capture with span propagation
#[params(auto_capture, span, fields(parent_id))]
fn test_auto_capture_with_span(parent_id: u64, child_operations: Vec<String>) {
    info!("Auto capture with span propagation");
    
    child_operations.into_iter().for_each(|op| {
        info!("Executing child operation: {}", op);
    });
    
    // Regular function call should also inherit context
    auto_capture_span_child();
}

#[params(auto_capture, span)]
fn auto_capture_span_child() {
    info!("Auto capture span child function");
}

// Test without auto capture for comparison
#[params(fields(comparison_id))] // No auto_capture
fn test_without_auto_capture(comparison_id: u64, items: Vec<String>) {
    info!("Without auto capture function");
    
    // Context may not be preserved in closures
    items.iter().for_each(|item| {
        info!("Processing item without auto capture: {}", item);
    });
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
    fn test_basic_auto_capture() {
        setup_tracing();
        
        // Context should be preserved in closure
        test_auto_capture_basic(
            123,
            "auto_test".to_string(),
            "secret_data".to_string(),
        );
    }

    #[test]
    fn test_complex_auto_capture() {
        setup_tracing();
        
        let items = vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
        ];
        
        // Complex closures should preserve context
        test_auto_capture_complex("batch_001".to_string(), 3, items);
    }

    #[test]
    fn test_nested_auto_capture() {
        setup_tracing();
        
        let mut workflow_data = HashMap::new();
        workflow_data.insert("preparation".to_string(), vec!["setup".to_string(), "validate".to_string()]);
        workflow_data.insert("execution".to_string(), vec!["process".to_string(), "transform".to_string()]);
        
        // Nested closures should preserve context
        test_auto_capture_nested("workflow_nested".to_string(), workflow_data);
    }

    #[tokio::test]
    async fn test_async_auto_capture() {
        setup_tracing();
        
        let tasks = vec![
            "async_task1".to_string(),
            "async_task2".to_string(),
            "async_task3".to_string(),
        ];
        
        // Async closures should preserve context
        test_auto_capture_async("async_session_001".to_string(), 3, tasks).await;
    }

    #[test]
    fn test_error_handling_auto_capture() {
        setup_tracing();
        
        // Error handling closures should preserve context
        let result = test_auto_capture_error_handling("op_retry_001".to_string(), 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_method_auto_capture() {
        setup_tracing();
        
        let service = TestAutoService {
            service_name: "auto-test-service".to_string(),
            config: AutoServiceConfig {
                timeout: 5000,
                max_retries: 3,
            },
        };
        
        let items = vec!["method_item1".to_string(), "method_item2".to_string()];
        
        // Method closures should preserve context
        service.auto_method("method_operation".to_string(), items);
    }

    #[tokio::test]
    async fn test_async_method_auto_capture() {
        setup_tracing();
        
        let service = TestAutoService {
            service_name: "async-auto-service".to_string(),
            config: AutoServiceConfig {
                timeout: 3000,
                max_retries: 2,
            },
        };
        
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        // Async method closures should preserve context
        service.auto_async_method("async_batch_001".to_string(), data).await;
    }

    #[test]
    fn test_auto_capture_with_custom_fields() {
        setup_tracing();
        
        let processors = vec![
            "processor1".to_string(),
            "processor2".to_string(),
        ];
        
        // Should preserve both parameter and custom fields in closures
        test_auto_capture_with_custom("custom_req_001".to_string(), processors);
    }

    #[test]
    fn test_auto_capture_span_propagation() {
        setup_tracing();
        
        let child_ops = vec![
            "child_op1".to_string(),
            "child_op2".to_string(),
        ];
        
        // Should combine auto capture with span propagation
        test_auto_capture_with_span(456, child_ops);
    }

    #[test]
    fn test_comparison_with_without_auto_capture() {
        setup_tracing();
        
        let items = vec!["comp_item1".to_string(), "comp_item2".to_string()];
        
        // Compare behavior with and without auto capture
        test_without_auto_capture(789, items);
    }

    #[test]
    fn test_auto_capture_performance_closures() {
        setup_tracing();
        
        // Test that auto capture works with performance-oriented closures
        #[params(auto_capture, fields(batch_size))]
        fn performance_test(batch_size: usize, data: Vec<u64>) {
            info!("Performance test with auto capture");
            
            // Filter operation
            let filtered: Vec<_> = data.into_iter().filter(|&x| {
                info!("Filtering value: {}", x);
                x > 10
            }).collect();
            
            // Reduce operation
            let sum = filtered.iter().fold(0, |acc, &x| {
                info!("Accumulating value: {}", x);
                acc + x
            });
            
            info!("Final sum: {}", sum);
        }
        
        let test_data = (1..=20).collect();
        performance_test(20, test_data);
    }

    #[test]
    fn test_auto_capture_with_move_closures() {
        setup_tracing();
        
        // Test auto capture with move closures
        #[params(auto_capture, fields(owner_id))]
        fn test_move_closures(owner_id: u64, items: Vec<String>) {
            info!("Testing move closures with auto capture");
            
            let owned_data = "owned_data".to_string();
            
            // Move closure should still preserve context
            let closure = move || {
                info!("Move closure with owned data: {}", owned_data);
            };
            closure();
            
            // Iterator with move
            items.into_iter().for_each(|item| {
                info!("Processing moved item: {}", item);
            });
        }
        
        let items = vec!["move1".to_string(), "move2".to_string()];
        test_move_closures(999, items);
    }
}
