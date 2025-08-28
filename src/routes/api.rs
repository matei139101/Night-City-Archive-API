use axum::{
    Router,
};
use crate::routes::{gear};

pub fn routes(db: sea_orm::DatabaseConnection) -> Router{
    let gear_router = gear::routes();
    let router = Router::new()
        .nest("/gear", gear_router)
        .with_state(db);
    router
}