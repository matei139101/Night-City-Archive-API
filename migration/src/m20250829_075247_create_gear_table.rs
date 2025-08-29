use sea_orm_migration::{prelude::*, schema::*};

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
                .col(string(Gear::Title))
                .col(string(Gear::Description))
                .col(integer(Gear::Cost))
                .col(integer(Gear::Source))
                .foreign_key(
                    ForeignKey::create()
                    .name("source_fk")
                    .from(Gear::Table, Gear::Id)
                    .to(Source::Table, Source::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
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

#[derive(DeriveIden)]
enum Gear {
    Table,
    Id,
    Title,
    Description,
    Cost,
    Source
}

#[derive(DeriveIden)]
enum Source{
    Table,
    Id,
}
