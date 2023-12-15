use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::path::PathBuf;
use axum::extract::{Path, Query, State};
use axum::Json;
use futures_util::{AsyncReadExt as FutReadExt, AsyncWriteExt as FutWriteExt, StreamExt};
use mongodb::bson::{bson, doc};
use mongodb::GridFsUploadStream;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufReader};
use uuid::Uuid;
use axum::body::Body;
use tokio::io::AsyncReadExt;
use crate::api::AppState;
use crate::api::extensions::{AuthenticatedUser, UserPermission};
use crate::api::model::{AlbumMetadataResponse, ArtistMetadataResponse, PatchAlbumMetadata, PatchArtistMetadata, PatchTrackMetadata, TrackMetadataResponse, UploadTrackResponse};
use crate::api::paths::metadata::{extract_album_metadata, extract_artist_metadata, extract_track_metadata};
use crate::data::model::{BsonId, TrackFormat, UndefinedTrack};
use crate::err::AstralError;
use crate::metadata::binary::extract_metadata_from_bytes;
use crate::metadata::classify_insert_metadata;
use crate::metadata::merged::extract_merged_metadata;
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
    mut stream: Body
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

    let mut hasher = Sha256::new();
    let path = std::path::Path::new("astral_tracks").join(format!("{track_id}.bin"));
    let mut file = File::create(&path).await?;
    let mut stream = stream.into_data_stream();
    while let Some(Ok(mut chunk)) = stream.next().await {
        file.write_all(&mut chunk).await?;
        hasher.update(&chunk);
    }
    let hash = hex::encode(&hasher.finalize()[..]);

    if let Some(track) = db.undefined_tracks.find_one(doc! { "hash": &hash }, None).await? {
        tokio::fs::remove_file(&path).await?;
        return Ok(Json(UploadTrackResponse {
            track_id: track.track_id.to_uuid_1(),
        }))
    }
    file.flush().await?;

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

#[derive(Deserialize)]
pub struct MetadataProps {
    musix_priority: Option<bool>,
    skip_musix: Option<bool>,
    musix_artist_override: Option<String>,
    musix_album_override: Option<String>,
    musix_name_override: Option<String>
}

