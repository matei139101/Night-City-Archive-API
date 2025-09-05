use sea_orm_migration::{prelude::*};

#[derive(DeriveIden)]
pub enum Source{
    Table,
    Id,
    Name,
    ProductPage,
    ReleaseDate,
}

#[derive(DeriveIden)]
pub enum Gear {
    Table,
    Id,
    Name,
    Description,
    Cost,
    SourceId
}

#[derive(DeriveIden)]
pub enum Stat {
    Table,
    Id,
    Name,
    Group,
    Description,
    SourceId,
}
#[derive(DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "stat_group")]
pub enum StatGroup {
    #[sea_orm(string_value = "Mental Group")]
    Mental,
    #[sea_orm(string_value = "Combat Group")]
    Combat,
    #[sea_orm(string_value = "Fortune Group")]
    Fortune,
    #[sea_orm(string_value = "Physical Group")]
    Physical,
}

#[derive(DeriveIden)]
pub enum Skill {
    Table,
    Id,
    Name,
    Description,
    DerivedStatId,
    SourceId,
}
#[derive(DeriveIden)]
pub enum Weapon {
    Id,
    Name,
    Description,
    SourceId
}
#[derive(DeriveIden)]
pub enum MeleeWeapon {
    Table,
    Id,
    Name,
    Description,
    Damage,
    RateOfFire,
    CanBeConcealed,
    Cost,
    HandsRequired,
    IsExotic,
    SourceId,
}

#[derive(DeriveIden)]
pub enum RangedWeapon{
    Table,
    Id,
    Name,
    Description,
    SingleShotDamage,
    RateOfFire,
    CanBeConcealed,
    Cost,
    HandsRequired,
    IsExotic,
    SpecialFeatures,
    SourceId,
}
#[derive(DeriveIden)]
pub enum Armor{
    Table,
    Name,
    Description,
    StoppingPower,
    ArmorPenalty,
    Cost,
    SourceId
}