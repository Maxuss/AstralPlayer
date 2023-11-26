use std::str::FromStr;
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDateTime;
use futures_util::StreamExt;
use mongodb::bson::{doc, Document, from_bson};
use mongodb::options::FindOptions;
use serde::Deserialize;
use uuid::Uuid;
use crate::api::AppState;
use crate::api::extensions::AuthenticatedUser;
use crate::api::model::{AlbumMetadataResponse, FullAlbumMetadata, IndexedAlbum, MinifiedAlbumMetadata};
use crate::api::paths::metadata::extract_album_metadata;
use crate::data::AstralDatabase;
use crate::data::model::{AlbumMetadata, BsonId};
use crate::err::AstralError;
use crate::Res;

/// Parameters used for indexation
#[derive(Deserialize)]
pub struct IndexParameters {
    /// Amount of indices to skip. Used for pagination
    pub skip: u32,
    /// Count of indices to provide
    pub count: u32
}

/// Fetches all albums based on the skip and count parameters
#[utoipa::path(
    get,
    path = "/index/albums",
    params(
        ("skip" = u32, Query, description = "Amount of album indices to skip"),
        ("count" = u32, Query, description = "Amount of albums to provide")
    ),
    responses(
        (status = 200, body = [IndexedAlbum], description = "Successfully fetched album index"),
        (status = 400, response = AstralError)
    ),
    tag = "index"
)]

pub async fn index_albums(
    State(AppState { db, .. }): State<AppState>,
    Query(IndexParameters { skip, count }): Query<IndexParameters>,
    AuthenticatedUser(_): AuthenticatedUser
) -> Res<Json<Vec<IndexedAlbum>>> {
    let found = db.albums_metadata.aggregate(vec![
        doc! {
            "$sort": { "name": 1, "_id": 1 }
        },
        doc! {
            "$skip": skip,
        },
        doc!{
            "$limit": count,
        },
        doc! {
            "$lookup": {
                "from": "artists_metadata",
                "localField": "artists",
                "foreignField": "artist_id",
                "as": "artist_objects"
            },
        },
    ], None).await?;
    let mapped = found
        .filter_map(|each| async { each.ok() })
        .map(extract_indexed_album)
        .filter_map(|each| async { each.ok() });

    Ok(Json(mapped.collect::<Vec<_>>().await))
}

fn extract_indexed_album(doc: Document) -> Res<IndexedAlbum> {
    Ok(IndexedAlbum {
        id: from_bson::<BsonId>(doc.get("album_id").unwrap().to_owned())?.to_uuid_1(),
        name: doc.get_str("name")?.to_owned(),
        artists: doc.get_array("artist_objects")?.into_iter()
            .map(|each| each.as_document().unwrap())
            .map(|each| (
                from_bson::<BsonId>(each.get("artist_id").unwrap().to_owned()).unwrap().to_uuid_1(),
                each.get_str("name").unwrap().to_owned())
            )
            .collect(),
        tracks: from_bson::<Vec<BsonId>>(doc.get("tracks").unwrap().to_owned())?.into_iter().map(BsonId::to_uuid_1).collect(),
        release_date: NaiveDateTime::from_timestamp_millis(doc.get_i64("release_date")?).unwrap().and_utc(),
        genres: from_bson(doc.get("genres").unwrap().to_owned())?,
    })
}