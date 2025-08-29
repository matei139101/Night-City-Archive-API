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
                .table(Source::Table)
                //.if_not_exists()
                .col(pk_auto(Source::Id))
                .col(string(Source::Name))
                .col(string(Source::Href))
                .to_owned(),
        ).await?;

        // manager
        //     .create_table(
        //     Table::create()
        //         .table(Gear::Table)
        //         .if_not_exists()
        //         .to_owned(),
        // ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Source::Table).to_owned())
            .await?;

        // manager
        //     .drop_table(Table::drop().table(Gear::Table).to_owned())
        //     .await?;
        Ok(())
    }
}

// #[derive(DeriveIden)]
// enum Gear {
//     Table,
//     Id,
//     Title,
//     Text,
// }
#[derive(DeriveIden)]
enum Source{
    Table,
    Id,
    Name,
    Href
}