# use-prompt

Prompt text and confirmation primitives for `RustUse` CLI-adjacent code.

This crate stores prompt text and parses yes/no confirmation answers. It does not implement an interactive UI loop, prompt renderer, or terminal framework.

## Example

```rust
use use_prompt::{PromptText, YesNoAnswer, parse_confirmation};

let prompt = PromptText::new("Continue?")?;
let answer = parse_confirmation("yes")?;

assert_eq!(prompt.as_str(), "Continue?");
assert_eq!(answer, YesNoAnswer::Yes);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for prompt vocabulary and confirmation parsing only.
