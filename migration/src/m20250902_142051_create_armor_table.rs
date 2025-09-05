use crate::enums::{Armor, Source};
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
                    .table(Armor::Table)
                    .col(pk_auto(Armor::Id))
                    .col(string(Armor::Name))
                    .col(string(Armor::Description))
                    .col(string(Armor::ArmorPenalty))
                    .col(integer(Armor::StoppingPower))
                    .col(integer(Armor::Cost))
                    .col(integer(Armor::SourceId))
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_armor_to_source")
                        .from(Armor::Table, Armor::SourceId)
                        .to(Source::Table, Source::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Armor::Table).to_owned())
            .await
    }
}
