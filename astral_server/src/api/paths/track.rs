use axum::Json;
use crate::api::model::TrackMetadataResponse;

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/track/{id}/metadata",
    params(
        ("id" = Uuid, Path, description = "UUID of the track")
    ),
    responses(
        (status = 200, response = TrackMetadataResponse)
    )
)]
pub async fn get_track_metadata() -> Json<TrackMetadataResponse> {
    todo!()
}

