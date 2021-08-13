use serde::Deserialize;
use std::error::Error;
use std::process::{Command, Output};
use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

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

impl LanguageFile {
    pub fn from_str(lang: &str) -> Result<Self, Box<dyn Error>> {
        let langfiles: Vec<LanguageFile> = Self::langfiles()?;
        for langfile in langfiles {
            if langfile.language.aliases.contains(&lang.to_string()) {
                return Ok(langfile);
            }
            if langfile.language.name == lang {
                return Ok(langfile);
            }
        }

        Err("Language file not found. Consider creating one".into())
    }

	pub fn build(&self) -> Result<Output, Box<dyn Error>> {
        let mut outfile = self.workspace.src.clone();
        for ext in self.language.extensions.iter() {
            if let Some(e) = ext.strip_suffix(format!(".{}", ext).as_str()) {
                outfile = e.to_string();
            }
        }
        let on_build = self.cmd.build
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
        let mut langfiles = Vec::new();
        let cargo_dir = env::var("CARGO_MANIFEST_DIR")?;
        let cargo_dir = Path::new(&cargo_dir);
        let langfile_dir = cargo_dir.join("langs/");
        for entry in fs::read_dir(langfile_dir)? {
            let entry = entry?.path();
            let ext = match entry.extension() {
                Some(ext) => ext,
                None => OsStr::new(""),
            };
            if entry.is_file() && ext == "toml" {
                langfiles.push(toml::from_str(&fs::read_to_string(entry)?)?);
            }
        }
        Ok(langfiles)
    }
    
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
    use std::fs;
    use std::path::Path;
    use std::error::Error;
    
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

        Ok(())
    }
}
