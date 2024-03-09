use async_trait::async_trait;
use lupa::collection::Model;
use mongodb::bson::{doc, Bson, Document};
use regex::Regex;
use serde::__private::doc;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::pin::Pin;
use futures::StreamExt;
use std::hash::{Hash, Hasher};
use lupa::query::Neweable;
use serde::de::DeserializeOwned;
use crate::model::access_point::AccessPoint;
use crate::model::tag::Tag;
use std::default::Default;
use mongodb::{
    error::Error,
    options::FindOptions,
    Collection,
    
};

#[derive(Debug, Deserialize, Serialize, Clone, Model)]
pub struct Business {
    pub _id: Option<String>,
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
impl std::default::Default for Business {
    fn default() -> Self {
        Self { _id: Some("".to_string()), company_name: "".to_string(), locations: Vec::new(), description: "".to_string(), tags: Vec::new() }
    }
}

impl Business {
    pub fn new(company_name: String) -> Business {
        Default::default()
    }
    pub fn add_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_owned();
        self
    }
    pub fn add_location(&mut self, location: &AccessPoint) -> &mut Self {
        let ref mut locations = self.locations;
        if !locations.contains(&location) {
            locations.push(AccessPoint {
                _id: Some(AccessPoint::calculate_hash(&location.name).to_string()),
                ..location.to_owned()
            })
        } else {
            let first = locations
                .iter()
                .position(|_location| {
                    _location
                        ._id
                        .clone()
                        .eq(&Some(AccessPoint::calculate_hash(&location.name).to_string()))
                })
                .unwrap();
            locations.remove(first);
            locations.push(AccessPoint {
                _id: Some(AccessPoint::calculate_hash(&location.name).to_string()),
                ..location.to_owned()
            })
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


//         }
//     }
// }
