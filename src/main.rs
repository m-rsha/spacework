mod spacework;
use crate::spacework::workspace::{self, Workspace};

mod config;
// use crate::spacework::history;

// mod config;
// use crate::config::spacework as spaceworkcfg;

use clap::{App, Arg};
use std::{str, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new("Spacework: A workspace manager")
        .subcommand(
            App::new("new")
            .arg(
                Arg::new("name")
                    .value_name("WORKSPACE NAME")
                    .required(true)
                    .index(1)
                    .takes_value(true)
            )
            .arg(
                Arg::new("language")
                    .long("language")
                    .short('l')
                    .takes_value(true)
                    .min_values(0)
                    .max_values(1)
            )
        )
        .subcommand(
            App::new("build")
            .arg(
                Arg::new("release")
                    .long("release")
                    .takes_value(false)
                    .required(false)
            )
        )
        .arg(
            Arg::new("history")
                .long("history")
                .takes_value(false)
                .required(false)
        );

    let opts = app.get_matches_mut();

    if let Some(opts) = opts.subcommand_matches("new") {
        Workspace::from_options(
            opts.value_of("name"),
            opts.value_of("language"),
        )?;
        return Ok(());
    }

    if let Some(_opts) = opts.subcommand_matches("build") {
        let cmd = workspace::build()?;

        if let Ok(stdout) = str::from_utf8(&cmd.stdout) {
            println!("{}", stdout);
        }
        if let Ok(stderr) = str::from_utf8(&cmd.stderr) {
            eprintln!("{}", stderr);
        }
        return Ok(());
    }

    if opts.is_present("history") {
        // history::read()?;
        return Ok(());
    }
    
    // If no commands are given, we show help.
    // Also see `App.print_long_help()?`
    // app.print_help()?;

    // workspace::find_src_file()?;
    
    Ok(())
}
