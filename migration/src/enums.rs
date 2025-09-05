use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Source {
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
    SourceId,
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
pub enum RangedWeapon {
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
pub enum Armor {
    Table,
    Id,
    Name,
    Description,
    StoppingPower,
    ArmorPenalty,
    Cost,
    SourceId,
}
#[derive(DeriveIden)]
pub enum Cyberware {
    Table,
    Id,
    Name,
    CyberwareTypeId,
    Install,
    Description,
    Cost,
    HumanityLossFlat,
    HumanityLossFormula,

    RequiredFoundationalCyberwareId, //Nullable. For when the item has prerequisite foundational cyberware it needs installed.
    RequiredOptionSlots, //Nullable. Number of slots this piece of cyberware takes up of it's foundational cyberware

    /**
     * Nullable.
     * Should only be filled in if this piece of cyberware is foundational.
     * Number of option slots this piece of cyberware has available/
     */
    AvailableOptionSlots,

    SourceId,
}
#[derive(DeriveIden)]
pub enum CyberwareType {
    Table,
    Id,
    Name,
    Description,
}
#[derive(DeriveIden)]
pub enum CyberwareInstall {
    #[sea_orm(string_value = "Mall")] // enum.to_string() now returns this
    Mall,
    #[sea_orm(string_value = "Clinic")] // enum.to_string() now returns this
    Clinic,
    #[sea_orm(string_value = "Hospital")] // enum.to_string() now returns this
    Hospital,
}
