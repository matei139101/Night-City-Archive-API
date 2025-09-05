use crate::enums::{Cyberware, CyberwareInstall, CyberwareType, Source};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cyberware::Table)
                    .col(pk_auto(Cyberware::Id))
                    .col(string(Cyberware::Name))
                    .col(string(Cyberware::Description))
                    .col(integer(Cyberware::Cost))
                    .col(integer(Cyberware::HumanityLossFlat))
                    .col(string(Cyberware::HumanityLossFormula))
                    .col(ColumnDef::new(Cyberware::Install).enumeration(
                        Alias::new(Cyberware::Install.to_string()),
                        vec![
                            Alias::new(CyberwareInstall::Mall.to_string()),
                            Alias::new(CyberwareInstall::Clinic.to_string()),
                            Alias::new(CyberwareInstall::Hospital.to_string()),
                        ],
                    ))
                    .col(integer(Cyberware::CyberwareTypeId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cyberware_to_cyberware_type")
                            .from(Cyberware::Table, Cyberware::Id)
                            .to(CyberwareType::Table, CyberwareType::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Cyberware::RequiredOptionSlots))
                    .col(integer_null(Cyberware::RequiredFoundationalCyberwareId))
                    .col(integer_null(Cyberware::AvailableOptionSlots))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cyberware_to_required_foundational_cyberware")
                            .from(Cyberware::Table, Cyberware::RequiredFoundationalCyberwareId)
                            .to(Cyberware::Table, Cyberware::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(integer(Cyberware::SourceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cyberware_to_source")
                            .from(Cyberware::Table, Cyberware::SourceId)
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
            .drop_table(Table::drop().table(Cyberware::Table).to_owned())
            .await
    }
}
