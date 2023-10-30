use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Track metadata representation in the DB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
    /// UUID of the track
    pub track_id: Uuid,
    /// Name of the track
    pub name: String,
    /// Length of the track in ms
    pub length: u32,
    /// Artists who made this track
    pub artists: Vec<Uuid>,
    /// Albums this track is featured in
    pub albums: Vec<Uuid>,
    /// Whether this track contains explicit lyrics
    pub is_explicit: bool,
}

/// Artist metadata representation in the DB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistMetadata {
    /// UUID of the artist
    pub artist_id: Uuid,
    /// Name of the artist
    pub name: String,
    /// Albums this artist produced
    pub albums: Vec<Uuid>,
    /// Tracks this artist produced
    pub tracks: Vec<Uuid>,
    /// Map of all genres to the amount of times they occur within this artist's discography.
    pub genres: HashMap<String, u32>,
    /// Some description for this artist. Can contain markdown.
    pub about: String,
}

/// Album metadata representation in the DB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumMetadata {
    /// UUID of the album
    pub album_id: Uuid,
    /// Name of this album
    pub name: String,
    /// Artists who made this album
    pub artists: Vec<Uuid>,
    /// Tracks within this album
    pub tracks: Vec<Uuid>,
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
    pub user_id: Uuid,
    /// Username of this user
    pub username: String,
    /// Argon2 password hash
    pub password_hash: String,
    /// Milliseconds unix timestamp for the register date
    pub register_date: u64,
}

/// A single invite code record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteCode {
    /// The actual code
    pub code: String,
    /// UUID of the user who issued this invite code
    pub issued_by: Uuid,
    /// Unix timestamp for when this invite code expires
    pub expires_at: u64,
}