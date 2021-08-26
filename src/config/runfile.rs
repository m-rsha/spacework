use serde::Deserialize;

use std::error::Error;
use std::process::{Command, Output};

#[derive(Debug, Deserialize)]
struct RunFile {
    cmd: Vec<Cmd>,
}

#[derive(Debug, Deserialize)]
struct Cmd {
    name: String,
    bin: String,
    args: String,
}

pub fn run(command: &str) -> Result<Output, Box<dyn Error>> {
    let runfile: RunFile =
        toml::from_str(include_str!("../../runfiles/example.toml"))?;

    match runfile.cmd.iter().find(|&c| c.name == command) {
        Some(cmd) => Ok(Command::new(&cmd.bin)
            .args(cmd.args.split_whitespace())
            .output()?),
        None => Err(format!("`{}` not found in runfile", command).into()),
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floops_n_bloops() {
        assert!(true);
    }
}
*/
