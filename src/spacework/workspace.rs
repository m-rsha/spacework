use crate::config::languagefile::LanguageFile;
use crate::config::spaceworkfile::SpaceworkFile;
use crate::spacework::history;

use std::env::{self, VarError};
use std::error::Error;
use std::fs::{self, File};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Output;
use std::str;

pub struct Workspace;

impl Workspace {
    pub fn create(
        proj_name: &str,
        lang: &str,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let langfile = LanguageFile::from_language(lang)?;

        let workspace_root = workspace_dir()?;
        if !workspace_root.exists() {
            fs::create_dir_all(&workspace_root)?;
            println!(
                "{}",
                history::write(&format!(
                    "Created spacework directory: {}",
                    &workspace_root.display()
                ))?
            );
        }

        let proj_dir = create_proj_dir(&workspace_root, proj_name, &langfile)?;

        SpaceworkFile::create(&proj_dir, &langfile)?;

        let (src_dir, _) = create_subdirs(&proj_dir)?;
        create_src_file(&src_dir, &langfile)?;

        Ok(proj_dir)
    }

    pub fn create_from_options(
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

        Self::create(proj_name, lang)
    }
}

fn create_proj_dir(
    workspace_root: &Path,
    proj_name: &str,
    langfile: &LanguageFile,
) -> Result<PathBuf, Box<dyn Error>> {
    let proj_dir =
        workspace_root.join(&langfile.workspace.dir).join(proj_name);

    if proj_dir.exists() {
        Err("Project directory already exists".into())
    } else {
        fs::create_dir_all(&proj_dir)?;
        println!("Created project directory: {}", &proj_dir.display());

        Ok(proj_dir)
    }
}

fn create_src_file(
    src_dir: &Path,
    langfile: &LanguageFile,
) -> Result<(), Box<dyn Error>> {
    Ok(File::create(src_dir.join(&langfile.workspace.src))?
        .write_all(langfile.template()?.as_bytes())?)
}

fn create_subdirs(
    proj_dir: &Path,
) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
    let src_dir = proj_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let bin_dir = proj_dir.join("bin");
    fs::create_dir_all(&bin_dir)?;

    Ok((src_dir, bin_dir))
}

pub fn build() -> Result<Output, Box<dyn Error>> {
    let cfg = SpaceworkFile::find_in_dir(&mut env::current_dir()?)?;
    let langfile = LanguageFile::from_language(&cfg.workspace.language)?;

    langfile.build()
}

pub fn is_inside_workspace(path: &Path) -> Result<bool, Box<dyn Error>> {
    Ok(path.starts_with(workspace_dir()?))
}

pub fn workspace_dir() -> Result<PathBuf, &'static str> {
    let home_dir = match env::var("HOME") {
        Ok(home) => home,
        Err(e) => match e {
            VarError::NotPresent =>
                return Err("HOME environment variable not found. \
                    Unable to create workspace"),
            VarError::NotUnicode(_) => return Err(
                "Unable to parse HOME environment variable: Invalid unicode",
            ),
        },
    };

    Ok(Path::new(&home_dir).join("spacework"))
}

pub fn delete_workspace() -> Result<(), Box<dyn Error>> {
    match fs::remove_dir_all(&workspace_dir()?) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(()),
            _ => Err(format!(
                "Unable to remove workspace directories: {}", e
            ).into())
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn cfg_not_found_in_non_workspace_dir() {
        let mut tmp = env::temp_dir();

        SpaceworkFile::find_in_dir(&mut tmp).unwrap();
    }

    #[test]
    fn cfg_found_in_workspace_dir() -> Result<(), Box<dyn Error>> {
        let mut dir = Workspace::create(".spacework_test", "cpp")?;

        SpaceworkFile::find_in_dir(&mut dir)?;

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
