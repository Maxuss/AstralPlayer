use crate::api::start_axum;

mod api;
pub mod data;
pub mod err;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_axum().await?;

    Ok(())
}
