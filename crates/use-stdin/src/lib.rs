#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::io::{self, BufRead, Read};

/// Commonly used stdin primitives.
pub mod prelude {
    pub use crate::{
        StdinSource, read_line_from, read_stdin_line, read_stdin_to_string, read_to_string_from,
    };
}

/// Marker type for the process standard input stream.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StdinSource;

impl StdinSource {
    /// Creates a stdin source marker.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Reads all process stdin into a string.
///
/// # Errors
///
/// Returns any I/O error reported while reading from stdin.
pub fn read_stdin_to_string() -> io::Result<String> {
    let stdin = io::stdin();
    read_to_string_from(stdin.lock())
}

/// Reads one line from process stdin.
///
/// # Errors
///
/// Returns any I/O error reported while reading from stdin.
pub fn read_stdin_line() -> io::Result<String> {
    let stdin = io::stdin();
    read_line_from(stdin.lock())
}

/// Reads all content from a generic reader into a string.
///
/// # Errors
///
/// Returns any I/O error reported by `reader`.
pub fn read_to_string_from(mut reader: impl Read) -> io::Result<String> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    Ok(input)
}

/// Reads one line from a buffered reader.
///
/// # Errors
///
/// Returns any I/O error reported by `reader`.
pub fn read_line_from(mut reader: impl BufRead) -> io::Result<String> {
    let mut input = String::new();
    reader.read_line(&mut input)?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{StdinSource, read_line_from, read_to_string_from};

    #[test]
    fn marker_is_copyable() {
        let source = StdinSource::new();
        let copied = source;

        assert_eq!(source, copied);
    }

    #[test]
    fn reads_from_generic_readers() -> Result<(), std::io::Error> {
        assert_eq!(read_to_string_from(Cursor::new("hello"))?, "hello");
        assert_eq!(read_line_from(Cursor::new("first\nsecond"))?, "first\n");
        Ok(())
    }
}
