pub mod command;
pub mod flag;
pub mod help;
pub mod util;
pub mod version;

use std::env;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
