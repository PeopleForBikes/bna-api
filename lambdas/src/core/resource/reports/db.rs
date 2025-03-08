use super::schema::BnaReport;
use sea_orm::{DatabaseBackend, DatabaseConnection, FromQueryResult, Statement};

pub(crate) async fn fetch_report_year(
    db: &DatabaseConnection,
    year: u32,
) -> Result<Vec<BnaReport>, sea_orm::DbErr> {
    let query = format!(
        r#"
    SELECT
      -- Simple direct columns from tables
      city.id,
      city.name,
      city.state,
      city.state_abbrev,
      city.country,
      city.region,
      city.latitude,
      city.longitude,
      city.fips_code,
      city.residential_speed_limit,

      -- Summary version info (simple)
      s.version,
      s.pop_size,
      s.population,
      s.residential_speed_limit_override,
      s.score,

      -- Simple metrics
      infrastructure.low_stress_miles,
      infrastructure.high_stress_miles,
      opportunity.employment,
      opportunity.higher_education,
      opportunity.k12_education,
      opportunity.technical_vocational_college,
      recreation.community_centers,
      recreation.parks,
      recreation.recreation_trails,
      core_services.dentists,
      core_services.doctors,
      core_services.grocery,
      core_services.hospitals,
      core_services.pharmacies,
      core_services.social_services,

      -- Scores (derived/calculated metrics)
      opportunity.score AS opportunity_score,
      people.score AS people_score,
      recreation.score AS recreation_score,
      retail.score AS retail_score,
      transit.score AS transit_score,
      core_services.score AS core_services_score

    FROM city
    INNER JOIN LATERAL (
        SELECT s.*
        FROM summary AS s
        WHERE
            s.city_id = city.id
            AND EXTRACT(YEAR FROM s.created_at) = {year}
        ORDER BY s.version DESC
        LIMIT 1
    ) AS s ON true
    INNER JOIN infrastructure ON s.id = infrastructure.id
    INNER JOIN opportunity ON s.id = opportunity.id
    INNER JOIN people ON s.id = people.id
    INNER JOIN recreation ON s.id = recreation.id
    INNER JOIN retail ON s.id = retail.id
    INNER JOIN transit ON s.id = transit.id
    INNER JOIN core_services ON s.id = core_services.id;
    "#
    );

    let models = BnaReport::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        query,
        vec![],
    ))
    .all(db)
    .await?;
    Ok(models)
}

pub(crate) async fn fetch_all_reports(
    db: &DatabaseConnection,
) -> Result<Vec<BnaReport>, sea_orm::DbErr> {
    let query = r#"
  SELECT
    -- Simple direct columns from tables
    city.id,
    city.name,
    city.state,
    city.state_abbrev,
    city.country,
    city.region,
    city.latitude,
    city.longitude,
    city.fips_code,
    city.residential_speed_limit,

    -- Summary version info (simple)
    summary.version,
    summary.pop_size,
    summary.population,
    summary.residential_speed_limit_override,
    summary.score,

    -- Simple metrics
    infrastructure.low_stress_miles,
    infrastructure.high_stress_miles,
    opportunity.employment,
    opportunity.higher_education,
    opportunity.k12_education,
    opportunity.technical_vocational_college,
    recreation.community_centers,
    recreation.parks,
    recreation.recreation_trails,
    core_services.dentists,
    core_services.doctors,
    core_services.grocery,
    core_services.hospitals,
    core_services.pharmacies,
    core_services.social_services,


    -- Scores (derived/calculated metrics)
    opportunity.score AS opportunity_score,
    people.score AS people_score,
    recreation.score AS recreation_score,
    retail.score AS retail_score,
    transit.score AS transit_score,
    core_services.score AS core_services_score

  FROM city
  INNER JOIN summary ON city.id = summary.city_id
  INNER JOIN infrastructure ON summary.id = infrastructure.id
  INNER JOIN opportunity ON summary.id = opportunity.id
  INNER JOIN people ON summary.id = people.id
  INNER JOIN recreation ON summary.id = recreation.id
  INNER JOIN retail ON summary.id = retail.id
  INNER JOIN transit ON summary.id = transit.id
  INNER JOIN core_services ON summary.id = core_services.id;
  "#;

    let models = BnaReport::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        query,
        vec![],
    ))
    .all(db)
    .await?;
    Ok(models)
}
