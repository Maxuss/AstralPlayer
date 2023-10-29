use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

//#region Responses

/// Aggregated response for *metadata* of a single track.
///
/// Routes:
/// * GET /track/{id}/metadata -> to get metadata
#[derive(Debug, Clone, Serialize, ToResponse)]
pub struct TrackMetadataResponse {
    /// UUID of the track requested
    #[response(example = "4e4002e9-712f-405d-bb63-f48677e80522")]
    pub track_id: Uuid,
    /// The contained metadata
    pub metadata: FullTrackMetadata
}

//#endregion

//#region Object parts

/// The full aggregated metadata of a track
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FullTrackMetadata {
    /// Name of the track
    #[schema(example = "Gift")]
    pub track_name: String,
    /// Length of a track in milliseconds
    #[schema(example = 104000)]
    pub track_length: u32,
    /// Minified metadata for artists who made this track
    pub artists: Vec<MinifiedArtistMetadata>,
    /// Albums that this track is part of
    pub albums: Vec<TrackEmbeddedAlbumMetadata>
}

/// Album metadata that is included in track metadata, therefore is stripped of certain fields.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TrackEmbeddedAlbumMetadata {
    /// UUID of this album
    #[schema(example = "d156b05e-3270-4f03-bf82-1b89003c2d76")]
    pub album_id: Uuid,
    /// Name of this album
    #[schema(example = "haha")]
    pub album_name: String,
    /// IDs of artists that made this album
    #[schema(example = example_artist_ids)]
    pub artist_ids: Vec<Uuid>
}

/// Essential, but minified artist metadata
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MinifiedArtistMetadata {
    /// UUID of this artist
    pub artist_id: Uuid,
    /// Name of the artist
    #[schema(example = "The Garden")]
    pub artist_name: String,
    /// IDs of albums by this artist.
    ///
    /// You will have to do GET requests to `/album/{id}/metadata` to get album metadata, as this is minified metadata.
    #[schema(example = example_album_ids)]
    pub album_ids: Vec<Uuid>,
    /// Genres most prominent in this artist's discography. Returns top 3 genres.
    ///
    /// You can do a GET request to `/stats/artist/{id}/genres` to get all genres and their statistics.
    #[schema(example = example_genres)]
    pub genres: Vec<String>,
}

const fn example_album_ids() -> [&'static str; 2] {
    ["d156b05e-3270-4f03-bf82-1b89003c2d76", "1b2c7662-8fa8-4511-acb1-823e52c4241a"]
}

const fn example_artist_ids() -> [&'static str; 1] {
    ["19991d5d-f70f-4c8f-b009-666df17b30d9"]
}

const fn example_genres() -> [&'static str; 3] {
    ["experimental rock", "punk", "art punk"]
}

//#endregion Object parts