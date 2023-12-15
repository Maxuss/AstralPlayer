use tokio::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Stdio;
use std::str::FromStr;
use axum::body::Body;
use axum::extract::{Path, State};
use axum_extra::headers::ContentType;
use axum::response::IntoResponse;
use axum_extra::typed_header::TypedHeader;
use axum_macros::debug_handler;
use futures_util::{AsyncReadExt, StreamExt};
use mime::{Mime, MimeIter};
use mongodb::bson::doc;
use mongodb::GridFsDownloadStream;
use serde::Deserialize;
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::process::Command;
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt};
use tokio_util::io::ReaderStream;
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;
use crate::api::AppState;
use crate::api::extensions::AuthenticatedUser;
use crate::data::model::BsonId;
use crate::err::AstralError;
use crate::Res;

/// Returns a stream to the track in the original quality
#[utoipa::path(
    patch,
    path = "/stream/{uuid}",
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = BinaryFile, description = "Obtained track stream")
    ),
    params(
        ("uuid" = Uuid, Path, description = "UUID of the track to stream"),
    ),
    tag = "stream"
)]
#[debug_handler]
pub async fn stream_track(
    State(AppState { db, .. }): State<AppState>,
    Path(track_id): Path<Uuid>,
    AuthenticatedUser(_): AuthenticatedUser
) -> Res<impl IntoResponse> {
    let uid = BsonId::from_uuid_1(track_id);
    let metadata = db.tracks_metadata.find_one(doc! { "track_id": &uid }, None).await?
        .ok_or_else(|| AstralError::NotFound("Couldn't find a track with this ID".to_string()))?;

    let mut req = axum::extract::Request::new(Body::empty());
    ServeFile::new_with_mime(
        std::path::Path::new("astral_tracks").join(format!("{uid}.bin")),
        &Mime::from_str(&String::from(metadata.format)).unwrap()
    ).try_call(req).await.map_err(AstralError::from)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamQuality {
    Low,
    Medium,
}

#[derive(Debug, Deserialize)]
pub struct PathParams {
    track_id: Uuid,
    quality: StreamQuality
}

/// Returns a stream to the track in lowered quality with a transcoded bitrate in MP3 format
#[utoipa::path(
    patch,
    path = "/stream/{uuid}/{quality}",
    responses(
        (status = 400, response = AstralError),
        (status = 200, body = BinaryFile, description = "Obtained track stream")
    ),
    params(
        ("uuid" = Uuid, Path, description = "UUID of the track to stream"),
        ("quality" = String, Path, description = "Quality of the track. Either `low` for 128kb/s, `medium` for 256kb/s, or `high` for 320kb/s")
    ),
    tag = "stream"
)]
pub async fn stream_track_transcoded(
    State(AppState { db, .. }): State<AppState>,
    Path(PathParams { track_id, quality }): Path<PathParams>,
    AuthenticatedUser(_): AuthenticatedUser
) -> Res<impl IntoResponse> {
    let track_exists = db.tracks_metadata.find_one(doc! { "track_id": BsonId::from_uuid_1(track_id) }, None).await?.is_some();
    if !track_exists {
        return Err(AstralError::NotFound("Couldn't find a track with this UUID".to_string()))
    }
    let base_path = PathBuf::from("astral_tracks")
        .join(format!("transcoded_{}", match quality { StreamQuality::Low => "low", StreamQuality::Medium => "medium" }));
    let path = base_path
        .join(format!("{track_id}.bin"));

    let raw_path = PathBuf::from("astral_tracks")
        .join(format!("{track_id}.bin"));
    if !path.exists() {
        // need to transcode the track
        tokio::fs::create_dir_all(&base_path).await?;
        let mut command = Command::new("ffmpeg")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .arg("-v")
            .arg("0")
            .arg("-i")
            .arg(&raw_path)
            .arg("-map")
            .arg("0:a:0")
            .arg("-codec:a")
            .arg("libmp3lame")
            .arg("-b:a")
            .arg(match quality {
                StreamQuality::Low => "128k",
                StreamQuality::Medium => "256k",
            })
            .arg("-f")
            .arg("mp3")
            .arg(&path)
            .spawn()
            .unwrap();

        command.wait().await?;

        let req = axum::extract::Request::new(Body::empty());
        ServeFile::new_with_mime(path, &Mime::from_str("audio/mpeg").unwrap())
            .try_call(req).await.map_err(AstralError::from)
    } else {
        let req = axum::extract::Request::new(Body::empty());
        ServeFile::new_with_mime(path, &Mime::from_str("audio/mpeg").unwrap())
            .try_call(req).await.map_err(AstralError::from)
    }
}

