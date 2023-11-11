extern crate tracing;
use std::collections::HashMap;
use std::ops::Deref;

use futures::stream::{StreamExt, TryStream, TryStreamExt};

use mongodb::bson::doc;

use mongodb::bson::Document;
use mongodb::options::FindOptions;
use mongodb::Collection;
use serde::de::{ DeserializeOwned};

pub trait Neweable<T>{
    fn new()->T;
}

// pub enum MongoQueryResult<T, U>{
//     Completed(T),// Returns the expected type of the query
//     Partial(U, T),// Returns self and the expected type of the query.
// }


#[derive(Clone, Debug)]
pub struct MongoQuery<T>
where
    T: DeserializeOwned,
{
    entity: Option<String>,
    pub results: Option<Vec<T>>,
    collection: Option<mongodb::Collection<T>>,
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
            docs: Default::default(),
            results: None,
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
    pub async fn find(&mut self,  filter: Document) -> Result<&mut Self, mongodb::error::Error>
    where
        T: DeserializeOwned + Clone,
    
    {
        let opts = FindOptions::builder().batch_size(50).build();
        let cursor = &mut self
            .collection
            .as_ref()
            .expect("No collection associated")
            .find(filter, opts)
            .await?;
        let mut container: Vec<T> = Vec::with_capacity(1000);
        while let  Some(result) = cursor.next().await {
            match result {
                Ok(value) => container.push(value),
                Err(error) => panic!("{}", error),
            }
        }
        self.results = Some(container);
        Ok(self)
    }
    
    pub async fn scoped(&mut self)->Result<&mut Self, mongodb::error::Error> {

        Ok(self)
    }
}

impl<T> Default for MongoQuery<T>
where
    T: DeserializeOwned + Neweable<T>,
{
    fn default() -> Self {
        Self {
            collection: None,
            entity: None,
            docs: HashMap::new(),
            results: Some(Vec::new())
        }
    }
}
