mod config;
mod spacework;
use crate::spacework::history;
use crate::spacework::workspace::{self, Workspace};

use crate::config::runfile;
use crate::config::cli::CliArg;

use clap::App;

use std::error::Error;
use std::str::{self, FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new("Spacework: A workspace manager")
        .subcommand(
            App::new("new")
                .about("Create a new project")
                .arg(CliArg::new_name())
                .arg(CliArg::new_language())
        )
        .subcommand(
            App::new("build")
                .about("Compile source code into a binary")
        )
        .subcommand(
            App::new("history")
                .about("View previous spacework actions")
                .arg(CliArg::history_all())
                .arg(CliArg::history_count())
        )
        .arg(CliArg::command())
        .arg(CliArg::purge());

    let opts = app.get_matches_mut();

    if let Some(opts) = opts.subcommand_matches("new") {
        Workspace::from_options(
            opts.value_of("name"),
            opts.value_of("language"),
        )?;

        return Ok(());
    }

    if let Some(opts) = opts.subcommand_matches("history") {
        let history = history::History::new()?;

        if opts.is_present("all") {
            print!("{}", history.read_all()?);
        } else if let Some(count) = opts.value_of("count") {
            for line in history.read_last(usize::from_str(count)?)?.iter() {
                println!("{}", line);
            }
        }
        
        return Ok(());
    }

    if let Some(_opts) = opts.subcommand_matches("build") {
        let cmd = workspace::build()?;

        if cmd.status.success() {
            if let Ok(stdout) = str::from_utf8(&cmd.stdout) {
                println!("{}", stdout);
            }
        } else if let Ok(stderr) = str::from_utf8(&cmd.stderr) {
            eprintln!("`build` command exited with an error.\n");
            eprintln!("{}", stderr);
            eprintln!("{}", &cmd.status);
        }

        return Ok(());
    }

    if opts.is_present("purge") {
        workspace::delete_all()?;
        history::delete_history_file()?;
        println!(
            "Deleted spacework directory and .spacework_history file"
        );

        return Ok(());
    }

    if let Some(cmds) = opts.values_of("command") {
        for cmd in cmds {
            let output = runfile::run(cmd)?;
            if output.status.success() {
                if let Ok(stdout) = str::from_utf8(&output.stdout) {
                    print!("{}", stdout);
                }
            } else if let Ok(stderr) = str::from_utf8(&output.stderr) {
                eprintln!("`{}` exited with an error.\n", cmd);
                eprintln!("{}", stderr);
                eprintln!("{}", output.status);
            }
        }
    } else {
        // If no commands are given, we show help.
        // Also see `App.print_long_help()?;`
        app.print_help()?;
    }

    Ok(())
}
