use std::pin::Pin;

use async_trait::async_trait;
use futures::stream::{StreamExt, TryStream, TryStreamExt};
use serde::de::DeserializeOwned;
pub use macros_utils::*;
use mongodb::{
    bson::{doc, Document},
    error::Error,
    Collection,
    options::FindOptions,
};
use crate::query::Neweable;



#[async_trait]
pub trait Model {
    fn get_id(&mut self) -> &String;
    fn tag_from(&mut self, paratext: &str) -> &mut Self;
    async fn find<T>(conn: &Collection<T>, filter: Document) -> Pin<Box<Result<Vec<T>, Error>>>
    where
        T: DeserializeOwned + Clone + Default+Neweable<T> + Unpin;
}


