use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use axum::extract::FromRequestParts;
use axum::headers::authorization::{Bearer, Credentials};
use axum::headers::{Cookie, HeaderMapExt};
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
use crate::data::model::{BsonId, UserAccount};
use crate::err::AstralError;

/// Attempts to obtain PASETO secret symmetric key from local file.
// (maybe insecure?)
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

/// A single permission for a user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserPermission {
    /// Allows user to upload new tracks to the servers
    UploadTracks,
    /// Allows user to change track/album/artist metadata
    ChangeMetadata,
    /// Allows user to invite user and assign them permissions that they have
    InviteUsers,
}

/// Creates a short-lived PASETO access token
pub fn create_user_access_key(sk: &SymmetricKey<V4>, uid: Uuid) -> anyhow::Result<String> {
    let mut claims = Claims::new()?;
    claims.add_additional("uid", uid.to_string())?;
    claims.subject("Astral-Access")?;

    let token = local::encrypt(sk, &claims, None, None)?;
    Ok(token)
}

/// Creates a long-lived PASETO refresh token that can be used to obtain an access token
pub fn create_user_refresh_key(sk: &SymmetricKey<V4>, uid: Uuid) -> anyhow::Result<String> {
    let mut claims = Claims::new_expires_in(&Duration::from_secs(30 * 86400u64))?;
    claims.add_additional("uid", uid.to_string())?;

    let token = local::encrypt(sk, &claims, None, None)?;
    Ok(token)
}

/// Validates refresh token specifically
pub fn validate_key(sk: &SymmetricKey<V4>, key: &str) -> anyhow::Result<Uuid> {
    let validation_rules = ClaimsValidationRules::new();
    let untrusted = UntrustedToken::<Local, V4>::try_from(key)?;
    let trusted = local::decrypt(sk, &untrusted, &validation_rules, None, None)?;

    let uid = Uuid::from_str(trusted.payload_claims().unwrap().get_claim("uid").unwrap().as_str().unwrap())?;
    Ok(uid)
}

/// Validates access token specifically
pub fn validate_access_key(sk: &SymmetricKey<V4>, key: &str) -> anyhow::Result<Uuid> {
    let mut validation_rules = ClaimsValidationRules::new();
    validation_rules.validate_subject_with("Astral-Access");
    let untrusted = UntrustedToken::<Local, V4>::try_from(key)?;
    let trusted = local::decrypt(sk, &untrusted, &validation_rules, None, None)?;

    let uid = Uuid::from_str(trusted.payload_claims().unwrap().get_claim("uid").unwrap().as_str().unwrap())?;
    Ok(uid)
}

/// Extension used to validate that user is authenticated
#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub UserAccount);

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AstralError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth = parts.headers.get(AUTHORIZATION);
        let uid = auth.map(Bearer::decode).flatten()
            .map(|it| validate_access_key(&state.paseto_key, it.token()).ok()).flatten()
            .or_else(||
                parts.headers.typed_get::<Cookie>()
                    .map(|it|
                        it
                            .get("auth-token")
                            .map(|inner| validate_access_key(&state.paseto_key, inner).ok())
                            .flatten()
                    )
                    .flatten()
            );

        if let Some(uid) = uid {
            let bson = BsonId::from_uuid_1(uid);
            state.db.accounts.find_one(doc! { "user_id": &bson }, None).await?
                .map(Self)
                .ok_or_else(|| AstralError::BadRequest(String::from("Couldn't find user with this id.")))
        } else {
            Err(AstralError::Unauthorized(String::from("Expected bearer or cookie authorization for this endpoint")))
        }
    }
}