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

        -- Census data (simple columns)
        c.fips_code,
        c.pop_size,
        c.population,

        -- Summary version info (simple)
        s.version,

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

        -- Calculated/transformed fields
        s.score,

        -- Scores (derived/calculated metrics)
        opportunity.score AS opportunity_score,
        people.score AS people_score,
        recreation.score AS recreation_score,
        retail.score AS retail_score,
        transit.score AS transit_score,
        core_services.score AS core_services_score,
        COALESCE(sl.residential, 25) AS residential

    FROM city
    INNER JOIN LATERAL (
        SELECT c.*
        FROM census AS c
        WHERE
            c.city_id = city.id
            AND EXTRACT(YEAR FROM c.created_at) = {year}
        ORDER BY c.created_at DESC
        LIMIT 1
    ) AS c ON true
    INNER JOIN LATERAL (
        SELECT s.*
        FROM summary AS s
        WHERE
            s.city_id = city.id
            AND EXTRACT(YEAR FROM s.created_at) = {year}
        ORDER BY s.version DESC
        LIMIT 1
    ) AS s ON true
    LEFT JOIN LATERAL (
        SELECT DISTINCT ON (speed_limit.city_id) speed_limit.residential
        FROM speed_limit
        WHERE
            speed_limit.city_id = city.id
            AND EXTRACT(YEAR FROM speed_limit.created_at) = {year}
        ORDER BY speed_limit.city_id ASC, speed_limit.created_at DESC
        LIMIT 1
    ) AS sl ON true
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

      -- Census data (simple columns)
      c.fips_code,
      c.pop_size,
      c.population,

      -- Summary version info (simple)
      s.version,

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

      -- Calculated/transformed fields
      s.score,

      -- Scores (derived/calculated metrics)
      opportunity.score AS opportunity_score,
      people.score AS people_score,
      recreation.score AS recreation_score,
      retail.score AS retail_score,
      transit.score AS transit_score,
      core_services.score AS core_services_score,
      COALESCE(sl.residential, 25) AS residential

  INNER JOIN census ON city.id = census.city_id
  INNER JOIN summary ON city.id = summary.city_id
  INNER JOIN speed_limit ON city.id = speed_limit.city_id
  INNER JOIN infrastructure ON s.id = infrastructure.id
  INNER JOIN opportunity ON s.id = opportunity.id
  INNER JOIN people ON s.id = people.id
  INNER JOIN recreation ON s.id = recreation.id
  INNER JOIN retail ON s.id = retail.id
  INNER JOIN transit ON s.id = transit.id
  INNER JOIN core_services ON s.id = core_services.id;
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
