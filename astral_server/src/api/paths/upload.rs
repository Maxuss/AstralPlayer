use axum::extract::{BodyStream, Path, State};
use axum::Json;
use futures_util::{AsyncWriteExt, StreamExt};
use mongodb::bson::doc;
use sha2::{Digest, Sha256};
use crate::api::AppState;
use crate::api::extensions::{AuthenticatedUser, UserPermission};
use crate::api::model::UploadTrackResponse;
use crate::data::model::{BsonId, TrackFormat, UndefinedTrack};
use crate::err::AstralError;
use crate::Res;

/// Uploads a track to the servers with zero metadata assigned. Returned UUID can be used to update metadata.
#[utoipa::path(
    post,
    path = "/upload/track/{format}",
    request_body = BinaryFileUpload,
    responses(
        (status = 400, response = AstralError),
        (status = 200, response = UploadTrackResponse)
    ),
    params(
        ("format" = TrackFormat, Path, description = "Track format. Supported formats are: flac, m4a, mp3"),
    ),
    tag = "upload"
)]
pub async fn upload_track(
    State(AppState { db, .. }): State<AppState>,
    Path(hint): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    mut stream: BodyStream
) -> Res<Json<UploadTrackResponse>> {
    let track_format = match hint.to_lowercase().as_str() {
        "flac" => TrackFormat::Flac,
        "m4a" => TrackFormat::M4a,
        "mp3" => TrackFormat::Mp3,
        _ => return Err(AstralError::BadRequest(String::from("Invalid track format hint, expected one of flac,m4a,mp3")))
    };

    if !user.permissions.contains(&UserPermission::UploadTracks) {
        return Err(AstralError::Unauthorized(String::from("You are not authorized to upload tracks")))
    }

    let track_id = BsonId::new();
    let mut connection = db.gridfs_tracks.open_upload_stream(track_id.to_string(), None);
    let mut hasher = Sha256::new();

    while let Some(chunk) = stream.next().await {
        if let Ok(chunk) = chunk {
            hasher.update(&chunk);
            connection.write(&chunk).await?;
        } else {
            return Err(AstralError::BadRequest(String::from("Corrupted upload stream")))
        }
    }

    let hash = hex::encode(&hasher.finalize()[..]);

    if let Some(track) = db.undefined_tracks.find_one(doc! { "hash": &hash }, None).await? {
        connection.abort().await?;
        return Ok(Json(UploadTrackResponse {
            track_id: track.track_id.to_uuid_1(),
        }))
    }
    connection.flush().await?;
    connection.close().await?;

    let new_track = UndefinedTrack {
        track_id,
        hash,
        uploaded_by: user.user_id.clone(),
        format: track_format
    };
    db.undefined_tracks.insert_one(&new_track, None).await?;

    Ok(Json(UploadTrackResponse {
        track_id: new_track.track_id.to_uuid_1()
    }))
}