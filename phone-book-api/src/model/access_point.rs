use std::fmt;

use crate::model::{
    address::Address,
    city::City,
    phone::Phone,

};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccessPoint{
    access_point_name: String,
    city: City,
    address: Address,
    phones: Vec<Phone>
}

impl AccessPoint {
    pub (crate) fn new(access_point_name:String,city: City, address:Address)->Self{
        Self{ access_point_name,city, address, phones: Vec::new() }
    }
}

impl fmt::Display for AccessPoint{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.address )
    }
}