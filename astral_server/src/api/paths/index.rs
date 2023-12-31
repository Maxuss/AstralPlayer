use std::str::FromStr;
use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDateTime;
use futures_util::StreamExt;
use mongodb::bson::{doc, Document, from_bson};
use serde::Deserialize;
use crate::api::AppState;
use crate::api::extensions::AuthenticatedUser;
use crate::api::model::{IndexedAlbum, IndexedArtist, IndexedTrack};
use crate::data::model::{AlbumMetadata, BsonId, TrackFormat, UserAccount};
use crate::err::AstralError;
use crate::Res;

/// Parameters used for indexation
#[derive(Deserialize)]
pub struct IndexParameters {
    /// Amount of indices to skip. Used for pagination
    pub skip: u32,
    /// Count of indices to provide
    pub count: u32,
    /// Optional search query
    pub search: Option<String>,
}

/// Fetches all albums based on the skip and count parameters
#[utoipa::path(
    get,
    path = "/index/albums",
    params(
        ("skip" = u32, Query, description = "Amount of album indices to skip"),
        ("count" = u32, Query, description = "Amount of albums to provide"),
        ("search" = Option<String>, Query, description = "Optional search query"),
    ),
    responses(
        (status = 200, body = [IndexedAlbum], description = "Successfully fetched album index"),
        (status = 400, response = AstralError)
    ),
    tag = "index"
)]
pub async fn index_albums(
    State(AppState { db, .. }): State<AppState>,
    Query(IndexParameters { skip, count, search }): Query<IndexParameters>,
    AuthenticatedUser(user): AuthenticatedUser
) -> Res<Json<Vec<IndexedAlbum>>> {
    let (first_doc, second_doc) = if let Some(search) = search {
        (doc! { "$or": [{"$text": { "$search": &search }}, { "name": { "$regex": format!("(?i){search}") } }] }, doc! { "score": {"$meta": "textScore"}, "name": 1, "_id": 1 })
    } else {
        (doc! { }, doc! { "name": 1, "_id": 1 })
    };
    let found = db.albums_metadata.aggregate(vec![
        doc! { "$match": first_doc },
        doc! { "$sort": second_doc },
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
        .map(|each| extract_indexed_album(each, &user))
        .filter_map(|each| async { each.ok() });

    Ok(Json(mapped.collect::<Vec<_>>().await))
}

/// Fetches all artists based on the skip and count parameters
#[utoipa::path(
    get,
    path = "/index/artists",
    params(
        ("skip" = u32, Query, description = "Amount of artist indices to skip"),
        ("count" = u32, Query, description = "Amount of artists to provide"),
        ("search" = Option<String>, Query, description = "Optional search query"),
    ),
    responses(
        (status = 200, body = [IndexedArtist], description = "Successfully fetched artist index"),
        (status = 400, response = AstralError)
    ),
    tag = "index"
)]
pub async fn index_artists(
    State(AppState { db, .. }): State<AppState>,
    Query(IndexParameters { skip, count, search }): Query<IndexParameters>,
    AuthenticatedUser(_): AuthenticatedUser
) -> Res<Json<Vec<IndexedArtist>>> {
    let (first_doc, second_doc) = if let Some(search) = search {
        (doc! { "$or": [{"$text": { "$search": &search }}, { "name": { "$regex": format!("(?i){search}") } }] }, doc! { "score": {"$meta": "textScore"}, "name": 1, "_id": 1 })
    } else {
        (doc! { }, doc! { "name": 1, "_id": 1 })
    };
    let found = db.artists_metadata.aggregate(vec![
        doc! { "$match": first_doc },
        doc! { "$sort": second_doc },
        doc! {
            "$skip": skip,
        },
        doc!{
            "$limit": count,
        },
    ], None).await?;
    let mapped = found
        .filter_map(|each| async { each.ok() })
        .map(extract_indexed_artist)
        .filter_map(|each| async { each.ok() });

    Ok(Json(mapped.collect::<Vec<_>>().await))
}

