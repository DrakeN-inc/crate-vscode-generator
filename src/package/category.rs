use serde::Serialize;

/// The package category
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Category {
    Snippets,
}
