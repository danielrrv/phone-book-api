pub use macros_utils::*;
use mongodb::{
    bson::{doc, Document},
    Collection,
};
pub trait Model{
    fn get_id(&mut self)-> &String;
    fn tag_from(&mut self, paratext: &str)->&mut Self;
}