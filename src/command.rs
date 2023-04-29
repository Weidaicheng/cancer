use crate::{
    help::{DefaultHelpRender, HelpRender},
    util::get_args,
    version::{DefaultVersionRender, VersionRender},
    PKG_NAME,
};

use super::flag::{is_flag, Flag, FlagValue};

const HELP_SHORT: &str = "h";
const HELP_LONG: &str = "help";
const VERSION_SHORT: &str = "v";
const VERSION_LONG: &str = "version";

/// A command type
pub struct Command {
    /// Command description
    ///
    /// # Example
    ///
    /// gives a friendly hello
    pub description: String,
    /// Command usage
    ///
    /// # Example
    ///
    /// hello TEXt
    pub usage: String,
    /// Command flags
    pub flags: Vec<Flag>,
    /// Command execution logic
    ///
    /// # Arguments
    ///
    /// `text` - An optional string that holds input text
    /// `flags` - A vector of Flag that holds added flags without help or version
    ///
    /// # Example
    ///
    /// ```
    /// |text, flags| { println!("hello, {}!", text.unwrap()); }
    /// ```
    run: fn(text: Option<String>, flags: Vec<&Flag>),
    /// Help render which is a `HelpRender` trait that supports for rendering help information
    help_render: Box<dyn HelpRender>,
    /// Version render which is a `VersionRender` trait that supports for rendering version information
    version_render: Box<dyn VersionRender>,
}

impl Command {
    /// Returns a command with the arguments provided and help and version flags, also with default help render and version render
    ///
    /// # Arguments
    ///
    /// * `description` - A string slice that holds the description of the command
    /// * `usage` - A string slice that holds the usage of the command
    /// * `run` - A function with an optional string parameter that holds the logic of the command, this function will be called on command execution
    ///
    /// # Examples
    ///
    /// ```
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
    ///     println!("hello, {}!", text.unwrap());
    /// });
    /// ```
    pub fn new(description: &str, usage: &str, run: fn(Option<String>, Vec<&Flag>)) -> Self {
        let mut command = Self {
            description: String::from(description),
            usage: String::from(usage),
            run,
            flags: vec![],
            help_render: Box::new(DefaultHelpRender::new()),
            version_render: Box::new(DefaultVersionRender::new()),
        };
        command.add_flag(Flag::new_bool(
            HELP_SHORT,
            HELP_LONG,
            &format!("help for {}", PKG_NAME),
        ));
        command.add_flag(Flag::new_bool(
            VERSION_SHORT,
            VERSION_LONG,
            &format!("version for {}", PKG_NAME),
        ));

        command
    }
}

impl Command {
    /// Add a new flag to command
    ///
    /// # Arguments
    ///
    /// `flag` - A `Flag` object
    ///
    /// # Examples
    ///
    /// ```
    /// command.add_flag(Flag::new_bool("f", "ferris", "say hello from ferris"));
    /// ```
    pub fn add_flag(&mut self, flag: Flag) {
        self.flags.push(flag);
    }
}

impl Command {
    /// Execute command
    ///
    /// # Example
    ///
    /// ```
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
    ///     todo!();
    /// });
    /// command.execute();
    /// ```
    pub fn execute(&mut self) {
        let args = get_args();

        let args = self.update_flags(args.iter().map(|x| &x[..]).collect());

        if self.help_exit() {
            return;
        }
        if self.version_exit() {
            return;
        }

        if args.len() <= 1 {
            println!("{}", self.help_render.help_text(self));
            return;
        }

        let input = &args[1];
        let flags = self.get_flags();
        (self.run)(Some(String::from(input)), flags);
    }

    /// Update flags value by giving args and returns simple args vector without any flag
    ///
    /// # Arguments
    ///
    /// `args` - A vector of string slice that holds arguments
    ///
    /// # Return
    ///
    /// A vector of string that without any flag from args
    ///
    /// # Example
    ///
    /// ```
    /// let args = vec!["target/debug/hello", "-f", "world"];
    /// let args = self.set_flags(args);
    /// dbg!(&args);
    /// // output:
    /// // [src/command.rs:95] &args = [
    /// //     "target/debug/hello",
    /// //     "world",
    /// // ]
    /// ```
    fn update_flags(&mut self, args: Vec<&str>) -> Vec<String> {
        let mut simple_args: Vec<String> = vec![];

        for arg in args {
            if !(is_flag(arg)) {
                simple_args.push(String::from(arg));
                continue;
            }
            for mut flag in self.flags.iter_mut() {
                if flag.is_match(arg) {
                    flag.value = FlagValue::Bool(true);
                }
            }
        }

        simple_args
    }

    /// Returns added flags without help or version
    ///
    /// # Example
    ///
    /// ```
    /// // returns empty flag vector when not providing any flag
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
    ///     todo!();
    /// });
    /// let flags = command.get_flags();
    /// assert_eq!(0, flags.len());
    ///
    /// // returns non empty flag vector when providing flag
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
    ///     todo!();
    /// });
    /// command.add_flag("f", "ferris", "say hello from ferris");
    /// let flags = command.get_flags();
    /// assert_eq!(1, flags.len());
    /// ```
    fn get_flags(&self) -> Vec<&Flag> {
        let mut simple_flags: Vec<&Flag> = vec![];

        for flag in self.flags.iter() {
            if !(flag.short == HELP_SHORT || flag.short == VERSION_SHORT) {
                simple_flags.push(flag);
            }
        }

        simple_flags
    }

    /// Check if help needed to display and exit,
    ///
    /// by if arguments contains `-h` or `--help`
    fn help_exit(&self) -> bool {
        let mut exit = false;

        for flag in self.flags.iter() {
            if flag.short == HELP_SHORT {
                match flag.value {
                    FlagValue::Bool(value) => {
                        if value {
                            println!("{}", self.help_render.help_text(self));
                            exit = true;
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }

        exit
    }

    /// Check if version needed to display and exit,
    ///
    /// by if arguments contains `-v` or `--version`
    fn version_exit(&self) -> bool {
        let mut exit = false;

        for flag in self.flags.iter() {
            if flag.short == VERSION_SHORT {
                match flag.value {
                    FlagValue::Bool(value) => {
                        if value {
                            println!("{}", self.version_render.version_text(self));
                            exit = true;
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }

        exit
    }
}
