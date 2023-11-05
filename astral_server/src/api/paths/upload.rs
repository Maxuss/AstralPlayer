use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use axum::extract::{BodyStream, Path, State};
use axum::Json;
use futures_util::{AsyncReadExt, AsyncWriteExt, StreamExt};
use mongodb::bson::{bson, doc};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use crate::api::AppState;
use crate::api::extensions::{AuthenticatedUser, UserPermission};
use crate::api::model::{AlbumMetadataResponse, PatchAlbumMetadata, PatchTrackMetadata, TrackMetadataResponse, UploadTrackResponse};
use crate::api::paths::metadata::{extract_album_metadata, extract_track_metadata};
use crate::data::model::{BsonId, TrackFormat, UndefinedTrack};
use crate::err::AstralError;
use crate::metadata::binary::extract_metadata_from_bytes;
use crate::metadata::classify_insert_metadata;
use crate::Res;

/// Uploads a track to the servers with zero metadata assigned. Returned UUID can be used to update metadata.
#[utoipa::path(
    post,
    path = "/upload/track/{format}",
    request_body = BinaryFile,
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

/// Attempts to guess track data from audio metadata
#[utoipa::path(
    post,
    path = "/upload/guess_metadata/{uuid}",
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = TrackMetadataResponse, description = "Successfully guessed metadata from file. Use /metadata/track to view it.")
    ),
    params(
        ("uuid" = Uuid, Path, description = "UUID of the track to use for metadata guessing"),
    ),
    tag = "upload"
)]
pub async fn guess_metadata(
    State(AppState { db, .. }): State<AppState>,
    Path(track_id): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Res<Json<TrackMetadataResponse>> {
    let track = db.undefined_tracks.find_one_and_delete(doc! {"track_id": BsonId::from_uuid_1(track_id) }, None).await?
        .ok_or_else(|| AstralError::BadRequest(String::from("Track with this UUID does not exist")))?;

    let mut track_audio_bytes = vec![];
    let mut stream = db.gridfs_tracks.open_download_stream_by_name(track.track_id.to_string(), None).await?;
    stream.read_to_end(&mut track_audio_bytes).await?;
    drop(stream);

    let extracted = extract_metadata_from_bytes(&track_audio_bytes, track.format)?;
    drop(track_audio_bytes);

    let uid = classify_insert_metadata(&db, extracted).await?;

    let metadata = extract_track_metadata(&db, uid.clone()).await?;

    Ok(Json(TrackMetadataResponse {
        track_id: uid.to_uuid_1(),
        metadata
    }))
}

/// Updates metadata for a single track
#[utoipa::path(
    patch,
    path = "/upload/track/{uuid}/patch",
    request_body = PatchTrackMetadata,
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = TrackMetadataResponse, description = "Successfully patched track metadata")
    ),
    params(
        ("uuid" = Uuid, Path, description = "UUID of the track to patch"),
    ),
    tag = "upload"
)]
pub async fn patch_track_metadata(
    State(AppState { db, .. }): State<AppState>,
    Path(track_id): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(PatchTrackMetadata { track_name, track_length, is_explicit, number, disc_number }): Json<PatchTrackMetadata>
) -> Res<Json<TrackMetadataResponse>> {
    if !user.permissions.contains(&UserPermission::ChangeMetadata) {
        return Err(AstralError::Unauthorized(String::from("You are not authorized to change metadata.")))
    }

    let mut doc_object = doc!();
    if let Some(track_name) = track_name {
        doc_object.insert("name", track_name);
    }
    if let Some(length) = track_length {
        doc_object.insert("length", track_length);
    }
    if let Some(is_explicit) = is_explicit {
        doc_object.insert("is_explicit", is_explicit);
    }
    if let Some(number) = number {
        doc_object.insert("number", number as i32);
    }
    if let Some(disc_number) = disc_number {
        doc_object.insert("disc_number", disc_number as i32);
    }
    let uid = BsonId::from_uuid_1(track_id.clone());
    db.tracks_metadata.update_one(doc! { "track_id": &uid }, doc! { "$set": doc_object }, None).await?;

    let metadata = extract_track_metadata(&db, uid).await?;

    Ok(Json(TrackMetadataResponse {
        track_id,
        metadata
    }))
}

