use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct City {
    name: String,
    province: String,

}

impl City{
    pub fn new(name:String, province: String)->Self{
        Self{name,  province}
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.province)
    }
}
