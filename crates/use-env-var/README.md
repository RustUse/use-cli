# use-env-var

Environment variable primitives for `RustUse` CLI-adjacent code.

This crate validates environment variable names and wraps safe reads from `std::env`. It does not provide a global configuration system, cache, mutation API, or process-wide runtime.

## Example

```rust
use use_env_var::{EnvVarName, is_valid_env_var_name};

let name = EnvVarName::new("RUST_LOG")?;

assert_eq!(name.as_str(), "RUST_LOG");
assert!(is_valid_env_var_name("RUST_LOG"));
# Ok::<(), use_env_var::EnvVarNameError>(())
```

## Scope

Use this crate when a library needs stable env var vocabulary without owning configuration policy.
