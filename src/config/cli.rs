use clap::Arg;

pub struct CliArg;

impl CliArg {
    pub fn command() -> Arg<'static> {
        Arg::new("command")
            .value_name("COMMAND")
            .exclusive(true)
    }
    
    pub fn purge() -> Arg<'static> {
        Arg::new("purge")
            .long("purge")
            .about("Delete the spacework directory and history files")
    }

    pub fn new_language() -> Arg<'static> {
        Arg::new("language")
            .long("language")
            .short('l')
            .required(true)
            .takes_value(true)
    }

    pub fn new_name() -> Arg<'static> {
        Arg::new("name")
            .value_name("WORKSPACE NAME")
            .required(true)
            .takes_value(true)
            // .index(1)
    }
    
    pub fn history_all() -> Arg<'static> {
        Arg::new("all")
            .long("all")
            .short('a')
            .about("View all spacework history")
    }
}
