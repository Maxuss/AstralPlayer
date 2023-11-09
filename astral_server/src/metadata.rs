pub mod binary;
pub mod musix;
pub mod merged;

use audiotags::{MimeType, Picture};
use futures_util::{AsyncWriteExt, StreamExt};
use mongodb::bson::doc;
use mongodb::options::GridFsUploadOptions;
use reqwest::Url;
use crate::data::AstralDatabase;
use crate::data::model::{AlbumMetadata, ArtistMetadata, BsonId, LyricsStatus, TrackFormat, TrackLyrics, TrackMetadata};
use crate::Res;

/// Classifies extracted metadata and inserts it into the Database. Will also download/upload album cover art and lyrics if needed.
pub async fn classify_insert_metadata(
    db: &AstralDatabase,
    metadata: ExtractedTrackMetadata,
    new_uid: BsonId,
) -> Res<BsonId> {
    // first check if track even exists
    if let Some(track) = db.tracks_metadata.find_one(doc! { "name": &metadata.name, "length": metadata.duration as u32 }, None).await? {
        // track already exists, we shouldn't do anything
        return Ok(track.track_id)
    }

    let mut new_track_metadata = TrackMetadata {
        track_id: new_uid,
        name: metadata.name,
        length: metadata.duration as u32,
        artists: vec![],
        albums: vec![],
        is_explicit: metadata.is_explicit,
        format: metadata.format,
        number: metadata.number,
        disc_number: metadata.disc_number
    };

    // album
    let (mut album, should_insert_album) = match db.albums_metadata.find_one(doc! { "name": &metadata.album_name }, None).await? {
        Some(mut album) => {
            album.tracks.push(new_track_metadata.track_id.clone());

            db.albums_metadata.update_one(doc! { "album_id": &album.album_id },  doc! { "$set": {"tracks": &album.tracks} }, None).await?;
            new_track_metadata.albums.push(album.album_id.clone());
            (album, false)
        },
        None => {
            let new_album = AlbumMetadata {
                album_id: BsonId::new(),
                name: metadata.album_name,
                artists: vec![],
                tracks: vec![new_track_metadata.track_id],
                release_date: metadata.release_date,
                genres: vec![],
            };
            new_track_metadata.albums.push(new_album.album_id.clone());

            (new_album, true)
        }
    };

    let mut processed_artists: Vec<String> = vec![];

    // artists that produced this album as a whole
    for artist in metadata.album_artists {
        match db.artists_metadata.find_one(doc! { "name": &artist }, None).await? {
            Some(mut artist) => {
                album.artists.push(artist.artist_id);
                if !artist.albums.contains(&album.album_id) {
                    artist.albums.push(album.album_id.clone());
                }
                artist.tracks.push(new_track_metadata.track_id.clone());
                new_track_metadata.artists.push(artist.artist_id.clone());

                db.artists_metadata.update_one(doc! { "artist_id": &artist.artist_id }, doc! { "$set": { "albums": &artist.albums, "tracks": &artist.tracks } }, None).await?;
            }
            None => {
                let new_artist = ArtistMetadata {
                    artist_id: BsonId::new(),
                    name: artist.clone(),
                    albums: vec![album.album_id.clone()],
                    tracks: vec![new_track_metadata.track_id.clone()],
                    genres: Default::default(),
                    about: "".to_string(),
                };
                new_track_metadata.artists.push(new_artist.artist_id.clone());
                album.artists.push(new_artist.artist_id.clone());
                db.artists_metadata.insert_one(&new_artist, None).await?;
            }
        };
        processed_artists.push(artist);
    }

    // artists that produced this song specifically
    for artist in metadata.artists {
        if processed_artists.contains(&artist) {
            // already fully processed artist as an album artist
            continue
        }
        match db.artists_metadata.find_one(doc! { "name": &artist }, None).await? {
            Some(mut artist) => {
                artist.tracks.push(new_track_metadata.track_id.clone());
                new_track_metadata.artists.push(artist.artist_id.clone());

                db.artists_metadata.update_one(doc! { "artist_id": &artist.artist_id }, doc! { "$set": { "tracks": &artist.tracks } }, None).await?;
            }
            None => {
                let new_artist = ArtistMetadata {
                    artist_id: BsonId::new(),
                    name: artist.clone(),
                    albums: vec![],
                    tracks: vec![new_track_metadata.track_id.clone()],
                    genres: Default::default(),
                    about: "".to_string(),
                };
                new_track_metadata.artists.push(new_artist.artist_id.clone());
                db.artists_metadata.insert_one(&new_artist, None).await?;
            }
        };
    }

    if should_insert_album {
        db.albums_metadata.insert_one(&album, None).await?;
    }

    db.tracks_metadata.insert_one(&new_track_metadata, None).await?;

    // lyrics
    if let Some(lyrics) = metadata.lyrics {
        db.lyrics.insert_one(TrackLyrics {
            track_id: new_track_metadata.track_id.clone(),
            status: lyrics,
        }, None).await?;
    }

    // cover art
    if let Some(picture) = metadata.cover_art {
        let found = db.gridfs_album_arts.find(doc! { "filename": &album.album_id.to_string() }, None).await?;
        if found.count().await > 0 {
            return Ok(new_track_metadata.track_id)
        }

        match picture {
            AlbumArt::Bytes(picture) => {
                let mt: &str = picture.mime.into();
                let mut upload_stream = db.gridfs_album_arts
                    .open_upload_stream(album.album_id.to_string(), GridFsUploadOptions::builder().metadata(doc! { "mime_type": mt }).build());
                upload_stream.write_all(&picture.data).await?;
                upload_stream.flush().await?;
                upload_stream.close().await?;
            }
            AlbumArt::Url(uri, mt) => {
                let mt: String = mt.into();

                let mut d_stream = reqwest::get(uri).await?.bytes_stream();
                let mut u_stream = db.gridfs_album_arts
                    .open_upload_stream(album.album_id.to_string(), GridFsUploadOptions::builder().metadata(doc! { "mime_type": mt }).build());
                while let Some(Ok(mut chunk)) = d_stream.next().await {
                    u_stream.write(&mut chunk).await?;
                }
                u_stream.flush().await?;
                u_stream.close().await?;
            }
        }
    }

    Ok(new_track_metadata.track_id)
}

