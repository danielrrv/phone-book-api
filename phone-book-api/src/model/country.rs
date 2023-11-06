use crate::data::countrties_repository::COUNTRIES_DATA;
use crate::model::business::Business;

use async_trait::async_trait;
use futures::stream::{StreamExt, TryStream, TryStreamExt};
use mongodb::{
    bson::{doc, Document},
    error,
    options::FindOptions,
    Client, Collection, Cursor,
};
use serde;
use std::{collections::VecDeque, fmt};

// macro_rules! find_by {
//     ($name: ident, $name_type:ty) => {
//         paste! { pub fn [<find_by_$name>]($name: $name_type)->Option<&'static Self>{
//                 COUNTRIES_DATA.to_vec().iter().filter(|country| country.$name == $name).nth(0)
//             }
//         }
//     };
// }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue {
    Value { value: String },
    Operator { operator: String, value: String },
}

impl Default for FieldValue {
    fn default() -> Self {
        Self::Value {
            value: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Field {
    field_name: String,
    field_value: FieldValue,
}

impl Field {
    fn new(field_name: String, field_value: FieldValue) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Query {
    entity: String,
    fields: Vec<Field>,
}
impl Default for Query {
    fn default() -> Self {
        Self {
            entity: String::from("businesses"),
            fields: Vec::new(),
        }
    }
}

impl Query {
    fn set_entity(mut self, value: String) -> Self {
        self.entity = value.clone();
        self
    }
    fn set_field<'c>(mut self, value: Field) -> Self {
        if !self.fields.contains(&value) {
            self.fields.push(value)
        }
        self
    }
    fn to_filter(&self) -> VecDeque<Option<mongodb::bson::Document>> {
        self.fields
            .clone()
            .into_iter()
            .map(|field| match field.field_value {
                FieldValue::Value { value } => {
                   
                    Some(doc! {field.field_name :  value})
                }
                FieldValue::Operator { operator, value } => {
                
                    Some(doc! {format!("{}", operator): format!("{}", value)})
                }
                _ => None,
            })
            .collect()
    }
}
#[async_trait]
pub trait Model<T> {
    fn when(&mut self, field_name: String, field_value: String) -> &mut T;
    fn from_collection(&mut self, entity: String) -> &mut T;
    async fn execute(&mut self) -> Result<&Country, mongodb::error::Error>;
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Country {
    #[serde(skip)]
    pub _client: Option<mongodb::Database>,
    #[serde(skip)]
    pub _query: Option<Query>,
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
            _query: Some(Query::default()),
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
            _query: Default::default(),
            company_name: value,
            code: Default::default(),
            businesses: Default::default(),
        }
    }
}

// #[async_trait]
impl Country {
    pub fn when<'c>(&'c mut self, field_name: String, field_value: String) -> &'c mut Self {
        let query = match self._query.clone() {
            Some(query) => Some(query.set_field(Field::from(Field::new(
                field_name,
                FieldValue::Value { value: field_value },
            )))),
            _ => panic!("No query provided."),
        };
        self._query = query;
        self
    }
    pub fn from_collection(&mut self, entity: String) -> &mut Self {
        self._query.clone().unwrap().set_entity(entity.to_string());
        self
    }

    pub async fn execute(&mut self) -> Result<Country, mongodb::error::Error> {
        let raw_filter = self._query.to_owned().unwrap().to_filter();
        let filters: Vec<_> = Vec::from(raw_filter)
            .iter()
            .map(|f| f.clone().unwrap())
            .collect();
        println!("{:?}", filters);
        // type Output = Result<Cursor<Business>, mongodb::error::Error>;
        let businesses_col: Collection<Business> = self
            .clone()
            ._client
            .unwrap()
            .collection(&self._query.as_ref().unwrap().entity);
      
        // let opts = FindOptions::builder()
        //     .projection(Some(doc! {"description": 0}))
        //     .build();
        let mut cursor = businesses_col.find(doc! {"$and": filters}, None).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(value) => {
              
                    self.businesses.push(value)
                }
                Err(_error) => {
                    println!("Error")
                }
            }
        }
        Ok(self.clone())
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
