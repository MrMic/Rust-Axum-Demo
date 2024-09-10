use serde::{Deserialize, Serialize};

/// Demo book structure
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

/// Display the book using the format "{title} by {author}".
/// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}
