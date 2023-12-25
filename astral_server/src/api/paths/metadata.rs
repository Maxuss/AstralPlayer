use std::str::FromStr;
use axum::extract::{Path, Query, State};
use axum::{Json};
use axum::body::Body;
use axum::response::IntoResponse;
use axum_extra::headers::ContentType;
use axum_extra::TypedHeader;
use chrono::NaiveDateTime;
use futures_util::{StreamExt};
use mongodb::bson::doc;
use mongodb::GridFsDownloadStream;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt};
use tokio_util::io::ReaderStream;
use uuid::Uuid;
use crate::api::AppState;
use crate::api::extensions::AuthenticatedUser;
use crate::api::model::{TrackMetadataResponse, AlbumMetadataResponse, ArtistMetadataResponse, FullTrackMetadata, MinifiedArtistMetadata, MinifiedAlbumMetadata, MinifiedTrackMetadata, FullArtistMetadata, FullAlbumMetadata};
use crate::data::AstralDatabase;
use crate::data::model::{AlbumMetadata, ArtistMetadata, BsonId, TrackMetadata, UserAccount};
use crate::err::AstralError;
use crate::metadata::ExtractedTrackMetadata;
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
    AuthenticatedUser(user): AuthenticatedUser
) -> Res<Json<TrackMetadataResponse>> {
    let bid = BsonId::from_uuid_1(uuid.clone());
    let metadata = extract_track_metadata(&db, bid.clone()).await?;
    Ok(Json(TrackMetadataResponse {
        track_id: uuid,
        metadata,
        loved: user.loved_tracks.contains(&bid)
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
pub async fn get_artist_metadata(
    State(AppState { db, .. }): State<AppState>,
    Path(uuid): Path<Uuid>,
    AuthenticatedUser(_): AuthenticatedUser
) -> Res<Json<ArtistMetadataResponse>> {
    let metadata = extract_artist_metadata(&db, BsonId::from_uuid_1(uuid.clone())).await?;
    Ok(Json(
        ArtistMetadataResponse {
            artist_id: uuid,
            metadata
        }
    ))

}

/// Gets full metadata of a single album
#[utoipa::path(
    get,
    path = "/metadata/album/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the album")
    ),
    responses(
        (status = 200, response = AlbumMetadataResponse),
        (status = 400, response = AstralError)
    ),
    tag = "metadata"
)]
pub async fn get_album_metadata(
    State(AppState { db, .. }): State<AppState>,
    Path(uuid): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser
) -> Res<Json<AlbumMetadataResponse>> {
    let bid = BsonId::from_uuid_1(uuid.clone());
    let metadata = extract_album_metadata(&db, bid.clone(), &user).await?;
    Ok(Json(
        AlbumMetadataResponse {
            album_id: uuid,
            metadata,
            loved: user.loved_albums.contains(&bid)
        }
    ))
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
//    AuthenticatedUser(_): AuthenticatedUser,
) -> Res<(TypedHeader<ContentType>, impl IntoResponse)> {
    let metadata = db.gridfs_album_arts.find(doc! { "filename": uuid.to_string() }, None).await?.next().await
        .ok_or_else(|| AstralError::NotFound(String::from("Couldn't find album cover for this UUID")))??.metadata;
    let metadata = metadata.unwrap();
    let mime_type = metadata.get("mime_type").unwrap();
    let mime_type = mime_type.as_str().unwrap();
    let download_stream = db.gridfs_album_arts.open_download_stream_by_name(uuid.to_string(), None).await?;
    let stream_body = Body::from_stream(ReaderStream::new(download_stream.compat()));
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
//    AuthenticatedUser(_): AuthenticatedUser,
) -> Res<(TypedHeader<ContentType>, impl IntoResponse)> {
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
    let stream_body = Body::from_stream(ReaderStream::new(download_stream.compat()));
    Ok(
        (
            TypedHeader(ContentType::from_str(mime_type).unwrap()),
            stream_body
        )
    )
}

#[derive(Deserialize, Debug)]
pub struct MusixmatchQuery {
    q_album: String,
    q_artist: String,
    q_track: String,
    track_spotify_id: String,
    q_duration: String,
}

