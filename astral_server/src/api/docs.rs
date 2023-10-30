use utoipa::OpenApi;

use super::model::*;
use super::paths::metadata::*;
use crate::err::AstralError;

#[derive(OpenApi)]
#[openapi(
    components(
        responses(TrackMetadataResponse, ArtistMetadataResponse, AlbumMetadataResponse, AstralError),
        schemas(FullTrackMetadata, FullArtistMetadata, FullAlbumMetadata, MinifiedTrackMetadata, MinifiedAlbumMetadata, MinifiedArtistMetadata)
    ),
    paths(
        get_track_metadata, get_artist_metadata, get_album_metadata
    ),
    tags(
        (name = "metadata", description = "Operations related to getting or updating metadata"),
        (name = "auth", descriptiption = "Operations related to authentication, authorization and account creation")
    )
)]
pub struct ApiDoc;
