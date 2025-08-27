use crate::Gear;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::entity::gear;

#[derive(Serialize)]
struct GearResponse {
    id: i32,
    name: String,
}

impl From<gear::Model> for GearResponse {
    fn from(g: gear::Model) -> Self {
        Self {
            id: g.id,
            name: g.name,
        }
    }
}

pub struct GearRouter;

impl GearRouter {
    pub fn initialize() -> Router<sea_orm::DatabaseConnection> {
        Router::new()
            .route("/get_all", get(get_all_gear))
            .route("/get/{gear_id}", get(get_by_id))
    }
}

async fn get_all_gear(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<GearResponse>>, StatusCode> {
    let gear = Gear::find()
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(gear.into_iter().map(GearResponse::from).collect()))
}

async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(gear_id): Path<i32>,
) -> Result<Json<Vec<GearResponse>>, StatusCode> {
    let gear = Gear::find_by_id(gear_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(gear.into_iter().map(GearResponse::from).collect()))
}
