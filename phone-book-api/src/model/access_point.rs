use std::fmt;

use crate::model::{
    address::Address,
    city::City,
    phone::Phone,
    tag::Tag

};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccessPoint{
    access_point_name: String,
    city: City,
    address: Address,
    phones: Vec<Phone>,
    tags: Vec<Tag>// Max 4M unicos por cuidad tags
    //Tag tiene un valor unico y puede ser compartido por otros.
}
//ciudad.businesses.access_point(por ciudad).tags(calle nueva, centro comercial, servico, producto, evento)[scoped by ciudad]
//1-4048.256-65534.256-65534.[256-65534/independiente]

impl AccessPoint {
    pub (crate) fn new(access_point_name:String,city: City, address: Address)->Self{
        Self{ access_point_name,city, address, phones: Vec::new(), tags:Vec::<Tag>::new() }
    }
}

impl fmt::Display for AccessPoint{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.address )
    }
}