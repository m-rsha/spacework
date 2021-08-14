use crate::config::languagefile::LanguageFile;
use crate::config::spaceworkfile::SpaceworkFile;

use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Output;
use std::str;
use std::io::Write;

pub struct Workspace;

impl Workspace {
    pub fn create(
        proj_name: &str,
        lang: &str
    ) -> Result<PathBuf, Box<dyn Error>> {
        let langfile = LanguageFile::from_language(lang)?;

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
        println!("Created project directory: {}", &proj_dir.display());

        create_spacework_toml(&proj_dir, &langfile)?;
        
        let src_dir = &proj_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        let mut src_file = File::create(&src_dir.join(&langfile.workspace.src))?;
        src_file.write_all(&langfile.template()?.as_bytes())?;

        let bin_dir = &proj_dir.join("bin");
        fs::create_dir_all(&bin_dir)?;

        Ok(proj_dir)
    }

    pub fn from_options(
        proj_name: Option<&str>,
        lang: Option<&str>,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let proj_name = match proj_name {
            Some(proj_name) => proj_name,
            None => return Err("Workspace requires a name".into()),
        };
        let lang = match lang {
            Some(lang) => lang,
            None => return Err("Workspace requires a language".into()),
        };

        Ok(Self::create(proj_name, lang)?)
    }
}

fn create_spacework_toml(
    dir: &PathBuf,
    langfile: &LanguageFile
) -> Result<File, Box<dyn Error>> {
    let mut cfg = File::create(&dir.join("spacework.toml"))?;
    let toml = format!("[workspace]\nlanguage = \"{}\"", langfile.language.name);
    cfg.write_all(&toml.as_bytes())?;

    Ok(cfg)
}

pub fn build() -> Result<Output, Box<dyn Error>> {
    let cfg: SpaceworkFile = find_cfg(&mut env::current_dir()?)?;
    let langfile = LanguageFile::from_language(&cfg.workspace.language)?;

    Ok(langfile.build()?)
}

pub fn find_cfg(dir: &mut PathBuf) -> Result<SpaceworkFile, Box<dyn Error>> {
    if !is_inside_workspace(&dir)? {
        Err("Must be inside a spacework workspace.".into())
    } else if let Ok(cfg) = fs::read_to_string("spacework.toml") {
        Ok(SpaceworkFile::from_str(&cfg)?)
    } else if dir.pop() {
        find_cfg(dir)
    } else {
        Err("`spacework` config file not found".into())
    }
}

fn is_inside_workspace(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    Ok(path.starts_with(workspace_dir()?))
}

fn workspace_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(Path::new(&env::var("HOME")?).join("spacework"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn cfg_not_found_in_temp_dir() {
        let mut tmp = env::temp_dir();
        find_cfg(&mut tmp).unwrap();
    }

    #[test]
    fn cfg_found_in_workspace() -> Result<(), Box<dyn Error>> {
        let mut dir = Workspace::create(".spacework_test", "cpp")?;
        find_cfg(&mut dir)?;
        fs::remove_dir_all(dir)?;

        Ok(())
    }
    
    #[test]
    fn detects_inside_workspace_dir() -> Result<(), Box<dyn Error>> {
        assert!(!is_inside_workspace(&env::temp_dir())?);
        assert!(is_inside_workspace(&workspace_dir()?)?);
        
        Ok(())
    }
}
