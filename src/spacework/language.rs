// use super::workspace::*;

use std::{
    error::Error,
    process::{Output, Command},
};

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
