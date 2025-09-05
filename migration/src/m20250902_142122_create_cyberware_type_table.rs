use crate::enums::{CyberwareType};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CyberwareType::Table)
                    .col(pk_auto(CyberwareType::Id))
                    .col(string(CyberwareType::Name))
                    .col(string(CyberwareType::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CyberwareType::Table).to_owned())
            .await
    }
}
