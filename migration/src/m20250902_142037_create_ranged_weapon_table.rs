use crate::enums::{RangedWeapon, Source};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(RangedWeapon::Table)
                    .col(pk_auto(RangedWeapon::Id))
                    .col(string(RangedWeapon::Name))
                    .col(string(RangedWeapon::Description))
                    .col(integer(RangedWeapon::SingleShotDamage))
                    .col(integer_null(RangedWeapon::HandsRequired))
                    .col(integer(RangedWeapon::Cost))
                    .col(integer(RangedWeapon::RateOfFire))
                    .col(boolean(RangedWeapon::CanBeConcealed))
                    .col(boolean(RangedWeapon::IsExotic))
                    .col(string(RangedWeapon::SpecialFeatures))
                    .col(integer(RangedWeapon::SourceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ranged_weapon_source_id")
                            .from(RangedWeapon::Table, RangedWeapon::SourceId)
                            .to(Source::Table, Source::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RangedWeapon::Table).to_owned())
            .await
    }
}
