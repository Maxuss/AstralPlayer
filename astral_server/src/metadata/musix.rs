use std::str::FromStr;
use audiotags::MimeType;
use chrono::{DateTime, NaiveDateTime, Utc};
use futures_util::{AsyncWriteExt, StreamExt, TryStreamExt};
use mongodb::bson::doc;
use mongodb::options::GridFsUploadOptions;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;
use serde_json::Value;
use tokio_util::compat::{FuturesAsyncReadCompatExt, FuturesAsyncWriteCompatExt};
use crate::data::AstralDatabase;
use crate::data::model::{BsonId, TrackFormat};
use crate::err::AstralError;
use crate::metadata::{ExtractedTrackMetadata, PictureOwned};
use crate::Res;

pub async fn fetch_musixmatch_metadata(
    format: TrackFormat,
    title: String,
    artist: String,
    album: Option<String>,
    usertoken: Option<String>,
) -> Res<ExtractedTrackMetadata> {
    const BASE_URL: &str = "https://apic-desktop.musixmatch.com/ws/1.1/macro.subtitles.get?format=json&namespace=lyrics_richsynched&subtitle_format=mxm&app_id=web-desktop-app-v1.0";

    let uri = Url::parse_with_params(BASE_URL, [
        ("q_album", &album.unwrap_or_default()),
        ("q_artist", &artist),
        ("q_artists", &artist),
        ("q_track", &title),
        ("track_spotify_id", &String::new()),
        ("q_duration", &String::new()),
        ("f_subtitle_length", &String::new()),
        ("usertoken", &usertoken.unwrap_or("2005218b74f939209bda92cb633c7380612e14cb7fe92dcd6a780f".to_string()))
    ]).unwrap();

    let client = reqwest::Client::new();
    let json = client.get(uri)
        .headers(HeaderMap::from_iter([
            (HeaderName::from_static("authority"), HeaderValue::from_static("apic-desktop.musixmatch.com")),
            (HeaderName::from_static("cookie"), HeaderValue::from_static("x-mmm-token-guid="))
        ]))
        .send().await?
        .json::<Value>().await?;

    let body = &json["message"]["body"]["macro_calls"];
    let status_code = body["matcher.track.get"]["message"]["header"]["status_code"].as_i64().unwrap();
    if status_code != 200 {
        return match status_code {
            404 => Err(AstralError::NotFound(String::from("Could not find this track in Musixmatch"))),
            401 => Err(AstralError::BadRequest(String::from("Timed out. Wait a few minutes before trying again."))),
            other => Err(AstralError::BadRequest(format!("Request error {other}: {:?}", body["matcher.track.get"]["message"]["header"])))
        }
    }
    let meta = &body["matcher.track.get"]["message"]["body"]["track"];

    let album = meta["album_name"].as_str().unwrap();
    let artist = meta["artist_name"].as_str().unwrap();
    let release_date = NaiveDateTime::parse_from_str(meta["first_release_date"].as_str().unwrap(), "%+").unwrap().timestamp_millis() as u64;
    let name = meta["track_name"].as_str().unwrap();

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
            let mut stream = client.get(Url::from_str(cover).unwrap())
                .send().await?
                .bytes_stream();

            let mut bytes: Vec<u8> = Vec::with_capacity(match &cover[0..2] { "800" => 800 * 800, "500" => 500 * 500, "350" => 350 * 350, "100" => 100 * 100, _ => 0 });

            while let Some(Ok(chunk)) = stream.next().await {
                bytes.extend_from_slice(&chunk);
            }

            cover_art = Some(PictureOwned {
                data: bytes,
                mime: mime_type,
            });
            break;
        }
    };

    // let artist = meta["artist_name"]
    let artists = artist.split("feat.").map(str::trim).map(String::from).collect::<Vec<_>>();

    Ok(ExtractedTrackMetadata {
        name: name.to_string(),
        album_name: album.to_string(),
        artists: artists.clone(),
        album_artists: artists,
        cover_art,
        duration: meta["track_length"].as_f64().unwrap(),
        format,
        number: 0,
        disc_number: 0,
        release_date,
        is_explicit: meta["explicit"].as_i64().unwrap() == 1
    })
}