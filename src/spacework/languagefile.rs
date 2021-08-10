use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LanguageFile {
    pub language: Language,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub compiler: String,
    pub name: String,
    pub output: String,
    pub standard: Option<String>,

    pub extensions: Vec<String>,
    pub optimization_levels: Vec<String>,
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::error::Error;
    
    #[test]
    fn example_file_found_and_parseable() -> Result<(), Box<dyn Error>> {
        let langfile = Path::new(&env::var("CARGO_MANIFEST_DIR")?)
            .join("langs/example.toml");
        let _: LanguageFile = toml::from_str(&fs::read_to_string(langfile)?)?;

        Ok(())
    }
}
