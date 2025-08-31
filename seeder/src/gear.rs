use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ActiveValue::Set};
use entity;
use crate::Seed;


macro_rules! gear {
    ($name:expr, $description:expr, $cost:expr, $source_id:expr) => {
        entity::gear::ActiveModel {
            name: Set($name.to_string()),
            description: Set($description.to_string()),
            cost: Set($cost),
            source_id: Set($source_id),
            ..Default::default()
        }
    };
}
pub struct Gear;
impl Seed for Gear {
    async fn seed(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        entity::gear::Entity::insert_many([
            gear!("Agent", "Self-adaptive AI powered smartphone.", 100, 1),
            gear!("Airhypo", "Easy to use drug distribution platform.", 50, 1),
            gear!("Anti-Smog Breathing Mask", "Useful for filtering out airborne toxins.", 50, 1),
            gear!("Memory Chip", "Standard storage device.", 10, 1),
        ])
        .exec(db)
        .await?;
        Ok(())
    }
}

    // pub id: i32,
    // pub name: String,
    // pub description: String,
    // pub cost: i32,
    // pub source: i32,