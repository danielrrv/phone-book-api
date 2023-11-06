#[macro_export]
macro_rules! field {
    ($field_name: ident, $field_type: ty) => {
        $field_name: $field_type,
    };
}

// #[macro_export]
// macro_rules! model{

//     ($class_name:ident, $({$field:ident :$type_field:ty})*) => {
//         pub struct $class_name{
//             field!($type, &type_field)
//         }
//         // impl $class_name{}
//     }
// }

// macro_rules! model{
//     (
//      $(#[$meta:meta])*
//      struct $struct_name:ident {
//         $(
//         $(#[$field_meta:meta])*
//         $field_vis:vis $field_name:ident : $field_type:ty
//         ),*$(,)+
//     }
//     ) => {

//             $(#[$meta])*
//             pub struct $struct_name{
//                 $(
//                 $(#[$field_meta:meta])*
//                 pub $field_name : $field_type,
//                 )*
//             }

//             impl $struct_name {
//                 pub(crate) fn new($($field_name : $field_type)*) -> Self {
//                     Self { $($field_name)* }
//                 }
//             }
//     }
// }

// model!(Home, {name: String});

// impl Home {
//     // pub fn new(name: String, song: String)->Self{
//     //     Self{ name, song}
//     // }
// }

// #[derive(Debug)]
// basic handler that responds with a static string
pub mod handlers {
    use crate::{
        data::countrties_repository::COUNTRIES_DATA,
        model::{business::Business, city::City, country::Country},
    };
    use axum::{
        extract::{Path, Query, State},
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use futures::{join, try_join};
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
            (StatusCode::OK, Json(json!(country))).into_response()
        }
    }
    // use crate::model::country::Country;
    // pub async fn country_with_businesses<'a>(
    //     State(_mongo_client): State<mongodb::Client>,
    //     Path(_country): Path<String>,
    //     Query(_businesses): Query<String>,
    // ) -> Result<Json<Country>, AppError> {
    //     //Look up the country information.
    //     //Country's featured businesses
    //     // let country = Country::find_by_name(String::from("Colombia")).unwrap();
    //     Ok(Json(country.to_owned()))
    // }
    // fn internal_error<E>(err: E) -> (StatusCode, String)
    // where
    //     E: std::error::Error,
    // {
    //     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    // }
    pub async fn get_country<'a>(
        State(mongo_client): State<mongodb::Client>,
        Path(_country): Path<String>,
    ) -> Result<Country, AppError> {
        let country = Country::from(mongo_client.database(_country.as_ref()))
            .when("company_name".to_string(), "Avianca".to_string())
            .from_collection("businesses".to_string())
            .execute()
            .await;
        match country {
            Ok(value)=>Ok(value),
            Err(_error)=>{
                println!("{}", _error);
                Err(AppError::CountryRepo(CountryRepoError::InvalidUsername))
            }
        }
    }
}
