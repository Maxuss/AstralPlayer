use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::api::extensions::UserPermission;

pub type BsonId = mongodb::bson::Uuid;

/// Track metadata representation in the DB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
    /// UUID of the track
    pub track_id: BsonId,
    /// Name of the track
    pub name: String,
    /// Length of the track in ms
    pub length: u32,
    /// Artists who made this track
    pub artists: Vec<BsonId>,
    /// Albums this track is featured in
    pub albums: Vec<BsonId>,
    /// Whether this track contains explicit lyrics
    pub is_explicit: bool,
    /// File format of this track
    pub format: TrackFormat,
    /// Positional number of this track
    pub number: u16,
    /// Number of the disc this track is in
    pub disc_number: u16,
}

/// Artist metadata representation in the DB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistMetadata {
    /// UUID of the artist
    pub artist_id: BsonId,
    /// Name of the artist
    pub name: String,
    /// Albums this artist produced
    pub albums: Vec<BsonId>,
    /// Tracks this artist produced
    pub tracks: Vec<BsonId>,
    /// Map of all genres to the amount of times they occur within this artist's discography.
    pub genres: HashMap<String, u32>,
    /// Some description for this artist. Can contain markdown.
    pub about: String,
}

/// Album metadata representation in the DB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumMetadata {
    /// UUID of the album
    pub album_id: BsonId,
    /// Name of this album
    pub name: String,
    /// Artists who made this album
    pub artists: Vec<BsonId>,
    /// Tracks within this album
    pub tracks: Vec<BsonId>,
    /// Milliseconds unix timestamp for the release date
    pub release_date: u64,
    /// Most prominent genres for this album.
    // fetch from last.fm?
    pub genres: Vec<String>,
}

/// A single user account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    /// UUID of this user
    pub user_id: BsonId,
    /// Username of this user
    pub username: String,
    /// Argon2 password hash
    pub password_hash: String,
    /// Milliseconds unix timestamp for the register date
    pub register_date: u64,
    /// Permissions granted to this user
    pub permissions: Vec<UserPermission>
}

/// A single invite code record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteCode {
    /// The actual code
    pub code: String,
    /// UUID of the user who issued this invite code
    pub issued_by: BsonId,
    /// Unix timestamp for when this invite code expires
    pub expires_at: u64,
    /// Permissions that will be granted to this user on register
    pub permissions: Vec<UserPermission>
}

/// Type of a track format. Other track formats are currently unsupported
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum TrackFormat {
    Flac,
    M4a,
    Mp3
}

/// A single track without metadata assigned yet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndefinedTrack {
    /// UUID of this track
    pub track_id: BsonId,
    /// Sha256 hash of the stored track
    pub hash: String,
    /// UUID of the user who uploaded this track
    pub uploaded_by: BsonId,
    /// Format of the track
    pub format: TrackFormat
}

/// Lyrics container for a single track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackLyrics {
    /// ID of the track
    pub track_id: BsonId,
    /// The lyrics container
    #[serde(flatten)]
    pub status: LyricsStatus,
}

/// Lyrics status for track lyrics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum LyricsStatus {
    /// No lyrics available
    NoLyrics {
        /// No lines
        lines: ()
    },
    /// Time synced lines available
    Synced {
        /// Synced lines
        lines: Vec<SyncedLyricLine>
    },
    /// Time unsynced lines available
    Unsynced {
        /// Unsynced lines
        lines: Vec<String>
    }
}

/// A single time synced lyrics line
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncedLyricLine {
    /// Time in milliseconds when this lyric line starts
    pub start_time_ms: u32,
    /// Contents of this line
    pub line: String
}