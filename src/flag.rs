use std::fmt;

pub const FLAG_SHORT_START: &str = "-";
pub const FLAG_LONG_START: &str = "--";

/// An enum that represents for flag value, which includes boolean, string, int and float
///
/// # Example
///
/// ```
/// let value = FlagValue::Bool(true);
/// ```
#[derive(Debug)]
pub enum FlagValue {
    /// An boolean enum that represents for `bool` flag value
    ///
    /// # Example
    ///
    /// ```
    /// let value = FlagValue::Bool(true);
    /// ```
    Bool(bool),
    /// A string enum that represents for `String` flag value
    ///
    /// # Example
    ///
    /// ```
    /// let value = FlagValue::String(Some(String::from("Hello, world!")));
    /// ```
    String(Option<String>),
    /// An int enum that represents for `i32` flag value
    ///
    /// # Example
    ///
    /// ```
    /// let value = FlagValue::Int(Some(3));
    /// ```
    Int(Option<i32>),
    /// A float enum that represents for `f32` flag value
    ///
    /// # Example
    ///
    /// ```
    /// let value = FlagValue::Float(Some(2.7));
    /// ```
    Float(Option<f32>),
}

/// A command flag type
#[derive(Debug)]
pub struct Flag {
    /// Flag short identifier
    ///
    /// # Example
    ///
    /// f
    pub short: String,
    /// Flag long identifier
    ///
    /// # Example
    ///
    /// ferris
    pub long: String,
    /// Flag description
    ///
    /// # Example
    ///
    /// say hello from ferris
    pub description: String,
    /// Flag value
    ///
    /// # Default values
    ///
    /// `FlagValue::Bool` - default value is `false`
    ///
    /// `FlagValue::String` - default value is `""`
    ///
    /// `FlagValue::Int` - default value is `0`
    ///
    /// `FlagValue::Float` - default value is `0.0`
    pub value: FlagValue,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "  {}{}, {}{}\t{}",
            FLAG_SHORT_START, self.short, FLAG_LONG_START, self.long, self.description
        ))
    }
}

impl Flag {
    /// Returns a flag with the arguments provided
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier
    ///
    /// `long` - A string slice that holds the long identifier
    ///
    /// `description` - A string slice that holds the description
    ///
    /// `value` - An enum that holds the value
    ///
    /// # Examples
    ///
    /// ```
    /// let flag = Flag::new("f", "ferris", "say hello from ferris", FlagValue::Bool(false));
    /// ```
    fn new(short: &str, long: &str, description: &str, value: FlagValue) -> Self {
        Self {
            short: String::from(short),
            long: String::from(long),
            description: String::from(description),
            value,
        }
    }

    /// Returns a flag with bool(false by default) value and the arguments provided
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier
    ///
    /// `long` - A string slice that holds the long identifier
    ///
    /// `description` - A string slice that holds the description
    ///
    /// # Examples
    ///
    /// ```
    /// let flag = Flag::new_bool("f", "ferris", "say hello from ferris");
    /// ```
    pub fn new_bool(short: &str, long: &str, description: &str) -> Self {
        Flag::new(short, long, description, FlagValue::Bool(false))
    }

    /// Returns a flag with string(None by default) value and the arguments provided
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier
    ///
    /// `long` - A string slice that holds the long identifier
    ///
    /// `description` - A string slice that holds the description
    ///
    /// # Examples
    ///
    /// ```
    /// let flag = Flag::new_string("f", "ferris", "say hello from ferris");
    /// ```
    pub fn new_string(short: &str, long: &str, description: &str) -> Self {
        Flag::new(short, long, description, FlagValue::String(None))
    }

    /// Returns a flag with int(None by default) value and the arguments provided
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier
    ///
    /// `long` - A string slice that holds the long identifier
    ///
    /// `description` - A string slice that holds the description
    ///
    /// # Examples
    ///
    /// ```
    /// let flag = Flag::new_int("f", "ferris", "say hello from ferris");
    /// ```
    pub fn new_int(short: &str, long: &str, description: &str) -> Self {
        Flag::new(short, long, description, FlagValue::Int(None))
    }

    /// Returns a flag with float(None by default) value and the arguments provided
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier
    ///
    /// `long` - A string slice that holds the long identifier
    ///
    /// `description` - A string slice that holds the description
    ///
    /// # Examples
    ///
    /// ```
    /// let flag = Flag::new_float("f", "ferris", "say hello from ferris");
    /// ```
    pub fn new_float(short: &str, long: &str, description: &str) -> Self {
        Flag::new(short, long, description, FlagValue::Float(None))
    }
}

impl Flag {
    /// Returns if provided arg match flag
    ///
    /// # Arguments
    ///
    /// `arg` - A string slice that holds argument that needs to check
    ///
    /// # Examples
    ///
    /// ```
    /// // matched scenario
    /// let flag = Flag::new("f", "ferris", "say hello from ferris", FlagValue::Bool(false));
    /// let result = flag.is_match("-f");
    ///
    /// assert_eq!(true, result);
    ///
    /// // non-matched scenario
    /// let flag = Flag::new("f", "ferris", "say hello from ferris", FlagValue::Bool(false));
    /// let result = flag.is_match("--rollercoaster");
    ///
    /// assert_eq!(false, result);
    /// ```
    pub fn is_match(&self, arg: &str) -> bool {
        arg == format!("{}{}", FLAG_SHORT_START, self.short)
            || arg == format!("{}{}", FLAG_LONG_START, self.long)
    }
}

/// Returns if provided arg is a flag
///
/// # Arguments
///
/// `arg` - A string slice that holds argument that needs to check
///
/// # Examples
///
/// ```
/// // flag scenario
/// let result = is_flag("-f");
///
/// assert_eq!(true, result);
///
/// // not flag scenario
/// let result = is_flag("f");
///
/// assert_eq!(false, result);
/// ```
pub fn is_flag(arg: &str) -> bool {
    arg.starts_with(FLAG_SHORT_START) || arg.starts_with(FLAG_LONG_START)
}
