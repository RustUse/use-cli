# use-stderr

Synchronous stderr primitives for `RustUse` CLI-adjacent code.

This crate provides error output destination markers and basic write helpers. It does not provide a logging framework.

## Example

```rust
use use_stderr::write_error_line;

let mut buffer = Vec::new();
write_error_line(&mut buffer, "warning")?;

assert_eq!(buffer, b"warning\n");
# Ok::<(), std::io::Error>(())
```

## Scope

Use this crate for explicit stderr writes, not logging or diagnostics policy.
