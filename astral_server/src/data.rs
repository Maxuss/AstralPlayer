use mongodb::{Client, Collection, Database};
use crate::data::model::{AlbumMetadata, ArtistMetadata, InviteCode, TrackMetadata, UserAccount};

/// Contains all database models
pub mod model;

/// MongoDB database wrapper
#[derive(Debug, Clone)]
pub struct AstralDatabase {
    /// Metadata for tracks
    pub tracks_metadata: Collection<TrackMetadata>,
    /// Metadata for artists
    pub artists_metadata: Collection<ArtistMetadata>,
    /// Metadata for albums
    pub albums_metadata: Collection<AlbumMetadata>,
    /// User accounts
    pub accounts: Collection<UserAccount>,
    /// Invite codes used to register
    pub invite_codes: Collection<InviteCode>,
    /// Access to the inner database
    pub inner: Database
}

impl AstralDatabase {
    /// Connects to database using MongoDB connection uri
    pub async fn connect(url: String) -> anyhow::Result<Self> {
        let client = Client::with_uri_str(url).await?;
        let inner = client.database("astral");
        let tracks_metadata = inner.collection("tracks_metadata");
        let artists_metadata = inner.collection("artists_metadata");
        let albums_metadata = inner.collection("albums_metadata");
        let accounts = inner.collection("accounts");
        let invite_codes = inner.collection("invite_codes");
        Ok(Self {
            inner,
            tracks_metadata,
            artists_metadata,
            albums_metadata,
            invite_codes,
            accounts
        })
    }
}