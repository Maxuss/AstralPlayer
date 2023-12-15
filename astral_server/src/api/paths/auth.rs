use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum::{Json};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use axum_extra::TypedHeader;
use chrono::Utc;
use mongodb::bson::doc;
use crate::api::AppState;
use crate::api::extensions::{create_user_access_key, create_user_refresh_key, validate_key};
use crate::api::model::{AuthenticationRequest, AuthenticationResponse, RegisterRequest};
use crate::data::model::{BsonId, UserAccount};
use crate::err::AstralError;
use crate::Res;

/// Registers a user with an invite code
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 400, response = AstralError),
        (status = 200, response = AuthenticationResponse)
    ),
    tag = "auth"
)]
#[axum_macros::debug_handler]
pub async fn register_with_token(
    State(AppState { db, paseto_key }): State<AppState>,
    Json(req): Json<RegisterRequest>
) -> Res<Json<AuthenticationResponse>> {
    let code = req.invite_code;
    let invite_code = db.invite_codes.find_one(doc! { "code": &code }, None).await?.ok_or_else(|| AstralError::BadRequest(String::from("Invalid invite code")))?;

    db.invite_codes.delete_one(doc! { "code": &code }, None).await?;

    if invite_code.expires_at < Utc::now().timestamp_millis() as u64 {
        return Err(AstralError::BadRequest(String::from("This invite code has expired!")))
    }

    let new_user = UserAccount {
        user_id: BsonId::new(),
        username: req.username,
        password_hash: hash_password(req.password),
        register_date: Utc::now().timestamp_millis() as u64,
        permissions: invite_code.permissions,
        loved_albums: vec![],
        loved_tracks: vec![],
    };

    db.accounts.insert_one(&new_user, None).await?;

    let refresh_key = create_user_refresh_key(&paseto_key, new_user.user_id.clone().to_uuid_1())?;

    Ok(Json(AuthenticationResponse {
        refresh_token: refresh_key,
        invited_by: invite_code.issued_by.to_uuid_1()
    }))
}



/// Verifies the validity of an access token
#[utoipa::path(
    post,
    path = "/auth/verify",
    request_body = String,
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = bool, description = "Whether the code is valid")
    ),
    tag = "auth"
)]
#[axum_macros::debug_handler]
pub async fn verify(
    State(AppState { paseto_key, .. }): State<AppState>,
    body: String
) -> Json<bool> {
    let code = validate_key(&paseto_key, &body);
    Json(code.is_ok())
}

/// Log in using username and password
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = AuthenticationRequest,
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = String, description = "Successfully obtained refresh token")
    ),
    tag = "auth"
)]
pub async fn login(
    State(AppState { db, paseto_key }): State<AppState>,
    jar: CookieJar,
    Json(req): Json<AuthenticationRequest>,
) -> Res<(CookieJar, String)> {
    let user = db.accounts.find_one(doc! { "username": &req.username }, None).await?
        .ok_or_else(|| AstralError::BadRequest(String::from("Invalid username. Couldn't find a user with this username.")))?;
    if !validate_password(req.password, user.password_hash) {
        return Err(AstralError::Unauthorized(String::from("Invalid password or username.")))
    }
    let key = create_user_refresh_key(&paseto_key, user.user_id.to_uuid_1())?;
    let mut cookie = Cookie::new("refresh-token", key.clone());
    cookie.set_path("/");

    Ok((
        jar.add(cookie),
        key
    ))
}

/// Obtains access token from refresh token
#[utoipa::path(
    get,
    path = "/auth/token",
    params(
        ("Authorization" = Bearer, Header, description = "Refresh token in bearer format")
    ),
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = String, description = "Successfully obtained access token")
    ),
    tag = "auth"
)]
#[axum_macros::debug_handler]
pub async fn obtain_access_token(
    State(AppState { paseto_key, .. }): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    jar: CookieJar,
) -> Res<(CookieJar, String)> {
    let uid = validate_key(&paseto_key, bearer.token())?;
    let key = create_user_access_key(&paseto_key, uid)?;

    let mut cookie = Cookie::new("auth-token", key.clone());
    cookie.set_path("/");
    Ok((jar.add(cookie), key))
}

/// Hashes password with Argon2
pub fn hash_password(password: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    hash
}

/// Validates password hash
pub fn validate_password(password: String, hash: String) -> bool {
    let hash = PasswordHash::new(&hash).unwrap();
    let argon = Argon2::default();
    argon.verify_password(password.as_bytes(), &hash).is_ok()
}