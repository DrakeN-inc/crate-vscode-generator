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
/// * language - the snippets programming language
/// * path - the snippets file path
#[derive(Debug, Clone, Serialize)]
pub struct SnippetsContribute {
    pub language: String,
    pub path: PathBuf,
}

impl SnippetsContribute {
    /// Creates a new snippets contributes object
    /// * lang - the snippets programming language
    /// * file_name - the file_name to the snippets file
    pub fn new<S, P>(lang: S, file_name: P) -> Self
    where S: Into<String>, P: Into<PathBuf> {
        let path = PathBuf::from("snippets").join( &file_name.into() );
        
        Self {
            language: lang.into(),
            path,
        }
    }
}
