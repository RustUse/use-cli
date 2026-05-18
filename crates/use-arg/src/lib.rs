#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, slice};

/// Commonly used argument primitives.
pub mod prelude {
    pub use crate::{Arg, RawArgs, is_flag_like, is_option_separator, is_positional};
}

/// An owned command-line argument token.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Arg {
    value: String,
}

impl Arg {
    /// Creates an owned argument token.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the borrowed token string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the owned token string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }

    /// Returns whether this token is the `--` option separator.
    #[must_use]
    pub fn is_option_separator(&self) -> bool {
        is_option_separator(&self.value)
    }

    /// Returns whether this token looks like a flag or option token.
    #[must_use]
    pub fn is_flag_like(&self) -> bool {
        is_flag_like(&self.value)
    }

    /// Returns whether this token looks positional.
    #[must_use]
    pub fn is_positional(&self) -> bool {
        is_positional(&self.value)
    }
}

impl AsRef<str> for Arg {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for Arg {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

/// A raw owned argument collection.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RawArgs {
    args: Vec<Arg>,
}

impl RawArgs {
    /// Creates a raw argument collection from owned tokens.
    #[must_use]
    pub const fn new(args: Vec<Arg>) -> Self {
        Self { args }
    }

    /// Creates an empty raw argument collection.
    #[must_use]
    pub const fn empty() -> Self {
        Self { args: Vec::new() }
    }

    /// Adds an argument token to the end of the collection.
    pub fn push(&mut self, arg: Arg) {
        self.args.push(arg);
    }

    /// Returns the number of stored tokens.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.args.len()
    }

    /// Returns whether the collection has no tokens.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    /// Returns a borrowed iterator over stored tokens.
    pub fn iter(&self) -> slice::Iter<'_, Arg> {
        self.args.iter()
    }

    /// Returns the underlying owned vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<Arg> {
        self.args
    }
}

impl FromIterator<Arg> for RawArgs {
    fn from_iter<T: IntoIterator<Item = Arg>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl FromIterator<String> for RawArgs {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        iter.into_iter().map(Arg::from).collect()
    }
}

impl IntoIterator for RawArgs {
    type Item = Arg;
    type IntoIter = std::vec::IntoIter<Arg>;

    fn into_iter(self) -> Self::IntoIter {
        self.args.into_iter()
    }
}

impl<'a> IntoIterator for &'a RawArgs {
    type Item = &'a Arg;
    type IntoIter = slice::Iter<'a, Arg>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Returns whether a token is the `--` option separator.
#[must_use]
pub fn is_option_separator(token: &str) -> bool {
    token == "--"
}

/// Returns whether a token looks like a flag or option token.
#[must_use]
pub fn is_flag_like(token: &str) -> bool {
    token.starts_with('-') && token != "-" && !is_option_separator(token)
}

/// Returns whether a token looks positional.
#[must_use]
pub fn is_positional(token: &str) -> bool {
    !is_option_separator(token) && !is_flag_like(token)
}

#[cfg(test)]
mod tests {
    use super::{Arg, RawArgs, is_flag_like, is_option_separator, is_positional};

    #[test]
    fn classifies_argument_tokens() {
        assert!(is_positional("file.txt"));
        assert!(is_positional("-"));
        assert!(is_option_separator("--"));
        assert!(is_flag_like("-v"));
        assert!(is_flag_like("--verbose"));
        assert!(!is_flag_like("--"));
    }

    #[test]
    fn stores_owned_arguments() {
        let mut args = RawArgs::empty();
        args.push(Arg::from("tool"));
        args.push(Arg::from("README.md"));

        let tokens: Vec<_> = args.iter().map(Arg::as_str).collect();

        assert_eq!(args.len(), 2);
        assert_eq!(tokens, vec!["tool", "README.md"]);
        assert!(args.into_vec()[1].is_positional());
    }
}