/// Updates metadata for a single album
#[utoipa::path(
    patch,
    path = "/upload/album/{uuid}/patch",
    request_body = PatchAlbumMetadata,
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = AlbumMetadataResponse, description = "Successfully patched album metadata")
    ),
    params(
        ("uuid" = Uuid, Path, description = "UUID of the album to patch"),
    ),
    tag = "upload"
)]
pub async fn patch_album_metadata(
    State(AppState { db, .. }): State<AppState>,
    Path(album_id): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(PatchAlbumMetadata { album_name, tracks, artists, release_date, genres }): Json<PatchAlbumMetadata>
) -> Res<Json<AlbumMetadataResponse>> {
    if !user.permissions.contains(&UserPermission::ChangeMetadata) {
        return Err(AstralError::Unauthorized(String::from("You are not authorized to change metadata.")))
    }

    let album_id = BsonId::from_uuid_1(album_id);
    let old_data = db.albums_metadata.find_one(doc! { "album_id": &album_id }, None).await?
        .ok_or_else(|| AstralError::NotFound(String::from("Could not find an album with this UUID")))?;
    let mut doc_object = doc!();
    if let Some(album_name) = album_name {
        doc_object.insert("name", album_name);
    }
    if let Some(release_date) = release_date {
        doc_object.insert("release_date", release_date as i64);
    }
    if let Some(genres) = genres {
        doc_object.insert("genres", genres);
    }
    if let Some(tracks) = tracks {
        let tracks = tracks.into_iter().map(BsonId::from_uuid_1);
        let old_tracks: HashSet<BsonId, RandomState> = HashSet::from_iter(old_data.tracks.clone().into_iter());
        let new_tracks: HashSet<BsonId, RandomState> = HashSet::from_iter(tracks.clone());
        // these are the tracks that we will have to remove album reference from
        let remove_album: Vec<&BsonId> = old_tracks.difference(&new_tracks).collect();
        // these are the tracks that we will have to add album reference to
        let add_album: Vec<&BsonId> = new_tracks.difference(&old_tracks).collect();

        // adding album data
        db.tracks_metadata.update_many(doc! { "track_id": { "$in": &add_album } }, doc! { "$addToSet": { "albums": &album_id } }, None).await?;
        db.tracks_metadata.update_many(doc! { "track_id": { "$in": &remove_album} }, doc! { "$pull": { "albums": &album_id } }, None).await?;
        doc_object.insert("tracks", tracks.collect::<Vec<_>>());
    }
    if let Some(artists) = artists {
        let artists = artists.into_iter().map(BsonId::from_uuid_1);
        let old_artists: HashSet<BsonId, RandomState> = HashSet::from_iter(old_data.artists.into_iter());
        let new_artists: HashSet<BsonId, RandomState> = HashSet::from_iter(artists.clone());

        // these are the artists that we will have to remove album reference from
        let remove_album: Vec<&BsonId> = old_artists.difference(&new_artists).collect();
        // these are the artists that we will have to add album reference to
        let add_album: Vec<&BsonId> = new_artists.difference(&old_artists).collect();

        let tracks = if doc_object.contains_key("tracks") {
            doc_object.get("tracks").unwrap().clone()
        } else {
            bson!(old_data.tracks)
        };

        db.artists_metadata.update_many(doc! { "artist_id": { "$in": &add_album } }, doc! { "$addToSet": { "albums": &album_id, "tracks": { "$each": &tracks } }}, None).await?;
        db.artists_metadata.update_many(doc! { "artist_id": { "$in": &remove_album } }, doc! { "$pull": { "albums": &album_id }, "$pullAll": { "tracks": &tracks } }, None).await?;
        doc_object.insert("artists", artists.collect::<Vec<_>>());
    }
    db.albums_metadata.update_one(doc! { "album_id": &album_id }, doc! { "$set": doc_object }, None).await?;

    let metadata = extract_album_metadata(&db, album_id).await?;

    Ok(Json(AlbumMetadataResponse {
        album_id: album_id.to_uuid_1(),
        metadata
    }))
}