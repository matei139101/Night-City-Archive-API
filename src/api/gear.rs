use crate::Gear;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::entity::gear;

#[derive(Serialize)]
pub struct GearResponse {
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

pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<GearResponse>>, StatusCode> {
    let gear = Gear::find()
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(gear.into_iter().map(GearResponse::from).collect()))
}

pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(gear_id): Path<i32>,
) -> Result<Json<Vec<GearResponse>>, StatusCode> {
    let gear = Gear::find_by_id(gear_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(gear.into_iter().map(GearResponse::from).collect()))
}
