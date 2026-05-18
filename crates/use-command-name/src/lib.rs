#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::path::Path;

/// Commonly used command name primitives.
pub mod prelude {
    pub use crate::{
        CommandName, CommandNameError, ExecutableName, executable_name_from_path,
        is_valid_command_name,
    };
}

/// Validation errors for command and executable names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandNameError {
    /// The name was empty.
    Empty,
    /// The name contained a path separator.
    ContainsSeparator,
    /// The name contained a control character.
    InvalidCharacter,
    /// A path component could not be represented as Unicode.
    NonUnicode,
}

impl fmt::Display for CommandNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("command name cannot be empty"),
            Self::ContainsSeparator => {
                formatter.write_str("command name cannot contain path separators")
            },
            Self::InvalidCharacter => {
                formatter.write_str("command name cannot contain control characters")
            },
            Self::NonUnicode => formatter.write_str("executable name is not valid Unicode"),
        }
    }
}

impl std::error::Error for CommandNameError {}

/// A validated command name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandName {
    name: String,
}

impl CommandName {
    /// Creates a validated command name.
    ///
    /// # Errors
    ///
    /// Returns [`CommandNameError`] when `name` is empty, contains path separators, or contains
    /// control characters.
    pub fn new(name: impl Into<String>) -> Result<Self, CommandNameError> {
        let name = name.into();
        validate_command_name(&name)?;
        Ok(Self { name })
    }

    /// Returns the command name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Returns the owned command name.
    #[must_use]
    pub fn into_string(self) -> String {
        self.name
    }
}

impl AsRef<str> for CommandName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CommandName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// A validated executable or binary name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExecutableName {
    command_name: CommandName,
}

impl ExecutableName {
    /// Creates a validated executable name.
    ///
    /// # Errors
    ///
    /// Returns [`CommandNameError`] when `name` is not a valid command name.
    pub fn new(name: impl Into<String>) -> Result<Self, CommandNameError> {
        Ok(Self {
            command_name: CommandName::new(name)?,
        })
    }

    /// Extracts a validated executable name from a path.
    ///
    /// # Errors
    ///
    /// Returns [`CommandNameError`] when the path has no file name, the file name is not Unicode,
    /// or the file name is not a valid command name.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, CommandNameError> {
        executable_name_from_path(path)
    }

    /// Returns the executable name as a command name.
    #[must_use]
    pub const fn command_name(&self) -> &CommandName {
        &self.command_name
    }

    /// Returns the display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        self.command_name.as_str()
    }

    /// Returns the owned executable name.
    #[must_use]
    pub fn into_command_name(self) -> CommandName {
        self.command_name
    }
}

impl fmt::Display for ExecutableName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.display_name())
    }
}

/// Returns whether `name` is valid for this crate's command name primitive.
#[must_use]
pub fn is_valid_command_name(name: &str) -> bool {
    validate_command_name(name).is_ok()
}

/// Extracts a validated executable name from a path.
///
/// # Errors
///
/// Returns [`CommandNameError`] when the path has no file name, the file name is not Unicode,
/// or the file name is not a valid command name.
pub fn executable_name_from_path(
    path: impl AsRef<Path>,
) -> Result<ExecutableName, CommandNameError> {
    let name = path
        .as_ref()
        .file_name()
        .ok_or(CommandNameError::Empty)?
        .to_str()
        .ok_or(CommandNameError::NonUnicode)?;

    ExecutableName::new(name)
}

fn validate_command_name(name: &str) -> Result<(), CommandNameError> {
    if name.is_empty() {
        return Err(CommandNameError::Empty);
    }

    if name.contains('/') || name.contains('\\') {
        return Err(CommandNameError::ContainsSeparator);
    }

    if name.chars().any(char::is_control) {
        return Err(CommandNameError::InvalidCharacter);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        CommandName, CommandNameError, ExecutableName, executable_name_from_path,
        is_valid_command_name,
    };

    #[test]
    fn validates_command_names() {
        assert!(is_valid_command_name("rustuse"));
        assert!(is_valid_command_name("rustuse.exe"));
        assert!(!is_valid_command_name(""));
        assert!(!is_valid_command_name("bin/rustuse"));
        assert_eq!(
            CommandName::new("bin/rustuse"),
            Err(CommandNameError::ContainsSeparator)
        );
    }

    #[test]
    fn stores_command_and_executable_names() -> Result<(), CommandNameError> {
        let command = CommandName::new("rustuse")?;
        let executable = ExecutableName::new("rustuse.exe")?;

        assert_eq!(command.as_str(), "rustuse");
        assert_eq!(executable.display_name(), "rustuse.exe");
        Ok(())
    }

    #[test]
    fn extracts_executable_name_from_path() -> Result<(), CommandNameError> {
        let executable = executable_name_from_path("target/debug/rustuse")?;

        assert_eq!(executable.display_name(), "rustuse");
        Ok(())
    }
}
