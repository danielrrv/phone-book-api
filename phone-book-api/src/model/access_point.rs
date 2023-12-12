use std::{collections::{HashMap, hash_map::DefaultHasher}, fmt};
use regex::Regex;
use std::hash::{Hash, Hasher};
use crate::model::{address::Address, city::City, phone::Phone, tag::Tag};

use lupa::collection::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Model)]
pub struct AccessPoint {
    
    pub _id: Option<String>,
    pub name: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub lat: f32,
    pub long: f32,
    pub phones: Vec<Phone>,
    pub tags: Vec<String>, // Max 4M unicos por cuidad tags
                        //Tag tiene un valor unico y puede ser compartido por otros.
}
//ciudad.businesses.access_point(por ciudad).tags(calle nueva, centro comercial, servico, producto, evento)[scoped by ciudad]
//1-4048.256-65534.256-65534.[256-65534/independiente]

impl AccessPoint {
    pub(crate) fn new(
        name: String,
        address1: String,
        city: String,
        address2: String,
        lat: f32,
        long: f32,
        phones: Vec<Phone>,
        tags: Vec<String>,
    ) -> Self {
        Self {
            _id:Some(AccessPoint::calculate_hash(&name).to_string()),
            name:name,
            city:city,
            address1:address1,
            address2: address2,
            lat:lat,
            long:long,
            phones:phones,
            tags
        }
    }
    
    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

impl fmt::Display for AccessPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl PartialEq for AccessPoint{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.city == other.city || self._id == other._id
    }
}
