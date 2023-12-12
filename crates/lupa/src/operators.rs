use mongodb::{
    bson::{doc, Bson, Document},
    options::UpdateModifications,
};
use serde::{Serialize, Deserialize};
use std::{fmt, str::FromStr, marker::PhantomData};

macro_rules! empty {
    () => {
        ""
    };
}

#[derive(Clone, Debug)]
pub enum QueryOperator<'x, T>
{
    /// Matches values that are equal to a specified value.
    Eq(&'x str, &'x str),
    /// Matches values that are greater than a specified value.
    Gt(String),
    /// Matches values that are greater than or equal to a specified value.
    Gte(String),
    /// Matches any of the values specified in an array.
    In(Vec<T>),
    /// Matches values that are less than a specified value.
    Lt(String),
    /// Matches values that are less than or equal to a specified value.
    Lte(String),
    /// Matches all values that are not equal to a specified value.
    Ne(String),
    /// Matches none of the values specified in an array.
    Nin,
    /// Matches documents that have the specified field.
    Exists,
    /// Joins query clauses with a logical AND returns all documents that match the conditions of both clauses.
    And(Vec<Document>),
    /// Inverts the effect of a query expression and returns documents that do not match the query expression.
    Or(Vec<Document>),
    /// Joins query clauses with a logical OR returns all documents that match the conditions of either clause.
    Not,
    /// Joins query clauses with a logical NOR returns all documents that fail to match both clauses.
    Nor,
    /// Allows use of aggregation expressions within the query language.
    Expr,
    /// Validate documents against the given JSON Schema.
    JsonSchema,
    /// Performs a modulo operation on the value of a field and selects documents with a specified result.
    Mod,
    /// Selects documents where values match a specified regular expression.
    Regex(String),
    /// Performs text search.
    Text(String),
    /// Matches documents that satisfy a JavaScript expression.
    Where,
    /// Matches arrays that contain all elements specified in the query.
    All,
    /// Selects documents if element in the array field matches all the specified $elemMatch conditions.
    ElemMatch(Document),
    /// Selects documents if the array field is a specified size.
    Size(u64),
    /// Matches numeric or binary values in which a set of bit positions all have a value of 0.
    BitsAllClear,
    /// Matches numeric or binary values in which a set of bit positions all have a value of 1.
    BitsAllSets,
    /// Matches numeric or binary values in which any bit from a set of bit positions has a value of 0.
    BitsAnyClear,
    /// Matches numeric or binary values in which any bit from a set of bit positions has a value of 1.
    BitsAnySet,
    Set(Document),
}

    
impl<'x, T> QueryOperator<'x, T> {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match &*self {
            QueryOperator::Eq(_, __) => "$eq",
            QueryOperator::Gt(_) => "$gt",
            QueryOperator::Gte(_) => "$gte",
            QueryOperator::In(_) => "$in",
            QueryOperator::Lt(_) => "$lt",
            QueryOperator::Lte(_) => todo!(),
            QueryOperator::Ne(_) => todo!(),
            QueryOperator::Nin => todo!(),
            QueryOperator::And(_) => "$and",
            QueryOperator::Or(_) => "$or",
            QueryOperator::Not => todo!(),
            QueryOperator::Nor => todo!(),
            QueryOperator::Expr => todo!(),
            QueryOperator::JsonSchema => todo!(),
            QueryOperator::Mod => todo!(),
            QueryOperator::Regex(_) => todo!(),
            QueryOperator::Text(_) => todo!(),
            QueryOperator::Where => todo!(),
            QueryOperator::All => todo!(),
            QueryOperator::ElemMatch(_) => "$elemMatch",
            QueryOperator::Size(_) => todo!(),
            QueryOperator::BitsAllClear => todo!(),
            QueryOperator::BitsAllSets => todo!(),
            QueryOperator::BitsAnyClear => todo!(),
            QueryOperator::BitsAnySet => todo!(),
            QueryOperator::Set(_) => "$set",
            _ => panic!("Unknown operator"),
        }
    }
}