// this is kind of a hack, so i dont think we really need documentation for now
// TODO: docs later?
pub async fn pass_to_musixmatch(
    State(_): State<AppState>,
    AuthenticatedUser(_): AuthenticatedUser,
    Query(query): Query<MusixmatchQuery>
) -> Res<impl IntoResponse> {
    const BASE_URL: &str = "https://apic-desktop.musixmatch.com/ws/1.1/macro.subtitles.get?format=json&namespace=lyrics_richsynched&subtitle_format=mxm&app_id=web-desktop-app-v1.0";

    let uri = Url::parse_with_params(BASE_URL, [
        ("q_album", &query.q_album),
        ("q_artist", &query.q_artist),
        ("q_artists", &query.q_artist),
        ("q_track", &query.q_track),
        ("track_spotify_id", &query.track_spotify_id),
        ("q_duration", &query.q_duration),
        ("f_subtitle_length", &String::new()),
        ("usertoken", &String::from("2005218b74f939209bda92cb633c7380612e14cb7fe92dcd6a780f"))
    ]).unwrap();

    let client = reqwest::Client::new();
    let json = client.get(uri)
        .headers(HeaderMap::from_iter([
            (HeaderName::from_static("authority"), HeaderValue::from_static("apic-desktop.musixmatch.com")),
            (HeaderName::from_static("cookie"), HeaderValue::from_static("x-mmm-token-guid="))
        ]))
        .send().await?
        .json::<Value>().await?;
    Ok(Json(json))
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
        disc_number: track.disc_number,
        number: track.number,
        format: track.format
    })
}

pub async fn extract_album_metadata(db: &AstralDatabase, album_id: BsonId, user: &UserAccount) -> Res<FullAlbumMetadata> {
    let album = db.albums_metadata.find_one(doc! { "album_id": album_id }, None).await?
        .ok_or_else(|| AstralError::NotFound(format!("Could not find album with UUID: {album_id}")))?;
    Ok(FullAlbumMetadata {
        album_name: album.name,
        artists: extract_minified_artists(&db, album.artists).await?,
        tracks: extract_minified_tracks(&db, album.tracks, user).await?,
        release_date: NaiveDateTime::from_timestamp_millis(album.release_date as i64).unwrap().and_utc(),
        genres: album.genres
    })
}

pub async fn extract_artist_metadata(db: &AstralDatabase, artist_id: BsonId) -> Res<FullArtistMetadata> {
    let artist = db.artists_metadata.find_one(doc! { "artist_id": artist_id }, None).await?
        .ok_or_else(|| AstralError::NotFound(format!("Could not find album with UUID: {artist_id}")))?;
    Ok(FullArtistMetadata {
        artist_name: artist.name,
        albums: extract_minified_albums(&db, artist.albums).await?,
        genres: artist.genres.into_keys().collect(),
        about_artist: artist.about,
        tracks: artist.tracks.into_iter().map(BsonId::to_uuid_1).collect()
    })
}


async fn extract_minified_artists(db: &AstralDatabase, artists: Vec<BsonId>) -> Res<Vec<MinifiedArtistMetadata>> {
    let all = db.artists_metadata.find(doc! { "artist_id": { "$in": &artists } }, None).await?
        .map(|it| it.unwrap()).collect::<Vec<ArtistMetadata>>().await;
    Ok(all.into_iter().map(|each| MinifiedArtistMetadata {
        artist_id: each.artist_id.to_uuid_1(),
        artist_name: each.name.clone(),
        album_ids: each.albums.into_iter().map(BsonId::to_uuid_1).collect(),
        genres: each.genres.into_keys().collect(),
    }).collect())
}

async fn extract_minified_albums(db: &AstralDatabase, albums: Vec<BsonId>) -> Res<Vec<MinifiedAlbumMetadata>> {
    let all = db.albums_metadata.find(doc! { "album_id": { "$in": &albums } }, None).await?
        .map(|it| it.unwrap()).collect::<Vec<AlbumMetadata>>().await;
    Ok(all.into_iter().map(|each| MinifiedAlbumMetadata {
        album_id: each.album_id.to_uuid_1(),
        album_name: each.name.clone(),
        artist_ids: each.artists.into_iter().map(BsonId::to_uuid_1).collect(),
        track_ids: each.tracks.into_iter().map(BsonId::to_uuid_1).collect(),
        genres: each.genres.clone(),
        release_date: NaiveDateTime::from_timestamp_millis(each.release_date as i64).unwrap().and_utc(),
    }).collect())
}

async fn extract_minified_tracks(db: &AstralDatabase, tracks: Vec<BsonId>, user: &UserAccount) -> Res<Vec<MinifiedTrackMetadata>> {
    let all = db.tracks_metadata.find(doc! { "track_id": { "$in": &tracks } }, None).await?
        .map(|it| it.unwrap()).collect::<Vec<TrackMetadata>>().await;
    Ok(all.into_iter().map(|each| MinifiedTrackMetadata {
        track_id: each.track_id.to_uuid_1(),
        track_name: each.name,
        track_length: each.length,
        artist_ids: each.artists.into_iter().map(BsonId::to_uuid_1).collect(),
        album_ids: each.albums.into_iter().map(BsonId::to_uuid_1).collect(),
        format: each.format,
        number: each.number,
        disc_number: each.disc_number,
        is_explicit: each.is_explicit,
        is_loved: user.loved_tracks.contains(&each.track_id),
    }).collect())
}
