use utoipa::OpenApi;

use super::model::*;
use super::paths::metadata::*;
use super::paths::auth::*;
use crate::err::AstralError;

#[derive(OpenApi)]
#[openapi(
    components(
        responses(
            TrackMetadataResponse, ArtistMetadataResponse, AlbumMetadataResponse,
            AuthenticationResponse, InviteCodeCheckResponse,
            AstralError,
        ),
        schemas(
            FullTrackMetadata, FullArtistMetadata, FullAlbumMetadata, MinifiedTrackMetadata, MinifiedAlbumMetadata, MinifiedArtistMetadata,
            AuthenticationRequest, RegisterRequest
        )
    ),
    paths(
        get_track_metadata, get_artist_metadata, get_album_metadata,
        register_with_token, login, obtain_access_token
    ),
    tags(
        (name = "metadata", description = "Operations related to getting or updating metadata"),
        (name = "auth", description = "Operations related to authentication, authorization and account creation")
    )
)]
pub struct ApiDoc;
