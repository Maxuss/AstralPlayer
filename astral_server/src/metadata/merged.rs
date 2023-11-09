use audiotags::MimeType;
use chrono::NaiveDateTime;
use reqwest::Url;
use crate::api::paths::lyrics::extract_lyrics_from_musix;
use crate::data::model::TrackFormat;
use crate::err::AstralError;
use crate::metadata::binary::extract_metadata_from_bytes;
use crate::metadata::{AlbumArt, ExtractedTrackMetadata};
use crate::metadata::musix::musix_request;
use crate::Res;

/// Attempts to extract metadata from two sources at the same time:
/// 1. Track mp3/flac/m4a metadata
/// 2. MusixMatch services
///
/// It then merges these sources.
pub async fn extract_merged_metadata(
    bytes: &[u8],
    format: TrackFormat,
    prioritize_musix: bool,
    musix_artist_override: Option<String>,
    musix_album_override: Option<String>,
    musix_name_override: Option<String>
) -> Res<ExtractedTrackMetadata> {
    let extracted = extract_metadata_from_bytes(bytes, format)?;
    // we might need to fix track name/album/artist

    let body = musix_request(
        musix_name_override.as_ref().unwrap_or(&extracted.name),
        musix_artist_override.as_ref().unwrap_or(extracted.artists.first().unwrap()),
        &musix_album_override.or(Some(extracted.album_name.clone())), &None)
        .await?;

    let status_code = body["matcher.track.get"]["message"]["header"]["status_code"].as_i64().unwrap();
    if status_code != 200 {
        return match status_code {
            404 => Err(AstralError::NotFound(String::from("Could not find this track in Musixmatch"))),
            401 => Err(AstralError::BadRequest(String::from("Timed out. Wait a few minutes before trying again."))),
            other => Err(AstralError::BadRequest(format!("Request error {other}: {:?}", body["matcher.track.get"]["message"]["header"])))
        }
    }

    let meta = &body["matcher.track.get"]["message"]["body"]["track"];

    let name = if prioritize_musix { meta["track_name"].as_str().unwrap().to_owned() } else { extracted.name };
    let album_name = if prioritize_musix { meta["album_name"].as_str().unwrap().to_owned() } else { extracted.album_name };
    let release_date = if prioritize_musix || extracted.release_date == 0 {
        NaiveDateTime::parse_from_str(meta["first_release_date"].as_str().unwrap(), "%+").unwrap().timestamp_millis() as u64
    } else {
        extracted.release_date
    };
    let artists = meta["artist_name"].as_str().unwrap();
    let artists = if prioritize_musix { artists.split("feat.").map(str::trim).map(String::from).collect::<Vec<_>>() } else { extracted.artists };
    let duration = if prioritize_musix || extracted.duration as i32 == 0 { meta["track_length"].as_f64().unwrap() } else { extracted.duration };
    let is_explicit = meta["explicit"].as_i64().unwrap() == 0;
    let cover_art = if let None = extracted.cover_art {
        let mut cover_art = None;
        for cover_quality in ["800x800", "500x500", "350x350", "100x100"] {
            let cover = meta[&format!("album_coverart_{cover_quality}")].as_str().unwrap();
            // downloading highest quality cover art
            if !cover.is_empty() {
                let mime_type = match cover.split(".").last().unwrap() {
                    "jpg" => MimeType::Jpeg,
                    "png" => MimeType::Png,
                    other => return Err(AstralError::BadRequest(format!("Unhandled mime type! This is an error, mime type: {other}!")))
                };
                cover_art = Some(AlbumArt::Url(Url::parse(cover).unwrap(), mime_type));
                    break;
                }
        };
        cover_art
    } else {
        extracted.cover_art
    };

    Ok(ExtractedTrackMetadata {
        name,
        album_name,
        artists: artists.clone(),
        album_artists: if prioritize_musix { artists } else { extracted.album_artists },
        cover_art,
        duration,
        format: extracted.format,
        number: extracted.number,
        disc_number: extracted.disc_number,
        release_date,
        is_explicit,
        lyrics: extract_lyrics_from_musix(&body).ok()
    })
}