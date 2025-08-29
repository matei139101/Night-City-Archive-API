//use crate::{entity, Gear};
use entity::prelude::Gear;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<entity::gear::Model>>, StatusCode> {
    let gear = Gear::find()
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(gear.into_iter().collect()))
}

pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(gear_id): Path<i32>,
) -> Result<Json<Vec<entity::gear::Model>>, StatusCode> {
    let gear = Gear::find_by_id(gear_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(gear.into_iter().collect()))
}