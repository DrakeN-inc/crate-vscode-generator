use serde::Serialize;
use std::path::PathBuf;

/// The package contributes
#[derive(Debug, Clone, Serialize)]
pub struct Contributes {
    pub snippets: Vec<SnippetsContribute>,
}

impl Contributes {
    /// Creates a new package contributes
    pub fn new() -> Self {
        Self {
            snippets: vec![],
        }
    }

    /// Registrates a new snippets file
    pub fn reg_snippets(&mut self, contribute: SnippetsContribute) {
        self.snippets.push(contribute);
    }
}


/// The snippets contribute object
#[derive(Debug, Clone, Serialize)]
pub struct SnippetsContribute {
    pub language: String,
    pub path: PathBuf,
}

impl SnippetsContribute {
    /// Creates a new snippets contributes object
    /// * lang - the snippets programming language
    /// * path - the path to the snippets file
    pub fn new<S, R>(lang: S, path: R) -> Self
    where S: Into<String>, R: Into<PathBuf>
    {
        Self {
            language: lang.into(),
            path: path.into(),
        }
    }
}
