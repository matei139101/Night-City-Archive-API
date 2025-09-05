use sea_orm_migration::{prelude::*, schema::*};
use crate::enums::{Source, Stat, StatGroup};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
            .table(Stat::Table)
            .col(pk_auto(Stat::Id))
            .col(string(Stat::Name))
            .col(string(Stat::Description))
            .col(ColumnDef::new(Stat::Group).enumeration(Alias::new("stat_group"), 
            vec![
                Alias::new(StatGroup::Mental.to_string()),
                Alias::new(StatGroup::Combat.to_string()),
                Alias::new(StatGroup::Fortune.to_string()),
                Alias::new(StatGroup::Physical.to_string()),
            ]))
            .col(integer(Stat::SourceId))
                .foreign_key(
                    ForeignKey::create()
                    .name("fk_stat_source_id")
                    .from(Stat::Table, Stat::SourceId)
                    .to(Source::Table, Source::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                )
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Stat::Table).to_owned())
            .await
    }
}