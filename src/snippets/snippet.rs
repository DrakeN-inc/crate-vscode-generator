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
    /// * name - the snippet name id
    /// * descr - the snippet description
    /// * prefix - the snippet prefix
    /// * body - the body of snippet
    pub fn new<S>(name: S, descr: S, prefix: S, body: Vec<S>) -> Self
    where S: Into<String> {
        Self {
            language: "".into(),
            name: name.into(),
            description: descr.into(),
            prefix: prefix.into(),
            body: body
                .into_iter()
                .map(|s| s.into())
                .collect::<Vec<_>>()
        }
    }
    
    /// Creates a new simple text snippet
    /// * name - the snippet name id
    /// * prefix - the snippet prefix
    /// * value - the snippet body value
    pub fn text<S>(name: S, prefix: S, value: S) -> Self
    where S: Into<String> {
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
    
    /// Creates a new comment snippet
    /// * name - the snippet name id
    /// * cmnt_name - the comment name
    pub fn comment<S>(name: S, cmnt_name: &str) -> Self
    where S: Into<String> {
        Self {
            language: "".into(),
            name: name.into(),
            description: format!("// {cmnt_name}: ..."),
            prefix: format!("/{cmnt_name}"),
            body: vec![
                format!("// {cmnt_name}: {}", "${1:...}")
            ]
        }
    }

    /// Creates a new attribute snippet (for Rust lang)
    /// * name - the snippet name
    /// * attr_name - the attiribute name
    pub fn attribute<S>(name: S, attr_name: &str, values: Option<Vec<&str>>) -> Self
    where S: Into<String> {
        Self {
            language: "".into(),
            name: name.into(),
            description: format!("#[{attr_name}{}]", if values.is_some(){ "(...)" }else{ "" }),
            prefix: format!("#[{attr_name}{}]", if values.is_some(){ "()" }else{ "" }),
            body: vec![
                format!("#[{attr_name}{}]", if let Some(values) = values{ if !values.is_empty(){ "(${1|".to_owned() + &values.join(",") + "|})"}else{ "($1)".to_owned() } }else{ "".into() })
            ]
        }
    }

    /// Creates a new block snippet
    /// * name - the snippet name id
    /// * block_name - the block name
    pub fn block<S>(name: S, block_name: S) -> Self
    where S: Into<String> {
        let block_name = block_name.into();
        
        Self {
            language: "".into(),
            name: name.into(),
            description: format!("{block_name} ... {}", "{ ... }"),
            prefix: block_name.clone() + " {}",
            body: vec![
                format!("{block_name} $1 {}", "{\n    $2\n}")
            ]
        }
    }

    /// Creates a new double block snippet
    /// * name - the snippet name id
    /// * first_block_name - the first block name
    /// * other_block_name - the other block name
    pub fn double_block<S>(name: S, first_block_name: &str, other_block_name: &str) -> Self
    where S: Into<String> {
        Self {
            language: "".into(),
            name: name.into(),
            description: format!("{first_block_name} ... {}  {other_block_name} ... {0}", "{ ... }"),
            prefix: first_block_name.to_owned() + " {}" + " " + &other_block_name + " {}",
            body: vec![
                format!("{first_block_name} $1 {}", "{\n    $2\n}"),
                "".to_owned(),
                format!("{other_block_name} $1 {0}", "{\n    $3\n}")
            ]
        }
    }

    /// Creates a new simple block snippet without arguments
    /// * name - the snippet name id
    /// * block_name - the block name
    pub fn simple_block<S>(name: S, block_name: &str) -> Self
    where S: Into<String> {
        Self {
            language: "".into(),
            name: name.into(),
            description: format!("{block_name} {}", "{ ... }"),
            prefix: block_name.to_owned() + " {}",
            body: vec![
                format!("{block_name} {}", "{\n    $1\n}"),
            ]
        }
    }

    /// Creates a new function block snippet
    /// * name - the snippet name id
    /// * name - the block name
    pub fn function_block<S>(name: S, block_name: &str) -> Self
    where S: Into<String> {
        Self {
            language: "".into(),
            name: name.into(),
            description: format!("{block_name} ...() {}", "{ ... }"),
            prefix: block_name.to_owned() + "() {}",
            body: vec![
                format!("{block_name} $1($2) {}", "{\n    $3\n}"),
            ]
        }
    }

    /// Creates a new operator snippet without arguments
    /// * name - the snippet name id
    /// * oper_name - the operator name
    pub fn operator<S>(name: S, oper_name: &str, value: Option<&str>) -> Self
    where S: Into<String> {
        Self {
            language: "".into(),
            name: name.into(),
            description: if value.is_some() { format!("{oper_name} ...;") }else{ format!("{oper_name};") },
            prefix: if value.is_some() { format!("{oper_name} ") }else{ format!("{oper_name}") },
            body: vec![
                if let Some(value) = value { format!("{oper_name} ${}1:{value}{};", "{", "}") }else{ format!("{oper_name};") },
            ]
        }
    }

    /// Creates a new function snippet
    /// * name - the snippet name id
    /// * fn_name - the function name
    /// * pars - the non standart function parenthesis
    pub fn function<S>(name: S, fn_name: &str, pars: Option<(&str, &str)>, value: Option<&str>) -> Self
    where S: Into<String> {
        let (lpar, rpar) = if let Some(pars) = pars { pars }else{ ("(", ")") };
        
        Self {
            language: "".into(),
            name: name.into(),
            description: format!("{fn_name}{lpar}{}{rpar}", if value.is_some(){ "..." }else{ "" }),
            prefix: fn_name.to_owned() + lpar + rpar,
            body: vec![
                if let Some(value) = value { format!("{fn_name}{lpar}${}1:{value}{}{rpar}", "{", "}") }else{ format!("{fn_name}{lpar}{rpar}") },
            ]
        }
    }

    /// Converts the snippet to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(Error::from)
    }
}
