use crate::prelude::*;
use serde::Serialize;

/// The snippet generator
/// * language - the programming language name
/// * name - the snippet name
/// * description - the snippet description
/// * prefix - the snippet prefix
/// * body - the snippet body contents
#[derive(Debug, Clone, Serialize)]
pub struct Snippet {
    #[serde(rename = "scope")]
    pub language: String,
    #[serde(skip_serializing)]
    pub name: String,
    pub description: String,
    pub prefix: String,
    pub body: Vec<String>,
}

impl Snippet {
    /// Sets a new snippet programming language name && returns 'Self'
    /// * lang - a new programming language name
    pub fn set_lang<S>(mut self, lang: S) -> Self
    where S: Into<String> {
        self.language = lang.into();
        self
    }

    /// Sets a new snippet name && returns 'Self'
    /// * name - a new snippet name
    pub fn set_name<S>(mut self, name: S) -> Self
    where S: Into<String> {
        self.name = name.into();
        self
    }

    /// Sets a new snippet description && returns 'Self'
    /// * descr - a new snippet description
    pub fn set_descr<S>(mut self, descr: S) -> Self
    where S: Into<String> {
        self.description = descr.into();
        self
    }

    /// Adds a some more description to snippet && returns 'Self'
    /// * descr - the description
    pub fn add_descr(mut self, text: &str) -> Self {
        self.description.push_str(text);
        self
    }

    /// Sets a new snippet prefix && returns 'Self'
    /// * prefix - a new snippet prefix
    pub fn set_prefix<S>(mut self, prefix: S) -> Self
    where S: Into<String> {
        self.prefix = prefix.into();
        self
    }
    
    /// Sets a new body contents of snippet && returns 'Self'
    /// * body - a new body contents
    pub fn set_body<S>(mut self, body: Vec<S>) -> Self
    where S: Into<String> {
        self.body = body
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<_>>();
        self
    }
    
    /// Adds a new line to snippet body && returns 'Self'
    /// * text - a line contents
    pub fn add_line<S>(mut self, text: S) -> Self
    where S: Into<String> {
        self.body.push(text.into());
        self
    }

    /// Clears the all body contents of snippet && returns 'Self'
    pub fn clear_body(mut self) -> Self {
        self.body.clear();
        self
    }
}

impl Snippet {
    /// Creates a new simple text snippet
    /// * name - the snippet name
    /// * prefix - the snippet prefix
    /// * value - the snippet body value
    pub fn text<S>(name: S, prefix: S, value: S) -> Self
    where S: Into<String>
    {
        let value = value.into();
        
        Self {
            language: "".into(),
            name: name.into(),
            description: value.clone(),
            prefix: prefix.into(),
            body: vec![
                value
            ]
        }
    }

    /// Converts the snippet to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(Error::from)
    }
}
