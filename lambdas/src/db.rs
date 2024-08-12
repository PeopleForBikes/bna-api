use entity::{country, prelude::*};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn find_country(db: &DatabaseConnection, country: &str) -> Result<bool, DbErr> {
    Ok(Country::find()
        .filter(country::Column::Name.eq(country))
        .one(db)
        .await?
        .is_some())
}
