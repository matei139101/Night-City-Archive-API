use sea_orm::{DatabaseConnection, EntityTrait, ActiveValue::Set, prelude::Date};
use crate::Seed;

macro_rules! source {
    ($name:expr, $product_page:expr, $release_date:expr) => {
        entity::source::ActiveModel {
            name: Set($name.to_string()),
            product_page: Set($product_page.to_string()),
            release_date: Set($release_date),
            ..Default::default()
        }
    };
}
macro_rules! date {
    ($year:expr, $month:expr, $day:expr) => {
        Date::from_ymd_opt($year, $month, $day).unwrap().into()
    };
}

pub struct Source;
impl Seed for Source {
    async fn seed(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        entity::source::Entity::insert_many([
            source!("Cyberpunk RED", "https://www.drivethrurpg.com/en/product/333585", date!(2020, 11, 14)),
            source!("Interface RED Volume 1", "https://www.drivethrurpg.com/en/product/521632", date!(2020, 11, 14)),
            source!("Interface RED Volume 2", "https://www.drivethrurpg.com/en/product/424019", date!(2020, 11, 14)),
            source!("Interface RED Volume 3", "https://www.drivethrurpg.com/en/product/469414", date!(2024, 2, 14)),
            source!("Interface RED Volume 4", "https://www.drivethrurpg.com/en/product/333585", date!(2020, 11, 14)),
        ])
        .exec(db)
        .await?;
        Ok(())
    }
}