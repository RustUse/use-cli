#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::io::{self, Write};

/// Commonly used stderr primitives.
pub mod prelude {
    pub use crate::{
        write_error, write_error_line, write_stderr, write_stderr_line, StderrDestination,
    };
}

/// Marker type for the process standard error stream.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StderrDestination;

impl StderrDestination {
    /// Creates a stderr destination marker.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Writes text to process stderr.
///
/// # Errors
///
/// Returns any I/O error reported while writing to stderr.
pub fn write_stderr(text: &str) -> io::Result<()> {
    let stderr = io::stderr();
    write_error(stderr.lock(), text)
}

/// Writes text plus one newline to process stderr.
///
/// # Errors
///
/// Returns any I/O error reported while writing to stderr.
pub fn write_stderr_line(text: &str) -> io::Result<()> {
    let stderr = io::stderr();
    write_error_line(stderr.lock(), text)
}

/// Writes error text to a generic writer.
///
/// # Errors
///
/// Returns any I/O error reported by `writer`.
pub fn write_error(mut writer: impl Write, text: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())
}

/// Writes error text plus one newline to a generic writer.
///
/// # Errors
///
/// Returns any I/O error reported by `writer`.
pub fn write_error_line(mut writer: impl Write, text: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())?;
    writer.write_all(b"\n")
}

#[cfg(test)]
mod tests {
    use super::{write_error, write_error_line, StderrDestination};

    #[test]
    fn marker_is_copyable() {
        let destination = StderrDestination::new();
        let copied = destination;

        assert_eq!(destination, copied);
    }

    #[test]
    fn writes_to_generic_writer() -> Result<(), std::io::Error> {
        let mut buffer = Vec::new();
        write_error(&mut buffer, "warning")?;
        write_error_line(&mut buffer, "!")?;

        assert_eq!(buffer, b"warning!\n");
        Ok(())
    }
}
