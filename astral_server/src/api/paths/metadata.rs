use axum::Json;
use crate::api::model::{TrackMetadataResponse, AlbumMetadataResponse, ArtistMetadataResponse};

/// Gets full metadata of a single track
#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/metadata/track/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the track")
    ),
    responses(
        (status = 200, response = TrackMetadataResponse)
    ),
    tag = "metadata"
)]
pub async fn get_track_metadata() -> Json<TrackMetadataResponse> {
    todo!()
}

/// Gets full metadata of a single artist
#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/metadata/artist/{id}",
    params(
        ("id" = Uuid, Path, description = "UUID of the artist")
    ),
    responses(
        (status = 200, response = ArtistMetadataResponse)
    ),
    tag = "metadata"
)]
pub async fn get_artist_metadata() -> Json<ArtistMetadataResponse> {
    todo!()
}

/// Gets full metadata of a single album
#[axum_macros::debug_handler]
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