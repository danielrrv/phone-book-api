use std::marker::PhantomData;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::stream::Collect;
use lupa::{collection::Model, operators::QueryOperator, query::MongoQuery};
use mongodb::{
    bson::{doc, Document},
    Collection,
};
use serde::Deserialize;
use serde_json::json;

use crate::model::{
    access_point::AccessPoint,
    business::{self, Business},
    tag::Tag,
};

#[derive(Debug)]
pub enum BusinessError {
    BusinessNotFound(String),
    TagNotProvided,
    CompanyNameAbsent,
    DescriptionAbsent,
}

impl IntoResponse for BusinessError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            BusinessError::BusinessNotFound(reason) => (StatusCode::NOT_FOUND, reason),
            BusinessError::TagNotProvided => {
                (StatusCode::BAD_REQUEST, String::from("Not tag provided"))
            }
            BusinessError::CompanyNameAbsent => (
                StatusCode::BAD_REQUEST,
                String::from("Company name is required."),
            ),
            BusinessError::DescriptionAbsent => (
                StatusCode::BAD_REQUEST,
                String::from("Description is required."),
            ),
        };
        let body = Json(json!({
            "error": &error_message,
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

#[derive(Deserialize)]
pub struct BusinessUpdateJsonRequest {
    pub company_name: Option<String>,
    pub locations: Vec<AccessPoint>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl Default for BusinessUpdateJsonRequest {
    fn default() -> Self {
        Self {
            company_name: None,
            locations: Vec::new(),
            description: None,
            tags: Vec::new(),
        }
    }
}

pub async fn get_businesses<'a>(
    State(mongo_client): State<mongodb::Client>,
    Path((country, city)): Path<(String, String)>,
    Query(query): Query<BussinessQuery>,
) -> Result<Json<Vec<Business>>, BusinessError> {
    //TODO: validate the country database and search availability
    let mut mongo_query: MongoQuery<Business> = MongoQuery::from(
        mongo_client
            .database(country.as_ref())
            .collection("businesses"),
    );
    let mut tags: Vec<Document> = query
        .tags
        .ok_or(BusinessError::TagNotProvided)?
        .split(",")
        .map(|tag| doc! {"tags":tag})
        .collect();
    let city_doc: Document = QueryOperator::<String, String>::ElemMatch(doc! {"city": city}).into();

    tags.push(doc! {"locations": city_doc});

    let query_result = mongo_query
        .find(QueryOperator::<String, String>::And(tags).into())
        .await
        .unwrap();

    match &query_result.results {
        Some(businesses) => Ok(Json(businesses.clone())),
        None => panic!(),
    }
}

pub async fn put_business(
    State(mongo_client): State<mongodb::Client>,
    Path(country): Path<String>,
    request_body: Option<Json<BusinessUpdateJsonRequest>>,
) -> Result<Json<Business>, BusinessError> {
    let mut mongo_query: MongoQuery<Business> = MongoQuery::from(
        mongo_client
            .database(country.as_ref())
            .collection("businesses"),
    );
    let Json(request_body) = request_body.unwrap_or_default();
    let company_name = request_body
        .company_name
        .ok_or(BusinessError::CompanyNameAbsent)
        .unwrap();

    let query_result = mongo_query
        .find(QueryOperator::<String, String>::Eq("company_name", &company_name).into())
        .await
        .unwrap();

    match &query_result.results {
        Some(businesses) => match businesses.first() {
            Some(business) => {
                println!("Business Found");
                let mut existing_business = business.clone();
                if let Some(description) = request_body.description {
                    existing_business.add_description(description);
                }
                for location in request_body.locations {
                    existing_business.add_location(location);
                }
                let bi = mongo_query.save(&existing_business).await.unwrap();

                // existing_business.save(business_collection.clone());
                return Ok(Json(bi.to_owned()));
            }
            None => {
                println!("New Business");
                let mut new_bussiness = Business::new(company_name);
                if let Some(description) = request_body.description {
                    new_bussiness.add_description(description);
                }
                for location in request_body.locations {
                    new_bussiness.add_location(location);
                }
                let bi = mongo_query.save(&new_bussiness).await.unwrap();
                // new_bussiness.save(business_collection.clone());
                return Ok(Json(bi.to_owned()));
            }
        },
        None => panic!(),
    }
}
