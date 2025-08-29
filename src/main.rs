mod api;
mod database;
mod routes;

use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::{database::database_utils::make_database_connection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_connection: DatabaseConnection = make_database_connection().await;

    //Run's all migrations that have not yet been applied on current database
    Migrator::up(&database_connection, None).await?;

    let app = routes::api::routes(database_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}