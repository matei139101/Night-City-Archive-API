use sea_orm_migration::{prelude::*, schema::*};
use crate::enums::{Source};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
            Table::create()
                .table(Source::Table)
                //.if_not_exists()
                .col(pk_auto(Source::Id))
                .col(string(Source::Name))
                .col(string(Source::ProductPage))
                .col(date(Source::ReleaseDate))
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Source::Table).to_owned())
            .await
    }
}