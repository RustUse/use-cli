# use-exit-code

Portable exit status primitives for `RustUse` CLI-adjacent code.

`use-exit-code` keeps common process exit meanings explicit while staying small and platform-aware.

## Example

```rust
use use_exit_code::{CONFIG_ERROR, ExitCode, SUCCESS};

let success = ExitCode::from_u8(0);

assert_eq!(success, SUCCESS);
assert_eq!(CONFIG_ERROR.as_i32(), 78);
```

## Scope

This crate does not spawn processes, inspect child status values, or manage application shutdown.
