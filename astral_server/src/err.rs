use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::json;
use thiserror::Error;
use utoipa::openapi::{ContentBuilder, ObjectBuilder, RefOr, ResponseBuilder, SchemaType};
use utoipa::{ToResponse, ToSchema};

/// The error container
#[derive(Debug, Error)]
pub enum AstralError {
    /// Anyhow-provoked error
    #[error("{0}")]
    Unknown(#[from] anyhow::Error),
    /// Mongo-provoked error
    #[error("An error with DB has occurred: {0}")]
    Database(#[from] mongodb::error::Error)
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
}

pub type Res<T> = axum::response::Result<T, AstralError>;