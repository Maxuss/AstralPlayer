use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};
use axum::extract::{Path, State};
use axum::{Json, TypedHeader};
use axum::body::StreamBody;
use axum::headers::ContentType;
use chrono::NaiveDateTime;
use futures_util::{AsyncRead, AsyncReadExt, Stream, StreamExt};
use mongodb::bson::doc;
use mongodb::GridFsDownloadStream;
use mongodb::options::GridFsDownloadByNameOptions;
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt};
use tokio_util::io::ReaderStream;
use uuid::Uuid;
use crate::api::AppState;
use crate::api::extensions::AuthenticatedUser;
use crate::api::model::{TrackMetadataResponse, AlbumMetadataResponse, ArtistMetadataResponse, FullTrackMetadata, MinifiedArtistMetadata, MinifiedAlbumMetadata};
use crate::data::AstralDatabase;
use crate::data::model::{AlbumMetadata, ArtistMetadata, BsonId, TrackMetadata};
use crate::err::AstralError;
use crate::Res;

/// Gets full metadata of a single track
#[utoipa::path(
    get,
    path = "/metadata/track/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the track")
    ),
    responses(
        (status = 200, response = TrackMetadataResponse),
        (status = 400, response = AstralError)
    ),
    tag = "metadata"
)]
pub async fn get_track_metadata(
    State(AppState { db, .. }): State<AppState>,
    Path(uuid): Path<Uuid>,
    AuthenticatedUser(_): AuthenticatedUser
) -> Res<Json<TrackMetadataResponse>> {
    let metadata = extract_track_metadata(&db, BsonId::from_uuid_1(uuid.clone())).await?;
    Ok(Json(TrackMetadataResponse {
        track_id: uuid,
        metadata
    }))
}

/// Gets full metadata of a single artist
#[utoipa::path(
    get,
    path = "/metadata/artist/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the artist")
    ),
    responses(
        (status = 200, response = ArtistMetadataResponse),
        (status = 400, response = AstralError)
    ),
    tag = "metadata"
)]
pub async fn get_artist_metadata() -> Json<ArtistMetadataResponse> {
    todo!()
}

/// Gets full metadata of a single album
#[utoipa::path(
    get,
    path = "/metadata/album/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the album")
    ),
    responses(
        (status = 200, response = AlbumMetadataResponse)
    ),
    tag = "metadata"
)]
pub async fn get_album_metadata() -> Json<AlbumMetadataResponse> {
    todo!()
}

/// Gets cover art of an album
#[utoipa::path(
    get,
    path = "/metadata/album/{id}/cover",
    params(
        ("id" = Uuid, Path, description = "UUID of the album")
    ),
    responses(
        (status = 200, body = BinaryFile, description = "Found the album cover art"),
        (status = 400, response = AstralError)
    ),
    tag = "metadata"
)]
pub async fn get_album_cover_art(
    State(AppState { db, .. }): State<AppState>,
    Path(uuid): Path<Uuid>,
    AuthenticatedUser(_): AuthenticatedUser,
) -> Res<(TypedHeader<ContentType>, StreamBody<ReaderStream<Compat<GridFsDownloadStream>>>)> {
    let metadata = db.gridfs_album_arts.find(doc! { "filename": uuid.to_string() }, None).await?.next().await
        .ok_or_else(|| AstralError::NotFound(String::from("Couldn't find album cover for this UUID")))??.metadata;
    let metadata = metadata.unwrap();
    let mime_type = metadata.get("mime_type").unwrap();
    let mime_type = mime_type.as_str().unwrap();
    let download_stream = db.gridfs_album_arts.open_download_stream_by_name(uuid.to_string(), None).await?;
    let stream_body = StreamBody::new(ReaderStream::new(download_stream.compat()));
    Ok(
        (
            TypedHeader(ContentType::from_str(mime_type).unwrap()),
            stream_body
        )
    )
}

