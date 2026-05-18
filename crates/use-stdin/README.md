# use-stdin

Synchronous stdin primitives for `RustUse` CLI-adjacent code.

This crate provides small read helpers and marker types. It does not assume an async runtime or interactive UI framework.

## Example

```rust
use std::io::Cursor;
use use_stdin::read_to_string_from;

let input = read_to_string_from(Cursor::new("hello"))?;

assert_eq!(input, "hello");
# Ok::<(), std::io::Error>(())
```

## Scope

Use this crate for testable, synchronous stdin-adjacent helpers.
