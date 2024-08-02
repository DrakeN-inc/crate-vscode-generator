use super::Version;
use serde::Serialize;

/// The package engines option
#[derive(Debug, Clone, Serialize)]
pub struct Engines {
    pub vscode: Version,
}

impl Engines {
    /// Creates a new package engines option
    pub fn new(vscode_ver: Version) -> Self {
        Self {
            vscode: vscode_ver,
        }
    }
}

impl std::default::Default for Engines {
    fn default() -> Self {
        Self {
            vscode: "^1.90.0".parse().unwrap()
        }
    }
}
