use std::env;
use std::net::{Ipv4Addr, SocketAddr};

use axum::{Router, Server};
use axum::routing::{get, post};
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

        // auth
        .route("/auth/register", post(auth::register_with_token))
        .route("/auth/login", post(auth::login))
        .route("/auth/token", get(auth::obtain_access_token))
        .with_state(state);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    Server::bind(&address).serve(router.into_make_service()).await.map_err(anyhow::Error::from)
}