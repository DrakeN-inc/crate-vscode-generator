use crate::prelude::*;
use std::{ path::PathBuf, fs };

/// The README file generator
#[derive(Debug, Clone)]
pub struct Readme {
    pub value: String,
}

impl Readme {
    /// Creates a new README file generator
    pub fn new(title: &str, descr: &str) -> Self {
        Self {
            value: format!("# {title}\n## {descr}"),
        }
    }

    /// Adds a new line to README text
    /// * text - the addition text
    pub fn add_line(&mut self, text: &str) {
        self.value.push_str("\n\n");
        self.value.push_str(text);
    }

    /// Writes README file to "%DIR/README.md"
    /// * dir - the package root directory
    pub fn write_to<P>(&self, dir: P) -> Result<()>
    where P: Into<PathBuf> {
        let dir = dir.into();
        let path = dir.join("README.md");

        Ok(fs::write(path, &self.value).map_err(Error::from)?)
    }
}
