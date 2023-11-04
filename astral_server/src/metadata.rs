pub mod binary;

use audiotags::{MimeType, Picture};
use futures_util::StreamExt;
use mongodb::bson::doc;
use crate::data::AstralDatabase;
use crate::data::model::{AlbumMetadata, ArtistMetadata, BsonId, TrackMetadata};
use crate::Res;

pub async fn classify_insert_metadata(
    db: &AstralDatabase,
    metadata: ExtractedTrackMetadata
) -> Res<BsonId> {
    // first check if track even exists
    if let Some(track) = db.tracks_metadata.find_one(doc! { "name": &metadata.name, "length": metadata.duration as u32 }, None).await? {
        // track already exists, we shouldn't do anything
        return Ok(track.track_id)
    }

    let mut new_track_metadata = TrackMetadata {
        track_id: BsonId::new(),
        name: metadata.name,
        length: metadata.duration as u32,
        artists: vec![],
        albums: vec![],
        is_explicit: false, // TODO: implement later
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
            let mut new_album = AlbumMetadata {
                album_id: BsonId::new(),
                name: metadata.album_name,
                artists: vec![],
                tracks: vec![new_track_metadata.track_id],
                release_date: 0,
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

    Ok(new_track_metadata.track_id)
}

#[derive(Debug, Clone)]
pub struct ExtractedTrackMetadata {
    pub name: String,
    pub album_name: String,
    pub artists: Vec<String>,
    pub album_artists: Vec<String>,
    pub cover_art: Option<PictureOwned>,
    pub duration: f64,
}

#[derive(Debug, Clone)]
pub struct PictureOwned {
    pub data: Vec<u8>,
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