# use-stdout

Synchronous stdout primitives for `RustUse` CLI-adjacent code.

This crate provides output destination markers, newline behavior helpers, and small write helpers. It does not provide a formatting framework.

## Example

```rust
use use_stdout::{NewlineBehavior, apply_newline_behavior, write_text};

let output = apply_newline_behavior("ready", NewlineBehavior::EnsureTrailingNewline);
let mut buffer = Vec::new();
write_text(&mut buffer, &output)?;

assert_eq!(buffer, b"ready\n");
# Ok::<(), std::io::Error>(())
```

## Scope

Use this crate for small output primitives, not application rendering policy.
