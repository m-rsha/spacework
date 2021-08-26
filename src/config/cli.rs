use clap::Arg;

pub struct CliArg;

impl CliArg {
    pub fn command() -> Arg<'static> {
        Arg::new("command").value_name("COMMAND").exclusive(true)
    }

    pub fn purge() -> Arg<'static> {
        Arg::new("purge")
            .about("Delete the spacework directory and history files")
            .long("purge")
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
            .about("View all spacework history")
            .long("all")
            .short('a')
            .required_unless_present("count")
    }

    pub fn history_count() -> Arg<'static> {
        Arg::new("count")
            .about("View last N spacework commands used")
            .long("count")
            .short('c')
            .value_name("N")
            .takes_value(true)
            .required_unless_present("all")
    }
}
