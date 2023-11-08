use std::env;
use std::net::{Ipv4Addr, SocketAddr};

use axum::{Router, Server};
use axum::routing::{get, patch, post};
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
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

        // upload
        .route("/upload/track/:hint", post(upload::upload_track))
        .route("/upload/guess_metadata/:uuid", post(upload::guess_metadata))
        .route("/upload/fetch_metadata/:uuid", post(upload::fetch_metadata))
        .route("/upload/track/:uuid/patch", patch(upload::patch_track_metadata))
        .route("/upload/album/:uuid/patch", patch(upload::patch_album_metadata))
        .route("/upload/artist/:uuid/patch", patch(upload::patch_artist_metadata))
        .with_state(state);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    Server::bind(&address).serve(router.into_make_service()).await.map_err(anyhow::Error::from)
}