impl<'x, T> fmt::Display for QueryOperator<'x, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'x, T> Into<UpdateModifications>
    for QueryOperator<'x, T>
where
    T: Into<Bson> + Clone
{
    fn into(self) -> UpdateModifications {
        match self {
            QueryOperator::Eq(_, _) => todo!(),
            QueryOperator::Gt(_) => todo!(),
            QueryOperator::Gte(_) => todo!(),
            QueryOperator::In(_) => todo!(),
            QueryOperator::Lt(_) => todo!(),
            QueryOperator::Lte(_) => todo!(),
            QueryOperator::Ne(_) => todo!(),
            QueryOperator::Nin => todo!(),
            QueryOperator::Exists => todo!(),
            QueryOperator::And(_) => todo!(),
            QueryOperator::Or(_) => todo!(),
            QueryOperator::Not => todo!(),
            QueryOperator::Nor => todo!(),
            QueryOperator::Expr => todo!(),
            QueryOperator::JsonSchema => todo!(),
            QueryOperator::Mod => todo!(),
            QueryOperator::Regex(_) => todo!(),
            QueryOperator::Text(_) => todo!(),
            QueryOperator::Where => todo!(),
            QueryOperator::All => todo!(),
            QueryOperator::ElemMatch(_) => todo!(),
            QueryOperator::Size(_) => todo!(),
            QueryOperator::BitsAllClear => todo!(),
            QueryOperator::BitsAllSets => todo!(),
            QueryOperator::BitsAnyClear => todo!(),
            QueryOperator::BitsAnySet => todo!(),
            QueryOperator::Set(doc) => UpdateModifications::Document(doc),
        }
    }
}
impl<'x, T> Into<Document> for QueryOperator<'x, T>
where
    T: Into<Bson> + Clone
{
    fn into(self) -> Document {
        match self {
            QueryOperator::Eq(field, value) => doc! {field:{self.as_str(): value}},
            QueryOperator::Gt(ref value) | QueryOperator::Gte(ref value) => {
                doc! {self.as_str(): value}
            }

            QueryOperator::Lt(ref value)
            | QueryOperator::Lte(ref value)
            | QueryOperator::Ne(ref value) => todo!(),
            QueryOperator::Nin => todo!(),
            QueryOperator::In(ref docs) => doc! {self.as_str(): docs},
            QueryOperator::And(ref docs) | QueryOperator::Or(ref docs) => {
                doc! {self.as_str(): docs}
            }
            QueryOperator::Not => todo!(),
            QueryOperator::Nor => todo!(),
            QueryOperator::Expr => todo!(),
            QueryOperator::JsonSchema => todo!(),
            QueryOperator::Mod => todo!(),
            QueryOperator::Exists => todo!(),
            QueryOperator::Regex(ref value) => todo!(),
            QueryOperator::Text(ref value) => todo!(),
            QueryOperator::Where => todo!(),
            QueryOperator::All => todo!(),
            QueryOperator::ElemMatch(ref doc) => doc! {self.as_str(): doc},
            QueryOperator::Size(ref value) => todo!(),
            QueryOperator::BitsAllClear => todo!(),
            QueryOperator::BitsAllSets => todo!(),
            QueryOperator::BitsAnyClear => todo!(),
            QueryOperator::BitsAnySet => todo!(),
            QueryOperator::Set(ref doc) => {
                doc! {self.as_str(): doc }
            },
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::QueryOperator;
//     use std::str::FromStr;

//     #[test]
//     fn operator_from_str() {
//         let eq = QueryOperator::<u64>::from_str("$eq").unwrap();
//         assert_eq!(
//             eq.as_str(),
//             QueryOperator::<u64>::Eq(String::from("")).as_str()
//         )
//     }
// }
