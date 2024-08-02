pub type Result<T> = std::result::Result<T, Error>;

/// The package errors
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    IncorrectVersionFormat,
}

impl std::fmt::Display for Error {
    /// The error formatter
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Json(e) => write!(f, "{e}"),
            Self::IncorrectVersionFormat => write!(f, "Incorrect package version format"),
        }
    }
}

impl From<std::io::Error> for Error {
    /// Creates the Error from 'std::io::Error'
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    /// Creates the Error from 'serde_json::Error'
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
