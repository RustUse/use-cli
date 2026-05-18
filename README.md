# RustUse/use-cli

Composable command-line-adjacent primitives for Rust.

`use-cli` is not a command-line application, CLI framework, argument parser, command router, runtime, TUI toolkit, logging layer, or config framework. It is a `RustUse` crate set for small primitive types that help libraries, tests, examples, and applications describe CLI-related behavior without adopting a framework.

The crate set complements crates like `clap`, `argh`, `bpaf`, and `dialoguer`. Use those crates when you need parsing, derive macros, routing, shell completion, or interactive workflows. Use these crates when you want stable primitive vocabulary for CLI-adjacent code.

## Workspace crates

| Crate              | Path                       | Purpose                                              |
| ------------------ | -------------------------- | ---------------------------------------------------- |
| `use-cli`          | `crates/use-cli/`          | Facade over the focused CLI primitive crates         |
| `use-arg`          | `crates/use-arg/`          | Owned argument tokens and raw argument collections   |
| `use-flag`         | `crates/use-flag/`         | Short, long, and boolean flag primitives             |
| `use-option`       | `crates/use-option/`       | CLI key/value option primitives                      |
| `use-env-var`      | `crates/use-env-var/`      | Environment variable names, values, and read helpers |
| `use-exit-code`    | `crates/use-exit-code/`    | Portable exit status primitives                      |
| `use-stdin`        | `crates/use-stdin/`        | Synchronous stdin source and read helpers            |
| `use-stdout`       | `crates/use-stdout/`       | Synchronous stdout destination and newline helpers   |
| `use-stderr`       | `crates/use-stderr/`       | Synchronous stderr destination and write helpers     |
| `use-prompt`       | `crates/use-prompt/`       | Prompt text and confirmation answer primitives       |
| `use-command-name` | `crates/use-command-name/` | Command and executable name primitives               |
| `use-terminal`     | `crates/use-terminal/`     | Terminal size, color, and interactivity primitives   |

## Facade modules

The `use-cli` facade re-exports child crates behind explicit module names:

```rust
use use_cli::{arg, command_name, exit_code, flag, option};

let command = command_name::CommandName::new("rustuse")?;
let verbose = flag::BooleanFlag::enabled(flag::Flag::try_from_token("--verbose")?);
let color = option::CliOption::from_name_value("color", "auto")?;
let input = arg::Arg::from("README.md");

assert_eq!(command.as_str(), "rustuse");
assert!(verbose.is_enabled());
assert_eq!(color.to_equals_token(), "--color=auto");
assert!(input.is_positional());
assert_eq!(exit_code::SUCCESS.as_i32(), 0);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Each focused crate can also be used independently when a downstream crate only needs one primitive surface.

## Development

Run the standard workspace checks from this directory:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either the MIT license or Apache License, Version 2.0.
