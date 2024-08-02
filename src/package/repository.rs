use serde::Serialize;

/// The repository info
/// * type - the repository type (example: git)
/// * url - the path to repository page
#[derive(Debug, Clone, Serialize)]
pub struct Repository {
    pub r#type: String,
    pub url: String,
}

impl From<&str> for Repository {
    fn from(s: &str) -> Self {
        Self {
            r#type: "git".to_owned(),
            url: s.to_owned(),
        }
    }
}
