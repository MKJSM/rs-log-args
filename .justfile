build:
    cargo build

# Run all examples
run-examples:
    @echo "Running all log_args examples..."
    just run-basic-usage
    just run-selective-fields
    just run-custom-fields
    just run-span-propagation
    just run-all-parameters
    just run-auto-capture
    just run-method-support
    @echo "All examples completed!"

# Individual example commands
run-basic-usage:
    @echo "=== Running Basic Usage Example ==="
    cargo run --example basic_usage
    @echo ""

run-selective-fields:
    @echo "=== Running Selective Fields Example ==="
    cargo run --example selective_fields
    @echo ""

run-custom-fields:
    @echo "=== Running Custom Fields Example ==="
    cargo run --example custom_fields
    @echo ""

run-span-propagation:
    @echo "=== Running Span Propagation Example ==="
    cargo run --example span_propagation
    @echo ""

run-all-parameters:
    @echo "=== Running All Parameters Example ==="
    @echo "⚠️  WARNING: This example demonstrates logging ALL parameters including sensitive data!"
    cargo run --example all_parameters
    @echo ""

run-auto-capture:
    @echo "=== Running Auto Capture Example ==="
    cargo run --example auto_capture
    @echo ""

run-method-support:
    @echo "=== Running Method Support Example ==="
    cargo run --example method_support
    @echo ""

# Run tests
test:
    @echo "Running all tests..."
    cargo test --workspace

test-basic:
    cargo test test_basic_functionality

test-selective:
    cargo test test_selective_fields

test-custom:
    cargo test test_custom_fields

test-span:
    cargo test test_span_propagation

test-all:
    cargo test test_all_parameters

test-auto:
    cargo test test_auto_capture

test-method:
    cargo test test_method_support

# Helper commands
clean:
    cargo clean

check:
    cargo check --workspace

fmt:
    cargo fmt --all

clippy:
    cargo clippy --workspace --all-targets --all-features
