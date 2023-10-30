use axum::extract::State;
use axum::Json;
use crate::api::model::{AuthenticationResponse, RegisterRequest};
use crate::data::AstralDatabase;
use crate::Res;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 400, response = crate::err::AstralError),
        (status = 200, response = AuthenticationResponse)
    )
)]
pub async fn register_with_token(
    State(db): State<AstralDatabase>,
    Json(req): Json<RegisterRequest>
) -> Res<AuthenticationResponse> {

}