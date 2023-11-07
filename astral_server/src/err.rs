use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::json;
use thiserror::Error;
use utoipa::openapi::{ContentBuilder, ObjectBuilder, RefOr, ResponseBuilder, SchemaType};
use utoipa::ToResponse;

/// The error container
#[derive(Debug, Error)]
pub enum AstralError {
    /// Anyhow-provoked error
    #[error("{0}")]
    Unknown(#[from] anyhow::Error),
    /// Mongo-provoked error
    #[error("An error with DB has occurred: {0}")]
    Database(#[from] mongodb::error::Error),
    /// Bad client reuqest
    #[error("Invalid request: {0}")]
    BadRequest(String),
    /// Resource could not be found
    #[error("Not found: {0}")]
    NotFound(String),
    /// Client is not authorized to access this endpoint
    #[error("You are unauthorized to access this endpoint: {0}")]
    Unauthorized(String),
    /// IO error. Most likely related to streaming
    #[error("An error has occurred within the IO: {0}")]
    IOError(#[from] std::io::Error),

    /// Metaflac error
    #[error("An error has occurred when reading Flac metadata: {0}")]
    FlacError(#[from] metaflac::Error),
    /// ID3 error
    #[error("An error has occurred when reading Mp3 (id3) metadata: {0}")]
    Id3Error(#[from] id3::Error),
    /// Mp4ameta error
    #[error("An error has occurred when reading M4A metadata: {0}")]
    M4aError(#[from] mp4ameta::Error),
    /// Reqwest Error
    #[error("An error occurred when fetching API internally: {0}")]
    ReqwestError(#[from] reqwest::Error),
    /// JSON error
    #[error("An error occurred when parsing JSON: {0}")]
    JsonError(#[from] serde_json::Error),
}

// <editor-fold defaultstate="collapsed" desc="impl macro">
macro_rules! error_impls {
    (
        $(
        $id:ident: ($code:ident, $ty:literal)
        );* $(;)?
    ) => {
        impl<'r> ToResponse<'r> for AstralError {
            fn response() -> (&'r str, RefOr<utoipa::openapi::Response>) {
                (
                    "AstralError",
                    ResponseBuilder::new().description("An error has occurred on the server or with your request").content(
                        "application/json", ContentBuilder::new().schema(
                            ObjectBuilder::new()
                                .property("error_type", ObjectBuilder::new().schema_type(SchemaType::String).example(Some(json!("unauthorized"))).enum_values(Some([$( $ty ),*])).description(Some("Type of error that occurred")))
                                .property("message", ObjectBuilder::new().schema_type(SchemaType::String).example(Some(json!("An unknown error has occurred"))).description(Some("Message regarding this error")))
                                .build()
                        ).build()
                    ).build().into()
                )
            }
        }
        impl IntoResponse for AstralError {
            fn into_response(self) -> Response {
                let code = match &self {
                    $(
                        Self::$id(_) => axum::http::StatusCode::$code
                    ),*
                };
                let json = Json(self);
                (code, json).into_response()
            }
        }
        impl Serialize for AstralError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                let mut str = serializer.serialize_struct("AstralError", 2)?;
                let ty = match self {
                    $(
                        Self::$id(_) => $ty
                    ),*
                };
                str.serialize_field("error_type", ty)?;
                str.serialize_field("message", &self.to_string())?;
                str.end()
            }
        }
    };
}
// </editor-fold>

error_impls! {
    Unknown: (INTERNAL_SERVER_ERROR, "unknown");
    Database: (INTERNAL_SERVER_ERROR, "database");
    BadRequest: (BAD_REQUEST, "bad_request");
    NotFound: (NOT_FOUND, "not_found");
    Unauthorized: (UNAUTHORIZED, "unauthorized");
    IOError: (INTERNAL_SERVER_ERROR, "io");

    FlacError: (INTERNAL_SERVER_ERROR, "flac");
    Id3Error: (INTERNAL_SERVER_ERROR, "id3");
    M4aError: (INTERNAL_SERVER_ERROR, "m4a");
    ReqwestError: (INTERNAL_SERVER_ERROR, "reqwest");
    JsonError: (INTERNAL_SERVER_ERROR, "json");
}

pub type Res<T> = axum::response::Result<T, AstralError>;