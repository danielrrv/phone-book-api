use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    uid: u64,
    name: String,
}

impl Tag {
    pub(crate) fn new(name: String) -> Self {
        Self { uid: 65_534, name }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


