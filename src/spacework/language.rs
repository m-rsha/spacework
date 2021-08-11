use crate::spacework::languagefile::LanguageFile;

use std::fs;
use std::env;
use std::path::Path;
use std::error::Error;
use std::process::{Output, Command};

#[derive(Debug, PartialEq)]
pub enum Language {
    C,
    Cpp,
    Python,
}

impl Language {
    pub fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        match s.to_lowercase().as_str() {
            "c" => Ok(Language::C),
            "cpp" | "c++" => Ok(Language::Cpp),
            "py" | "python" => Ok(Language::Python),
            _ => return Err("Unknown or unsupported language".into()),
        }
    }

    pub fn compile(&self) -> Result<Output, Box<dyn Error>> {
        match self {
            Language::C => self.compile_c(),
            Language::Cpp => self.compile_cpp(),
            Language::Python => self.compile_python(),
        }
    }
    
    pub fn src_file(&self) -> &str {
        match self {
            Language::C => "main.c",
            Language::Cpp => "main.cpp",
            Language::Python => "main.py",
        }
    }
    
    fn compile_cpp(&self) -> Result<Output, Box<dyn Error>> {
        let langfile = Path::new(&env::var("CARGO_MANIFEST_DIR")?)
            .join("langs/cpp/cpp.toml");
        let cpp: LanguageFile = toml::from_str(&fs::read_to_string(langfile)?)?;
        eprintln!("{:#?}", &cpp);

        let mut args = Vec::new();
        let compiler = cpp.language.compiler;
        if let Some(std) = cpp.language.standard {
            args.push(std);
        };
        if let Some(lvl) = cpp.language.optimization_levels.last() {
            args.push(lvl.to_string());
        };
        args.push(format!("src/{}", self.src_file()));
        args.push("-o".to_string());
        args.push("bin/testing".to_string());

        let cmd = Command::new(compiler).args(&args).output()?;
        eprintln!("{:#?}", &cmd);

        Ok(cmd)
    }

    fn compile_c(&self) -> Result<Output, Box<dyn Error>> {
        let compiler = "gcc";
        let std = "-std=c17";
        let src = format!("src/{}", self.src_file());

        let args = [std, src.as_str(), "-o", "bin/testing"];
        let cmd = Command::new(compiler)
            .args(&args)
            .output()?;
        Ok(cmd)
    }
    
    fn compile_python(&self) -> Result<Output, Box<dyn Error>> {
        Err(
            "Unable to compile python; try `spacework run` instead"
            .into()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn src_files_match() {
        assert_eq!(Language::src_file(&Language::C), "main.c");

        let lang = Language::C;
        assert_eq!(lang.src_file(), "main.c");
        let lang = Language::Cpp;
        assert_eq!(lang.src_file(), "main.cpp");
        let lang = Language::Python;
        assert_eq!(lang.src_file(), "main.py");
    }

    #[test]
    #[should_panic]
    fn cant_compile_python() {
        let lang = Language::Python;
        lang.compile().unwrap();
    }

    #[test]
    fn parses_language_from_str() -> Result<(), Box<dyn Error>> {
        let lang = Language::from_str("c")?;
        assert_eq!(lang, Language::C);
        
        let lang = Language::from_str("C++")?;
        assert_eq!(lang, Language::Cpp);

        let lang = Language::from_str("pYthon")?;
        assert_eq!(lang, Language::Python);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn cant_parse_weird_stuffs() {
        Language::from_str("eldritch horrors").unwrap();
        unreachable!();
    }
}
