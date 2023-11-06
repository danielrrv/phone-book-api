use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Address {
    lat: f32,
    long: f32,
}

impl Address {
    pub(crate) fn new(lat: f32, long: f32) -> Self {
        Self { lat, long }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.lat, self.long)
    }
}
