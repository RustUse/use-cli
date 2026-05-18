#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// Commonly used flag primitives.
pub mod prelude {
    pub use crate::{
        BooleanFlag, Flag, FlagNameError, LongFlag, ShortFlag, is_valid_long_flag_name,
        is_valid_short_flag,
    };
}

/// Validation errors for primitive flag names and tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlagNameError {
    /// A flag name was empty.
    Empty,
    /// A short flag was not a single ASCII alphanumeric character.
    InvalidShortFlag,
    /// A long flag name was not a plain ASCII flag name.
    InvalidLongFlagName,
    /// A token did not look like a supported primitive flag token.
    InvalidToken,
}

impl fmt::Display for FlagNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("flag name cannot be empty"),
            Self::InvalidShortFlag => {
                formatter.write_str("short flag must be one ASCII alphanumeric character")
            },
            Self::InvalidLongFlagName => formatter.write_str(
                "long flag name must be ASCII alphanumeric with optional internal hyphens",
            ),
            Self::InvalidToken => formatter.write_str("flag token must look like -x or --name"),
        }
    }
}

impl std::error::Error for FlagNameError {}

/// A one-character short flag such as `-v`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShortFlag {
    name: char,
}

impl ShortFlag {
    /// Creates a short flag from a single name character.
    ///
    /// # Errors
    ///
    /// Returns [`FlagNameError::InvalidShortFlag`] when `name` is not ASCII alphanumeric.
    pub const fn new(name: char) -> Result<Self, FlagNameError> {
        if is_valid_short_flag(name) {
            Ok(Self { name })
        } else {
            Err(FlagNameError::InvalidShortFlag)
        }
    }

    /// Returns the short flag name character.
    #[must_use]
    pub const fn name(self) -> char {
        self.name
    }

    /// Returns the token form, such as `-v`.
    #[must_use]
    pub fn to_token(self) -> String {
        format!("-{}", self.name)
    }
}

impl fmt::Display for ShortFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_token())
    }
}

/// A long flag name such as `verbose` for `--verbose`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LongFlag {
    name: String,
}

impl LongFlag {
    /// Creates a long flag from a validated name without the leading `--`.
    ///
    /// # Errors
    ///
    /// Returns [`FlagNameError::InvalidLongFlagName`] when `name` is not a basic long flag name.
    pub fn new(name: impl Into<String>) -> Result<Self, FlagNameError> {
        let name = name.into();
        if is_valid_long_flag_name(&name) {
            Ok(Self { name })
        } else if name.is_empty() {
            Err(FlagNameError::Empty)
        } else {
            Err(FlagNameError::InvalidLongFlagName)
        }
    }

    /// Returns the long flag name without the leading `--`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Returns the token form, such as `--verbose`.
    #[must_use]
    pub fn to_token(&self) -> String {
        format!("--{}", self.name)
    }
}

impl AsRef<str> for LongFlag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for LongFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_token())
    }
}

/// A primitive flag token.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Flag {
    /// A short flag such as `-v`.
    Short(ShortFlag),
    /// A long flag such as `--verbose`.
    Long(LongFlag),
}

impl Flag {
    /// Parses a primitive flag token.
    ///
    /// # Errors
    ///
    /// Returns [`FlagNameError`] when the token is not a supported short or long flag token.
    pub fn try_from_token(token: &str) -> Result<Self, FlagNameError> {
        if let Some(name) = token.strip_prefix("--") {
            return LongFlag::new(name).map(Self::Long);
        }

        if let Some(name) = token.strip_prefix('-') {
            let mut characters = name.chars();
            return match (characters.next(), characters.next()) {
                (Some(character), None) => ShortFlag::new(character).map(Self::Short),
                _ => Err(FlagNameError::InvalidToken),
            };
        }

        Err(FlagNameError::InvalidToken)
    }

    /// Returns the token form for this flag.
    #[must_use]
    pub fn to_token(&self) -> String {
        match self {
            Self::Short(flag) => flag.to_token(),
            Self::Long(flag) => flag.to_token(),
        }
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_token())
    }
}

/// A flag paired with a boolean state.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BooleanFlag {
    flag: Flag,
    enabled: bool,
}

impl BooleanFlag {
    /// Creates a boolean flag primitive.
    #[must_use]
    pub const fn new(flag: Flag, enabled: bool) -> Self {
        Self { flag, enabled }
    }

    /// Creates an enabled boolean flag.
    #[must_use]
    pub const fn enabled(flag: Flag) -> Self {
        Self::new(flag, true)
    }

    /// Creates a disabled boolean flag.
    #[must_use]
    pub const fn disabled(flag: Flag) -> Self {
        Self::new(flag, false)
    }

    /// Returns the underlying flag.
    #[must_use]
    pub const fn flag(&self) -> &Flag {
        &self.flag
    }

    /// Returns whether the flag is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Returns whether `name` is valid for a short flag.
#[must_use]
pub const fn is_valid_short_flag(name: char) -> bool {
    name.is_ascii_alphanumeric()
}

/// Returns whether `name` is valid for a long flag without the leading `--`.
#[must_use]
pub fn is_valid_long_flag_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() || bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' {
        return false;
    }

    bytes
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
}

#[cfg(test)]
mod tests {
    use super::{
        BooleanFlag, Flag, FlagNameError, LongFlag, ShortFlag, is_valid_long_flag_name,
        is_valid_short_flag,
    };

    #[test]
    fn validates_short_and_long_names() {
        assert!(is_valid_short_flag('v'));
        assert!(!is_valid_short_flag('-'));
        assert!(is_valid_long_flag_name("dry-run"));
        assert!(!is_valid_long_flag_name("-dry"));
        assert!(!is_valid_long_flag_name("dry_"));
    }

    #[test]
    fn creates_flag_tokens() -> Result<(), FlagNameError> {
        assert_eq!(ShortFlag::new('v')?.to_token(), "-v");
        assert_eq!(LongFlag::new("verbose")?.to_token(), "--verbose");
        assert_eq!(Flag::try_from_token("--dry-run")?.to_token(), "--dry-run");
        assert_eq!(Flag::try_from_token("-q")?.to_token(), "-q");
        assert_eq!(
            Flag::try_from_token("---"),
            Err(FlagNameError::InvalidLongFlagName)
        );
        Ok(())
    }

    #[test]
    fn stores_boolean_flag_state() -> Result<(), FlagNameError> {
        let flag = BooleanFlag::enabled(Flag::try_from_token("--verbose")?);

        assert!(flag.is_enabled());
        assert_eq!(flag.flag().to_token(), "--verbose");
        Ok(())
    }
}
