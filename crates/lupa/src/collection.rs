pub use macros_utils::*;
use mongodb::{
    bson::{doc, Document},
    Collection,
};
pub trait Model{
    fn save<T>(&self, coll:Collection<T>)->bool{
        true
    }
}