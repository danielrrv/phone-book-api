


// use async_trait::async_trait;
use serde;
use std::{collections::VecDeque, fmt};
use futures::stream::{StreamExt, TryStream, TryStreamExt};
use mongodb::{
    bson::{doc, Document},
    error,
    options::FindOptions,
    Client, Collection, Cursor,
};

use crate::model::business::Business;

// macro_rules! find_by {
//     ($name: ident, $name_type:ty) => {
//         paste! { pub fn [<find_by_$name>]($name: $name_type)->Option<&'static Self>{
//                 COUNTRIES_DATA.to_vec().iter().filter(|country| country.$name == $name).nth(0)
//             }
//         }
//     };
// }



#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Country {
    #[serde(skip)]
    pub _client: Option<mongodb::Database>,
    #[serde(skip)]
    pub company_name: String,
    pub code: u16,
    pub businesses: Vec<Business>,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.company_name, self.code)
    }
}

impl From<mongodb::Database> for Country {
    fn from(value: mongodb::Database) -> Self {
        Self {
            _client: Some(value),
            company_name: Default::default(),
            code: Default::default(),
            businesses: Default::default(),
        }
    }
}

impl From<String> for Country {
    fn from(value: String) -> Self {
        Self {
            _client: None,
            company_name: value,
            code: Default::default(),
            businesses: Default::default(),
        }
    }
}
impl From<Vec<Business>> for Country {
    fn from(value: Vec<Business>) -> Self {
        Self {
            _client: None,
            company_name: String::from("Avianca"),
            code: Default::default(),
            businesses: value,
        }
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    // #[test]
    // fn test_add() {
    //     assert_eq!(add(1, 2), 3);
    // }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
