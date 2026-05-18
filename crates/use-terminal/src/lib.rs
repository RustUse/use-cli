#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::{
    env,
    io::{self, IsTerminal},
};

/// Commonly used terminal primitives.
pub mod prelude {
    pub use crate::{
        ColorSupport, Interactivity, TerminalDimensionError, TerminalHeight, TerminalSize,
        TerminalWidth, detect_color_support, stderr_interactivity, stdin_interactivity,
        stdout_interactivity,
    };
}

/// Validation errors for terminal dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalDimensionError {
    /// Width must be nonzero.
    ZeroWidth,
    /// Height must be nonzero.
    ZeroHeight,
}

impl fmt::Display for TerminalDimensionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => formatter.write_str("terminal width must be greater than zero"),
            Self::ZeroHeight => formatter.write_str("terminal height must be greater than zero"),
        }
    }
}

impl std::error::Error for TerminalDimensionError {}

/// A nonzero terminal width in columns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalWidth {
    columns: u16,
}

impl TerminalWidth {
    /// Creates a terminal width.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalDimensionError::ZeroWidth`] when `columns` is zero.
    pub const fn new(columns: u16) -> Result<Self, TerminalDimensionError> {
        if columns == 0 {
            Err(TerminalDimensionError::ZeroWidth)
        } else {
            Ok(Self { columns })
        }
    }

    /// Returns the width in columns.
    #[must_use]
    pub const fn columns(self) -> u16 {
        self.columns
    }
}

/// A nonzero terminal height in rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalHeight {
    rows: u16,
}

impl TerminalHeight {
    /// Creates a terminal height.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalDimensionError::ZeroHeight`] when `rows` is zero.
    pub const fn new(rows: u16) -> Result<Self, TerminalDimensionError> {
        if rows == 0 {
            Err(TerminalDimensionError::ZeroHeight)
        } else {
            Ok(Self { rows })
        }
    }

    /// Returns the height in rows.
    #[must_use]
    pub const fn rows(self) -> u16 {
        self.rows
    }
}

/// A terminal size value made from validated dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TerminalSize {
    width: TerminalWidth,
    height: TerminalHeight,
}

impl TerminalSize {
    /// Creates a terminal size from validated dimensions.
    #[must_use]
    pub const fn new(width: TerminalWidth, height: TerminalHeight) -> Self {
        Self { width, height }
    }

    /// Creates a terminal size from raw column and row counts.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalDimensionError`] when either dimension is zero.
    pub const fn try_new(columns: u16, rows: u16) -> Result<Self, TerminalDimensionError> {
        Ok(Self::new(
            match TerminalWidth::new(columns) {
                Ok(width) => width,
                Err(error) => return Err(error),
            },
            match TerminalHeight::new(rows) {
                Ok(height) => height,
                Err(error) => return Err(error),
            },
        ))
    }

    /// Returns the validated width.
    #[must_use]
    pub const fn width(self) -> TerminalWidth {
        self.width
    }

    /// Returns the validated height.
    #[must_use]
    pub const fn height(self) -> TerminalHeight {
        self.height
    }
}

/// Primitive terminal color capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorSupport {
    /// Color output should not be used.
    NoColor,
    /// Basic ANSI color support is likely available.
    Basic,
    /// 256-color ANSI support is likely available.
    Ansi256,
    /// 24-bit truecolor support is likely available.
    TrueColor,
}

impl ColorSupport {
    /// Infers color support from environment variable values.
    #[must_use]
    pub fn from_env_values(
        no_color: Option<&str>,
        term: Option<&str>,
        colorterm: Option<&str>,
    ) -> Self {
        if no_color.is_some() {
            return Self::NoColor;
        }

        if colorterm.is_some_and(|value| {
            value.eq_ignore_ascii_case("truecolor") || value.eq_ignore_ascii_case("24bit")
        }) {
            return Self::TrueColor;
        }

        match term {
            Some(value) if value.eq_ignore_ascii_case("dumb") => Self::NoColor,
            Some(value) if value.contains("256color") => Self::Ansi256,
            Some("") | None => Self::NoColor,
            Some(_) => Self::Basic,
        }
    }
}

/// Primitive stream interactivity state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Interactivity {
    /// The stream appears to be attached to a terminal.
    Interactive,
    /// The stream does not appear to be attached to a terminal.
    NonInteractive,
}

impl Interactivity {
    /// Converts a boolean terminal result into an interactivity value.
    #[must_use]
    pub const fn from_bool(is_terminal: bool) -> Self {
        if is_terminal {
            Self::Interactive
        } else {
            Self::NonInteractive
        }
    }

    /// Returns whether the stream is interactive.
    #[must_use]
    pub const fn is_interactive(self) -> bool {
        matches!(self, Self::Interactive)
    }
}

/// Detects color support using `NO_COLOR`, `TERM`, and `COLORTERM`.
#[must_use]
pub fn detect_color_support() -> ColorSupport {
    let no_color = env::var_os("NO_COLOR").map(|_| "");
    let term = env::var("TERM").ok();
    let colorterm = env::var("COLORTERM").ok();

    ColorSupport::from_env_values(no_color, term.as_deref(), colorterm.as_deref())
}

/// Detects whether stdin is attached to a terminal.
#[must_use]
pub fn stdin_interactivity() -> Interactivity {
    Interactivity::from_bool(io::stdin().is_terminal())
}

/// Detects whether stdout is attached to a terminal.
#[must_use]
pub fn stdout_interactivity() -> Interactivity {
    Interactivity::from_bool(io::stdout().is_terminal())
}

/// Detects whether stderr is attached to a terminal.
#[must_use]
pub fn stderr_interactivity() -> Interactivity {
    Interactivity::from_bool(io::stderr().is_terminal())
}

#[cfg(test)]
mod tests {
    use super::{
        ColorSupport, Interactivity, TerminalDimensionError, TerminalHeight, TerminalSize,
        TerminalWidth,
    };

    #[test]
    fn validates_terminal_dimensions() -> Result<(), TerminalDimensionError> {
        let size = TerminalSize::try_new(80, 24)?;

        assert_eq!(size.width().columns(), 80);
        assert_eq!(size.height().rows(), 24);
        assert_eq!(
            TerminalWidth::new(0),
            Err(TerminalDimensionError::ZeroWidth)
        );
        assert_eq!(
            TerminalHeight::new(0),
            Err(TerminalDimensionError::ZeroHeight)
        );
        Ok(())
    }

    #[test]
    fn infers_color_support_from_env_values() {
        assert_eq!(
            ColorSupport::from_env_values(Some("1"), Some("xterm-256color"), Some("truecolor")),
            ColorSupport::NoColor
        );
        assert_eq!(
            ColorSupport::from_env_values(None, Some("xterm-256color"), None),
            ColorSupport::Ansi256
        );
        assert_eq!(
            ColorSupport::from_env_values(None, Some("xterm"), Some("truecolor")),
            ColorSupport::TrueColor
        );
        assert_eq!(
            ColorSupport::from_env_values(None, Some("dumb"), None),
            ColorSupport::NoColor
        );
    }

    #[test]
    fn converts_interactivity_from_bool() {
        assert!(Interactivity::from_bool(true).is_interactive());
        assert!(!Interactivity::from_bool(false).is_interactive());
    }
}
