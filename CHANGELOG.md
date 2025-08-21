# Changelog

All notable changes to the log-args workspace will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation improvements across all crates
- Enhanced code comments and inline documentation
- Function name logging with multiple casing styles:
  - `function-names-snake` - snake_case (original)
  - `function-names-camel` - camelCase  
  - `function-names-pascal` - PascalCase (recommended)
  - `function-names-screaming` - SCREAMING_SNAKE_CASE
  - `function-names-kebab` - kebab-case
  - `function-names` - alias for PascalCase
- Unified context handling in runtime crate
- Enhanced security warnings and best practices guidance

### Changed
- **BREAKING**: Simplified runtime crate context handling to use single unified thread-local stack
- **BREAKING**: Deprecated `get_inherited_context_string()` in favor of `get_inherited_fields_map()`
- **BREAKING**: Merged `AsyncContextGuard` functionality into `ContextGuard`
- Refactored `get_context_fields_quote` into smaller, well-documented functions
- Improved `clone_upfront` documentation to clarify sync/async usage
- Enhanced attribute documentation with comprehensive examples
- Improved workspace structure and removed unnecessary package sections
- Enhanced error handling and code robustness
- Streamlined README documentation across workspace
- Simplified context retrieval and propagation logic
- Enhanced performance through reduced mutex contention

### Fixed
- Syntax errors and compilation issues
- Function name logging feature restoration
- Unused import warnings in runtime crate
- Context propagation consistency issues
- Context synchronization issues across async boundaries
- Inconsistent context inheritance behavior
- Removed redundant `WITH_CONTEXT_ENABLED` flag usage
- Fixed inconsistent clone_upfront logic

### Removed
- Redundant global context mutex (replaced with unified thread-local storage)
- Unused `auto_capture_context` function
- Complex multi-source context lookup logic
- Unnecessary workspace configuration sections
- Dead code and unused helper functions
- Redundant context checking logic
- Separate async context stack (merged into unified stack)

### Deprecated
- `get_inherited_context_string()` - Use `get_inherited_fields_map()` instead

---

## Migration Guide

### Upgrading to 0.1.6+ (log_args)

#### Function Name Logging
Function name logging is now available with multiple casing styles:

```toml
# Choose your preferred casing style
log_args = { version = "0.1.6", features = ["function-names-pascal"] }
```

#### Enhanced Documentation
All macro attributes now have comprehensive documentation. Use `cargo doc --open` to view detailed usage examples.

### Upgrading to 0.1.4+ (log-args-runtime)

#### Context Handling Changes
The runtime crate now uses a unified context handling system:

**Before:**
```rust
// Multiple context sources were checked internally
let value = get_context_value("key"); // Complex multi-source lookup
```

**After:**
```rust
// Single unified context stack (no API changes, just better performance)
let value = get_context_value("key"); // Simple stack-based lookup
```

#### Deprecated Functions
Replace usage of deprecated functions:

**Before:**
```rust
let context_str = get_inherited_context_string(); // Deprecated
```

**After:**
```rust
let context_map = get_inherited_fields_map(); // Recommended
let context_str = context_map
    .iter()
    .map(|(k, v)| format!("{k}={v}"))
    .collect::<Vec<_>>()
    .join(",");
```

## Contributing

When adding entries to this changelog:

1. Add unreleased changes to the `[Unreleased]` section
2. Follow the format: `### [Added/Changed/Fixed/Removed]`
3. Use present tense for descriptions
4. Include breaking change warnings with `**BREAKING**:`
5. Reference issue numbers when applicable
6. Move items from `[Unreleased]` to version sections on release

## Version Support

- **Current**: 0.1.6 (log_args), 0.1.4 (log-args-runtime)
- **Minimum Supported Rust Version**: 1.70.0