/// Attempts to guess track data from audio metadata and musixmatch metadata
#[utoipa::path(
    post,
    path = "/upload/guess_metadata/{uuid}",
    responses(
        (status = 400, response = AstralError),
        (status = 200, response = TrackMetadataResponse)
    ),
    params(
        ("uuid" = Uuid, Path, description = "UUID of the track to use for metadata guessing"),
        ("musix_priority" = inline(Option<bool>), Query, description = "Whether to prioritize Musixmatch metadata over bundled metadaata"),
        ("skip_musix" = inline(Option<bool>), Query, description = "Whether to fully skip Musixmatch metadata fetching"),
        ("musix_artist_override" = inline(Option<String>), Query, description = "Custom override for track artist when fetching Musixmatch"),
        ("musix_album_override" = inline(Option<String>), Query, description = "Custom override for track album when fetching Musixmatch"),
        ("musix_name_override" = inline(Option<String>), Query, description = "Custom override for track name when fetching Musixmatch"),

    ),
    tag = "upload"
)]
pub async fn guess_metadata(
    State(AppState { db, .. }): State<AppState>,
    Path(track_id): Path<Uuid>,
    Query(MetadataProps { musix_priority, skip_musix, musix_album_override, musix_artist_override, musix_name_override }): Query<MetadataProps>,
    AuthenticatedUser(_): AuthenticatedUser,
) -> Res<Json<TrackMetadataResponse>> {
    let uid = BsonId::from_uuid_1(track_id);
    let track = db.undefined_tracks.find_one(doc! {"track_id": &uid }, None).await?
        .ok_or_else(|| AstralError::BadRequest(String::from("Track with this UUID does not exist")))?;

    let mut track_audio_bytes = vec![];
    let mut stream = BufReader::new(File::open(PathBuf::from("astral_tracks").join(format!("{uid}.bin"))).await?);
    stream.read_to_end(&mut track_audio_bytes).await?;
    drop(stream);

    let extracted = if skip_musix.unwrap_or(false) {
        let mut extracted = extract_metadata_from_bytes(&track_audio_bytes, track.format)?;
        if(musix_priority.unwrap_or(false)) {
            extracted.artists = musix_artist_override.clone().map(|it| it.split(", ").map(String::from).collect::<Vec<_>>()).unwrap_or(extracted.artists);
            extracted.album_artists = musix_artist_override.map(|it| it.split(", ").map(String::from).collect::<Vec<_>>()).unwrap_or(extracted.album_artists);
            extracted.name = musix_name_override.unwrap_or(extracted.name);
            extracted.album_name = musix_album_override.unwrap_or(extracted.album_name);
            extracted
        } else {
            extracted
        }
    } else {
        extract_merged_metadata(&track_audio_bytes, track.format, musix_priority.unwrap_or(false), musix_artist_override, musix_album_override, musix_name_override).await?
    };
    drop(track_audio_bytes);

    db.undefined_tracks.delete_one(doc! { "track_id": &uid }, None).await?;

    let uid = classify_insert_metadata(&db, extracted, uid).await?;

    let metadata = extract_track_metadata(&db, uid.clone()).await?;

    Ok(Json(TrackMetadataResponse {
        track_id: uid.to_uuid_1(),
        metadata,
        loved: false,
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
    Json(PatchTrackMetadata { track_name, track_length, is_explicit, number, disc_number, artists }): Json<PatchTrackMetadata>
) -> Res<Json<TrackMetadataResponse>> {
    if !user.permissions.contains(&UserPermission::ChangeMetadata) {
        return Err(AstralError::Unauthorized(String::from("You are not authorized to change metadata.")))
    }

    let mut doc_object = doc!();
    if let Some(track_name) = track_name {
        doc_object.insert("name", track_name);
    }
    if let Some(length) = track_length {
        doc_object.insert("length", length);
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
    if let Some(artists) = artists {
        doc_object.insert("artists", bson!(artists.into_iter().map(BsonId::from_uuid_1).collect::<Vec<_>>()));
    }
    let uid = BsonId::from_uuid_1(track_id.clone());
    db.tracks_metadata.update_one(doc! { "track_id": &uid }, doc! { "$set": doc_object }, None).await?;

    let metadata = extract_track_metadata(&db, uid.clone()).await?;

    Ok(Json(TrackMetadataResponse {
        track_id,
        metadata,
        loved: user.loved_albums.contains(&uid),
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
        metadata,
        loved: user.loved_albums.contains(&album_id)
    }))
}

/// Updates metadata for a single artist
#[utoipa::path(
    patch,
    path = "/upload/artist/{uuid}/patch",
    request_body = PatchAlbumMetadata,
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = ArtistMetadataResponse, description = "Successfully patched artist metadata")
    ),
    params(
        ("uuid" = Uuid, Path, description = "UUID of the artist to patch"),
    ),
    tag = "upload"
)]
pub async fn patch_artist_metadata(
    State(AppState { db, .. }): State<AppState>,
    Path(artist_id): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(PatchArtistMetadata { artist_name, albums, about_artist }): Json<PatchArtistMetadata>
) -> Res<Json<ArtistMetadataResponse>> {
    if !user.permissions.contains(&UserPermission::ChangeMetadata) {
        return Err(AstralError::Unauthorized(String::from("You are not authorized to change metadata.")))
    }

    let artist_id = BsonId::from_uuid_1(artist_id);
    let old_data = db.artists_metadata.find_one(doc! { "artist_id": &artist_id }, None).await?
        .ok_or_else(|| AstralError::NotFound(String::from("Could not find an artist with this UUID")))?;
    let mut doc_object = doc!();

    if let Some(name) = artist_name {
        doc_object.insert("name", name);
    }
    if let Some(about) = about_artist {
        doc_object.insert("about", about);
    }
    if let Some(albums) = albums {
        let albums = albums.into_iter().map(BsonId::from_uuid_1);
        let old_albums: HashSet<BsonId, RandomState> = HashSet::from_iter(old_data.albums.clone().into_iter());
        let new_albums: HashSet<BsonId, RandomState> = HashSet::from_iter(albums.clone());

        // these are the albums that we will have to remove artist reference from
        let remove_artist: Vec<&BsonId> = old_albums.difference(&new_albums).collect();
        // these are the albums that we will have to add artist reference to
        let add_artist: Vec<&BsonId> = new_albums.difference(&old_albums).collect();

        db.albums_metadata.update_many(doc! { "album_id": { "$in": add_artist } }, doc! { "$addToSet": { "artists": &artist_id } }, None).await?;
        db.albums_metadata.update_many(doc! { "album_id": { "$in": remove_artist } }, doc! { "$pull": { "artists": &artist_id } }, None).await?;

        doc_object.insert("albums", albums.collect::<Vec<_>>());
    }

    db.artists_metadata.update_one(doc! { "artist_id": &artist_id }, doc! { "$set": doc_object }, None).await?;

    let metadata = extract_artist_metadata(&db, artist_id).await?;

    Ok(Json(ArtistMetadataResponse {
        artist_id: artist_id.to_uuid_1(),
        metadata
    }))
}

#[utoipa::path(
    post,
    path = "/upload/cover/{uuid}",
    request_body = BinaryFile,
    responses(
        (status = 400, response = AstralError),
        (status = 200, description = "Successfully changed album cover")
    ),
    params(
        ("uuid" = Uuid, Path, description = "UUID of the album"),
    ),
    tag = "upload"
)]
pub async fn change_cover(
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<Uuid>,
    AuthenticatedUser(user): AuthenticatedUser,
    mut stream: Body
) -> Res<()> {
    if !user.permissions.contains(&UserPermission::ChangeMetadata) {
        return Err(AstralError::Unauthorized(String::from("You are not authorized to change metadata")))
    }

    let found = db.gridfs_album_arts.find(doc! { "filename": id.to_string() }, None).await?.next().await;
    if let Some(Ok(found)) = found {
        db.gridfs_album_arts.delete(found.id).await?;
    }

    let mut u_stream = db.gridfs_album_arts.open_upload_stream(id.to_string(), None);
    let mut stream = stream.into_data_stream();
    while let Some(Ok(chunk)) = stream.next().await {
        u_stream.write(&chunk).await?;
    }
    u_stream.flush().await?;
    GridFsUploadStream::close(&mut u_stream).await?;


    Ok(())
}