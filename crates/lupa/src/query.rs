extern crate tracing;
use crate::collection::Model;
use crate::operators;
use std::collections::HashMap;
use std::ops::Deref;

use futures::stream::{StreamExt, TryStream, TryStreamExt};

use mongodb::bson::doc;
use mongodb::bson::Bson;

use mongodb::bson::from_document;
use mongodb::bson::to_document;
use mongodb::bson::Document;
use mongodb::options::FindOptions;
use mongodb::options::UpdateModifications;
use mongodb::Collection;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::operators::QueryOperator;

type MongoResult<'a, T> = Result<&'a mut MongoQuery<T>, mongodb::error::Error>;
pub trait Neweable<T> {
    fn new() -> T;
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

impl<R> MongoQuery<R>
where
    R: DeserializeOwned,
{
    pub async fn find(&mut self, filter: Document) -> Result<&mut Self, mongodb::error::Error>
    where
        R: DeserializeOwned + Clone,
    {
        let opts = FindOptions::builder().batch_size(50).build();
        let cursor = &mut self
            .collection
            .as_ref()
            .expect("No collection associated")
            .find(filter, opts)
            .await?;
        let mut container: Vec<R> = Vec::with_capacity(1000);
        while let Some(result) = cursor.next().await {
            match result {
                Ok(value) => container.push(value),
                Err(error) => panic!("{}", error),
            }
        }
        self.results = Some(container);
        Ok(self)
    }
    pub async fn save<'a>(&mut self, entity: &'a R) -> Result<&'a R, mongodb::error::Error>
    where
        R: Clone + Serialize,
    {
        let insert_one_result = self
            .collection
            .as_ref()
            .expect("No collection associated")
            .insert_one(entity, None)
            .await?;

        println!(
            "Inserted document with _id: {}",
            insert_one_result.inserted_id
        );
        Ok(entity)
    }

    pub async fn update<'a>(
        &mut self,
        filter: Document,
        entity: &'a R,
    ) -> Result<&'a R, mongodb::error::Error>
    where
        R: Serialize,
    {
        let model = to_document(&entity).unwrap();
        let update_doc: Document = QueryOperator::<String>::Set(model).into();
        let insert_one_result = self
            .collection
            .as_ref()
            .expect("No collection associated")
            .update_one(filter, update_doc, None)
            .await?;

        println!(
            "Inserted document with _id: {:?}",
            insert_one_result.upserted_id
        );
        Ok(entity)
    }

    pub fn scoped(&mut self, by: &str, when: &str) -> Result<&mut Self, ()> {
        let _nested: Vec<&str> = by.split(".").collect();
        // let mut results = self.results.unwrap();
        // results.iter().filter(|element|element. )

        todo!()
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
            results: Some(Vec::new()),
        }
    }
}