/// Fetches all albums based on the skip and count parameters
#[utoipa::path(
    get,
    path = "/index/tracks",
    params(
        ("skip" = u32, Query, description = "Amount of track indices to skip"),
        ("count" = u32, Query, description = "Amount of tracks to provide"),
        ("search" = Option<String>, Query, description = "Optional search query"),
    ),
    responses(
        (status = 200, body = [IndexedTrack], description = "Successfully fetched track index"),
        (status = 400, response = AstralError)
    ),
    tag = "index"
)]
pub async fn index_tracks(
    State(AppState { db, .. }): State<AppState>,
    Query(IndexParameters { skip, count, search }): Query<IndexParameters>,
    AuthenticatedUser(user): AuthenticatedUser
) -> Res<Json<Vec<IndexedTrack>>> {
    let (first_doc, second_doc) = if let Some(search) = search {
        (doc! { "$or": [{"$text": { "$search": &search }}, { "name": { "$regex": format!("(?i){search}") } }] }, doc! { "score": {"$meta": "textScore"}, "name": 1, "_id": 1 })
    } else {
        (doc! { }, doc! { "name": 1, "_id": 1 })
    };
    let found = db.tracks_metadata.aggregate(vec![
        doc! { "$match": first_doc },
        doc! { "$sort": second_doc },
        doc! { "$skip": skip },
        doc! { "$limit": count },
        doc! {
            "$lookup": {
                "from": "artists_metadata",
                "localField": "artists",
                "foreignField": "artist_id",
                "as": "artist_objects"
            },
        },
        doc! {
            "$lookup": {
                "from": "albums_metadata",
                "localField": "albums",
                "foreignField": "album_id",
                "as": "album_objects"
            }
        }
    ], None).await?;
    let mapped = found
        .filter_map(|each| async { each.ok() })
        .map(|each| extract_indexed_track(each, &user))
        .filter_map(|each| async { each.ok() });

    Ok(Json(mapped.collect::<Vec<_>>().await))
}


fn extract_indexed_track(doc: Document, user: &UserAccount) -> Res<IndexedTrack> {
    let id = from_bson::<BsonId>(doc.get("track_id").unwrap().to_owned())?;
    if doc.get_array("albums").unwrap().is_empty() {
        return Err(AstralError::NotFound("Invalid track data".to_owned()))
    }
    Ok(IndexedTrack {
        id: id.to_uuid_1(),
        name: doc.get_str("name")?.to_owned(),
        album_id: from_bson::<Vec<BsonId>>(doc.get("albums").unwrap().to_owned())?.first().unwrap().to_uuid_1(),
        album_name: from_bson::<AlbumMetadata>(doc.get_array("album_objects")?.first().unwrap().to_owned()).unwrap().name,
        artists: doc.get_array("artist_objects")?.into_iter()
            .map(|each| each.as_document().unwrap())
            .map(|each| (
                from_bson::<BsonId>(each.get("artist_id").unwrap().to_owned()).unwrap().to_uuid_1(),
                each.get_str("name").unwrap().to_owned())
            )
            .collect(),
        duration: doc.get_i64("length")? as i32,
        format: from_bson::<TrackFormat>(doc.get("format").unwrap().to_owned())?,
        loved: user.loved_tracks.contains(&id),
    })
}

fn extract_indexed_artist(doc: Document) -> Res<IndexedArtist> {
    Ok(IndexedArtist {
        id: from_bson::<BsonId>(doc.get("artist_id").unwrap().to_owned())?.to_uuid_1(),
        name: doc.get_str("name")?.to_owned(),
    })
}

fn extract_indexed_album(doc: Document, user: &UserAccount) -> Res<IndexedAlbum> {
    let id = from_bson::<BsonId>(doc.get("album_id").unwrap().to_owned())?;
    Ok(IndexedAlbum {
        id: id.to_uuid_1(),
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
        loved: user.loved_albums.contains(&id)
    })
}