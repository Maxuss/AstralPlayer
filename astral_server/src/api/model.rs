use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;
use crate::data::model::{SyncedLyricLine, TrackFormat};

//#region Responses

//#region Metadata

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

//#region Auth

/// Authenticated successfully
#[derive(Debug, Clone, Serialize, ToResponse)]
pub struct AuthenticationResponse {
    /// PASETO refresh token (valid for 30 days)
    pub refresh_token: String,
    /// UUID of the user this user was invited by
    pub invited_by: Uuid
}

/// Checks if the invite code is valid
#[derive(Debug, Clone, Serialize, ToResponse)]
pub struct InviteCodeCheckResponse {
    /// Whether the invite code is valid
    pub is_valid: bool
}

//#endregion

//#region Upload

/// Successfully uploaded the track
#[derive(Debug, Clone, Serialize, ToResponse)]
pub struct UploadTrackResponse {
    /// UUID of the track that can be used to upload metadata later
    pub track_id: Uuid
}

//#endregion

//#region Lyrics

/// Found (or not) lyrics for this track
#[derive(Debug, Clone, Serialize, ToResponse)]
#[serde(rename_all = "snake_case", tag = "lyrics_status")]
pub enum LyricsResponse {
    /// No lyrics available for this song
    #[response(example = json!({ "lyrics_status": "no_lyrics" }))]
    NoLyrics,
    /// Unsynced lyrics available
    #[response(example = json!({ "lyrics_status": "unsynced", "lines": ["abc", "def", "ghi"] }))]
    Unsynced {
        /// Unsynced lines
        lines: Vec<String>
    },
    /// Time synced lyrics available
    #[response(example = json!({ "lyrics_status": "synced", "lines": [{ "start_time_ms": 600, "line": "abc" }, { "start_time_ms": 1200, "line": "def" }] }))]
    Synced {
        /// Synced lines
        lines: Vec<SyncedLyricLine>
    }
}

//#endregion

//#region Indexation + Search

/// A single indexed album data
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct IndexedAlbum {
    /// UUID of the album
    pub id: Uuid,
    /// Name of the album
    pub name: String,
    /// Vec of pairs of artist ID to artist name
    pub artists: Vec<(Uuid, String)>,
    /// Vec of track ids in this album
    pub tracks: Vec<Uuid>,
    /// Release date of the album
    pub release_date: DateTime<Utc>,
    /// Prevalent genres of this album
    pub genres: Vec<String>
}

/// A single indexed artist data
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct IndexedArtist {
    /// UUID of the artist
    pub id: Uuid,
    /// Name of the artist
    pub name: String,
}

/// A single indexed track data
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct IndexedTrack {
    /// UUID of the track
    pub id: Uuid,
    /// Name of the track
    pub name: String,
    /// Album UUID
    pub album_id: Uuid,
    /// Album name
    pub album_name: String,
    /// Artist UUIDs and names
    pub artists: Vec<(Uuid, String)>,
    /// Duration in seconds
    pub duration: i32,
    /// Format of this track
    pub format: TrackFormat,
}

//#endregion

//#endregion

//#region Requests

/// An attempt to login into account
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AuthenticationRequest {
    /// Username of the user to authenticate into
    #[schema(example = "maxus")]
    pub username: String,
    /// Password to authenticate with
    #[schema(example = "**********")]
    pub password: String,
}

/// An attempt to create an account with invite code
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RegisterRequest {
    /// Username for the account
    #[schema(example = "maxus")]
    pub username: String,
    /// Password for the account
    #[schema(example = "**********")]
    pub password: String,
    /// A required invite code to register
    #[schema(example = "A53gBf7A")]
    pub invite_code: String,
}

/// Request to change assigned track metadata
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PatchTrackMetadata {
    /// New name of this track
    #[schema(example = "AMPM Truck")]
    pub track_name: Option<String>,
    /// New length of this track in seconds
    #[schema(example = 340)]
    pub track_length: Option<u32>,
    /// Whether this track contains explicit lyrics
    pub is_explicit: Option<bool>,
    /// Positional number of this track
    pub number: Option<u16>,
    /// Number of the disc this track is on
    pub disc_number: Option<u16>,
    /// Artists to be changed for this track
    pub artists: Option<Vec<Uuid>>
}

/// Request to change assigned artist metadata
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PatchArtistMetadata {
    /// New name of the artist
    #[schema(example = "The Garden")]
    pub artist_name: Option<String>,
    /// Albums by this artist
    pub albums: Option<Vec<Uuid>>,
    /// String containing description for this artist. Can contain markdown.
    pub about_artist: Option<String>,
}

/// Request to change assigned artist metadata
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PatchAlbumMetadata {
    /// New name of this album
    #[schema(example = "Kiss My Super Bowl Ring")]
    pub album_name: Option<String>,
    /// All tracks inside this album
    pub tracks: Option<Vec<Uuid>>,
    /// Artists who worked on this album
    pub artists: Option<Vec<Uuid>>,
    /// Unix timestamp in millis for the release date of this album
    pub release_date: Option<u64>,
    /// Most prominent genres in this album.
    #[schema(example = example_genres)]
    pub genres: Option<Vec<String>>,
}

/// Request to fetch track metadata from musixmatch with minimal track info
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct FetchMusixmatchMetadata {
    /// Artist who produced this track
    pub artist: String,
    /// Title of this track
    pub title: String,
    /// Optional album for this track
    pub album: Option<String>
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
    /// Top 3 most prominent genres in this album.
    #[schema(example = example_genres)]
    pub genres: Vec<String>,
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
    /// String containing description for this artist. Can contain markdown.
    pub about_artist: String,
    /// All tracks by this artist
    pub tracks: Vec<Uuid>
}

/// The full aggregated metadata of a track
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FullTrackMetadata {
    /// Name of this track
    #[schema(example = "AMPM Truck")]
    pub track_name: String,
    /// Length of this track in seconds
    #[schema(example = 340)]
    pub track_length: u32,
    /// Minified metadata for artists who made this track
    pub artists: Vec<MinifiedArtistMetadata>,
    /// Albums that this track is part of
    pub albums: Vec<MinifiedAlbumMetadata>,
    /// Whether this track contains explicit lyrics
    pub is_explicit: bool,
    /// Format of this track
    pub format: TrackFormat,
    /// Positional number of this track
    pub number: u16,
    /// Number of the disc this track is on
    pub disc_number: u16,
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
    /// Length of this track in seconds
    #[schema(example = 340)]
    pub track_length: u32,
    /// List of artist IDs who made this track
    #[schema(example = example_artist_ids)]
    pub artist_ids: Vec<Uuid>,
    /// Album IDs this track is part of
    #[schema(example = example_album_ids)]
    pub album_ids: Vec<Uuid>,
    /// Format of this track
    pub format: TrackFormat,
    /// Positional number of this track
    pub number: u16,
    /// Number of the disc this track is on
    pub disc_number: u16,
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
    /// Most prominent genres in this album.
    #[schema(example = example_genres)]
    pub genres: Vec<String>,
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