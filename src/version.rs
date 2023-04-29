use crate::{command::Command, PKG_NAME, PKG_VERSION};

/// A trait that supports for version rendering
///
/// # Example
///
/// ```
/// pub struct CustomVersionRender {}

/// impl VersionRender for CustomVersionRender {
///     fn version_text(&self, command: &Command) -> String {
///         format!("my custom version")
///     }
/// }
/// ```
pub trait VersionRender {
    /// Get version text
    ///
    /// # Parameters
    ///
    /// `command` - A `Command` that holds entire command information
    fn version_text(&self, command: &Command) -> String;
}

/// A that that supports for default version rendering
///
/// # Example
///
/// command version 1.0.0
pub struct DefaultVersionRender {}

impl DefaultVersionRender {
    /// Returns a `DefaultVersionRender` object
    pub fn new() -> Self {
        Self {}
    }
}

impl VersionRender for DefaultVersionRender {
    fn version_text(&self, _command: &Command) -> String {
        format!("{} version {}", PKG_NAME, PKG_VERSION)
    }
}
