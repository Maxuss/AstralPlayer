use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use mongodb::bson::doc;
use reqwest::Url;
use reqwest::header::{HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;
use crate::api::AppState;
use crate::api::extensions::AuthenticatedUser;
use crate::api::model::LyricsResponse;
use crate::data::model::{BsonId, LyricsStatus, SyncedLyricLine, TrackLyrics};
use crate::err::AstralError;
use crate::Res;

pub async fn get_lyrics(
    State(AppState { db, .. }): State<AppState>,
    Path(uuid): Path<Uuid>,
    AuthenticatedUser(_): AuthenticatedUser
) -> Res<Json<LyricsResponse>> {
    let uuid = BsonId::from_uuid_1(uuid);
    let found_lyrics = db.lyrics.find_one(doc! { "track_id": uuid }, None).await?;
    if let Some(found_lyrics) = found_lyrics {
        return match found_lyrics.status {
            LyricsStatus::NoLyrics { .. } => Ok(Json(LyricsResponse::NoLyrics)),
            LyricsStatus::Synced { lines } => Ok(Json(LyricsResponse::Synced { lines })),
            LyricsStatus::Unsynced { lines } => Ok(Json(LyricsResponse::Unsynced { lines }))
        };
    }

    let track = db.tracks_metadata.find_one(doc! { "track_id": uuid }, None).await?
        .ok_or_else(|| AstralError::NotFound(String::from("Could not find a track with this UUID")))?;
    let artist = db.artists_metadata.find_one(doc! { "artist_id": { "$in": &track.artists } }, None).await?.map(|it| it.name).unwrap_or(String::new());
    let album = db.albums_metadata.find_one(doc! { "album_id": { "$in": &track.albums } }, None).await?.map(|it| it.name).unwrap_or(String::new());

    let lyrics = fetch_musixmatch_lyrics(track.name, artist, Some(album), None).await?;
    let lyrics = TrackLyrics { track_id: uuid, status: lyrics };
    db.lyrics.insert_one(&lyrics, None).await?;

    return match lyrics.status {
        LyricsStatus::NoLyrics { .. } => Ok(Json(LyricsResponse::NoLyrics)),
        LyricsStatus::Synced { lines } => Ok(Json(LyricsResponse::Synced { lines })),
        LyricsStatus::Unsynced { lines } => Ok(Json(LyricsResponse::Unsynced { lines }))
    };
}

/// TODO: merge this with metadata fetching
pub async fn fetch_musixmatch_lyrics(
    title: String,
    artist: String,
    album: Option<String>,
    usertoken: Option<String>
) -> Res<LyricsStatus> {
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

    println!("{uri}");

    let client = reqwest::Client::new();
    let json = client.get(uri)
        .headers(HeaderMap::from_iter([
            (HeaderName::from_static("authority"), HeaderValue::from_static("apic-desktop.musixmatch.com")),
            (HeaderName::from_static("cookie"), HeaderValue::from_static("x-mmm-token-guid="))
        ]))
        .send().await?
        .json::<Value>().await?;

    let body = serde_json::from_str::<Vec<MusixmatchLyricLine>>(json
        .get("message")
        .unwrap().get("body")
        .unwrap().get("macro_calls")
        .unwrap().get("track.subtitles.get")
        .unwrap().get("message")
        .unwrap().get("body")
        .unwrap().get("subtitle_list")
        .unwrap().as_array()
        .unwrap().get(0)
        .unwrap().get("subtitle")
        .unwrap().get("subtitle_body")
        .unwrap().as_str().unwrap()
    )?.into_iter().map(|each| SyncedLyricLine { start_time_ms: (each.time.total * 1000f32) as u32, line: each.text }).collect::<Vec<_>>();

    Ok(LyricsStatus::Synced { lines: body })
}

#[derive(Debug, Deserialize)]
struct MusixmatchLyricLine {
    text: String,
    time: MusixmatchLyricTime
}

#[derive(Debug, Deserialize)]
struct MusixmatchLyricTime {
    total: f32
}