//! Describes the Ratings schemas.
use super::db::Bna;
use crate::core::resource::schema::City;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize, Deserialize)]
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

    /// Measurement of the city's bike infrastructure.
    measure: Measure,
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
            measure: Measure {
                buffered_lane: value.buffered_lane,
                lane: value.lane,
                path: value.path,
                sharrow: value.sharrow,
                track: value.track,
            },
        }
    }
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct Infrastructure {
    /// Total miles of low-stress streets and paths in the measured area.
    #[schema(examples(127))]
    pub(crate) low_stress_miles: Option<f64>,
    /// Total miles of high-stress streets in the measured area.
    #[schema(examples(253))]
    pub(crate) high_stress_miles: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct Recreation {
    /// BNA category subscore for access to community centers.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) community_centers: Option<f64>,
    /// BNA category subscore for access to parks.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) parks: Option<f64>,
    /// BNA category subscore for access to bikeable trails.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) trails: Option<f64>,
    /// BNA category score for access to recreational facilities.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) score: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct Opportunity {
    /// BNA category subscore for access to job location areas.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) employment: Option<f64>,
    /// BNA category subscore for access to universities and colleges.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) higher_education: Option<f64>,
    /// BNA category subscore for access to k12 schools
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) k12_education: Option<f64>,
    /// BNA category score for access to education and jobs.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) score: Option<f64>,
    /// BNA category subscore for access to technical and vocational colleges.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) technical_vocational_college: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct CoreServices {
    /// BNA category subscore for access to dentists.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) dentists: Option<f64>,
    /// BNA category subscore for access to doctors.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) doctors: Option<f64>,
    /// BNA category subscore for access to grocery stores.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) grocery: Option<f64>,
    /// BNA category subscore for access to hospitals.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) hospitals: Option<f64>,
    /// BNA category subscore for access to pharmacies.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) pharmacies: Option<f64>,
    /// BNA category score for access to core services.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) score: Option<f64>,
    /// BNA category subscore for access to social services.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) social_services: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct People {
    /// BNA category score for access to residential areas.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) people: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct Retail {
    /// BNA category score for access to major retail centers.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) retail: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct Transit {
    /// BNA category score for access to major transit stops.
    #[schema(minimum = 0, maximum = 100)]
    pub(crate) transit: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct Ratings(Vec<Rating>);

impl From<Vec<Bna>> for Ratings {
    fn from(value: Vec<Bna>) -> Self {
        let ratings = value.into_iter().map(Rating::from).collect::<Vec<Rating>>();
        Self(ratings)
    }
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct RatingWithCity {
    pub(crate) rating: Rating,
    pub(crate) city: City,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct RatingPost {
    /// City identifier
    pub(crate) city_id: Uuid,
    /// Rating version
    /// The format follows the [calver](https://calver.org) specification with
    /// the YY.0M[.Minor] scheme.
    pub(crate) version: String,
    /// City rating score
    pub(crate) score: f64,
    /// City population size category (small, medium, large).
    pub(crate) pop_size: i32,
    /// City population based on the annual U.S. Census American Community Survey.
    pub(crate) population: i32,
    /// Residential speed limit, if any.
    pub(crate) speed_limit_override: Option<i32>,

    /// BNAInfrastructure
    pub(crate) infrastructure: Infrastructure,

    /// BNA Recreation
    pub(crate) recreation: Recreation,

    /// BNA Opportunity
    pub(crate) opportunity: Opportunity,

    /// BNA Core Services
    pub(crate) core_services: CoreServices,

    /// BNA People
    pub(crate) people: People,

    /// BNA Retail
    pub(crate) retail: Retail,

    /// BNA Transit
    pub(crate) transit: Transit,

    /// Measurement of the city's bike infrastructure.
    pub(crate) measure: Measure,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct Measure {
    /// Miles of buffered bike lanes.
    #[schema(examples(53.859))]
    pub(crate) buffered_lane: Option<f64>,
    /// Miles of bike lanes.
    #[schema(examples(0.0))]
    pub(crate) lane: Option<f64>,
    /// Miles of off-street paths.
    #[schema(examples(53.859))]
    pub(crate) path: Option<f64>,
    /// Miles of sharrows.
    #[schema(examples(53.859))]
    pub(crate) sharrow: Option<f64>,
    /// Miles of tracks.
    #[schema(examples(53.859))]
    pub(crate) track: Option<f64>,
}
