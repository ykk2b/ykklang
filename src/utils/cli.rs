// TODO: handle parsing cli commands

use clap::{Arg, ArgGroup, Command, ValueHint};

pub struct CLI {
    pub run: Option<String>,
}

impl CLI {
    pub fn new() -> CLI {
        let matches = Command::new("YKKLanguage")
            .author("ykk2b")
            .version("0.0.1-alpha.0")
            .about("Minimalistic programming language")
            .group(ArgGroup::new("main"))
            .arg(
                Arg::new("run")
                    .short('r')
                    .long("run")
                    .aliases(["dev", "start"])
                    .value_name("FILE")
                    .help("Interpret the FILE")
                    .value_hint(ValueHint::FilePath)
                    .display_order(0)
                    .group("main"),
            )
            .get_matches();

        CLI {
            run: matches
                .clone()
                .get_one::<String>("run")
                .map(|s| s.to_string()),
        }
    }

    pub fn run(&self) -> Option<String> {
        match self.run {
            Some(_) => self.run.clone(),
            None => None,
        }
    }
}