/// Extracted metadata for a single track
#[derive(Debug, Clone)]
pub struct ExtractedTrackMetadata {
    /// Track name
    pub name: String,
    /// Name of the album
    pub album_name: String,
    /// Artists who worked on this track
    pub artists: Vec<String>,
    /// Artists who worked on this album
    pub album_artists: Vec<String>,
    /// Cover art of this track's album
    pub cover_art: Option<AlbumArt>,
    /// Duration of this track in seconds
    pub duration: f64,
    /// Partially-mime format of this track
    pub format: TrackFormat,
    /// Index of this track in the album
    pub number: u16,
    /// Number of the disc this track appears on
    pub disc_number: u16,
    /// Unix timestamp of the album release date
    pub release_date: u64,
    /// Whether this track contains explicit lyrics
    pub is_explicit: bool,
    /// Lyrics of this track.
    pub lyrics: Option<LyricsStatus>
}

/// Cover art of some album
#[derive(Debug, Clone)]
pub enum AlbumArt {
    /// Bytes extracted from the track metadata
    Bytes(PictureOwned),
    /// URL pointing to where this album's art is stored
    Url(Url, MimeType)
}

/// Owned bytes of album cover art
#[derive(Debug, Clone)]
pub struct PictureOwned {
    /// Image body
    pub data: Vec<u8>,
    /// Image type
    pub mime: MimeType
}

impl<'a> From<Picture<'a>> for PictureOwned {
    fn from(value: Picture<'a>) -> Self {
        Self {
            data: value.data.to_vec(),
            mime: value.mime_type,
        }
    }
}