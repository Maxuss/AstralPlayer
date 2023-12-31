use std::env;
use std::net::{Ipv4Addr, SocketAddr};

use axum::{Router};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::routing::{get, patch, post};
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::docs::ApiDoc;
use crate::api::extensions::try_obtain_paseto_secret;
use crate::data::AstralDatabase;

use paths::*;

/// Contains model definition of requests and response objects
pub mod model;
mod docs;
pub mod paths;
pub mod extensions;

/// Shared app state
#[derive(Clone)]
pub struct AppState {
    /// Paseto symmetric secret key
    pub paseto_key: SymmetricKey<V4>,
    /// Database access
    pub db: AstralDatabase,
}

/// Starts the axum server
pub async fn start_axum() -> anyhow::Result<()> {
    let paseto_key = try_obtain_paseto_secret()?;
    let db = AstralDatabase::connect(env::var("MONGODB_URI")?).await?;

    let state = AppState {
        paseto_key,
        db,
    };

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi())) // swagger

        // metadata
        .route("/metadata/track/:uuid", get(metadata::get_track_metadata))
        .route("/metadata/artist/:uuid", get(metadata::get_artist_metadata))
        .route("/metadata/album/:uuid", get(metadata::get_album_metadata))
        .route("/metadata/album/:uuid/cover", get(metadata::get_album_cover_art))
        .route("/metadata/track/:uuid/cover", get(metadata::get_track_cover_art))

        // lyrics
        .route("/lyrics/:uuid", get(lyrics::get_lyrics))

        // auth
        .route("/auth/register", post(auth::register_with_token))
        .route("/auth/login", post(auth::login))
        .route("/auth/token", get(auth::obtain_access_token))
        .route("/auth/verify", post(auth::verify))

        // upload
        .route("/upload/track/:hint", post(upload::upload_track))
        .route("/upload/guess_metadata/:uuid", post(upload::guess_metadata))
        .route("/upload/track/:uuid/patch", patch(upload::patch_track_metadata))
        .route("/upload/album/:uuid/patch", patch(upload::patch_album_metadata))
        .route("/upload/artist/:uuid/patch", patch(upload::patch_artist_metadata))
        .route("/upload/cover/:uuid", post(upload::change_cover))
        .route("/upload/track/:uuid/delete", post(upload::delete_track))
        .route("/upload/album/:uuid/delete", post(upload::delete_album))

        // streaming
        .route("/stream/:uuid", get(stream::stream_track))
        .route("/stream/:track_id/:quality", get(stream::stream_track_transcoded))

        // indexation
        .route("/index/albums", get(index::index_albums))
        .route("/index/artists", get(index::index_artists))
        .route("/index/tracks", get(index::index_tracks))

        // personal endpoints
        .route("/user/love/track/:track", post(user::love_track))
        .route("/user/unlove/track/:track", post(user::unlove_track))
        .route("/user/love/album/:album", post(user::love_album))
        .route("/user/unlove/album/:album", post(user::unlove_album))

        // metadata (creepy edition)
        .route("/metadata/musixmatch", get(metadata::pass_to_musixmatch))

        .layer(cors)
        .with_state(state);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    axum::serve(TcpListener::bind(&address).await.unwrap(), router.into_make_service()).await.map_err(anyhow::Error::from)
}
