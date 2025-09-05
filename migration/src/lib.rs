pub use sea_orm_migration::prelude::*;

mod enums;
mod m20250828_191135_create_source_table;
mod m20250829_075247_create_gear_table;
mod m20250902_142019_create_stat_table;
mod m20250902_142020_create_skill_table;
mod m20250902_142037_create_ranged_weapon_table;
mod m20250902_142037_create_melee_weapon_table;
mod m20250902_142051_create_armor_table;
mod m20250902_142104_create_role_table;
mod m20250902_142112_create_rule_table;
mod m20250902_142122_create_cyberware_type_table;
mod m20250902_142123_create_cyberware_table;
mod m20250902_142313_create_program_table;
mod m20250902_142328_create_quickhack_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250828_191135_create_source_table::Migration),
            Box::new(m20250829_075247_create_gear_table::Migration),
            Box::new(m20250902_142019_create_stat_table::Migration),
            Box::new(m20250902_142020_create_skill_table::Migration),
            Box::new(m20250902_142037_create_ranged_weapon_table::Migration),
            Box::new(m20250902_142037_create_melee_weapon_table::Migration),
            Box::new(m20250902_142051_create_armor_table::Migration),
            // Box::new(m20250902_142104_create_role_table::Migration),
            // Box::new(m20250902_142112_create_rule_table::Migration),
            Box::new(m20250902_142122_create_cyberware_type_table::Migration),
            Box::new(m20250902_142123_create_cyberware_table::Migration),
            // Box::new(m20250902_142313_create_program_table::Migration),
            // Box::new(m20250902_142328_create_quickhack_table::Migration),
        ]
    }
}
