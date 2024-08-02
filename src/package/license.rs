use crate::prelude::*;
use chrono::{Datelike, Utc};
use std::{ path::PathBuf, fs };

const LICENSE_MIT: &'static str = r###"MIT License

Copyright (c) %YEAR %AUTHOR

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.""###;


/// The package LICENSE generator
#[derive(Debug, Clone)]
pub struct License {
    value: String,
}

impl License {
    /// Creates a new MIT license
    /// & author - the package author full name
    pub fn mit(author: &str) -> Self {
        Self {
            value: LICENSE_MIT
                .replace("%AUTHOR", author)
                .replace("%YEAR", &Utc::now().year().to_string())
        }
    }

    /// Writes LICENSE file to "%DIR/LICENSE.md"
    /// * dir - the package root directory
    pub fn write_to<P>(&self, dir: P) -> Result<()>
    where P: Into<PathBuf> {
        let dir = dir.into();
        let path = dir.join("LICENSE.md");

        Ok(fs::write(path, &self.value).map_err(Error::from)?)
    }
}
