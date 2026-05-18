# use-command-name

Command and executable name primitives for `RustUse` CLI-adjacent code.

This crate validates command names and extracts executable names from paths. It does not implement command routing, subcommands, aliases, or dispatch.

## Example

```rust
use use_command_name::{CommandName, ExecutableName};

let command = CommandName::new("rustuse")?;
let executable = ExecutableName::new("rustuse.exe")?;

assert_eq!(command.as_str(), "rustuse");
assert_eq!(executable.display_name(), "rustuse.exe");
# Ok::<(), use_command_name::CommandNameError>(())
```

## Scope

Use this crate for explicit command name vocabulary, not parser or runtime behavior.
