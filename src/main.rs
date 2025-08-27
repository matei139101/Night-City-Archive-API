mod api;
mod database;
mod entity;

use axum::Router;
use entity::prelude::Gear;
use sea_orm::DatabaseConnection;

use crate::{api::gear::GearRouter, database::database_utils::make_database_connection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_connection: DatabaseConnection = make_database_connection().await;

    let gear_router = GearRouter::initialize();

    let app = Router::new()
        .nest("/gear", gear_router)
        .with_state(database_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
