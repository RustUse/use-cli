#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::io::{self, Write};

/// Commonly used stdout primitives.
pub mod prelude {
    pub use crate::{
        apply_newline_behavior, write_line, write_stdout, write_stdout_line, write_text,
        NewlineBehavior, StdoutDestination,
    };
}

/// Marker type for the process standard output stream.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdoutDestination;

impl StdoutDestination {
    /// Creates a stdout destination marker.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Primitive newline policy for text output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NewlineBehavior {
    /// Preserve the input exactly.
    Preserve,
    /// Ensure the output ends with `\n`.
    EnsureTrailingNewline,
    /// Remove trailing `\r` and `\n` characters.
    StripTrailingNewline,
}

/// Applies a newline behavior to text and returns an owned string.
#[must_use]
pub fn apply_newline_behavior(text: &str, behavior: NewlineBehavior) -> String {
    match behavior {
        NewlineBehavior::Preserve => text.to_owned(),
        NewlineBehavior::EnsureTrailingNewline => ensure_trailing_newline(text),
        NewlineBehavior::StripTrailingNewline => text.trim_end_matches(['\r', '\n']).to_owned(),
    }
}

/// Writes text to process stdout.
///
/// # Errors
///
/// Returns any I/O error reported while writing to stdout.
pub fn write_stdout(text: &str) -> io::Result<()> {
    let stdout = io::stdout();
    write_text(stdout.lock(), text)
}

/// Writes text plus one newline to process stdout.
///
/// # Errors
///
/// Returns any I/O error reported while writing to stdout.
pub fn write_stdout_line(text: &str) -> io::Result<()> {
    let stdout = io::stdout();
    write_line(stdout.lock(), text)
}

/// Writes text to a generic writer.
///
/// # Errors
///
/// Returns any I/O error reported by `writer`.
pub fn write_text(mut writer: impl Write, text: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())
}

/// Writes text plus one newline to a generic writer.
///
/// # Errors
///
/// Returns any I/O error reported by `writer`.
pub fn write_line(mut writer: impl Write, text: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())?;
    writer.write_all(b"\n")
}

fn ensure_trailing_newline(text: &str) -> String {
    if text.ends_with('\n') {
        text.to_owned()
    } else {
        let mut output = String::with_capacity(text.len() + 1);
        output.push_str(text);
        output.push('\n');
        output
    }
}

#[cfg(test)]
mod tests {
    use super::{
        apply_newline_behavior, write_line, write_text, NewlineBehavior, StdoutDestination,
    };

    #[test]
    fn marker_is_copyable() {
        let destination = StdoutDestination::new();
        let copied = destination;

        assert_eq!(destination, copied);
    }

    #[test]
    fn applies_newline_behavior() {
        assert_eq!(
            apply_newline_behavior("ready", NewlineBehavior::Preserve),
            "ready"
        );
        assert_eq!(
            apply_newline_behavior("ready", NewlineBehavior::EnsureTrailingNewline),
            "ready\n"
        );
        assert_eq!(
            apply_newline_behavior("ready\r\n", NewlineBehavior::StripTrailingNewline),
            "ready"
        );
    }

    #[test]
    fn writes_to_generic_writer() -> Result<(), std::io::Error> {
        let mut buffer = Vec::new();
        write_text(&mut buffer, "hello")?;
        write_line(&mut buffer, " world")?;

        assert_eq!(buffer, b"hello world\n");
        Ok(())
    }
}
