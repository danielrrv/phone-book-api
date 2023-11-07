extern crate tracing;
use std::collections::HashMap;

use futures::stream::{StreamExt, TryStream, TryStreamExt};
use futures::Stream;
use mongodb::bson::doc;

use mongodb::bson::Document;
use mongodb::options::FindOptions;
use mongodb::Collection;
use serde::de::{value, DeserializeOwned};

#[derive(Clone, Debug)]
pub struct MongoQuery<T>
where
    T: DeserializeOwned,
{
    entity: Option<String>,
    collection: Option<mongodb::Collection<T>>,
    fields: Vec<Field>,
    docs: HashMap<String, Document>,
}

impl<T> From<Collection<T>> for MongoQuery<T>
where
    T: DeserializeOwned,
{
    fn from(collection: Collection<T>) -> Self {
        Self {
            collection: Some(collection.clone()),
            entity: Some(collection.name().to_string()),
            fields: Default::default(),
            docs: Default::default(),
        }
    }
}

pub enum MongoQueryError {
    QueryNotFound,
    BadQuery,
}

impl<T> MongoQuery<T>
where
    T: DeserializeOwned,
{
    //Operator
    pub fn bson_in(values: Vec<String>) -> Document {
        doc! {"$in": values}
    }
    pub fn bson_eq(values: String) -> Document {
        doc! {"$eq": values}
    }

    pub async fn find(
        &self,
        field: String,
        filter: Document,
    ) -> Result<Vec<T>, mongodb::error::Error>
    where
        T: DeserializeOwned,
    {
        
        let opts = FindOptions::builder().batch_size(50).build();
        let mut cursor = self
            .collection
            .clone()
            .expect("No collection associated")
            .find(doc! {field: filter}, opts)
            .await?;
        let mut container: Vec<T> = Vec::with_capacity(1000);
        while let Some(result) = cursor.next().await {
            match result {
                Ok(value) => container.push(value),
                Err(error) => panic!("{}", error),
            }
        }
        Ok(container)

        // Err(mongodb::error::Error::custom("Unable to wrap the query".to_string()))
    }
}

impl<T> Default for MongoQuery<T>
where
    T: DeserializeOwned,
{
    fn default() -> Self {
        Self {
            collection: None,
            entity: None,
            fields: Vec::new(),
            docs: HashMap::new(),
        }
    }
}

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
