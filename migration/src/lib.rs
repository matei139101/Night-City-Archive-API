pub use sea_orm_migration::prelude::*;

mod m20250828_191135_create_source_table;
mod m20250829_075247_create_gear_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250828_191135_create_source_table::Migration),
            Box::new(m20250829_075247_create_gear_table::Migration),
        ]
    }
}
