use std::env;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A command type
pub struct Command {
    description: String,
    usage: String,
    flags: Vec<Flag>,
    run: fn(Vec<&str>),
}

/// A command flag type
pub struct Flag {
    short: String,
    long: String,
    description: String,
    has: bool,
}

impl Command {
    pub fn new(description: String, usage: String, run: fn(Vec<&str>)) -> Self {
        let mut command = Command {
            description,
            usage,
            run,
            flags: vec![],
        };
        command.add_flag(
            String::from("h"),
            String::from("help"),
            format!("help for {}", NAME),
        );
        command.add_flag(
            String::from("v"),
            String::from("version"),
            format!("version for {}", NAME),
        );

        command
    }

    pub fn add_flag(&mut self, short: String, long: String, description: String) {
        self.flags.push(Flag {
            short,
            long,
            description,
            has: false,
        });
    }
}

impl Command {
    pub fn execute(&mut self) {
        let args: Vec<String> = env::args().collect();
        let mut args: Vec<&str> = args.iter().map(|x| &x[..]).collect();
        if args.len() <= 1 {
            args.push("--help");
        }

        for i in 0..args.len() {
            let arg = args[i];
            if !(arg.starts_with("-") || arg.starts_with("--")) {
                continue;
            }
            for mut flag in self.flags.iter_mut() {
                if *arg == format!("-{}", flag.short) || *arg == format!("--{}", flag.long) {
                    flag.has = true;
                    args.remove(i);
                }
            }
        }

        for flag in self.flags.iter() {
            if flag.has {
                if flag.short == "h" {
                    self.help();
                    return;
                }
                if flag.short == "v" {
                    self.version();
                    return;
                }
            }
        }

        args.remove(0);
        (self.run)(args);
    }
}

impl Command {
    pub fn help(&self) {
        println!("{}", self.description);
        println!();
        println!("Usage:");
        println!("  {}", self.usage);
        println!();
        println!("Flags:");
        for flag in self.flags.iter() {
            println!("  -{}, --{}\t{}", flag.short, flag.long, flag.description)
        }
    }

    pub fn version(&self) {
        println!("{} version {}", NAME, VERSION);
    }
}
