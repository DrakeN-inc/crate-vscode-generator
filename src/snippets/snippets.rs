use crate::prelude::*;
use super::{ Snippet, SnippetsDoc };
use std::{fs, path::PathBuf};

/// The snippets group
/// * language - the programming language name
/// * name - the snippets group name
/// * description - the snippets group description
/// * file_name - the snippets file name
/// * snippets - the snippets hash map, where <SNIPPET_NAME, SNIPPET>
/// * documentation - the snippets group [documentation](SnippetsDoc) generator
#[derive(Debug, Clone)]
pub struct Snippets {
    pub language: String,
    pub name: String,
    pub description: String,
    pub file_name: PathBuf,
    snippets: HashMap<String, Snippet>,
    documentation: SnippetsDoc,
}

impl Snippets {
    /// Creates a new 'Snippets' object
    /// * language - the programming language name
    /// * name - the snippets group name
    /// * description - the snippets group description
    /// * snippets - the snippets list
    pub fn new<S>(language: S, name: S, description: S, snippets: Vec<Snippet>) -> Self
    where S: Into<String>
    {
        let lang = language.into();
        let name = name.into();
        let descr = description.into();
        let file_name = format!("{}.code-snippets", to_latin_text(&name.to_lowercase(), true)).into();

        // creating the 'Snippets' object:
        let mut this = Self {
            language: lang.clone(),
            name: to_latin_text(&name, true),
            description: descr.clone(),
            file_name,
            snippets: HashMap::new(),
            documentation: SnippetsDoc::new(lang, name, descr)
        };

        // Adding the snippets:
        for snippet in snippets {
            this.add_snippet(snippet);
        }

        this
    }

    /// Sets the snippets programming language name
    /// * lang - the programming language name
    pub fn set_lang<S>(mut self, lang: S) -> Self
    where S: Into<String> {
        self.language = lang.into();
        self
    }

    /// Sets the snippets group name
    /// * name - the snippets group name
    pub fn set_name<S>(mut self, name: S) -> Self
    where S: Into<String> {
        self.name = name.into();
        self
    }

    /// Sets the snippets group description
    /// * descr - the snippets group description
    pub fn set_descr<S>(mut self, descr: S) -> Self
    where S: Into<String> {
        self.description = descr.into();
        self
    }

    /// Sets the snippets group file name
    /// * file_name - the snippets group file name
    pub fn set_file_name<P>(mut self, file_name: P) -> Self
    where P: Into<PathBuf> {
        self.file_name = file_name.into();
        self
    }

    /// Adds a new snippet
    pub fn add_snippet(&mut self, snippet: Snippet) {
        // write snippet info to documentation:
        self.documentation.write_line(snippet.prefix.clone(), snippet.description.clone());
        
        // generate new unique snippet name:
        let name = to_latin_text(snippet.name.trim(), true) + "-" + &gen_unique_name(6);
        // adding snippet:
        self.snippets.insert(name, snippet);
    }

    /// Converts the snippets group to JSON string
    pub fn to_json(&self) -> Result<String> {
        // check && set the snippets programming language name:
        let mut snippets = self.snippets.clone();
        for (_, snippet) in snippets.iter_mut() {
            if snippet.language.is_empty() {
                snippet.language = self.language.clone();
            }
        }

        // Converting snippets group to JSON string && Write it to file:
        serde_json::to_string_pretty(&snippets).map_err(Error::from)
    }
    
    /// Writes the snippets group to file "%DIR/snippets/%FILE_NAME.code-snippets"
    /// * dir - the package root directory path (without '/snippets' folder)
    pub fn write_to<P>(&self, dir: P) -> Result<&SnippetsDoc>
    where P: Into<PathBuf>
    {
        // creating dir path:
        let dir = dir.into().join("snippets");
        // dbg!(&dir);  // DEBUG:
        fs::create_dir_all(&dir).map_err(Error::from)?;

        // converting snippets to JSON string:
        let json_contents = self.to_json()?;

        // writing snippets to file:
        let path = dir.join(&self.file_name);
        fs::write(path, json_contents).map_err(Error::from)?;
        
        Ok(&self.documentation)
    }
}
