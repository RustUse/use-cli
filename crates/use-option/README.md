# use-option

Primitive CLI option key/value types for `RustUse`.

`CliOption` is a command-line option primitive, not Rust's `Option<T>`. It stores a validated option name and an owned value for forms such as `--key=value` and `--key value`.

## Example

```rust
use use_option::{CliOption, split_equals_token};

let from_equals = split_equals_token("--color=auto")?;
let from_pair = CliOption::from_name_value("format", "json")?;

assert_eq!(from_equals.name().as_str(), "color");
assert_eq!(from_pair.to_equals_token(), "--format=json");
# Ok::<(), use_option::CliOptionError>(())
```

## Scope

This crate does not implement argument parsing, option registries, defaults, type conversion, or parser DSLs.
