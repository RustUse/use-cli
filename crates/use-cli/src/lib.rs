#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Facade for `RustUse` CLI-adjacent primitive crates.

pub use use_arg as arg;
pub use use_command_name as command_name;
pub use use_env_var as env_var;
pub use use_exit_code as exit_code;
pub use use_flag as flag;
pub use use_option as option;
pub use use_prompt as prompt;
pub use use_stderr as stderr;
pub use use_stdin as stdin;
pub use use_stdout as stdout;
pub use use_terminal as terminal;

/// Commonly used CLI primitive types from the focused crates.
pub mod prelude {
    pub use crate::arg::{Arg, RawArgs};
    pub use crate::command_name::{CommandName, ExecutableName};
    pub use crate::env_var::{EnvVarName, EnvVarValue};
    pub use crate::exit_code::{CONFIG_ERROR, ExitCode, FAILURE, SUCCESS, USAGE_ERROR};
    pub use crate::flag::{BooleanFlag, Flag, LongFlag, ShortFlag};
    pub use crate::option::{CliOption, CliOptionName, OptionValue};
    pub use crate::prompt::{PromptText, YesNoAnswer};
    pub use crate::stderr::StderrDestination;
    pub use crate::stdin::StdinSource;
    pub use crate::stdout::{NewlineBehavior, StdoutDestination};
    pub use crate::terminal::{
        ColorSupport, Interactivity, TerminalHeight, TerminalSize, TerminalWidth,
    };
}

#[cfg(test)]
mod tests {
    use super::{arg, command_name, exit_code, flag, option, terminal};

    #[test]
    fn facade_exposes_cli_primitives() -> Result<(), Box<dyn std::error::Error>> {
        let command = command_name::CommandName::new("rustuse")?;
        let verbose = flag::BooleanFlag::enabled(flag::Flag::try_from_token("--verbose")?);
        let output = option::CliOption::from_name_value("format", "json")?;
        let input = arg::Arg::from("README.md");
        let size = terminal::TerminalSize::try_new(80, 24)?;

        assert_eq!(command.as_str(), "rustuse");
        assert!(verbose.is_enabled());
        assert_eq!(output.to_equals_token(), "--format=json");
        assert!(input.is_positional());
        assert_eq!(exit_code::SUCCESS.as_i32(), 0);
        assert_eq!(size.width().columns(), 80);
        Ok(())
    }
}
