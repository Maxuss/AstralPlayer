use utoipa::OpenApi;

use super::model::*;
use super::paths::metadata::*;

#[derive(OpenApi)]
#[openapi(
    components(
        responses(TrackMetadataResponse, ArtistMetadataResponse, AlbumMetadataResponse),
        schemas(FullTrackMetadata, FullArtistMetadata, FullAlbumMetadata, MinifiedTrackMetadata, MinifiedAlbumMetadata, MinifiedArtistMetadata)
    ),
    paths(
        get_track_metadata, get_artist_metadata, get_album_metadata
    ),
    tags(
        (name = "metadata", description = "Operations related to getting or updating metadata")
    )
)]
pub struct ApiDoc;
