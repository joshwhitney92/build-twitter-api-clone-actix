use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use derive_more::{Display, Error};


// NOTE: Implementation of some basic error handling with custom error types.

// NOTE: 1) Define our custom error type
#[allow(unused)]
#[derive(Debug, Display, Error)]
pub enum MyError {
    // NOTE: using derive_more crate to fill out extra \
    // information we want in the http response payload.
    #[display(fmt = "Internal Server Error")]
    InternalError,

    // NOTE: Adding the field name so we can output it in the error message.
    #[display(fmt = "A field value is invalid {}", field)]
    ValidationError { field: String },

    #[display(fmt = "An unknown error has occured")]
    UnknownError,
}

// NOTE: 2) Implement the ResponseError type for our custom error.
// This will translate our custom type to the appropriate actix_web response type.
impl ResponseError for MyError {

    // NOTE:  Take our custom type and show how to convert it \
    // to a type that actix web understands.
    fn status_code(&self) -> actix_web::http::StatusCode {
        // NOTE: Dereference the error type here so we can get the actual value of the error type.
        match *self {
            MyError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            MyError::ValidationError { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            MyError::UnknownError { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // Build the HTTPResponse and return it.
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::plaintext())
            // NOTE: The `derive_more` crate allows us to call `.to_string()` \
            // and use the message we defined above with the `display` macros.
            .body(self.to_string())
    }
}

/// Handler that will respond to our get requests.
pub async fn get() -> Result<String, actix_web::Error> {
    Err(MyError::ValidationError {
        field: "full_name".to_string(),
    }
    .into())
}
