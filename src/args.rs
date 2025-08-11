use std::{
    env::{args, Args},
    process,
};

use derive_builder::Builder;

const MAX_WIDTH: i32 = 32;

#[derive(Builder)]
pub struct Arguments {
    #[builder(default, setter(custom))]
    pub annotation: Option<String>,
    #[builder(default = "Passphrase".to_string())]
    pub prompt: String,

    #[builder(default, setter(strip_option))]
    pub error: Option<String>,
    #[builder(default, setter(strip_option))]
    pub attempt: Option<(u8, u8)>,

    #[builder(default = 70)]
    pub max_width: i32,
}

impl Arguments {
    pub fn parse_args(args: &mut Args) -> Self {
        let args: Vec<_> = args.collect();

        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
            Self::print_help();
        }

        let mut builder = ArgumentsBuilder::create_empty();

        let mut args = args.into_iter().peekable();
        while let (Some(key), Some(val)) = (args.next(), args.peek().cloned()) {
            match key.as_str() {
                "--annotate" | "-a" => builder.annotation(val),
                "--prompt" | "-p" => {
                    builder.prompt(val);
                }
                "--attempt" | "-A" => {
                    if val.contains('/') {
                        let mut iter = val.split('/');
                        let [curr, max] = [iter.next().unwrap(), iter.next().unwrap()];

                        builder.attempt((
                            curr.parse().expect("Passed invalid number"),
                            max.parse().expect("Parsed invalid number"),
                        ));
                    } else {
                        builder.attempt((val.parse().expect("Passed invalid number"), 0));
                    }
                }
                "--error" | "-e" => {
                    builder.error(val);
                }
                "--max-width" | "-w" => {
                    builder.max_width(val.parse().expect("Invalid number"));
                }
                key if key.starts_with('-') => println!("Unregognized flag: {key}"),
                _ => (),
            }
        }

        builder.build().expect("Builder should be infallable")
    }

    const HELP_MSG: &str = r#"askpass-curses: a minimal tool to prompt for secure input

USAGE: askpass-curses [--help|-h] [OPTIONS...]

OPTIONS:
         --help, -h: Shows this help message
     --annotate, -a: Add a line to the annotation shown for the prompt (e.g. '-a Enter password for user:')
       --prompt, -p: The label to use for the input (default. '-p Passphrase')
      --attempt, -A: The current log in attempt and attempt limits, either 0 or CURR/MAX (default. '-A 0')
        --error, -e: An error message to show potentially before the attempt counter
                     (default w/ attempt. '-e Bad Passphrase')
    --max-width, -w: The maximum width in columns the prompt is allowed to take up (default. '-w 70')"#;

    fn print_help() -> ! {
        println!("{}", Self::HELP_MSG);
        process::exit(0);
    }
}

impl ArgumentsBuilder {
    fn annotation(&mut self, value: String) {
        if self.annotation.is_none() {
            self.annotation = Some(Some(value));
            return;
        }

        let Some(Some(annotation)) = self.annotation.as_mut() else {
            unreachable!()
        };

        annotation.push('\n');
        annotation.push_str(&value);
    }
}