/// Gets cover art of a track (by querying it's album)
#[utoipa::path(
    get,
    path = "/metadata/track/{id}/cover",
    params(
        ("id" = Uuid, Path, description = "UUID of the track")
    ),
    responses(
    (status = 200, body = BinaryFile, description = "Found the album cover art"),
    (status = 400, response = AstralError)
    ),
    tag = "metadata"
)]
pub async fn get_track_cover_art(
    State(AppState { db, .. }): State<AppState>,
    Path(uuid): Path<Uuid>,
    AuthenticatedUser(_): AuthenticatedUser,
) -> Res<(TypedHeader<ContentType>, StreamBody<ReaderStream<Compat<GridFsDownloadStream>>>)> {
    let track = db.tracks_metadata.find_one(doc! { "track_id": uuid }, None).await?
        .ok_or_else(|| AstralError::NotFound(String::from("Couldn't find track with this UUID")))?;
    let album_id = track.albums.first().map(ToOwned::to_owned)
        .ok_or_else(|| AstralError::NotFound(String::from("This track does not have any albums associated with it")))?;
    let metadata = db.gridfs_album_arts.find(doc! { "filename": album_id.to_string() }, None).await?.next().await
        .ok_or_else(|| AstralError::NotFound(String::from("Album for this track does not have a cover")))??.metadata;
    let metadata = metadata.unwrap();
    let mime_type = metadata.get("mime_type").unwrap();
    let mime_type = mime_type.as_str().unwrap();
    let download_stream = db.gridfs_album_arts.open_download_stream_by_name(album_id.to_string(), None).await?;
    let stream_body = StreamBody::new(ReaderStream::new(download_stream.compat()));
    Ok(
        (
            TypedHeader(ContentType::from_str(mime_type).unwrap()),
            stream_body
        )
    )
}

pub async fn extract_track_metadata(db: &AstralDatabase, track_id: BsonId) -> Res<FullTrackMetadata> {
    let track = db.tracks_metadata.find_one(doc! { "track_id": track_id }, None).await?
        .ok_or_else(|| AstralError::NotFound(format!("Could not find track with UUID: {track_id}")))?;
    Ok(FullTrackMetadata {
        track_name: track.name,
        track_length: track.length,
        artists: extract_minified_artists(db, track.artists).await?,
        albums: extract_minified_albums(db, track.albums).await?,
        is_explicit: track.is_explicit,
    })
}

async fn extract_minified_artists(db: &AstralDatabase, artists: Vec<BsonId>) -> Res<Vec<MinifiedArtistMetadata>> {
    let all = db.artists_metadata.find(doc! { "artist_id": { "$in": &artists } }, None).await?.map(|it| it.unwrap()).collect::<Vec<ArtistMetadata>>().await;
    Ok(all.into_iter().map(|each| MinifiedArtistMetadata {
        artist_id: each.artist_id.to_uuid_1(),
        artist_name: each.name.clone(),
        album_ids: each.albums.into_iter().map(|other| other.to_uuid_1()).collect(),
        genres: each.genres.into_keys().collect(),
    }).collect())
}

async fn extract_minified_albums(db: &AstralDatabase, albums: Vec<BsonId>) -> Res<Vec<MinifiedAlbumMetadata>> {
    let all = db.albums_metadata.find(doc! { "album_id": { "$in": &albums } }, None).await?.map(|it| it.unwrap()).collect::<Vec<AlbumMetadata>>().await;
    Ok(all.into_iter().map(|each| MinifiedAlbumMetadata {
        album_id: each.album_id.to_uuid_1(),
        album_name: each.name.clone(),
        artist_ids: each.artists.into_iter().map(|other| other.to_uuid_1()).collect(),
        track_ids: each.tracks.into_iter().map(|other| other.to_uuid_1()).collect(),
        genres: each.genres.clone(),
        release_date: NaiveDateTime::from_timestamp_millis(each.release_date as i64).unwrap().and_utc(),
    }).collect())
}
