
use crate::{
    model::{business::Business, city::City, country::Country},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use lupa::{query::MongoQuery, operators::QueryOperator};
use serde::Deserialize;
use serde_json::json;
pub enum CountryRepoError {
    NotFound,
    InvalidUsername,
}

pub enum AppError {
    /// Something went wrong when calling the user repo.
    CountryRepo(CountryRepoError),
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::CountryRepo(CountryRepoError::NotFound) => {
                (StatusCode::NOT_FOUND, "City not found")
            }
            AppError::CountryRepo(CountryRepoError::InvalidUsername) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Invalid username")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
impl IntoResponse for Country {
    fn into_response(self) -> Response {
        let country = self;
        (StatusCode::CREATED, Json(json!(country))).into_response()
    }
}
#[derive(Deserialize)]
pub struct CountryQuery {
    pub company: Option<String>,
    pub tags: Option<String>,
    pub city: Option<String>,
}

pub async fn get_country<'a>(
    State(mongo_client): State<mongodb::Client>,
    Path(country): Path<String>,
    Query(query): Query<CountryQuery>,
) -> Result<Country, AppError> {
    //TODO: validate the country database and search availability
    let collection = mongo_client
        .database(country.as_ref())
        .collection("businesses");
    let mut mongo_query: MongoQuery<Business> = MongoQuery::from(collection);
    let query_result = mongo_query
        .find(           
            QueryOperator::<String>::Eq(String::from("field_company"),query.company.unwrap_or(String::from(""))).into(),
        )
        .await.unwrap();
    match &query_result.results{
        Some(businesses) => Ok(Country::from(businesses.clone())),
        None => panic!(),
    }
}
