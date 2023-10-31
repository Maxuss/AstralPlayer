use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use axum::extract::FromRequestParts;
use axum::headers::authorization::{Bearer, Credentials};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use mongodb::bson::doc;
use pasetors::{local, Local};
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::{Generate, SymmetricKey};
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::AppState;
use crate::data::model::UserAccount;
use crate::err::AstralError;


pub fn try_obtain_paseto_secret() -> anyhow::Result<SymmetricKey<V4>> {
    let path = Path::new(".paseto");
    if !path.exists() {
        // Generate symmetric key
        let key = SymmetricKey::<V4>::generate()?;
        let mut file = File::create(path)?;
        file.write(key.as_bytes())?;
        return Ok(key);
    }

    let mut file = File::open(path)?;
    let mut bytes = [0u8; 32];
    file.read(&mut bytes)?;

    SymmetricKey::<V4>::from(&bytes).map_err(anyhow::Error::from)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserPermission {
    UploadTracks,
    ChangeMetadata,
    InviteUsers,
}

pub fn create_user_access_key(sk: &SymmetricKey<V4>, uid: Uuid) -> anyhow::Result<String> {
    let mut claims = Claims::new()?;
    claims.add_additional("uid", uid.to_string())?;

    let token = local::encrypt(sk, &claims, None, None)?;
    Ok(token)
}

pub fn create_user_refresh_key(sk: &SymmetricKey<V4>, uid: Uuid) -> anyhow::Result<String> {
    let mut claims = Claims::new_expires_in(&Duration::from_secs(30 * 86400u64))?;
    claims.add_additional("uid", uid.to_string())?;

    let token = local::encrypt(sk, &claims, None, None)?;
    Ok(token)
}

pub fn validate_key(sk: &SymmetricKey<V4>, key: &str) -> anyhow::Result<Uuid> {
    let validation_rules = ClaimsValidationRules::new();
    let untrusted = UntrustedToken::<Local, V4>::try_from(key)?;
    let trusted = local::decrypt(sk, &untrusted, &validation_rules, None, None)?;

    let uid = Uuid::from_str(trusted.payload_claims().unwrap().get_claim("uid").unwrap().as_str().unwrap())?;
    Ok(uid)
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser(UserAccount);

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AstralError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth = parts.headers.get(AUTHORIZATION)
            .ok_or_else(|| AstralError::BadRequest(String::from("Expected an authorization header for this endpoint")))?;
        let bearer = Bearer::decode(auth)
            .ok_or_else(|| AstralError::BadRequest(String::from("Expected bearer authorization for this endpoint")))?;
        let token = bearer.token();
        let uid = validate_key(&state.paseto_key, token)
            .map_err(|_| AstralError::Unauthorized(String::from("Invalid access token")))?;
        let user = state.db.accounts.find_one(doc! { "user_id": uid }, None).await?
            .ok_or_else(|| AstralError::Unauthorized(String::from("Couldn't find a user with UUID {uid}. Invalid access token?")))?;

        Ok(Self(user))
    }
}