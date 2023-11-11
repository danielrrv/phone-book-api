// use crate::{
//     model::{business::Business, city::City, country::Country},
// };
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::stream::Collect;
use lupa::{operators::QueryOperator, query::MongoQuery};
use mongodb::{bson::{doc, Document}, Collection};
use serde::{Deserialize, __private::doc};
use serde_json::json;

use crate::model::{
    business::{self, Business},
    tag::Tag,
};

pub enum BusinessError {
    BusinessNotFound(String),
}

impl IntoResponse for BusinessError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            BusinessError::BusinessNotFound(reason) => (StatusCode::NOT_FOUND, reason),
        };
        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
impl IntoResponse for Business {
    fn into_response(self) -> Response {
        let businesses = self;
        (StatusCode::CREATED, Json(json!(businesses))).into_response()
    }
}
#[derive(Deserialize)]
pub struct BussinessQuery {
    pub company: Option<String>,
    pub tags: Option<String>,
}

pub async fn get_businesses<'a>(
    State(mongo_client): State<mongodb::Client>,
    Path((country, city)): Path<(String, String)>,
    Query(query): Query<BussinessQuery>,
) -> Result<Json<Vec<Business>>, BusinessError> {
    //TODO: validate the country database and search availability
    let collection = mongo_client
        .database(country.as_ref())
        .collection("businesses");
    let mut mongo_query: MongoQuery<Business> = MongoQuery::from(collection);
    let tags: Vec<Document> = query
        .tags
        .unwrap()
        .split(",")
        .map(|tag| doc! {"tags":tag})
        .collect();
    let query_result = mongo_query
        .find(QueryOperator::<String>::Or(tags).into())
        .await
        .unwrap();

    match &query_result.results {
        Some(businesses) => Ok(Json(businesses.clone())),
        None => panic!(),
    }
}
