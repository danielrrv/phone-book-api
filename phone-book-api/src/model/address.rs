use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub metadata: Vec<HashMap<String, String>>,
    pub lat: f32,
    pub long: f32,
}

impl Address {
    pub(crate) fn new(
        address1: String,
        city: String,
        address2: String,
        lat: f32,
        long: f32,
    ) -> Self {
        Self {
            address1,
            address2,
            city,
            metadata: Vec::new(),
            lat,
            long,
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.lat, self.long)
    }
}
