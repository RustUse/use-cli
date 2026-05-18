#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// Commonly used prompt primitives.
pub mod prelude {
    pub use crate::{
        is_no, is_yes, parse_confirmation, ConfirmationParseError, PromptText, PromptTextError,
        YesNoAnswer,
    };
}

/// Validation errors for prompt text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PromptTextError {
    /// The prompt text was empty or only whitespace.
    Empty,
}

impl fmt::Display for PromptTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("prompt text cannot be empty"),
        }
    }
}

impl std::error::Error for PromptTextError {}

/// Owned prompt text.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PromptText {
    text: String,
}

impl PromptText {
    /// Creates prompt text.
    ///
    /// # Errors
    ///
    /// Returns [`PromptTextError::Empty`] when `text` is empty or only whitespace.
    pub fn new(text: impl Into<String>) -> Result<Self, PromptTextError> {
        let text = text.into();
        if text.trim().is_empty() {
            Err(PromptTextError::Empty)
        } else {
            Ok(Self { text })
        }
    }

    /// Returns the prompt text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Returns the owned prompt text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.text
    }
}

impl AsRef<str> for PromptText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PromptText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

/// A primitive yes/no answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum YesNoAnswer {
    /// Affirmative answer.
    Yes,
    /// Negative answer.
    No,
}

impl YesNoAnswer {
    /// Returns whether this answer is affirmative.
    #[must_use]
    pub const fn is_yes(self) -> bool {
        matches!(self, Self::Yes)
    }

    /// Returns whether this answer is negative.
    #[must_use]
    pub const fn is_no(self) -> bool {
        matches!(self, Self::No)
    }
}

/// Errors returned while parsing confirmation answers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfirmationParseError {
    /// The answer was empty after trimming whitespace.
    Empty,
    /// The answer was not recognized as yes or no.
    Unrecognized,
}

impl fmt::Display for ConfirmationParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("confirmation answer cannot be empty"),
            Self::Unrecognized => formatter.write_str("confirmation answer must be yes or no"),
        }
    }
}

impl std::error::Error for ConfirmationParseError {}

/// Parses a yes/no confirmation answer.
///
/// # Errors
///
/// Returns [`ConfirmationParseError`] when `input` is empty or not a recognized yes/no answer.
pub fn parse_confirmation(input: &str) -> Result<YesNoAnswer, ConfirmationParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ConfirmationParseError::Empty);
    }

    match trimmed.to_ascii_lowercase().as_str() {
        "y" | "yes" => Ok(YesNoAnswer::Yes),
        "n" | "no" => Ok(YesNoAnswer::No),
        _ => Err(ConfirmationParseError::Unrecognized),
    }
}

/// Returns whether `input` parses as yes.
#[must_use]
pub fn is_yes(input: &str) -> bool {
    matches!(parse_confirmation(input), Ok(YesNoAnswer::Yes))
}

/// Returns whether `input` parses as no.
#[must_use]
pub fn is_no(input: &str) -> bool {
    matches!(parse_confirmation(input), Ok(YesNoAnswer::No))
}

#[cfg(test)]
mod tests {
    use super::{
        is_no, is_yes, parse_confirmation, ConfirmationParseError, PromptText, PromptTextError,
        YesNoAnswer,
    };

    #[test]
    fn validates_prompt_text() -> Result<(), PromptTextError> {
        let prompt = PromptText::new("Continue?")?;

        assert_eq!(prompt.as_str(), "Continue?");
        assert_eq!(PromptText::new("  "), Err(PromptTextError::Empty));
        Ok(())
    }

    #[test]
    fn parses_confirmation_answers() -> Result<(), ConfirmationParseError> {
        assert_eq!(parse_confirmation("yes")?, YesNoAnswer::Yes);
        assert_eq!(parse_confirmation(" N ")?, YesNoAnswer::No);
        assert!(is_yes("y"));
        assert!(is_no("no"));
        assert_eq!(
            parse_confirmation("maybe"),
            Err(ConfirmationParseError::Unrecognized)
        );
        Ok(())
    }
}
