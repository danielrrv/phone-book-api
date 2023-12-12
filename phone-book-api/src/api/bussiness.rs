use std::marker::PhantomData;

use axum::{
    extract::{Path, Query, State},
    http::{status, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use futures::stream::Collect;
use lupa::{collection::Model, operators::QueryOperator, query::MongoQuery};
use mongodb::{
    bson::{doc, to_document, Document},
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
    GenericDatabaseError(mongodb::error::Error),
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
            BusinessError::GenericDatabaseError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
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

#[derive(Deserialize, Debug)]
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
    let city_doc: Document = QueryOperator::<String>::ElemMatch(doc! {"city": city}).into();

    tags.push(doc! {"locations": city_doc});

    let query_result = mongo_query
        .find(QueryOperator::<String>::And(tags).into())
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
    Json(request_body): Json<BusinessUpdateJsonRequest>,
) -> Result<Json<Business>, BusinessError> {
    let mut mongo_query: MongoQuery<Business> = MongoQuery::from(
        mongo_client
            .database(country.as_ref())
            .collection("businesses"),
    );
    println!("{:?}", request_body);
    let company_name = request_body
        .company_name
        .ok_or(BusinessError::CompanyNameAbsent)?;
    
    let query_result = mongo_query
        .find(QueryOperator::<String>::Eq("company_name", &company_name).into())
        .await
        .unwrap();
    
    match &query_result.results {
        Some(businesses) => match businesses.first() {
            Some(business) => {
                let mut existing_business = business.clone();
                if let Some(description) = request_body.description {
                    existing_business.add_description(&description);
                }
                for mut location in request_body.locations {
                    let name = location.name.clone();
                    location.tag_from(&name);
                    existing_business.add_location(&location);
                }
                match mongo_query
                    .update(
                        doc! {"_id": existing_business.clone()._id},
                        &existing_business,
                    )
                    .await
                {
                    Ok(business) => Ok(Json(business.to_owned())),
                    Err(error) => Err(BusinessError::GenericDatabaseError(error)),
                }
            }
            None => {
                let mut new_bussiness = Business::new(company_name);
                if let Some(description) = request_body.description {
                    new_bussiness
                        .add_description(&description)
                        .tag_from(&description);
                }
                for mut location in request_body.locations {
                    let name = location.name.clone();
                    location.tag_from(&name);
                    new_bussiness.add_location(&location);
                }
                match mongo_query.save(&new_bussiness).await {
                    Ok(business) => Ok(Json(business.to_owned())),
                    Err(error) => Err(BusinessError::GenericDatabaseError(error)),
                }
            }
        },
        None => panic!(),
    }
}
