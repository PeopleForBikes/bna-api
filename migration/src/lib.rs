pub use sea_orm_migration::prelude::*;

mod m20220101_000001_main;
mod m20231010_232527_city_submission;
mod m20240202_004130_brokenspoke_analyzer_pipeline;
mod m20250529_151932_measure;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_main::Migration),
            Box::new(m20231010_232527_city_submission::Migration),
            Box::new(m20240202_004130_brokenspoke_analyzer_pipeline::Migration),
            Box::new(m20250529_151932_measure::Migration),
        ]
    }
}
