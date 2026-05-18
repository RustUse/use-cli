# use-cli

Facade crate for `RustUse` CLI-adjacent primitives.

This crate is not a CLI framework. It does not parse full command lines, route commands, define a DSL, run an async runtime, generate shell completions, log messages, or manage configuration. It re-exports focused primitive crates behind stable module names.

## Example

```rust
use use_cli::{command_name, exit_code, flag, option};

let command = command_name::CommandName::new("tool")?;
let flag = flag::Flag::try_from_token("--verbose")?;
let output = option::CliOption::from_name_value("format", "json")?;

assert_eq!(command.as_str(), "tool");
assert_eq!(flag.to_token(), "--verbose");
assert_eq!(output.to_equals_token(), "--format=json");
assert_eq!(exit_code::SUCCESS.as_u8(), 0);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Modules

The facade exposes `arg`, `flag`, `option`, `env_var`, `exit_code`, `stdin`, `stdout`, `stderr`, `prompt`, `command_name`, and `terminal`.

## Scope

Use this crate when one dependency and one import surface are useful. Use the focused child crates directly when a library only needs one primitive area.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
