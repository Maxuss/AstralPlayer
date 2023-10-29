use utoipa::OpenApi;

use super::model::*;
use super::paths::track::*;

#[derive(OpenApi)]
#[openapi(
    components(
        responses(TrackMetadataResponse),
        schemas(FullTrackMetadata, TrackEmbeddedAlbumMetadata, MinifiedArtistMetadata)
    ),
    paths(
        get_track_metadata
    )
)]
pub struct ApiDoc;
