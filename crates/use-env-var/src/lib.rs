#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, marker::PhantomData, str::FromStr};
use std::env;

/// Commonly used environment variable primitives.
pub mod prelude {
    pub use crate::{
        EnvVarName, EnvVarNameError, EnvVarReadError, EnvVarValue, TypedEnvVar, TypedEnvVarError,
        is_valid_env_var_name, read_env_var, read_optional_env_var,
    };
}

/// Validation errors for environment variable names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnvVarNameError {
    /// The variable name was empty.
    Empty,
    /// The variable name started with an ASCII digit.
    StartsWithDigit,
    /// The variable name contained an unsupported character.
    InvalidCharacter,
}

impl fmt::Display for EnvVarNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("environment variable name cannot be empty"),
            Self::StartsWithDigit => {
                formatter.write_str("environment variable name cannot start with a digit")
            }
            Self::InvalidCharacter => formatter.write_str(
                "environment variable name must use ASCII letters, digits, and underscores",
            ),
        }
    }
}

impl std::error::Error for EnvVarNameError {}

/// An owned, validated environment variable name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnvVarName {
    name: String,
}

impl EnvVarName {
    /// Creates a validated environment variable name.
    ///
    /// # Errors
    ///
    /// Returns [`EnvVarNameError`] when `name` is empty or contains unsupported characters.
    pub fn new(name: impl Into<String>) -> Result<Self, EnvVarNameError> {
        let name = name.into();
        validate_env_var_name(&name)?;
        Ok(Self { name })
    }

    /// Returns the environment variable name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl AsRef<str> for EnvVarName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for EnvVarName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// An owned environment variable value.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EnvVarValue {
    value: String,
}

impl EnvVarValue {
    /// Creates an owned environment variable value.
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

impl AsRef<str> for EnvVarValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for EnvVarValue {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for EnvVarValue {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for EnvVarValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

/// Errors returned while reading environment variables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnvVarReadError {
    /// The environment variable was not present.
    NotPresent { name: String },
    /// The environment variable value was not valid Unicode.
    NotUnicode { name: String },
}

impl fmt::Display for EnvVarReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotPresent { name } => {
                write!(formatter, "environment variable {name} is not set")
            }
            Self::NotUnicode { name } => {
                write!(
                    formatter,
                    "environment variable {name} is not valid Unicode"
                )
            }
        }
    }
}

impl std::error::Error for EnvVarReadError {}

/// A typed environment variable wrapper with caller-provided parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedEnvVar<T> {
    name: EnvVarName,
    marker: PhantomData<fn() -> T>,
}

impl<T> TypedEnvVar<T> {
    /// Creates a typed environment variable wrapper.
    #[must_use]
    pub const fn new(name: EnvVarName) -> Self {
        Self {
            name,
            marker: PhantomData,
        }
    }

    /// Returns the underlying environment variable name.
    #[must_use]
    pub const fn name(&self) -> &EnvVarName {
        &self.name
    }

    /// Reads and parses the environment variable with a caller-provided parser.
    ///
    /// # Errors
    ///
    /// Returns [`TypedEnvVarError::Read`] when the variable cannot be read and
    /// [`TypedEnvVarError::Parse`] when the parser rejects the value.
    pub fn read_with<E>(
        &self,
        parser: impl FnOnce(&str) -> Result<T, E>,
    ) -> Result<T, TypedEnvVarError<E>> {
        let value = read_env_var(&self.name).map_err(TypedEnvVarError::Read)?;
        parser(value.as_str()).map_err(TypedEnvVarError::Parse)
    }

    /// Reads and parses the environment variable with [`FromStr`].
    ///
    /// # Errors
    ///
    /// Returns [`TypedEnvVarError::Read`] when the variable cannot be read and
    /// [`TypedEnvVarError::Parse`] when `T::from_str` rejects the value.
    pub fn read_parse(&self) -> Result<T, TypedEnvVarError<T::Err>>
    where
        T: FromStr,
    {
        self.read_with(str::parse)
    }
}

