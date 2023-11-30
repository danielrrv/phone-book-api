
use lupa::collection::Model;
use mongodb::bson::{Document, doc};
use serde::__private::doc;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::model::access_point::AccessPoint;
use crate::model::tag::Tag;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Business {
    pub _id: String,
    pub company_name: String,
    pub locations: Vec<AccessPoint>,
    pub description: String,
    pub tags: Vec<String>,
}

impl Hash for Business {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.company_name.hash(state)
    }
}
impl Into<Document> for Business{
    fn into(self) -> Document {
        doc!{}
    }
}

impl Business {
    pub fn new(company_name: String) -> Business {
        Self {
            _id: Business::calculate_hash(&company_name).to_string(),
            company_name: company_name,
            locations: Vec::new(),
            description: String::from(""),
            tags: Vec::new(),
        }
    }
    pub fn add_description(&mut self, description: String)->&mut Self{
        self.description = description;
        self
    }
    pub fn add_location(&mut self, location: AccessPoint)->&mut Self{
        let ref mut locations = self.locations;
        if !locations.contains(&location){
            locations.push(AccessPoint { _id: Some(AccessPoint::calculate_hash(&location.name).to_string()), ..location})
        }
        self
    }
 
    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

impl fmt::Display for Business {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{:?}", self.company_name, self.locations)
    }
}

// impl Model<Business> for Business {
//     fn scoped(&mut self, by: &str, when: &str)->T {
//         let this = self;
//         match by {
//             "locations.name"=> {
//                 let filtered = this.locations
//                 .to_vec()
//                 .iter()
//                 .filter(|location| location.name)
//             },
//             "locations.city"=> {
//                 let filtered = this.locations
//                 .to_vec()
//                 .iter()
//                 .filter(|location| location.name)
//             }
//         }
//     }
// }

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
