use sea_orm_migration::{prelude::*, schema::*};
use crate::enums::{Source, Skill, Stat};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
            .table(Skill::Table)
            .col(pk_auto(Skill::Id))
            .col(string(Skill::Name))
            .col(string(Skill::Description))
            .col(integer(Skill::DerivedStatId))
                .foreign_key(
                    ForeignKey::create()
                    .name("fk_skill_stat_id")
                    .from(Skill::Table, Skill::DerivedStatId)
                    .to(Stat::Table, Stat::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                )
            .col(integer(Skill::SourceId))
                .foreign_key(
                    ForeignKey::create()
                    .name("fk_skill_source_id")
                    .from(Skill::Table, Skill::SourceId)
                    .to(Source::Table, Source::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                )
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                manager
            .drop_table(Table::drop().table(Skill::Table).to_owned())
            .await
    }
}
