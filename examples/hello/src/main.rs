use std::io::{stdout, BufWriter};

use ferris_says::say;
use cancer::{command::Command, flag::FlagValue};

fn main() {
    let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
        let mut use_ferris = false;

        for flag in flags {
            if flag.is_match("-f") {
                match flag.value {
                    FlagValue::Bool(value) => {
                        use_ferris = value;
                    }
                    _ => (),
                }
            }
        }

        let message = format!("hello, {}!", text.unwrap());
        if use_ferris {
            let stdout = stdout();
            let width = message.chars().count();

            let mut writer = BufWriter::new(stdout.lock());
            say(message.as_str(), width, &mut writer).unwrap();
        } else {
            println!("{}", message);
        }
    });
    command.add_boolean_flag("f", "ferris", "say hello from ferris");
    command.execute();
}
