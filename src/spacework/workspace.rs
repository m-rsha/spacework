use crate::config::languagefile::LanguageFile;
// use crate::spacework::history;

use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Output;
use std::str;
use std::io::Write;

pub struct Workspace;

impl Workspace {
    pub fn create(name: &str, lang: &str) -> Result<(), Box<dyn Error>> {
        let proj_name = name;
        let langfile: LanguageFile = LanguageFile::from_str(lang)?;

        let ws = workspace_dir()?;
        if !ws.exists() {
            fs::create_dir_all(&ws)?;
            println!("Created `spacework` directory: {}", &ws.display());
        }

        let mut proj_dir = ws.join(&langfile.workspace.dir);
        proj_dir.push(proj_name);
        if proj_dir.exists() {
            return Err("Project directory already exists".into());
        }
        fs::create_dir_all(&proj_dir)?;
        // history::write("We made a directoryyyy!")?;
        println!("Created directory: {}", &proj_dir.display());

        create_spacework_toml(&proj_dir, &langfile)?;
        
        let src_dir = &proj_dir.join("src");
        fs::create_dir_all(&src_dir)?;
        File::create(&src_dir.join(langfile.workspace.src))?;

        let bin_dir = &proj_dir.join("bin");
        fs::create_dir_all(&bin_dir)?;

        Ok(())
    }

    pub fn from_options(
        name: Option<&str>,
        lang: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let name = match name {
            Some(name) => name,
            None => return Err("Workspace requires a name".into()),
        };
        let lang = match lang {
            Some(lang) => lang,
            None => return Err("Workspace requires a language".into()),
        };

        Ok(Self::create(name, lang)?)
    }
}

fn create_spacework_toml(
    dir: &PathBuf,
    langfile: &LanguageFile
) -> Result<File, Box<dyn Error>> {
    let mut cfg = File::create(&dir.join("spacework.toml"))?;
    let toml = format!("[workspace]\nlanguage = {}", langfile.language.name);
    cfg.write_all(&toml.as_bytes())?;

    Ok(cfg)
}

pub fn build() -> Result<Output, Box<dyn Error>> {
    let src = find_src_file()?;
    let langfile = LanguageFile::from_str(
        &fs::read_to_string("spacework.toml")?
    )?;
    // let src = langfile.workspace.src;
    let cmd = langfile.build()?;
    Ok(cmd)
}

pub fn find_src_file() -> Result<(), Box<dyn Error>> {
    // Look for a spacework.toml file in current dir, then search each ancestor
    // for it until we find it, then use that dir to find the source file in
    // `./src/main.ext`
    if !is_inside_workspace()? {
        return Err("Must be inside a spacework workspace".into());
    }
    let cwd = env::current_dir()?;
    let full_path = Path::new(&cwd).ancestors();
    let mut stripped = Path::new(".");

    for path in full_path {
        if path.starts_with(workspace_dir()?) {
            stripped = path.strip_prefix(workspace_dir()?).unwrap();
            eprintln!("Stripped: {}", stripped.display());
        }
    }

    Ok(())
}

fn is_inside_workspace() -> Result<bool, Box<dyn Error>> {
    Ok(env::current_dir()?.starts_with(workspace_dir()?))
}

fn workspace_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(Path::new(&env::var("HOME")?).join("spacework"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn detects_inside_workspace_dir() -> Result<(), Box<dyn Error>> {
        env::set_current_dir(env::temp_dir())?;
        assert!(!is_inside_workspace()?);

        env::set_current_dir(&workspace_dir()?)?;
        assert!(is_inside_workspace()?);
        
        Ok(())
    }
}
