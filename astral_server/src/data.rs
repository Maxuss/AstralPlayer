use mongodb::{Client, Collection, Database};
use crate::data::model::{AlbumMetadata, ArtistMetadata, InviteCode, TrackMetadata, UserAccount};

/// Contains all database models
pub mod model;

#[derive(Debug, Clone)]
pub struct AstralDatabase {
    pub tracks_metadata: Collection<TrackMetadata>,
    pub artists_metadata: Collection<ArtistMetadata>,
    pub albums_metadata: Collection<AlbumMetadata>,
    pub accounts: Collection<UserAccount>,
    pub invite_codes: Collection<InviteCode>,
    pub inner: Database
}

impl AstralDatabase {
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