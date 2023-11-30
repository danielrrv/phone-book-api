use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Tag {
    name: String,
}

impl Tag {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


