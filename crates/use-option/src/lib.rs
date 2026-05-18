#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// Commonly used CLI option primitives.
pub mod prelude {
    pub use crate::{
        CliOption, CliOptionError, CliOptionName, OptionNameError, OptionValue,
        is_valid_option_name, split_equals_token,
    };
}

/// Validation errors for CLI option names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionNameError {
    /// The option name was empty.
    Empty,
    /// The option name started or ended with a hyphen.
    EdgeHyphen,
    /// The option name contained a character outside the supported primitive set.
    InvalidCharacter,
}

impl fmt::Display for OptionNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("option name cannot be empty"),
            Self::EdgeHyphen => formatter.write_str("option name cannot start or end with '-'"),
            Self::InvalidCharacter => formatter
                .write_str("option name must be ASCII alphanumeric with optional internal hyphens"),
        }
    }
}

impl std::error::Error for OptionNameError {}

/// Errors for primitive CLI option construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliOptionError {
    /// The option name was invalid.
    InvalidName(OptionNameError),
    /// The token did not begin with `--`.
    MissingLongPrefix,
    /// The token did not contain an equals sign.
    MissingEquals,
}

impl fmt::Display for CliOptionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(error) => write!(formatter, "{error}"),
            Self::MissingLongPrefix => formatter.write_str("option token must start with --"),
            Self::MissingEquals => formatter.write_str("option token must contain '='"),
        }
    }
}

impl std::error::Error for CliOptionError {}

impl From<OptionNameError> for CliOptionError {
    fn from(error: OptionNameError) -> Self {
        Self::InvalidName(error)
    }
}

/// A validated CLI option name without a leading `--`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CliOptionName {
    name: String,
}

impl CliOptionName {
    /// Creates a validated CLI option name.
    ///
    /// # Errors
    ///
    /// Returns [`OptionNameError`] when `name` is not a basic option name.
    pub fn new(name: impl Into<String>) -> Result<Self, OptionNameError> {
        let name = name.into();
        validate_option_name(&name)?;
        Ok(Self { name })
    }

    /// Returns the option name without a leading `--`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl AsRef<str> for CliOptionName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CliOptionName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// An owned CLI option value.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OptionValue {
    value: String,
}

impl OptionValue {
    /// Creates an owned option value.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the borrowed value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the owned value.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

impl AsRef<str> for OptionValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for OptionValue {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for OptionValue {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for OptionValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

/// A primitive command-line key/value option.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CliOption {
    name: CliOptionName,
    value: OptionValue,
}

impl CliOption {
    /// Creates a CLI option from an already validated name and owned value.
    #[must_use]
    pub const fn new(name: CliOptionName, value: OptionValue) -> Self {
        Self { name, value }
    }

    /// Creates a CLI option from a name and value pair, such as `--key value`.
    ///
    /// # Errors
    ///
    /// Returns [`CliOptionError`] when `name` is not a valid option name.
    pub fn from_name_value(name: &str, value: impl Into<String>) -> Result<Self, CliOptionError> {
        Ok(Self::new(
            CliOptionName::new(name)?,
            OptionValue::new(value),
        ))
    }

    /// Creates a CLI option from a token like `--key=value`.
    ///
    /// # Errors
    ///
    /// Returns [`CliOptionError`] when the token does not use `--key=value` form.
    pub fn from_equals_token(token: &str) -> Result<Self, CliOptionError> {
        split_equals_token(token)
    }

    /// Returns the option name.
    #[must_use]
    pub const fn name(&self) -> &CliOptionName {
        &self.name
    }

    /// Returns the option value.
    #[must_use]
    pub const fn value(&self) -> &OptionValue {
        &self.value
    }

    /// Returns the `--key=value` token form.
    #[must_use]
    pub fn to_equals_token(&self) -> String {
        format!("--{}={}", self.name, self.value)
    }
}

/// Returns whether `name` is a valid primitive CLI option name.
#[must_use]
pub fn is_valid_option_name(name: &str) -> bool {
    validate_option_name(name).is_ok()
}

/// Splits a token like `--key=value` into a primitive CLI option.
///
/// # Errors
///
/// Returns [`CliOptionError`] when the token is missing the `--` prefix, the `=`, or a valid name.
pub fn split_equals_token(token: &str) -> Result<CliOption, CliOptionError> {
    let token = token
        .strip_prefix("--")
        .ok_or(CliOptionError::MissingLongPrefix)?;
    let (name, value) = token.split_once('=').ok_or(CliOptionError::MissingEquals)?;

    CliOption::from_name_value(name, value)
}

fn validate_option_name(name: &str) -> Result<(), OptionNameError> {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return Err(OptionNameError::Empty);
    }

    if bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' {
        return Err(OptionNameError::EdgeHyphen);
    }

    if bytes
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
    {
        Ok(())
    } else {
        Err(OptionNameError::InvalidCharacter)
    }
}

#[cfg(test)]
mod tests {
    use super::{CliOption, CliOptionError, OptionNameError, is_valid_option_name};

    #[test]
    fn validates_option_names() {
        assert!(is_valid_option_name("color"));
        assert!(is_valid_option_name("dry-run"));
        assert!(!is_valid_option_name(""));
        assert!(!is_valid_option_name("dry_run"));
        assert!(!is_valid_option_name("dry-"));
    }

    #[test]
    fn builds_options_from_pair_and_equals_token() -> Result<(), CliOptionError> {
        let pair = CliOption::from_name_value("format", "json")?;
        let equals = CliOption::from_equals_token("--color=auto")?;

        assert_eq!(pair.to_equals_token(), "--format=json");
        assert_eq!(equals.name().as_str(), "color");
        assert_eq!(equals.value().as_str(), "auto");
        Ok(())
    }

    #[test]
    fn rejects_invalid_option_tokens() {
        assert_eq!(
            CliOption::from_equals_token("color=auto"),
            Err(CliOptionError::MissingLongPrefix)
        );
        assert_eq!(
            CliOption::from_equals_token("--color"),
            Err(CliOptionError::MissingEquals)
        );
        assert_eq!(
            CliOption::from_equals_token("--dry_=true"),
            Err(CliOptionError::InvalidName(
                OptionNameError::InvalidCharacter
            ))
        );
    }
}
