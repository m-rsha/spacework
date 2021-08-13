use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpaceworkFile {
    pub workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub language: String,
}
