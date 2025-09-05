use sea_orm::DatabaseConnection;

pub mod prelude;
mod gear;
mod source;

pub struct Seeder;
impl Seed for Seeder {
    async fn seed(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        source::Source::seed(db).await?;
        gear::Gear::seed(db).await?;
    Ok(())
    }
}
#[allow(async_fn_in_trait)]
pub trait Seed {
    async fn seed(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr>;
}