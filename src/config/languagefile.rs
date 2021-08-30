use serde::Deserialize;

use std::collections::HashMap;
use std::error::Error;
use std::process::{Command, Output};
use std::str;

#[derive(Debug, Deserialize)]
pub struct LanguageFile {
    pub language: Language,
    pub cmd: Cmd,
    pub workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub name: String,
    pub aliases: Vec<String>,
    pub compiler: String,
    pub output: String,
    pub standard: Option<String>,

    pub extensions: Vec<String>,
    pub optimization_levels: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub dir: String,
    pub src: String,
}

#[derive(Debug, Deserialize)]
pub struct Cmd {
    pub build: String,
    pub run: String,
}

const LANGFILES: [&str; 2] = [
    include_str!("../../langs/c.toml"),
    include_str!("../../langs/cpp.toml"),
];

const TEMPLATES: [&str; 2] = [
    include_str!("../../langs/templates/main.c"),
    include_str!("../../langs/templates/main.cpp"),
];

const TEMPLATE_SRC: [&str; 2] = [
    "main.c",
    "main.cpp",
];

impl LanguageFile {
    pub fn from_language(lang_name: &str) -> Result<Self, Box<dyn Error>> {
        let lang_name = lang_name.to_lowercase();
        let langfiles: Vec<LanguageFile> = Self::langfiles()?;
        for langfile in langfiles {
            if langfile.language.aliases.contains(&lang_name)
                || langfile.language.name == lang_name
            {
                return Ok(langfile);
            }
        }

        Err(
            format!(
                "Language file not found for `{}`. Check your spelling \
                or consider creating one in your spacework directory.",
                lang_name
            ).into()
        )
    }

    pub fn template(&self) -> Result<&'static str, Box<dyn Error>> {
        let templates: HashMap<String, &str> = TEMPLATE_SRC
            .iter()
            .map(|e| e.to_string())
            .zip(TEMPLATES)
            .collect();

        if let Some(v) = templates.get(&self.workspace.src) {
            Ok(v)
        } else {
            Err("Unable to find matching template file.".into())
        }
    }


    pub fn build(&self) -> Result<Output, Box<dyn Error>> {
        let mut outfile = self.workspace.src.clone();

        for ext in self.language.extensions.iter() {
            if let Some(stripped) =
                outfile.strip_suffix(format!(".{}", ext).as_str())
            {
                outfile = stripped.to_string();
            }
        }

        if outfile == self.workspace.src {
            return Err(format!(
                "Unable to find matching file extension for `{}`.",
                outfile
            )
            .into());
        }

        // TODO:
        // Make a list of these variables.
        // `SRC`, `OUT`, etc.
        let on_build = self
            .cmd
            .build
            .replace("SRC", &self.workspace.src)
            .replace("OUT", &outfile);
        let (bin, args) = match on_build.split_once(' ') {
            Some((bin, args)) => (bin, args),
            _ => return Err("Problem parsing arguments".into()),
        };

        let cmd = Command::new(bin).args(args.split_whitespace()).output()?;

        Ok(cmd)
    }

    fn langfiles() -> Result<Vec<LanguageFile>, Box<dyn Error>> {
/*
        // I have absolutely zero idea how to do this without
        // calling `unwrap` directly :c
        Ok(LANGFILES
            .iter()
            .map(|file| toml::from_str(file).unwrap())
            .collect())
*/
        let mut langfiles = Vec::new();
        for langfile in LANGFILES.iter() {
            langfiles.push(toml::from_str(langfile)?);
        }
        Ok(langfiles)
    }

    // TODO:
    // Do something with this D:
    #[allow(dead_code)]
    pub fn available_languages(
    ) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
        let langfiles = Self::langfiles()?;
        let mut languages = Vec::new();
        let mut aliases = Vec::new();
        for lf in langfiles {
            languages.push(lf.language.name);
            for alias in lf.language.aliases {
                aliases.push(alias);
            }
        }

        Ok((languages, aliases))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    #[test]
    fn example_langfile_found_and_parseable() -> Result<(), Box<dyn Error>> {
        let langfile = Path::new(&env::var("CARGO_MANIFEST_DIR")?)
            .join("langs/example.toml");
        eprintln!("\nLoaded language file from\n  {:#?}\n", &langfile);
        let _: LanguageFile = toml::from_str(&fs::read_to_string(langfile)?)?;

        Ok(())
    }

    #[test]
    fn all_langfiles_found_and_parseable() -> Result<(), Box<dyn Error>> {
        /*
                let mut langfiles = Vec::new();
                for entry in fs::read_dir(
                    Path::new(&env::var("CARGO_MANIFEST_DIR")?)
                        .join("langs/"))? {
                    let entry = entry?.path();
                    if entry.is_file() {
                        langfiles.push(entry);
                    }
                }
                for langfile in langfiles.iter() {
                    let _: LanguageFile = toml::from_str(
                        &fs::read_to_string(langfile)?
                    )?;
                }
        */
        LanguageFile::langfiles()?;

        Ok(())
    }
}
