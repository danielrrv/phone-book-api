
use async_trait::async_trait;
use futures::stream::{StreamExt, TryStream, TryStreamExt};
use std::pin::Pin;
use std::{collections::{HashMap, hash_map::DefaultHasher}, fmt};
use regex::Regex;
use std::hash::{Hash, Hasher};
use crate::model::{address::Address, city::City, phone::Phone, tag::Tag};
use lupa::query::Neweable;
use lupa::collection::Model;
use serde::{Deserialize, Serialize, de::DeserializeOwned};


use mongodb::{
    bson::{doc, Document},
    error::Error,
    Collection,
    options::FindOptions
};

#[derive(Debug, Deserialize, Serialize, Clone, Model)]
pub struct AccessPoint{  
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

impl Default for AccessPoint{
    fn default() -> Self {
        Self { _id: Some("".to_string()), name: "".to_string(), address1: "".to_string(), address2: "".to_string(), city: "".to_string(), lat: 0.0, long: 0.0, phones: Vec::new(), tags: Vec::new() }
    }
}
impl AccessPoint {

    pub fn new()->AccessPoint{
        Default::default()
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
