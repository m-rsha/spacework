use std::error::Error;
use std::process::{Output, Command};

#[derive(Debug, PartialEq)]
pub enum Language {
    C,
    Cpp,
    Python,
}

impl Language {
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
        let compiler = "g++";
        let standard = "-std=c++20";
        let opt_lvl = "-0g";
        let src = format!("src/{}", self.src_file());

        let args = [standard, src.as_str(), opt_lvl, "-o", "bin/testing"];
        let cmd = Command::new(compiler)
            .args(&args)
            .output()?;
        Ok(cmd)
    }

    fn compile_c(&self) -> Result<Output, Box<dyn Error>> {
        let compiler = "gcc";
        let standard = "-std=c17";
        let src = format!("src/{}", self.src_file());

        let args = [standard, src.as_str(), "-o", "bin/testing"];
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
}
