use crate::prelude::*;

/// The package version parser
#[derive(Debug, Clone)]
pub struct Version {
    value: String,
}

impl Version {
    /// Get as string slice
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Converts to string
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl std::str::FromStr for Version {
    type Err = Error;
    
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"^\^?\d+(\.\d+)*$").unwrap();
        if re.is_match(s) {
            Ok(Self {
                value: s.to_owned(),
            })
        } else {
            Err(Error::IncorrectVersionFormat)
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: serde::Serializer
    {
        serializer.serialize_str(&self.value)
    }
}
