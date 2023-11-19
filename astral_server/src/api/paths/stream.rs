use std::io::Read;
use std::process::Stdio;
use std::str::FromStr;
use axum::body::StreamBody;
use axum::extract::{Path, State};
use axum::headers::ContentType;
use axum::response::IntoResponse;
use axum::TypedHeader;
use futures_util::{AsyncReadExt, StreamExt};
use mongodb::bson::doc;
use mongodb::GridFsDownloadStream;
use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt};
use tokio_util::io::ReaderStream;
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
pub async fn stream_track(
    State(AppState { db, .. }): State<AppState>,
    Path(track_id): Path<Uuid>,
    AuthenticatedUser(_): AuthenticatedUser
) -> Res<(TypedHeader<ContentType>, StreamBody<ReaderStream<Compat<GridFsDownloadStream>>>)> {
    let uid = BsonId::from_uuid_1(track_id);
    let metadata = db.tracks_metadata.find_one(doc! { "track_id": &uid }, None).await?
        .ok_or_else(|| AstralError::NotFound("Couldn't find a track with this ID".to_string()))?;

    let track = db.gridfs_tracks.open_download_stream_by_name(track_id.to_string(), None).await?;

    let stream = StreamBody::new(ReaderStream::new(track.compat()));
    Ok((
        TypedHeader(ContentType::from_str(&String::from(metadata.format)).unwrap()),
        stream
    ))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamQuality {
    Low,
    Medium,
    High
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
    let mut raw_stream = db.gridfs_tracks.open_download_stream_by_name(track_id.to_string(), None).await?;
    let mut raw_stream = ReaderStream::new(raw_stream.compat());

    let mut command = Command::new("ffmpeg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-v")
        .arg("0")
        .arg("-i")
        .arg("pipe:0")
        .arg("-map")
        .arg("0:a:0")
        .arg("-codec:a")
        .arg("libmp3lame")
        .arg("-b:a")
        .arg(match quality {
            StreamQuality::Low => "128k",
            StreamQuality::Medium => "256k",
            StreamQuality::High => "320k"
        })
        .arg("-f")
        .arg("mp3")
        .arg("-")
        .spawn()
        .unwrap();

    let mut stdin = command.stdin.take().unwrap();

    tokio::task::spawn(async move {
        while let Some(Ok(chunk)) = raw_stream.next().await {
            stdin.write_all(&chunk).await.unwrap();
        }
    });

    let stdout = command.stdout.take().unwrap();
    let stream = ReaderStream::new(stdout).boxed();
    let body = StreamBody::new(stream);

    Ok((
        TypedHeader(ContentType::from_str("audio/mpeg").unwrap()),
        body
    ))
}

