use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::Json;
use chrono::{NaiveDateTime, Utc};
use mongodb::bson::doc;
use crate::api::AppState;
use crate::api::extensions::create_user_refresh_key;
use crate::api::model::{AuthenticationResponse, RegisterRequest};
use crate::data::AstralDatabase;
use crate::data::model::{BsonId, UserAccount};
use crate::err::AstralError;
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
#[axum_macros::debug_handler]
pub async fn register_with_token(
    State(AppState { db, paseto_key }): State<AppState>,
    Json(req): Json<RegisterRequest>
) -> Res<Json<AuthenticationResponse>> {
    let code = req.invite_code;
    let invite_code = db.invite_codes.find_one(doc! { "code": code }, None).await?.ok_or_else(|| AstralError::BadRequest(String::from("Invalid invite code")))?;
    db.invite_codes.delete_one(doc! { "code": &invite_code.code }, None).await?;

    if invite_code.expires_at < Utc::now().timestamp_millis() as u64 {
        return Err(AstralError::BadRequest(String::from("This invite code has expired!")))
    }

    let new_user = UserAccount {
        user_id: BsonId::new(),
        username: req.username,
        password_hash: hash_password(req.password),
        register_date: Utc::now().timestamp_millis() as u64,
        permissions: invite_code.permissions
    };

    let refresh_key = create_user_refresh_key(&paseto_key, new_user.user_id.clone().to_uuid_1())?;

    Ok(Json(AuthenticationResponse {
        success: true,
        refresh_token: refresh_key,
        invited_by: invite_code.issued_by.to_uuid_1()
    }))
}

pub fn hash_password(password: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    hash
}
