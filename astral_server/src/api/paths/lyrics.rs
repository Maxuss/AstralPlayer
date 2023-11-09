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
use crate::metadata::musix::musix_request;
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

/// TODO: merge this with metadata fetching?
pub async fn fetch_musixmatch_lyrics(
    title: String,
    artist: String,
    album: Option<String>,
    usertoken: Option<String>
) -> Res<LyricsStatus> {
    let body = musix_request(&title, &artist, &album, &usertoken).await?;

    let status_code = body["matcher.track.get"]["message"]["header"]["status_code"].as_i64().unwrap();
    if status_code != 200 {
        return match status_code {
            404 => Err(AstralError::NotFound(String::from("Could not find lyrics for this track"))),
            401 => Err(AstralError::BadRequest(String::from("Timed out. Wait a few minutes before trying again."))),
            other => Err(AstralError::BadRequest(format!("Request error {other}: {:?}", body["matcher.track.get"]["message"]["header"])))
        }
    }

    let meta = &body["track.lyrics.get"]["message"]["body"];
    if meta.is_object() {
        if meta["lyrics"]["restricted"].as_i64().unwrap() != 0 {
            return Err(AstralError::BadRequest(String::from("Lyrics for this track are restricted.")))
        }
    }

    extract_lyrics_from_musix(&body)
}

pub fn extract_lyrics_from_musix(body: &Value) -> Res<LyricsStatus> {
    let meta = &body["matcher.track.get"]["message"]["body"];

    if meta["track"]["has_subtitles"].as_i64().unwrap() != 0 {
        // has synced lyrics
        let subtitle = &body["track.subtitles.get"]["message"]["body"]["subtitle_list"][0]["subtitle"];
        if subtitle.is_object() {
            let lines =
                serde_json::from_str::<Vec<MusixmatchLyricLine>>(subtitle["subtitle_body"].as_str().unwrap())?;
            let body =
                lines.into_iter()
                    .map(|each| SyncedLyricLine { start_time_ms: (each.time.total * 1000f32) as u32, line: if each.text.is_empty() { "♪".to_string() } else { each.text } })
                    .collect::<Vec<_>>();
            Ok(LyricsStatus::Synced { lines: body })
        } else {
            Err(AstralError::BadRequest(String::from("Invalid lyrics response")))
        }
    } else if meta["track"]["has_lyrics"].as_i64().unwrap() != 0 {
        // has unsynced
        let body = &body["track.lyrics.get"]["message"]["body"]["lyrics"]["lyrics_body"];
        let lines = body.as_str().unwrap().split('\n').filter(|it| !it.is_empty()).map(String::from).collect::<Vec<_>>();
        Ok(LyricsStatus::Unsynced { lines })
    } else if meta["track"]["instrumental"].as_i64().unwrap() != 0 {
        // instrumental
        Ok(LyricsStatus::Unsynced { lines: vec![String::from("♪ Instrumental")] })
    } else {
        Ok(LyricsStatus::NoLyrics { lines: () })
    }

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