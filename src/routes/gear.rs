use axum::{
    routing::get,
    Router,
};
use crate::api::gear;

pub fn routes() -> axum::Router<sea_orm::DatabaseConnection>{
    let router = Router::new()
        .route("/{id}", get(gear::get_by_id))
        .route("/", get(gear::get_all_with_source));
    router
}