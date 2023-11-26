use axum::extract::{Path, State};
use mongodb::bson::doc;
use uuid::Uuid;
use crate::api::AppState;
use crate::api::extensions::AuthenticatedUser;
use crate::data::model::BsonId;
use crate::err::AstralError;
use crate::Res;

/// Add a track to loved list
#[utoipa::path(
    post,
    path = "/user/love/track/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the track to love"),
    ),
    responses(
        (status = 200, description = "Successfully loved the track"),
        (status = 400, response = AstralError)
    ),
    tag = "user"
)]
pub async fn love_track(
    State(AppState { db, .. }): State<AppState>,
    Path(track): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser
) -> Res<()> {
    let track = BsonId::from_uuid_1(track);
    db.accounts.update_one(doc! { "user_id": &user.user_id }, doc! { "$addToSet": { "loved_tracks": &track }}, None).await?;
    Ok(())
}

/// Remove a track from loved list
#[utoipa::path(
    post,
    path = "/user/unlove/track/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the track to unlove"),
    ),
    responses(
        (status = 200, description = "Successfully unloved the track"),
        (status = 400, response = AstralError)
    ),
    tag = "user"
)]
pub async fn unlove_track(
    State(AppState { db, .. }): State<AppState>,
    Path(track): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser
) -> Res<()> {
    let track = BsonId::from_uuid_1(track);
    db.accounts.update_one(doc! { "user_id": &user.user_id }, doc! { "$pull": { "loved_tracks": &track }}, None).await?;
    Ok(())
}

/// Add an album to loved list
#[utoipa::path(
    post,
    path = "/user/love/album/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the album to love"),
    ),
    responses(
        (status = 200, description = "Successfully loved the album"),
        (status = 400, response = AstralError)
    ),
    tag = "user"
)]
pub async fn love_album(
    State(AppState { db, .. }): State<AppState>,
    Path(album): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser
) -> Res<()> {
    let album = BsonId::from_uuid_1(album);
    db.accounts.update_one(doc! { "user_id": &user.user_id }, doc! { "$addToSet": { "loved_albums": &album }}, None).await?;
    Ok(())
}

/// Remove an album from loved list
#[utoipa::path(
    post,
    path = "/user/unlove/album/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the album to unlove"),
    ),
    responses(
        (status = 200, description = "Successfully unloved the album"),
        (status = 400, response = AstralError)
    ),
    tag = "user"
)]
pub async fn unlove_album(
    State(AppState { db, .. }): State<AppState>,
    Path(album): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser
) -> Res<()> {
    let album = BsonId::from_uuid_1(album);
    db.accounts.update_one(doc! { "user_id": &user.user_id }, doc! { "$pull": { "loved_albums": &album }}, None).await?;
    Ok(())
}