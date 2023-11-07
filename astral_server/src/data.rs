use mongodb::{Client, Collection, Database, GridFsBucket};
use mongodb::options::GridFsBucketOptions;
use crate::data::model::{AlbumMetadata, ArtistMetadata, InviteCode, TrackLyrics, TrackMetadata, UndefinedTrack, UserAccount};

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
    /// Undefined tracks
    pub undefined_tracks: Collection<UndefinedTrack>,
    /// Track lyrics
    pub lyrics: Collection<TrackLyrics>,
    /// GridFS bucket for all the tracks
    pub gridfs_tracks: GridFsBucket,
    /// GridFS bucket for all the album arts
    pub gridfs_album_arts: GridFsBucket,
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
        let undefined_tracks = inner.collection("undefined_tracks");
        let lyrics = inner.collection("lyrics");
        let gridfs_tracks = inner.gridfs_bucket(GridFsBucketOptions::builder().bucket_name(String::from("tracks")).build());
        let gridfs_album_arts = inner.gridfs_bucket(GridFsBucketOptions::builder().bucket_name(String::from("album_arts")).build());
        Ok(Self {
            inner,
            tracks_metadata,
            artists_metadata,
            albums_metadata,
            invite_codes,
            undefined_tracks,
            lyrics,
            gridfs_tracks,
            accounts,
            gridfs_album_arts,
        })
    }
}