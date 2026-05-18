# use-terminal

Terminal capability primitives for `RustUse` CLI-adjacent code.

This crate provides terminal width and height wrapper types, color support enums, interactivity enums, and dependency-free detection helpers. It does not implement terminal UI, curses, layout, or styling.

## Example

```rust
use use_terminal::{ColorSupport, TerminalHeight, TerminalSize, TerminalWidth};

let size = TerminalSize::new(TerminalWidth::new(80)?, TerminalHeight::new(24)?);
let color = ColorSupport::from_env_values(None, Some("xterm-256color"), None);

assert_eq!(size.width().columns(), 80);
assert_eq!(color, ColorSupport::Ansi256);
# Ok::<(), use_terminal::TerminalDimensionError>(())
```

## Scope

Use this crate for capability vocabulary and basic std-based detection only.
