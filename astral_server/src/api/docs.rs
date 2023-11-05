use utoipa::{OpenApi, ToSchema};

use super::model::*;
use crate::data::model::*;
use super::paths::metadata::*;
use super::paths::auth::*;
use super::paths::upload::*;
use crate::err::AstralError;

#[derive(OpenApi)]
#[openapi(
    components(
        responses(
            TrackMetadataResponse, ArtistMetadataResponse, AlbumMetadataResponse,
            AuthenticationResponse, InviteCodeCheckResponse,
            UploadTrackResponse,
            AstralError,
        ),
        schemas(
            FullTrackMetadata, FullArtistMetadata, FullAlbumMetadata, MinifiedTrackMetadata, MinifiedAlbumMetadata, MinifiedArtistMetadata,
            AuthenticationRequest, RegisterRequest,
            TrackFormat, BinaryFile
        )
    ),
    paths(
        get_track_metadata, get_artist_metadata, get_album_metadata, get_album_cover_art, get_track_cover_art,
        register_with_token, login, obtain_access_token,
        upload_track, guess_metadata
    ),
    tags(
        (name = "metadata", description = "Operations related to reading metadata"),
        (name = "auth", description = "Operations related to authentication, authorization and account creation"),
        (name = "upload", description = "Operations related to uploading tracks and their metadata")
    )
)]
pub struct ApiDoc;

#[derive(ToSchema)]
#[schema(format = Binary)]
pub struct BinaryFile(Vec<u8>);