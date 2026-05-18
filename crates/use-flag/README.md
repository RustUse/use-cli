# use-flag

Primitive short, long, and boolean flag types for CLI-adjacent Rust code.

`use-flag` validates flag names and stores flag intent. It does not scan an argv stream, expand combined short flags, define negation policy, or implement a parser.

## Example

```rust
use use_flag::{BooleanFlag, Flag};

let flag = Flag::try_from_token("--verbose")?;
let verbose = BooleanFlag::enabled(flag);

assert!(verbose.is_enabled());
assert_eq!(verbose.flag().to_token(), "--verbose");
# Ok::<(), use_flag::FlagNameError>(())
```

## Scope

Use this crate for small flag vocabulary. Use a parser framework when you need parsing, validation across multiple tokens, or derived command definitions.
