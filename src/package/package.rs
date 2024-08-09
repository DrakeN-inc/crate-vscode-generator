use crate::{ prelude::*, Snippets };
use super::*;
use serde::Serialize;
use std::{ path::PathBuf, fs };

/// The package generator
#[derive(Debug, Clone, Serialize)]
pub struct Package {
    #[serde(rename = "name")]
    id: String,
    #[serde(rename = "displayName")]
    name: String,
    description: String,
    version: Version,
    categories: Vec<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<Repository>,
    engines: Engines,
    contributes: Contributes,

    #[serde(skip_serializing)]
    snippets: Option<Vec<Snippets>>,

    #[serde(skip_serializing)]
    license: License,
}

impl Package {
    /// Creates a new snippets package object
    /// * id - the id name of the package
    /// * name - the package name
    /// * title - the package title
    /// * description - the package description
    /// * version - the package version
    /// * icon - the package icon image
    /// * repository - the package repository URL
    /// * license - the package license
    pub fn snippets<S, R>(id: S, name: S, description: S, version: Version, icon: S, repository: Option<R>, snippets: Vec<Snippets>, license: License) -> Self
    where S: Into<String>, R: Into<Repository>
    {
        let mut this = Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            version,
            categories: vec![Category::Snippets],
            icon: Some(icon.into()),
            repository: repository.map(|v| v.into()),
            engines: Engines::default(),
            contributes: Contributes::new(),
            snippets: Some(vec![]),
            license,
        };

        for snips in snippets {
            this.add_snippets_group(snips)
        }

        this
    }

    /// Adds a new snippets group to package
    pub fn add_snippets_group(&mut self, snips: Snippets) {
        if let Some(snippets) = self.snippets.as_mut() {
            self.contributes.reg_snippets(SnippetsContribute::new(snips.language.clone(), &snips.file_name.clone()));
            snippets.push(snips);
        }
    }

    /// Converting to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(Error::from)
    }
    
    /// Writes the package to directory path
    pub fn write_to<P>(&self, dir: P) -> Result<()>
    where P: Into<PathBuf>
    {
        // preparing directory path:
        let dir = dir.into();
        fs::create_dir_all(&dir).map_err(Error::from)?;
        let pkg_path = dir.clone().join("package.json");
        // dbg!(&pkg_path);    // DEBUG: The package manifest path

        // generating a README file:
        let mut readme = Readme::new(&self.name, &self.description);
        
        // writing package manifest file "package.json":
        let json_cnts = self.to_json()?;
        fs::write(pkg_path, json_cnts).map_err(Error::from)?;
        
        // writing snippets files:
        if let Some(snippets) = &self.snippets {
            for snips in snippets {
                let doc = snips.write_to(&dir)?;
                readme.add_line(&doc.to_string());
            }
        }

        // writing the README file:
        readme.write_to(dir.clone())?;

        // writing LICENSE file:
        self.license.write_to(dir)?;

        Ok(())
    }
}