/// Errors returned while reading a typed environment variable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedEnvVarError<E> {
    /// Reading the environment variable failed.
    Read(EnvVarReadError),
    /// Parsing the environment variable value failed.
    Parse(E),
}

impl<E: fmt::Display> fmt::Display for TypedEnvVarError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(error) => write!(formatter, "{error}"),
            Self::Parse(error) => write!(formatter, "environment variable parse failed: {error}"),
        }
    }
}

impl<E> std::error::Error for TypedEnvVarError<E> where E: fmt::Debug + fmt::Display {}

/// Returns whether `name` is valid for this crate's environment variable primitive.
#[must_use]
pub fn is_valid_env_var_name(name: &str) -> bool {
    validate_env_var_name(name).is_ok()
}

/// Reads a present Unicode environment variable.
///
/// # Errors
///
/// Returns [`EnvVarReadError::NotPresent`] when the variable is missing and
/// [`EnvVarReadError::NotUnicode`] when the value is not valid Unicode.
pub fn read_env_var(name: &EnvVarName) -> Result<EnvVarValue, EnvVarReadError> {
    env::var(name.as_str())
        .map(EnvVarValue::new)
        .map_err(|error| match error {
            env::VarError::NotPresent => EnvVarReadError::NotPresent {
                name: name.as_str().to_owned(),
            },
            env::VarError::NotUnicode(_) => EnvVarReadError::NotUnicode {
                name: name.as_str().to_owned(),
            },
        })
}

/// Reads an optional Unicode environment variable.
///
/// # Errors
///
/// Returns [`EnvVarReadError::NotUnicode`] when the variable exists but is not valid Unicode.
pub fn read_optional_env_var(name: &EnvVarName) -> Result<Option<EnvVarValue>, EnvVarReadError> {
    match read_env_var(name) {
        Ok(value) => Ok(Some(value)),
        Err(EnvVarReadError::NotPresent { .. }) => Ok(None),
        Err(error) => Err(error),
    }
}

fn validate_env_var_name(name: &str) -> Result<(), EnvVarNameError> {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return Err(EnvVarNameError::Empty);
    }

    if bytes[0].is_ascii_digit() {
        return Err(EnvVarNameError::StartsWithDigit);
    }

    if bytes
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
    {
        Ok(())
    } else {
        Err(EnvVarNameError::InvalidCharacter)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EnvVarName, EnvVarNameError, EnvVarValue, TypedEnvVar, is_valid_env_var_name,
        read_optional_env_var,
    };

    #[test]
    fn validates_env_var_names() {
        assert!(is_valid_env_var_name("RUST_LOG"));
        assert!(is_valid_env_var_name("_RUSTUSE"));
        assert_eq!(EnvVarName::new(""), Err(EnvVarNameError::Empty));
        assert_eq!(
            EnvVarName::new("1RUST"),
            Err(EnvVarNameError::StartsWithDigit)
        );
        assert_eq!(
            EnvVarName::new("RUST-LOG"),
            Err(EnvVarNameError::InvalidCharacter)
        );
    }

    #[test]
    fn stores_owned_values_and_typed_names() -> Result<(), EnvVarNameError> {
        let name = EnvVarName::new("RUSTUSE_EXAMPLE")?;
        let typed = TypedEnvVar::<u16>::new(name.clone());
        let value = EnvVarValue::new("42");

        assert_eq!(typed.name(), &name);
        assert_eq!(value.as_str(), "42");
        Ok(())
    }

    #[test]
    fn optional_read_reports_missing_as_none() -> Result<(), Box<dyn std::error::Error>> {
        let name = EnvVarName::new("RUSTUSE_USE_CLI_TEST_SHOULD_NOT_EXIST_9B6AE5E0")?;

        assert_eq!(read_optional_env_var(&name)?, None);
        Ok(())
    }
}
