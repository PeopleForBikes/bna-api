//! Describes the Ratings schemas.
use super::db::Bna;
use crate::core::resource::schema::City;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize)]
pub(crate) struct Rating {
    // BNA Summary
    /// Rating identifier
    id: Uuid,
    /// City identifier
    city_id: Uuid,
    /// Total rating score
    #[schema(minimum = 0, maximum = 100)]
    score: f64,
    /// Rating version
    /// The format follows the [calver](https://calver.org) specification with
    /// the YY.0M[.Minor] scheme.
    version: String,

    /// BNAInfrastructure
    infrastructure: Infrastructure,

    /// BNA Recreation
    recreation: Recreation,

    /// BNA Opportunity
    opportunity: Opportunity,

    /// BNA Core Services
    core_services: CoreServices,

    /// BNA People
    people: People,

    /// BNA Retail
    retail: Retail,

    /// BNA Transit
    transit: Transit,
}

impl From<Bna> for Rating {
    fn from(value: Bna) -> Self {
        Self {
            id: value.id,
            city_id: value.city_id,
            score: value.score,
            version: value.version,
            infrastructure: Infrastructure {
                low_stress_miles: value.low_stress_miles,
                high_stress_miles: value.high_stress_miles,
            },
            recreation: Recreation {
                community_centers: value.community_centers,
                parks: value.parks,
                trails: value.recreation_trails,
                score: Some(value.score),
            },
            opportunity: Opportunity {
                employment: value.employment,
                higher_education: value.higher_education,
                k12_education: value.k12_education,
                score: Some(value.score),
                technical_vocational_college: value.technical_vocational_college,
            },
            core_services: CoreServices {
                dentists: value.dentists,
                doctors: value.doctors,
                grocery: value.grocery,
                hospitals: value.hospitals,
                pharmacies: value.pharmacies,
                score: Some(value.score),
                social_services: value.social_services,
            },
            people: People {
                people: value.people,
            },
            retail: Retail {
                retail: value.retail,
            },
            transit: Transit {
                transit: value.transit,
            },
        }
    }
}

#[derive(ToSchema, Serialize)]
pub(crate) struct Infrastructure {
    /// Total miles of low-stress streets and paths in the measured area.
    #[schema(examples(127))]
    low_stress_miles: Option<f64>,
    /// Total miles of high-stress streets in the measured area.
    #[schema(examples(253))]
    high_stress_miles: Option<f64>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct Recreation {
    /// BNA category subscore for access to community centers.
    #[schema(minimum = 0, maximum = 100)]
    community_centers: Option<f64>,
    /// BNA category subscore for access to parks.
    #[schema(minimum = 0, maximum = 100)]
    parks: Option<f64>,
    /// BNA category subscore for access to bikeable trails.
    #[schema(minimum = 0, maximum = 100)]
    trails: Option<f64>,
    /// BNA category score for access to recreational facilities.
    #[schema(minimum = 0, maximum = 100)]
    score: Option<f64>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct Opportunity {
    /// BNA category subscore for access to job location areas.
    #[schema(minimum = 0, maximum = 100)]
    employment: Option<f64>,
    /// BNA category subscore for access to universities and colleges.
    #[schema(minimum = 0, maximum = 100)]
    higher_education: Option<f64>,
    /// BNA category subscore for access to k12 schools
    #[schema(minimum = 0, maximum = 100)]
    k12_education: Option<f64>,
    /// BNA category score for access to education and jobs.
    #[schema(minimum = 0, maximum = 100)]
    score: Option<f64>,
    /// BNA category subscore for access to technical and vocational colleges.
    #[schema(minimum = 0, maximum = 100)]
    technical_vocational_college: Option<f64>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct CoreServices {
    /// BNA category subscore for access to dentists.
    #[schema(minimum = 0, maximum = 100)]
    dentists: Option<f64>,
    /// BNA category subscore for access to doctors.
    #[schema(minimum = 0, maximum = 100)]
    doctors: Option<f64>,
    /// BNA category subscore for access to grocery stores.
    #[schema(minimum = 0, maximum = 100)]
    grocery: Option<f64>,
    /// BNA category subscore for access to hospitals.
    #[schema(minimum = 0, maximum = 100)]
    hospitals: Option<f64>,
    /// BNA category subscore for access to pharmacies.
    #[schema(minimum = 0, maximum = 100)]
    pharmacies: Option<f64>,
    /// BNA category score for access to core services.
    #[schema(minimum = 0, maximum = 100)]
    score: Option<f64>,
    /// BNA category subscore for access to social services.
    #[schema(minimum = 0, maximum = 100)]
    social_services: Option<f64>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct People {
    /// BNA category score for access to residential areas.
    #[schema(minimum = 0, maximum = 100)]
    people: Option<f64>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct Retail {
    /// BNA category score for access to major retail centers.
    #[schema(minimum = 0, maximum = 100)]
    retail: Option<f64>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct Transit {
    /// BNA category score for access to major transit stops.
    #[schema(minimum = 0, maximum = 100)]
    transit: Option<f64>,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct Ratings(Vec<Rating>);

impl From<Vec<Bna>> for Ratings {
    fn from(value: Vec<Bna>) -> Self {
        let ratings = value.into_iter().map(Rating::from).collect::<Vec<Rating>>();
        Self(ratings)
    }
}

#[derive(ToSchema, Serialize)]
pub(crate) struct RatingWithCity {
    pub(crate) rating: Rating,
    pub(crate) city: City,
}

#[derive(ToSchema, Serialize)]
pub(crate) struct RatingPost {
    /// City identifier
    city_id: Uuid,
    /// Rating version
    /// The format follows the [calver](https://calver.org) specification with
    /// the YY.0M[.Minor] scheme.
    version: String,

    /// BNAInfrastructure
    infrastructure: Infrastructure,

    /// BNA Recreation
    recreation: Recreation,

    /// BNA Opportunity
    opportunity: Opportunity,

    /// BNA Core Services
    core_services: CoreServices,

    /// BNA People
    people: People,

    /// BNA Retail
    retail: Retail,

    /// BNA Transit
    transit: Transit,
}
