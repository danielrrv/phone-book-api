use crate::model::access_point::AccessPoint;

// use macros_utils::Collection;
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Business {
    pub _id: u64,
    pub company_name: String,
    pub locations: Vec<AccessPoint>,
    pub description: String
}

impl fmt::Display for Business {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{:?}", self.company_name, self.locations)
    }
}

// impl From<Vec<Business>> for Business {
//     fn from(value: Vec<Business>) -> Self {
//         Self {
//             _client: None,
//             company_name: String::from("Avianca"),
//             code: Default::default(),
//             businesses: value,
//         }
//     }
// }
