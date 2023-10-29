use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

use super::model::*;

#[derive(OpenApi)]
#[openapi(
    components(
        responses(TrackMetadataResponse),
        schemas(FullTrackMetadata, TrackEmbeddedAlbumMetadata, MinifiedArtistMetadata)
    )
)]
pub struct ApiDoc;
