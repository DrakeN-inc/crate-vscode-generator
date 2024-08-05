/// The snippets documentation generator
/// * language - the snippets programming language name
/// * title - the snippets group name
/// * description - the snippets group description
/// * body - the documentation body, where '<(SNIPPET_PREFIX, SNIPPET_DESCRIPTION)>'
#[derive(Debug, Clone)]
pub struct SnippetsDoc {
    pub language: String,
    pub title: String,
    pub description: String,
    pub body: Vec<(String, String)>,
}

impl SnippetsDoc {
    /// Creates a new 'SnippetsDoc' object
    /// * lang - the programming language name
    /// * title - the snippets group name
    /// * descr - the snippets group description
    pub fn new<S>(lang: S, title: S, descr: S) -> Self
    where S: Into<String>
    {
        Self {
            language: lang.into(),
            title: title.into(),
            description: descr.into(),
            body: vec![]
        }
    }

    /// Clears the documentation body contents
    pub fn clear_body(&mut self) {
        self.body.clear()
    }
    
    /// Writing a new line to documentation body
    /// * prefix - the snippet prefix text
    /// * descr - the snippet description
    /// ```no_run .write_line("st", "struct Name { ... }");```
    pub fn write_line<S>(&mut self, prefix: S, descr: S)
    where S: Into<String>
    {
        self.body.push((prefix.into(), descr.into()));
    }

    /// Converts the documentation object to string
    pub fn to_string(&self) -> String {
        // calculating the maximum of table length:
        // | ____l is prefix____ | ____r is description____ |
        let (mut l_max, mut r_max) = (0, 0);
        for (l, r) in &self.body {
            let (l_len, r_len) = (l.len() + 2, r.len() + 2);
            
            if l_len > l_max { l_max = l_len; }
            if r_len > r_max { r_max = r_len; }
        }

        // generating the base of table:
        let mut doc = format!(
            "# {title} [{lang}]:\n## {descr}\n| Prefix:{l_spaces} | Description:{r_spaces} |\n| {l_dashes} | {r_dashes} |\n",
            lang = &self.language,
            title = &self.title,
            descr = &self.description,
            l_spaces = " ".repeat(if l_max >= 7 { l_max - 7 }else{ 0 }),
            r_spaces = " ".repeat(if r_max >= 12 { r_max - 12 }else{ 0 }),
            l_dashes = "-".repeat(l_max),
            r_dashes = "-".repeat(r_max)
        );

        // adding the snippet instructions to documentation:
        for (prefix, descr) in &self.body {
            let (l_len, r_len) = (prefix.len(), descr.len());
            
            // generating the table line:
            let line = format!(
                "| {prefix}{l_spaces} | {descr}{r_spaces} |\n",
                l_spaces = " ".repeat(if l_max >= l_len { l_max - l_len }else{ 0 }),
                r_spaces = " ".repeat(if r_max >= r_len { r_max - r_len }else{ 0 }),
            );

            doc.push_str(&line);
        }

        doc
    }
}

impl Into<String> for SnippetsDoc {
    /// Converts the documentation object to string
    fn into(self) -> String {
        self.to_string()
    }
}
