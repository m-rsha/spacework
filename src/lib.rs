use std::{
    env,
    error::Error,
    fs,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::{Output, Command},
    str,
    // os::unix::ffi::OsStrExt,
};
use chrono::prelude::*;

pub struct Spacework {
    user_home: PathBuf,
    workspace_home: PathBuf,
    language: Language,
    project_name: String,
}

#[derive(Debug, PartialEq)]
pub enum Language {
    C,
    CPP,
    Python,
}

impl Language {
    pub fn compile(&self) -> Result<Output, Box<dyn Error>> {
        match self {
            Language::C => self.compile_c(),
            Language::CPP => self.compile_cpp(),
            Language::Python => self.compile_python(),
        }
    }

    fn compile_cpp(&self) -> Result<Output, Box<dyn Error>> {
        let compiler = "g++";
        let args = ["-std=c++20", "src/main.cpp", "-o", "bin/testing"];
        let cmd = Command::new(compiler)
            .args(&args)
            .output()?;
        Ok(cmd)
    }

    fn compile_c(&self) -> Result<Output, Box<dyn Error>> {
        let compiler = "gcc";
        let args = ["-std=c++20", "src/main.cpp", "-o", "bin/testing"];
        let cmd = Command::new(compiler)
            .args(&args)
            .output()?;
        Ok(cmd)
    }
    
    fn compile_python(&self) -> Result<Output, Box<dyn Error>> {
        Err(
            "Unable to compile python; try `spacework run` instead."
            .into()
        )
    }
}

impl Spacework {
    pub fn new(name: &str, language: Language) -> Result<Self, Box<dyn Error>> {
        let project_name = name.into();
        let user_home: PathBuf = env::var("HOME")?.parse()?;
        let workspace_home = user_home.join("spacework");
        if !workspace_home.exists() {
            fs::create_dir_all(&workspace_home)?;
            println!(
                "Created `spacework` directory: {}",
                &workspace_home.display()
            );
        }

        Ok(
            Spacework {
                user_home,
                workspace_home,
                language,
                project_name,
            }
        )
    }

    pub fn from_options(
        name: Option<&str>,
        language: Option<&str>
    ) -> Result<Self, Box<dyn Error>> {
        let name = match name {
            Some(name) => name,
            None => return Err("Name required".into()),
        };
        let language = match language {
            Some(language) => match language {
                "c" => Language::C,
                "cpp" | "c++" => Language::CPP,
                "py" | "python" => Language::Python,
                _ => return Err("Invalid language".into()),
            },
            None => return Err("Language required".into()),
        };
        Spacework::new(name, language)
    }

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        let mut dir: PathBuf = self.workspace_home.clone();
        match self.language {
            Language::C => dir.push("c"),
            Language::CPP => dir.push("cpp"),
            Language::Python => dir.push("py"),
        }
        dir.push(self.project_name.as_str());
        if dir.exists() {
            return Err(
                format!("Directory already exists: {}", &dir.display())
                .into()
            );
        } else {
            fs::create_dir_all(&mut dir)?;
            self.log(format!("Created {}", &dir.display()).as_str())?;
            println!("Created directory: {}", &dir.display());
            self.create_workspace_dirs(&dir)?;
        }
        Ok(())
    }


    fn create_workspace_dirs(&self, dir: &Path) -> Result<(), Box<dyn Error>> {
        // TODO:
        // Prevent these from creating `src` and `bin` directories
        // in the workspace root. They should only exist under
        // `workspace/<lang>/<proj>/`
        let src_dir = &dir.join("src");
        fs::create_dir_all(&src_dir)?;
        self.log(format!("Created {}", &src_dir.display()).as_str())?;

        let bin_dir = &dir.join("bin");
        fs::create_dir_all(&bin_dir)?;
        self.log(format!("Created {}", &bin_dir.display()).as_str())?;

        // TODO: 
        // Populate the `src` dir with a language-specific hello world file
        // similar to `cargo new`
        let main_file = match self.language {
            Language::C => "main.c",
            Language::CPP => "main.cpp",
            Language::Python => "main.py",
        };
        File::create(&src_dir.join(main_file))?; // And log it? idk.
        Ok(())
    }

    fn log(&self, data: &str) -> Result<(), Box<dyn Error>> {
        let histfile: PathBuf = self.user_home.join(".spacework_history");
        if !histfile.exists() {
            File::create(&histfile)?;
            self.log("Hello hello, world!")?;
            println!("Created spacework history file: {}", &histfile.display());
        }
        let mut file = OpenOptions::new()
            .append(true)
            .open(&histfile)?;
        let time = Local::now().format("%Y-%m-%d@%X: ");
        let mut s = time.to_string();
        s.push_str(data);
        s.push('\n');
        file.write_all(&s.as_bytes())?;
        // Not entirely sure if I need to call `flush`
        file.flush()?;
        Ok(())
    }
    
    pub fn compile() -> Result<Output, Box<dyn Error>> {
        if !is_inside_workspace()? {
            return Err("Not inside a `spacework` workspace.".into());
        }
        let lang = Spacework::get_language()?;
        eprintln!("We have a {:?} file!", lang);
        lang.compile()
    }
    
    fn get_language() -> Result<Language, Box<dyn Error>> {
        // This method feels so ugly. :c
        let mut extensions: Vec<String> = Vec::new();
        for directory in &[".", "./src"] {
            if Path::new(directory).exists() {
                for file in fs::read_dir(directory)? {
                    let file = file?.path();
                    if let Some(ext) = file.extension() {
                        if let Some(ext) = ext.to_str() {
                            extensions.push(ext.to_string());
                        }
                    }
                }
            }
        }
        for ext in extensions.iter() {
            eprintln!("{}", &ext);
            if ext == "c" {
                return Ok(Language::C);
            } else if ext == "cpp" {
                return Ok(Language::CPP);
            } else if ext == "py" {
                return Ok(Language::Python);
            }
        }
        Err("Found no files to compile.".into())
    }

}

pub fn print_history() -> Result<(), Box<dyn Error>> {
    // TODO:
    // Print last few items.
    // Print specific actions, such as last n creations.
    // Probably need to figure out how to use `Seek` and
    // `SeekFrom::End()`
    let file = fs::read_to_string(
        env::var("HOME")?
        .parse::<PathBuf>()?
        .join(".spacework_history")
    )?;
    print!("{}", &file);
    Ok(())
}
    
fn is_inside_workspace() -> Result<bool, Box<dyn Error>> {
    Ok(env::current_dir()?.starts_with(workspace_home()?))
}

fn workspace_home() -> Result<PathBuf, Box<dyn Error>> {
    Ok(env::var("HOME")?.parse::<PathBuf>()?.join("spacework"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn spacework_works() {
        let mut sw = Spacework::new("Test").unwrap();
        sw.language(Language::C);
        assert_eq!(sw.language, Language::C);
        assert_eq!(sw.project_name, "Test");
    }
}
