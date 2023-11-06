use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Phone {
    area_code: f32,
    phone: f32,
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}), {}", self.area_code, self.phone)
    }
}
