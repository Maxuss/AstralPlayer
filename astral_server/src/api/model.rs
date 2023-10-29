use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

//#region Responses

/// Aggregated response for *metadata* of a single track.
#[derive(Debug, Clone, Serialize, ToResponse)]
pub struct TrackMetadataResponse {
    /// UUID of the track requested
    #[response(example = "4e4002e9-712f-405d-bb63-f48677e80522")]
    pub track_id: Uuid,
    /// The contained metadata
    pub metadata: FullTrackMetadata
}

/// Aggregated response for *metadata* of a single artist.
#[derive(Debug, Clone, Serialize, ToResponse)]
pub struct ArtistMetadataResponse {
    /// UUID of the artist requested
    #[response(example = "19991d5d-f70f-4c8f-b009-666df17b30d9")]
    pub artist_id: Uuid,
    /// The contained metadata
    pub metadata: FullArtistMetadata
}


/// Aggregated response for *metadata* of a single album.
#[derive(Debug, Clone, Serialize, ToResponse)]
pub struct AlbumMetadataResponse {
    /// UUID of the album requested
    #[response(example = "d156b05e-3270-4f03-bf82-1b89003c2d76")]
    pub album_id: Uuid,
    /// The contained metadata
    pub metadata: FullAlbumMetadata
}

//#endregion

//#region Object parts
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FullAlbumMetadata {
    /// Name of this album
    #[schema(example = "Kiss My Super Bowl Ring")]
    pub album_name: String,
    /// Minified metadata for artists who made this album
    pub artists: Vec<MinifiedArtistMetadata>,
    /// Minified metadata for all tracks inside this album
    pub tracks: Vec<MinifiedTrackMetadata>,
    /// UTC release date of this album
    #[schema(example = example_date)]
    pub release_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FullArtistMetadata {
    /// Name of the artist
    #[schema(example = "The Garden")]
    pub artist_name: String,
    /// Albums by this artist
    pub albums: Vec<MinifiedAlbumMetadata>,
    /// Genres most prominent in this artist's discography. Returns top 3 genres.
    ///
    /// You can do a GET request to `/stats/artist/{id}/genres` to get all genres and their statistics.
    #[schema(example = example_genres)]
    pub genres: Vec<String>,
    /// String containing description for this artist
    pub about_artist: String,
}

/// The full aggregated metadata of a track
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FullTrackMetadata {
    /// Name of this track
    #[schema(example = "AMPM Truck")]
    pub track_name: String,
    /// Length of this track in milliseconds
    #[schema(example = 340000)]
    pub track_length: u32,
    /// Minified metadata for artists who made this track
    pub artists: Vec<MinifiedArtistMetadata>,
    /// Albums that this track is part of
    pub albums: Vec<MinifiedAlbumMetadata>,
    /// Whether this track contains explicit lyrics
    pub is_explicit: bool,
}

/// Essential, but minified track metadata
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MinifiedTrackMetadata {
    /// UUID of this track
    #[schema(example = "4e4002e9-712f-405d-bb63-f48677e80522")]
    pub track_id: Uuid,
    /// Name of this track
    #[schema(example = "AMPM Truck")]
    pub track_name: String,
    /// Length of this track in milliseconds
    #[schema(example = 340000)]
    pub track_length: u32,
    /// List of artist IDs who made this track
    #[schema(example = example_artist_ids)]
    pub artist_ids: Vec<Uuid>,
    /// Album IDs this track is part of
    #[schema(example = example_album_ids)]
    pub album_ids: Vec<Uuid>,
    /// Whether this track contains explicit lyrics
    pub is_explicit: bool,
}

/// Essential, but minified album metadata
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MinifiedAlbumMetadata {
    /// UUID of this album
    #[schema(example = "d156b05e-3270-4f03-bf82-1b89003c2d76")]
    pub album_id: Uuid,
    /// Name of this album
    #[schema(example = "Kiss My Super Bowl Ring")]
    pub album_name: String,
    /// IDs of artists that made this album
    #[schema(example = example_artist_ids)]
    pub artist_ids: Vec<Uuid>,
    /// IDs of tracks inside this album
    #[schema(example = example_track_ids)]
    pub track_ids: Vec<Uuid>,
    /// UTC release date of this album
    #[schema(example = example_date)]
    pub release_date: DateTime<Utc>,
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
    /// You will have to do GET requests to `/metadata/album/{id}` to get album metadata, as this is minified metadata.
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
const fn example_track_ids() -> [&'static str; 4] {
    [
        "232b0017-6abb-6a86-185d-bcd4d1499a68", "6706d235-cf9c-11f3-d450-c0c277baa1dd",
        "760cb692-c2b5-4727-3a9c-eebb834854d4", "591d1da3-b756-6f2b-84a4-49243ce8a413",
    ]
}

fn example_date() -> DateTime<Utc> {
    NaiveDateTime::from_timestamp_millis(1584046800000).unwrap().and_utc()
}

//#endregion Object parts