# use-arg

Primitive owned argument tokens for CLI-adjacent Rust code.

`use-arg` works at the token level. It can identify positional-looking tokens, the `--` option separator, and flag-like tokens, but it does not parse a command line or define parser state.

## Example

```rust
use use_arg::{Arg, RawArgs, is_positional};

let input = Arg::from("README.md");
let args = RawArgs::from_iter([Arg::from("tool"), input.clone()]);

assert!(input.is_positional());
assert!(is_positional(input.as_str()));
assert_eq!(args.len(), 2);
```

## Scope

This crate intentionally avoids full parsing, subcommands, option schemas, and grouped flag expansion.
