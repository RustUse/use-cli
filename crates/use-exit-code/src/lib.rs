#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// Commonly used exit code primitives.
pub mod prelude {
    pub use crate::{
        CONFIG_ERROR, ExitCode, ExitCodeError, FAILURE, PERMISSION_DENIED, SUCCESS, UNAVAILABLE,
        USAGE_ERROR,
    };
}

/// A portable process exit code primitive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExitCode(u8);

impl ExitCode {
    /// Creates an exit code from an unsigned byte.
    #[must_use]
    pub const fn from_u8(code: u8) -> Self {
        Self(code)
    }

    /// Creates an exit code from an `i32` when it fits in `u8`.
    ///
    /// # Errors
    ///
    /// Returns [`ExitCodeError::OutOfRange`] when `code` is outside `0..=255`.
    pub fn try_from_i32(code: i32) -> Result<Self, ExitCodeError> {
        u8::try_from(code)
            .map(Self)
            .map_err(|_| ExitCodeError::OutOfRange(code))
    }

    /// Returns the exit code as `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    /// Returns the exit code as `i32`.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0 as i32
    }

    /// Returns whether this is a success exit code.
    #[must_use]
    pub const fn is_success(self) -> bool {
        self.0 == 0
    }
}

impl TryFrom<i32> for ExitCode {
    type Error = ExitCodeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from_i32(value)
    }
}

impl From<ExitCode> for u8 {
    fn from(value: ExitCode) -> Self {
        value.as_u8()
    }
}

impl From<ExitCode> for i32 {
    fn from(value: ExitCode) -> Self {
        value.as_i32()
    }
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(value: ExitCode) -> Self {
        Self::from(value.as_u8())
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Exit code conversion errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExitCodeError {
    /// The signed integer did not fit in a portable `u8` exit code.
    OutOfRange(i32),
}

impl fmt::Display for ExitCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange(code) => write!(formatter, "exit code {code} is outside 0..=255"),
        }
    }
}

impl std::error::Error for ExitCodeError {}

/// Successful completion.
pub const SUCCESS: ExitCode = ExitCode::from_u8(0);

/// General failure.
pub const FAILURE: ExitCode = ExitCode::from_u8(1);

/// Command line usage error.
pub const USAGE_ERROR: ExitCode = ExitCode::from_u8(64);

/// Service or dependency unavailable.
pub const UNAVAILABLE: ExitCode = ExitCode::from_u8(69);

/// Permission denied.
pub const PERMISSION_DENIED: ExitCode = ExitCode::from_u8(77);

/// Configuration error.
pub const CONFIG_ERROR: ExitCode = ExitCode::from_u8(78);

#[cfg(test)]
mod tests {
    use super::{CONFIG_ERROR, ExitCode, ExitCodeError, FAILURE, SUCCESS, USAGE_ERROR};

    #[test]
    fn exposes_common_exit_codes() {
        assert!(SUCCESS.is_success());
        assert!(!FAILURE.is_success());
        assert_eq!(USAGE_ERROR.as_i32(), 64);
        assert_eq!(CONFIG_ERROR.as_u8(), 78);
    }

    #[test]
    fn converts_between_integer_forms() -> Result<(), ExitCodeError> {
        let code = ExitCode::try_from_i32(77)?;

        assert_eq!(code.as_u8(), 77);
        assert_eq!(i32::from(code), 77);
        assert_eq!(u8::from(code), 77);
        assert_eq!(
            ExitCode::try_from_i32(256),
            Err(ExitCodeError::OutOfRange(256))
        );
        Ok(())
    }
}
