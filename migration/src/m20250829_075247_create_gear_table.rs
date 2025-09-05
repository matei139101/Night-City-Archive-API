use sea_orm_migration::{prelude::*, schema::*};
use crate::enums::{Source, Gear};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
            Table::create()
                .table(Gear::Table)
                .col(pk_auto(Gear::Id))
                .col(string(Gear::Name))
                .col(string(Gear::Description))
                .col(integer(Gear::Cost))
                .col(integer(Gear::SourceId))
                .foreign_key(
                    ForeignKey::create()
                    .name("fk_gear_source_id")
                    .from(Gear::Table, Gear::SourceId)
                    .to(Source::Table, Source::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                )
                //.if_not_exists() //Not supported with mysql
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Gear::Table).to_owned())
            .await
    }
}
