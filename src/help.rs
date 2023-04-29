use crate::command::Command;

/// A trait that supports for help rendering
///
/// # Example
///
/// ```
/// pub struct CustomHelpRender {}

/// impl HelpRender for CustomHelpRender {
///     fn help_text(&self, command: &Command) -> String {
///         format!("my custom help")
///     }
/// }
/// ```
pub trait HelpRender {
    fn help_text(&self, command: &Command) -> String;
}

/// A type that supports for default help rendering
///
/// # Example
///
/// command description
///
/// Usage:
///
///       command TEXT
///
/// Flags:
///
///       -h, --help        help for command
///       -v, --version     version for command
pub struct DefaultHelpRender {}

impl DefaultHelpRender {
    /// Returns a `DefaultHelpRender` object
    pub fn new() -> Self {
        Self {}
    }
}

impl HelpRender for DefaultHelpRender {
    fn help_text(&self, command: &Command) -> String {
        let mut text = String::from("");
        text.push_str(&format!("{}\n", command.description));
        text.push_str(&String::from("\n"));
        text.push_str(&String::from("Usage:\n"));
        text.push_str(&format!("  {}\n", command.usage));
        text.push_str(&String::from("\n"));
        text.push_str(&String::from("Flags:\n"));
        for flag in command.flags.iter() {
            text.push_str(&format!("{}\n", flag));
        }

        text
    }
}
