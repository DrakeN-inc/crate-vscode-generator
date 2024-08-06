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

    /// Replaces a line in body contents by index (if index is not found, then nothing will happen)
    /// * index - the replacement line index
    /// * text - the new text of line
    pub fn replace_line<S>(mut self, index: usize, text: S) -> Self
    where S: Into<String> {
        if self.body.len() >= index {
            self.body[index] = text.into();
        }
        self
    }

    /// Removes a line in body contents by index (if index is not found, then nothing will happen)
    /// * index - the replacement line index
    pub fn remove_line(mut self, index: usize) -> Self {
        if self.body.len() >= index {
            self.body.remove(index);
        }
        self
    }
}

impl Snippet {
    /// Creates a new snippet object
    pub fn new() -> Self {
        todo!("Fn ::new()");   // TODO: Fn ::new()
    }
    
    /// Creates a new simple text snippet
    /// * snippet_name - the snippet name id
    /// * prefix - the snippet prefix
    /// * value - the snippet body value
    pub fn text<S>(snippet_name: S, prefix: S, value: S) -> Self
    where S: Into<String>
    {
        let value = value.into();
        
        Self {
            language: "".into(),
            name: snippet_name.into(),
            description: value.clone(),
            prefix: prefix.into(),
            body: vec![
                value
            ]
        }
    }
    
    /// Creates a new comment snippet
    pub fn comment() -> Self {
        todo!("Comment snippet..");  // TODO: Comment snippet..
    }

    /// Creates a new attribute snippet
    pub fn attribute() -> Self {
        todo!("Attribute snippet..");  // TODO: Attribute snippet..
    }

    /// Creates a new block snippet
    /// * snippet_name - the snippet name id
    /// * name - the block name
    pub fn block<S>(snippet_name: S, name: S) -> Self
    where S: Into<String>
    {
        let name = name.into();
        
        Self {
            language: "".into(),
            name: snippet_name.into(),
            description: format!("{name} ... {}", "{ ... }"),
            prefix: name.clone() + " {}",
            body: vec![
                format!("{name} $1 {}", "{\n    $2\n}")
            ]
        }
    }

    /// Creates a new double block snippet
    /// * snippet_name - the snippet name id
    /// * name1 - the first block name
    /// * name2 - the other block name
    pub fn double_block<S>(snippet_name: S, name1: S, name2: S) -> Self
    where S: Into<String>
    {
        let name1 = name1.into();
        let name2 = name2.into();
        
        Self {
            language: "".into(),
            name: snippet_name.into(),
            description: format!("{name1} ... {}  {name2} ... {0}", "{ ... }"),
            prefix: name1.clone() + " {}" + " " + &name2 + " {}",
            body: vec![
                format!("{name1} $1 {}", "{\n    $2\n}"),
                "".to_owned(),
                format!("{name2} $1 {0}", "{\n    $3\n}")
            ]
        }
    }

    /// Creates a new simple block snippet without arguments
    /// * snippet_name - the snippet name id
    /// * name - the block name
    pub fn simple_block<S>(snippet_name: S, name: S) -> Self
    where S: Into<String>
    {
        let name = name.into();
        
        Self {
            language: "".into(),
            name: snippet_name.into(),
            description: format!("{name} {}", "{ ... }"),
            prefix: name.clone() + " {}",
            body: vec![
                format!("{name} {}", "{\n    $1\n}"),
            ]
        }
    }

    /// Creates a new function block snippet
    /// * snippet_name - the snippet name id
    /// * name - the block name
    pub fn function_block<S>(snippet_name: S, name: S) -> Self
    where S: Into<String>
    {
        let name = name.into();
        
        Self {
            language: "".into(),
            name: snippet_name.into(),
            description: format!("{name} ...() {}", "{ ... }"),
            prefix: name.clone() + "() {}",
            body: vec![
                format!("{name} $1($2) {}", "{\n    $3\n}"),
            ]
        }
    }

    /// Creates a new operator snippet without arguments
    /// * snippet_name - the snippet name id
    /// * name - the block name
    pub fn operator<S>(snippet_name: S, name: S, value: Option<S>) -> Self
    where S: Into<String>
    {
        let name = name.into();
        let value = value.map(|s| s.into());
        
        Self {
            language: "".into(),
            name: snippet_name.into(),
            description: if value.is_some() { format!("{name} ...;") }else{ format!("{name};") },
            prefix: if value.is_some() { format!("{name} ") }else{ format!("{name}") },
            body: vec![
                if let Some(value) = value { format!("{name} ${}1:{value}{};", "{", "}") }else{ format!("{name};") },
            ]
        }
    }

    /// Creates a new function snippet
    /// * snippet_name - the snippet name id
    /// * name - the function name
    pub fn function<S>(snippet_name: S, name: S, value: Option<S>) -> Self
    where S: Into<String>
    {
        let name = name.into();
        let value = value.map(|s| s.into());
        
        Self {
            language: "".into(),
            name: snippet_name.into(),
            description: format!("{name}({})", if value.is_some(){ "..." }else{ "" }),
            prefix: name.clone() + "()",
            body: vec![
                if let Some(value) = value { format!("{name}(${}1:{value}{})", "{", "}") }else{ format!("{name}()") },
            ]
        }
    }
    
    /// Converts the snippet to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(Error::from)
    }
}
