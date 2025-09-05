use sea_orm_migration::{prelude::*, schema::*};
use crate::enums::{MeleeWeapon, Source};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.create_table(
            Table::create()
            .table(MeleeWeapon::Table)
            .col(pk_auto(MeleeWeapon::Id))
            .col(string(MeleeWeapon::Name))
            .col(string(MeleeWeapon::Description))
            .col(integer(MeleeWeapon::Damage))
            .col(integer_null(MeleeWeapon::HandsRequired))
            .col(integer(MeleeWeapon::Cost))
            .col(integer(MeleeWeapon::RateOfFire))
            .col(boolean(MeleeWeapon::CanBeConcealed))
            .col(boolean(MeleeWeapon::IsExotic))
            .col(integer(MeleeWeapon::SourceId))
                .foreign_key(                                    
                    ForeignKey::create()
                    .name("fk_melee_weapon_source_id")
                    .from(MeleeWeapon::Table, MeleeWeapon::SourceId)
                    .to(Source::Table, Source::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade))
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MeleeWeapon::Table).to_owned())
            .await
    }
}
