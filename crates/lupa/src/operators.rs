use mongodb::bson::{doc, Bson, Document};
use std::{fmt, str::FromStr};


macro_rules! empty {
    () => {
       String::from("") 
    };
}

#[derive(Clone, Debug)]
pub enum QueryOperator<T> {
    /// Matches values that are equal to a specified value.
    Eq(String,String),
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
    /// Joins query clauses with a logical AND returns all documents that match the conditions of both clauses.
    And,
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
}
impl<T> QueryOperator<T> {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match &*self {
            QueryOperator::Eq(_,__) => "$eq",
            QueryOperator::Gt(_) => "$gt",
            QueryOperator::Gte(_) => "$gte",
            QueryOperator::In(_) => "$in",
            QueryOperator::Lt(_) => "$lt",
            QueryOperator::Lte(_) => todo!(),
            QueryOperator::Ne(_) => todo!(),
            QueryOperator::Nin => todo!(),
            QueryOperator::And => todo!(),
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
            QueryOperator::ElemMatch(_) => "$elementMatch",
            QueryOperator::Size(_) => todo!(),
            QueryOperator::BitsAllClear => todo!(),
            QueryOperator::BitsAllSets => todo!(),
            QueryOperator::BitsAnyClear => todo!(),
            QueryOperator::BitsAnySet => todo!(),
            _ => panic!("Unknown operator"),
        }
    }
}

impl<T> fmt::Display for QueryOperator<T> {
    // #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
impl<T> Into<Document> for QueryOperator<T>
where
    T: Into<Bson> + Clone,
{
    fn into(self) -> Document {
        match self {
            QueryOperator::Eq(ref field, ref value) => doc! {field:{self.as_str(): value}},
            QueryOperator::Gt(ref value) => doc! {self.as_str(): value},
            QueryOperator::Gte(ref value) => doc! {self.as_str(): value},
            QueryOperator::In(ref values) => doc! {self.as_str(): values},
            QueryOperator::Lt(ref _value) => todo!(),
            QueryOperator::Lte(ref value) => todo!(),
            QueryOperator::Ne(ref value) => todo!(),
            QueryOperator::Nin => todo!(),
            QueryOperator::And => todo!(),
            QueryOperator::Or(ref docs) => doc! {self.as_str(): docs},
            QueryOperator::Not => todo!(),
            QueryOperator::Nor => todo!(),
            QueryOperator::Expr => todo!(),
            QueryOperator::JsonSchema => todo!(),
            QueryOperator::Mod => todo!(),
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
        }
    }
}
impl<T> FromStr for QueryOperator<T> {
    type Err = ();
    fn from_str(value: &str) -> Result<QueryOperator<T>, ()> {
        match value {
            v if QueryOperator::<T>::Eq(empty!(), value.to_owned()).as_str() == value => {
                Ok(QueryOperator::<T>::Eq(empty!(),v.to_owned()))
            }
            _ => Err(()),
        }
    }
}

// impl<T> From<T> for QueryOperator<T>
// where
//     T: Into<Vec<T>> + Sized,
//     T: Into<String>,
// {
//     fn from(value: T) -> Self {
//         let value: Vec<T> = value.into();

//         match value {

//         }
//         todo!()
//     }
// }

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
