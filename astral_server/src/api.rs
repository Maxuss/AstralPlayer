use std::net::{Ipv4Addr, SocketAddr};
use axum::{Router, Server};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use crate::api::docs::ApiDoc;

/// Contains model definition of requests and response objects
pub mod model;
mod docs;

pub async fn start_axum() -> anyhow::Result<()> {
    let router = Router::new()
        .merge(Redoc::with_url("/docs", ApiDoc::openapi())) // redoc
        .with_state(());

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    Server::bind(&address).serve(router.into_make_service()).await.map_err(anyhow::Error::from)
}