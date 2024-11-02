mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///Analysis
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "state_machine_id"
    ///  ],
    ///  "properties": {
    ///    "cost": {
    ///      "description": "Cost of an analysis in USD",
    ///      "examples": [
    ///        6.8941
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "end_time": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "fargate_task_arn": {
    ///      "description": "The ARN of the Fargate task that performed the
    /// analysis",
    ///      "examples": [
    ///        "arn:aws:ecs:us-west-2:123456789012:task/bna/
    /// 29f979fc9fca402d94b014aa23d2f6e0\n"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "results_posted": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "s3_bucket": {
    ///      "description": "the path of the S3 bucket where the results were
    /// stored",
    ///      "examples": [
    ///        "united states/new mexico/santa rosa/24.05.4"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "sqs_message": {
    ///      "description": "Copy of the JSON message that was sent for
    /// processing",
    ///      "examples": [
    ///        "{\"country\":\"United States\",\"city\":\"santa
    /// rosa\",\"region\":\"new mexico\", \"fips_code\":\"3570670\"}"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "start_time": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "state_machine_id": {
    ///      "$ref": "#/components/schemas/state_machine_id"
    ///    },
    ///    "step": {
    ///      "$ref": "#/components/schemas/step"
    ///    },
    ///    "torn_down": {
    ///      "description": "Flag indicating wether the resources were torn down
    /// or not at the end of the analysis\n",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Analysis {
        ///Cost of an analysis in USD
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cost: Option<f64>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end_time: Option<Vec<i64>>,
        ///The ARN of the Fargate task that performed the analysis
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fargate_task_arn: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub results_posted: Option<bool>,
        ///the path of the S3 bucket where the results were stored
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_bucket: Option<String>,
        ///Copy of the JSON message that was sent for processing
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sqs_message: Option<String>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start_time: Option<Vec<i64>>,
        pub state_machine_id: StateMachineId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub step: Option<Step>,
        ///Flag indicating wether the resources were torn down or not at the
        /// end of the analysis
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub torn_down: Option<bool>,
    }

    impl From<&Analysis> for Analysis {
        fn from(value: &Analysis) -> Self {
            value.clone()
        }
    }

    impl Analysis {
        pub fn builder() -> builder::Analysis {
            Default::default()
        }
    }

    ///AnalysisPatch
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cost": {
    ///      "description": "Cost of an analysis in USD",
    ///      "examples": [
    ///        6.8941
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "end_time": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "fargate_task_arn": {
    ///      "description": "The ARN of the Fargate task that performed the
    /// analysis",
    ///      "examples": [
    ///        "arn:aws:ecs:us-west-2:123456789012:task/bna/
    /// 29f979fc9fca402d94b014aa23d2f6e0\n"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "results_posted": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "s3_bucket": {
    ///      "description": "the path of the S3 bucket where the results were
    /// stored",
    ///      "examples": [
    ///        "united states/new mexico/santa rosa/24.05.4"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "sqs_message": {
    ///      "description": "Copy of the JSON message that was sent for
    /// processing",
    ///      "examples": [
    ///        "{\"country\":\"United States\",\"city\":\"santa
    /// rosa\",\"region\":\"new mexico\", \"fips_code\":\"3570670\"}"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "start_time": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "step": {
    ///      "$ref": "#/components/schemas/step"
    ///    },
    ///    "torn_down": {
    ///      "description": "Flag indicating wether the resources were torn down
    /// or not at the end of the analysis\n",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AnalysisPatch {
        ///Cost of an analysis in USD
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cost: Option<f64>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end_time: Option<Vec<i64>>,
        ///The ARN of the Fargate task that performed the analysis
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fargate_task_arn: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub results_posted: Option<bool>,
        ///the path of the S3 bucket where the results were stored
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_bucket: Option<String>,
        ///Copy of the JSON message that was sent for processing
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sqs_message: Option<String>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start_time: Option<Vec<i64>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub step: Option<Step>,
        ///Flag indicating wether the resources were torn down or not at the
        /// end of the analysis
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub torn_down: Option<bool>,
    }

    impl From<&AnalysisPatch> for AnalysisPatch {
        fn from(value: &AnalysisPatch) -> Self {
            value.clone()
        }
    }

    impl AnalysisPatch {
        pub fn builder() -> builder::AnalysisPatch {
            Default::default()
        }
    }

    ///AnalysisPost
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cost": {
    ///      "description": "Cost of an analysis in USD",
    ///      "examples": [
    ///        6.8941
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "end_time": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "fargate_task_arn": {
    ///      "description": "The ARN of the Fargate task that performed the
    /// analysis",
    ///      "examples": [
    ///        "arn:aws:ecs:us-west-2:123456789012:task/bna/
    /// 29f979fc9fca402d94b014aa23d2f6e0\n"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "result_posted": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "s3_bucket": {
    ///      "description": "the path of the S3 bucket where the results were
    /// stored",
    ///      "examples": [
    ///        "united states/new mexico/santa rosa/24.05.4"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "sqs_message": {
    ///      "description": "Copy of the JSON message that was sent for
    /// processing",
    ///      "examples": [
    ///        "{\"country\":\"United States\",\"city\":\"santa
    /// rosa\",\"region\":\"new mexico\", \"fips_code\":\"3570670\"}"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "step": {
    ///      "$ref": "#/components/schemas/step"
    ///    },
    ///    "torn_down": {
    ///      "description": "Flag indicating wether the resources were torn down
    /// or not at the end of the analysis\n",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AnalysisPost {
        ///Cost of an analysis in USD
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cost: Option<f64>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end_time: Option<Vec<i64>>,
        ///The ARN of the Fargate task that performed the analysis
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fargate_task_arn: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub result_posted: Option<bool>,
        ///the path of the S3 bucket where the results were stored
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_bucket: Option<String>,
        ///Copy of the JSON message that was sent for processing
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sqs_message: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub step: Option<Step>,
        ///Flag indicating wether the resources were torn down or not at the
        /// end of the analysis
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub torn_down: Option<bool>,
    }

    impl From<&AnalysisPost> for AnalysisPost {
        fn from(value: &AnalysisPost) -> Self {
            value.clone()
        }
    }

    impl AnalysisPost {
        pub fn builder() -> builder::AnalysisPost {
            Default::default()
        }
    }

    ///API Gateway ID associated with the request
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API Gateway ID associated with the request ",
    ///  "examples": [
    ///    "blfwkg8nvHcEJnQ="
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct ApiGatewayId(pub String);
    impl std::ops::Deref for ApiGatewayId {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<ApiGatewayId> for String {
        fn from(value: ApiGatewayId) -> Self {
            value.0
        }
    }

    impl From<&ApiGatewayId> for ApiGatewayId {
        fn from(value: &ApiGatewayId) -> Self {
            value.clone()
        }
    }

    impl From<String> for ApiGatewayId {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for ApiGatewayId {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for ApiGatewayId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///Bna
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city_id",
    ///    "rating_id",
    ///    "score",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "city_id": {
    ///      "description": "City identifier",
    ///      "examples": [
    ///        "6d1927b4-3474-4ce0-9b2e-2a1f5a7d91bd"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "community_centers": {
    ///      "description": "BNA category subscore for access to community
    /// centers",
    ///      "examples": [
    ///        70.7
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "coreservices_score": {
    ///      "description": "BNA category score for access to core services",
    ///      "examples": [
    ///        78.15
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "dentists": {
    ///      "description": "BNA category subscore for access to dentists",
    ///      "examples": [
    ///        68.69
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "doctors": {
    ///      "description": "BNA category subscore for access to doctors",
    ///      "examples": [
    ///        73.51
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "employment": {
    ///      "description": "BNA category subscore for access to job location
    /// areas",
    ///      "examples": [
    ///        0.0
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "grocery": {
    ///      "description": "BNA category subscore for access to grocery
    /// stores",
    ///      "examples": [
    ///        83.02
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "high_stress_miles": {
    ///      "description": "Total miles of high-stress streets in the measured
    /// area",
    ///      "examples": [
    ///        437.8
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "higher_education": {
    ///      "description": "BNA category subscore for access to universities
    /// and colleges",
    ///      "examples": [
    ///        84.76
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "hospitals": {
    ///      "description": "BNA category subscore for access to hospitals",
    ///      "examples": [
    ///        82.43
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "k12_education": {
    ///      "description": "BNA category subscore for access to k12 schools",
    ///      "examples": [
    ///        6.63
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "low_stress_miles": {
    ///      "description": "Total miles of low-stress streets and paths in the
    /// measured area",
    ///      "examples": [
    ///        1862.2
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "opportunity_score": {
    ///      "description": "BNA category score for access to education and
    /// jobs\"\"",
    ///      "examples": [
    ///        79.91
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "parks": {
    ///      "description": "BNA category subscore for access to parks",
    ///      "examples": [
    ///        78.49
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "people": {
    ///      "description": "BNA category score for access to residential
    /// areas",
    ///      "examples": [
    ///        75.81
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "pharmacies": {
    ///      "description": "BNA category subscore for access to pharmacies",
    ///      "examples": [
    ///        76.62
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "rating_id": {
    ///      "description": "Analysis identifier",
    ///      "examples": [
    ///        "1a759b85-cd87-4bb1-9efa-5789e38e9982"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "recreation_score": {
    ///      "description": "BNA category score for access to recreational
    /// facilities",
    ///      "examples": [
    ///        82.13
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "recreation_trails": {
    ///      "description": "BNA category subscore for access to bikeable
    /// trails",
    ///      "examples": [
    ///        94.45
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "retail": {
    ///      "description": "BNA category score for access to major retail
    /// centers",
    ///      "examples": [
    ///        73.71
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "score": {
    ///      "description": "BNA total score",
    ///      "examples": [
    ///        77.0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "social_services": {
    ///      "description": "BNA category subscore for access to social
    /// services",
    ///      "examples": [
    ///        77.82
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "technical_vocational_college": {
    ///      "description": "BNA category subscore for access to technical and
    /// vocational colleges",
    ///      "examples": [
    ///        81.67
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "transit": {
    ///      "description": "BNA category score for access to major transit
    /// stops",
    ///      "examples": [
    ///        71.59
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "version": {
    ///      "description": "Analysis version. The format follows the [calver](https://calver.org) specification with the YY.0M[.Minor] scheme.\n",
    ///      "examples": [
    ///        "23.02"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Bna {
        ///City identifier
        pub city_id: uuid::Uuid,
        ///BNA category subscore for access to community centers
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub community_centers: Option<f64>,
        ///BNA category score for access to core services
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub coreservices_score: Option<f64>,
        ///BNA category subscore for access to dentists
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dentists: Option<f64>,
        ///BNA category subscore for access to doctors
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub doctors: Option<f64>,
        ///BNA category subscore for access to job location areas
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub employment: Option<f64>,
        ///BNA category subscore for access to grocery stores
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub grocery: Option<f64>,
        ///Total miles of high-stress streets in the measured area
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub high_stress_miles: Option<f64>,
        ///BNA category subscore for access to universities and colleges
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub higher_education: Option<f64>,
        ///BNA category subscore for access to hospitals
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hospitals: Option<f64>,
        ///BNA category subscore for access to k12 schools
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub k12_education: Option<f64>,
        ///Total miles of low-stress streets and paths in the measured area
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub low_stress_miles: Option<f64>,
        ///BNA category score for access to education and jobs""
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub opportunity_score: Option<f64>,
        ///BNA category subscore for access to parks
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parks: Option<f64>,
        ///BNA category score for access to residential areas
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub people: Option<f64>,
        ///BNA category subscore for access to pharmacies
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pharmacies: Option<f64>,
        ///Analysis identifier
        pub rating_id: uuid::Uuid,
        ///BNA category score for access to recreational facilities
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub recreation_score: Option<f64>,
        ///BNA category subscore for access to bikeable trails
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub recreation_trails: Option<f64>,
        ///BNA category score for access to major retail centers
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub retail: Option<f64>,
        pub score: f64,
        ///BNA category subscore for access to social services
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub social_services: Option<f64>,
        ///BNA category subscore for access to technical and vocational
        /// colleges
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub technical_vocational_college: Option<f64>,
        ///BNA category score for access to major transit stops
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub transit: Option<f64>,
        ///Analysis version. The format follows the [calver](https://calver.org) specification with the YY.0M[.Minor] scheme.
        pub version: String,
    }

    impl From<&Bna> for Bna {
        fn from(value: &Bna) -> Self {
            value.clone()
        }
    }

    impl Bna {
        pub fn builder() -> builder::Bna {
            Default::default()
        }
    }

    ///BnaPost
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "core_services",
    ///    "features",
    ///    "infrastructure",
    ///    "opportunity",
    ///    "people",
    ///    "recreation",
    ///    "retail",
    ///    "summary",
    ///    "transit"
    ///  ],
    ///  "properties": {
    ///    "core_services": {
    ///      "$ref": "#/components/schemas/core_services"
    ///    },
    ///    "infrastructure": {
    ///      "$ref": "#/components/schemas/infrastructure"
    ///    },
    ///    "opportunity": {
    ///      "$ref": "#/components/schemas/opportunity"
    ///    },
    ///    "people": {
    ///      "$ref": "#/components/schemas/people"
    ///    },
    ///    "recreation": {
    ///      "$ref": "#/components/schemas/recreation"
    ///    },
    ///    "retail": {
    ///      "$ref": "#/components/schemas/retail"
    ///    },
    ///    "summary": {
    ///      "$ref": "#/components/schemas/bna_summary"
    ///    },
    ///    "transit": {
    ///      "$ref": "#/components/schemas/transit"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct BnaPost {
        pub core_services: CoreServices,
        pub features: serde_json::Value,
        pub infrastructure: Infrastructure,
        pub opportunity: Opportunity,
        pub people: People,
        pub recreation: Recreation,
        pub retail: Retail,
        pub summary: BnaSummary,
        pub transit: Transit,
    }

    impl From<&BnaPost> for BnaPost {
        fn from(value: &BnaPost) -> Self {
            value.clone()
        }
    }

    impl BnaPost {
        pub fn builder() -> builder::BnaPost {
            Default::default()
        }
    }

    ///BnaSummary
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city_id",
    ///    "rating_id",
    ///    "score",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "city_id": {
    ///      "description": "City identifier",
    ///      "examples": [
    ///        "6d1927b4-3474-4ce0-9b2e-2a1f5a7d91bd"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "created_at": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "rating_id": {
    ///      "description": "Analysis identifier",
    ///      "examples": [
    ///        "1a759b85-cd87-4bb1-9efa-5789e38e9982"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "score": {
    ///      "description": "BNA score",
    ///      "examples": [
    ///        77.0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "version": {
    ///      "description": "Analysis version. The format follows the [calver](https://calver.org) specification with the YY.0M[.Minor] scheme.\n",
    ///      "examples": [
    ///        "23.02"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct BnaSummary {
        ///City identifier
        pub city_id: uuid::Uuid,
        ///Date and time
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub created_at: Vec<i64>,
        ///Analysis identifier
        pub rating_id: uuid::Uuid,
        pub score: f64,
        ///Analysis version. The format follows the [calver](https://calver.org) specification with the YY.0M[.Minor] scheme.
        pub version: String,
    }

    impl From<&BnaSummary> for BnaSummary {
        fn from(value: &BnaSummary) -> Self {
            value.clone()
        }
    }

    impl BnaSummary {
        pub fn builder() -> builder::BnaSummary {
            Default::default()
        }
    }

    ///BnaSummaryWithCity
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "allOf": [
    ///      {
    ///        "$ref": "#/components/schemas/bna_summary"
    ///      },
    ///      {
    ///        "$ref": "#/components/schemas/city"
    ///      }
    ///    ]
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct BnaSummaryWithCity(pub Vec<BnaSummaryWithCityItem>);
    impl std::ops::Deref for BnaSummaryWithCity {
        type Target = Vec<BnaSummaryWithCityItem>;
        fn deref(&self) -> &Vec<BnaSummaryWithCityItem> {
            &self.0
        }
    }

    impl From<BnaSummaryWithCity> for Vec<BnaSummaryWithCityItem> {
        fn from(value: BnaSummaryWithCity) -> Self {
            value.0
        }
    }

    impl From<&BnaSummaryWithCity> for BnaSummaryWithCity {
        fn from(value: &BnaSummaryWithCity) -> Self {
            value.clone()
        }
    }

    impl From<Vec<BnaSummaryWithCityItem>> for BnaSummaryWithCity {
        fn from(value: Vec<BnaSummaryWithCityItem>) -> Self {
            Self(value)
        }
    }

    ///BnaSummaryWithCityItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/bna_summary"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/city"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct BnaSummaryWithCityItem {
        pub city_id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<Country>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub created_at: Vec<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub latitude: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub longitude: Option<f64>,
        ///City name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///Analysis identifier
        pub rating_id: uuid::Uuid,
        ///Region name. A region can be a state, a province, a community, or
        /// something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        pub score: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub speed_limit: Option<f64>,
        ///State name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state_abbrev: Option<f64>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub updated_at: Vec<i64>,
        ///Analysis version. The format follows the [calver](https://calver.org) specification with the YY.0M[.Minor] scheme.
        pub version: String,
    }

    impl From<&BnaSummaryWithCityItem> for BnaSummaryWithCityItem {
        fn from(value: &BnaSummaryWithCityItem) -> Self {
            value.clone()
        }
    }

    impl BnaSummaryWithCityItem {
        pub fn builder() -> builder::BnaSummaryWithCityItem {
            Default::default()
        }
    }

    ///Census information
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Census information",
    ///  "type": "object",
    ///  "properties": {
    ///    "census_id": {
    ///      "examples": [
    ///        788
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "city_id": {
    ///      "description": "City identifier",
    ///      "examples": [
    ///        "6d1927b4-3474-4ce0-9b2e-2a1f5a7d91bd"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "created_at": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities\n",
    ///      "examples": [
    ///        "4805000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "pop_size": {
    ///      "description": "City population size category (small (0), medium
    /// (1), large (2))\n",
    ///      "examples": [
    ///        2
    ///      ],
    ///      "type": "integer",
    ///      "enum": [
    ///        0,
    ///        1,
    ///        2
    ///      ]
    ///    },
    ///    "population": {
    ///      "description": "City population",
    ///      "examples": [
    ///        907779
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Census {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub census_id: Option<i64>,
        ///City identifier
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city_id: Option<uuid::Uuid>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub created_at: Vec<i64>,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fips_code: Option<String>,
        ///City population size category (small (0), medium (1), large (2))
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pop_size: Option<CensusPopSize>,
        ///City population
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub population: Option<i64>,
    }

    impl From<&Census> for Census {
        fn from(value: &Census) -> Self {
            value.clone()
        }
    }

    impl Census {
        pub fn builder() -> builder::Census {
            Default::default()
        }
    }

    ///City population size category (small (0), medium (1), large (2))
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "City population size category (small (0), medium (1),
    /// large (2))\n",
    ///  "examples": [
    ///    2
    ///  ],
    ///  "type": "integer",
    ///  "enum": [
    ///    0,
    ///    1,
    ///    2
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Serialize)]
    pub struct CensusPopSize(i64);
    impl std::ops::Deref for CensusPopSize {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<CensusPopSize> for i64 {
        fn from(value: CensusPopSize) -> Self {
            value.0
        }
    }

    impl From<&CensusPopSize> for CensusPopSize {
        fn from(value: &CensusPopSize) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<i64> for CensusPopSize {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
            if ![0_i64, 1_i64, 2_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for CensusPopSize {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }

    ///City
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "city_id": {
    ///      "description": "City identifier",
    ///      "examples": [
    ///        "6d1927b4-3474-4ce0-9b2e-2a1f5a7d91bd"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/country"
    ///    },
    ///    "created_at": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "latitude": {
    ///      "description": "Geographic coordinate that specifies the
    /// north-south position of a point on the surface of the Earth.\n",
    ///      "examples": [
    ///        51.2194
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "longitude": {
    ///      "description": "Geographic coordinate that specifies the east–west
    /// position of a point on the surface of the Earth.\n",
    ///      "examples": [
    ///        4.4025
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "name": {
    ///      "description": "City name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "description": "Region name. A region can be a state, a province, a
    /// community, or something similar depending on the country. If a country
    /// does not have this concept, then the country name is used.\n",
    ///      "examples": [
    ///        "Belgium"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "speed_limit": {
    ///      "description": "Speed limit in kilometer per hour (km/h).",
    ///      "examples": [
    ///        50
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "state": {
    ///      "description": "State name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "state_abbrev": {
    ///      "description": "A short version of the state name, usually 2 or 3
    /// character long.",
    ///      "examples": [
    ///        "VAN"
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "updated_at": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct City {
        ///City identifier
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city_id: Option<uuid::Uuid>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<Country>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub created_at: Vec<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub latitude: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub longitude: Option<f64>,
        ///City name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///Region name. A region can be a state, a province, a community, or
        /// something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub speed_limit: Option<f64>,
        ///State name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state_abbrev: Option<f64>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub updated_at: Vec<i64>,
    }

    impl From<&City> for City {
        fn from(value: &City) -> Self {
            value.clone()
        }
    }

    impl City {
        pub fn builder() -> builder::City {
            Default::default()
        }
    }

    ///CityPost
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "country",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "country": {
    ///      "$ref": "#/components/schemas/country"
    ///    },
    ///    "latitude": {
    ///      "description": "Geographic coordinate that specifies the
    /// north-south position of a point on the surface of the Earth.\n",
    ///      "examples": [
    ///        51.2194
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "longitude": {
    ///      "description": "Geographic coordinate that specifies the east–west
    /// position of a point on the surface of the Earth.\n",
    ///      "examples": [
    ///        4.4025
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "City name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "speed_limit": {
    ///      "description": "Speed limit in kilometer per hour (km/h).",
    ///      "examples": [
    ///        50
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "state": {
    ///      "description": "State name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "state_abbrev": {
    ///      "description": "A short version of the state name, usually 2 or 3
    /// character long.",
    ///      "examples": [
    ///        "VAN"
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct CityPost {
        pub country: Country,
        ///Geographic coordinate that specifies the north-south position of a
        /// point on the surface of the Earth.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub latitude: Option<f64>,
        ///Geographic coordinate that specifies the east–west position of a
        /// point on the surface of the Earth.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub longitude: Option<f64>,
        ///City name
        pub name: String,
        ///Speed limit in kilometer per hour (km/h).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub speed_limit: Option<f64>,
        ///State name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state_abbrev: Option<f64>,
    }

    impl From<&CityPost> for CityPost {
        fn from(value: &CityPost) -> Self {
            value.clone()
        }
    }

    impl CityPost {
        pub fn builder() -> builder::CityPost {
            Default::default()
        }
    }

    ///CoreServices
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "dentists": {
    ///      "description": "BNA category subscore for access to dentists",
    ///      "examples": [
    ///        68.69
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "doctors": {
    ///      "description": "BNA category subscore for access to doctors",
    ///      "examples": [
    ///        73.51
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "grocery": {
    ///      "description": "BNA category subscore for access to grocery
    /// stores",
    ///      "examples": [
    ///        83.02
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "hospitals": {
    ///      "description": "BNA category subscore for access to hospitals",
    ///      "examples": [
    ///        82.43
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "pharmacies": {
    ///      "description": "BNA category subscore for access to pharmacies",
    ///      "examples": [
    ///        76.62
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "score": {
    ///      "description": "BNA total score",
    ///      "examples": [
    ///        77.0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "social_services": {
    ///      "description": "BNA category subscore for access to social
    /// services",
    ///      "examples": [
    ///        77.82
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct CoreServices {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dentists: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub doctors: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub grocery: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hospitals: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pharmacies: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub score: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub social_services: Option<f64>,
    }

    impl From<&CoreServices> for CoreServices {
        fn from(value: &CoreServices) -> Self {
            value.clone()
        }
    }

    impl CoreServices {
        pub fn builder() -> builder::CoreServices {
            Default::default()
        }
    }

    ///Country
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "Australia",
    ///    "Belgium",
    ///    "Brazil",
    ///    "Canada",
    ///    "Chile",
    ///    "Colombia",
    ///    "Croatia",
    ///    "Cuba",
    ///    "England",
    ///    "France",
    ///    "Germany",
    ///    "Greece",
    ///    "Guatemala",
    ///    "Iran",
    ///    "Iraq",
    ///    "Ireland",
    ///    "Italy",
    ///    "Mexico",
    ///    "Netherlands",
    ///    "New Zealand",
    ///    "Northern Ireland",
    ///    "Portugal",
    ///    "Scotland",
    ///    "Spain",
    ///    "United States",
    ///    "Vietnam",
    ///    "Wales"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum Country {
        Australia,
        Belgium,
        Brazil,
        Canada,
        Chile,
        Colombia,
        Croatia,
        Cuba,
        England,
        France,
        Germany,
        Greece,
        Guatemala,
        Iran,
        Iraq,
        Ireland,
        Italy,
        Mexico,
        Netherlands,
        #[serde(rename = "New Zealand")]
        NewZealand,
        #[serde(rename = "Northern Ireland")]
        NorthernIreland,
        Portugal,
        Scotland,
        Spain,
        #[serde(rename = "United States")]
        UnitedStates,
        Vietnam,
        Wales,
    }

    impl From<&Country> for Country {
        fn from(value: &Country) -> Self {
            value.clone()
        }
    }

    impl ToString for Country {
        fn to_string(&self) -> String {
            match *self {
                Self::Australia => "Australia".to_string(),
                Self::Belgium => "Belgium".to_string(),
                Self::Brazil => "Brazil".to_string(),
                Self::Canada => "Canada".to_string(),
                Self::Chile => "Chile".to_string(),
                Self::Colombia => "Colombia".to_string(),
                Self::Croatia => "Croatia".to_string(),
                Self::Cuba => "Cuba".to_string(),
                Self::England => "England".to_string(),
                Self::France => "France".to_string(),
                Self::Germany => "Germany".to_string(),
                Self::Greece => "Greece".to_string(),
                Self::Guatemala => "Guatemala".to_string(),
                Self::Iran => "Iran".to_string(),
                Self::Iraq => "Iraq".to_string(),
                Self::Ireland => "Ireland".to_string(),
                Self::Italy => "Italy".to_string(),
                Self::Mexico => "Mexico".to_string(),
                Self::Netherlands => "Netherlands".to_string(),
                Self::NewZealand => "New Zealand".to_string(),
                Self::NorthernIreland => "Northern Ireland".to_string(),
                Self::Portugal => "Portugal".to_string(),
                Self::Scotland => "Scotland".to_string(),
                Self::Spain => "Spain".to_string(),
                Self::UnitedStates => "United States".to_string(),
                Self::Vietnam => "Vietnam".to_string(),
                Self::Wales => "Wales".to_string(),
            }
        }
    }

    impl std::str::FromStr for Country {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Australia" => Ok(Self::Australia),
                "Belgium" => Ok(Self::Belgium),
                "Brazil" => Ok(Self::Brazil),
                "Canada" => Ok(Self::Canada),
                "Chile" => Ok(Self::Chile),
                "Colombia" => Ok(Self::Colombia),
                "Croatia" => Ok(Self::Croatia),
                "Cuba" => Ok(Self::Cuba),
                "England" => Ok(Self::England),
                "France" => Ok(Self::France),
                "Germany" => Ok(Self::Germany),
                "Greece" => Ok(Self::Greece),
                "Guatemala" => Ok(Self::Guatemala),
                "Iran" => Ok(Self::Iran),
                "Iraq" => Ok(Self::Iraq),
                "Ireland" => Ok(Self::Ireland),
                "Italy" => Ok(Self::Italy),
                "Mexico" => Ok(Self::Mexico),
                "Netherlands" => Ok(Self::Netherlands),
                "New Zealand" => Ok(Self::NewZealand),
                "Northern Ireland" => Ok(Self::NorthernIreland),
                "Portugal" => Ok(Self::Portugal),
                "Scotland" => Ok(Self::Scotland),
                "Spain" => Ok(Self::Spain),
                "United States" => Ok(Self::UnitedStates),
                "Vietnam" => Ok(Self::Vietnam),
                "Wales" => Ok(Self::Wales),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for Country {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Country {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Country {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Enqueue
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "city": {
    ///      "description": "City name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/country"
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities\n",
    ///      "examples": [
    ///        "4805000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "description": "Region name. A region can be a state, a province, a
    /// community, or something similar depending on the country. If a country
    /// does not have this concept, then the country name is used.\n",
    ///      "examples": [
    ///        "Belgium"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Enqueue {
        ///City name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<Country>,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fips_code: Option<String>,
        ///Region name. A region can be a state, a province, a community, or
        /// something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
    }

    impl From<&Enqueue> for Enqueue {
        fn from(value: &Enqueue) -> Self {
            value.clone()
        }
    }

    impl Enqueue {
        pub fn builder() -> builder::Enqueue {
            Default::default()
        }
    }

    ///EnqueuePost
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "city": {
    ///      "description": "City name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/country"
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities\n",
    ///      "examples": [
    ///        "4805000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "description": "Region name. A region can be a state, a province, a
    /// community, or something similar depending on the country. If a country
    /// does not have this concept, then the country name is used.\n",
    ///      "examples": [
    ///        "Belgium"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct EnqueuePost {
        ///City name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<Country>,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fips_code: Option<String>,
        ///Region name. A region can be a state, a province, a community, or
        /// something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
    }

    impl From<&EnqueuePost> for EnqueuePost {
        fn from(value: &EnqueuePost) -> Self {
            value.clone()
        }
    }

    impl EnqueuePost {
        pub fn builder() -> builder::EnqueuePost {
            Default::default()
        }
    }

    ///API Error object as described in <https://jsonapi.org/format/#error-objects>
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API Error object as described in <https://jsonapi.org/format/#error-objects>\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "details": {
    ///      "description": "detailed error message",
    ///      "examples": [
    ///        "the entry was not found"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/api_gateway_id"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/source"
    ///    },
    ///    "status": {
    ///      "description": "HTTP status associated with the error",
    ///      "examples": [
    ///        404
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 400.0
    ///    },
    ///    "title": {
    ///      "description": "Error title",
    ///      "examples": [
    ///        "Item Not Found"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Error {
        ///detailed error message
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<ApiGatewayId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source: Option<Source>,
        ///HTTP status associated with the error
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        ///Error title
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
    }

    impl From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }

    impl Error {
        pub fn builder() -> builder::Error {
            Default::default()
        }
    }

    ///A collection of errors
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A collection of errors",
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/error"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Errors(pub Vec<Error>);
    impl std::ops::Deref for Errors {
        type Target = Vec<Error>;
        fn deref(&self) -> &Vec<Error> {
            &self.0
        }
    }

    impl From<Errors> for Vec<Error> {
        fn from(value: Errors) -> Self {
            value.0
        }
    }

    impl From<&Errors> for Errors {
        fn from(value: &Errors) -> Self {
            value.clone()
        }
    }

    impl From<Vec<Error>> for Errors {
        fn from(value: Vec<Error>) -> Self {
            Self(value)
        }
    }

    ///Features
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "people": {
    ///      "description": "BNA category score for access to residential
    /// areas",
    ///      "examples": [
    ///        75.81
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "retail": {
    ///      "description": "BNA category score for access to major retail
    /// centers",
    ///      "examples": [
    ///        73.71
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "transit": {
    ///      "description": "BNA category score for access to major transit
    /// stops",
    ///      "examples": [
    ///        71.59
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Features {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub people: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub retail: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub transit: Option<f64>,
    }

    impl From<&Features> for Features {
        fn from(value: &Features) -> Self {
            value.clone()
        }
    }

    impl Features {
        pub fn builder() -> builder::Features {
            Default::default()
        }
    }

    ///GetCityCensusResponseItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/city"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/census"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetCityCensusResponseItem {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<City>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<Census>,
    }

    impl From<&GetCityCensusResponseItem> for GetCityCensusResponseItem {
        fn from(value: &GetCityCensusResponseItem) -> Self {
            value.clone()
        }
    }

    impl GetCityCensusResponseItem {
        pub fn builder() -> builder::GetCityCensusResponseItem {
            Default::default()
        }
    }

    ///GetCityRatingsResponseItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/city"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/bna_summary"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetCityRatingsResponseItem {
        City(City),
        BnaSummary(BnaSummary),
    }

    impl From<&GetCityRatingsResponseItem> for GetCityRatingsResponseItem {
        fn from(value: &GetCityRatingsResponseItem) -> Self {
            value.clone()
        }
    }

    impl From<City> for GetCityRatingsResponseItem {
        fn from(value: City) -> Self {
            Self::City(value)
        }
    }

    impl From<BnaSummary> for GetCityRatingsResponseItem {
        fn from(value: BnaSummary) -> Self {
            Self::BnaSummary(value)
        }
    }

    ///GetRatingCityResponseItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/bna_summary_with_city"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/city"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetRatingCityResponseItem {
        BnaSummaryWithCity(BnaSummaryWithCity),
        City(City),
    }

    impl From<&GetRatingCityResponseItem> for GetRatingCityResponseItem {
        fn from(value: &GetRatingCityResponseItem) -> Self {
            value.clone()
        }
    }

    impl From<BnaSummaryWithCity> for GetRatingCityResponseItem {
        fn from(value: BnaSummaryWithCity) -> Self {
            Self::BnaSummaryWithCity(value)
        }
    }

    impl From<City> for GetRatingCityResponseItem {
        fn from(value: City) -> Self {
            Self::City(value)
        }
    }

    ///GetRatingComponent
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "All"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "All",
    ///    "CoreServices",
    ///    "Features",
    ///    "Infratructure",
    ///    "Opportunity",
    ///    "Recreation",
    ///    "Summary"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum GetRatingComponent {
        All,
        CoreServices,
        Features,
        Infratructure,
        Opportunity,
        Recreation,
        Summary,
    }

    impl From<&GetRatingComponent> for GetRatingComponent {
        fn from(value: &GetRatingComponent) -> Self {
            value.clone()
        }
    }

    impl ToString for GetRatingComponent {
        fn to_string(&self) -> String {
            match *self {
                Self::All => "All".to_string(),
                Self::CoreServices => "CoreServices".to_string(),
                Self::Features => "Features".to_string(),
                Self::Infratructure => "Infratructure".to_string(),
                Self::Opportunity => "Opportunity".to_string(),
                Self::Recreation => "Recreation".to_string(),
                Self::Summary => "Summary".to_string(),
            }
        }
    }

    impl std::str::FromStr for GetRatingComponent {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "All" => Ok(Self::All),
                "CoreServices" => Ok(Self::CoreServices),
                "Features" => Ok(Self::Features),
                "Infratructure" => Ok(Self::Infratructure),
                "Opportunity" => Ok(Self::Opportunity),
                "Recreation" => Ok(Self::Recreation),
                "Summary" => Ok(Self::Summary),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for GetRatingComponent {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for GetRatingComponent {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for GetRatingComponent {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///The name of a single request header which caused the error
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name of a single request header which caused the
    /// error",
    ///  "examples": [
    ///    "Authorization"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct Header(pub String);
    impl std::ops::Deref for Header {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Header> for String {
        fn from(value: Header) -> Self {
            value.0
        }
    }

    impl From<&Header> for Header {
        fn from(value: &Header) -> Self {
            value.clone()
        }
    }

    impl From<String> for Header {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Header {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for Header {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///Infrastructure
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "high_stress_miles": {
    ///      "description": "Total miles of high-stress streets in the measured
    /// area",
    ///      "examples": [
    ///        437.8
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "low_stress_miles": {
    ///      "description": "Total miles of low-stress streets and paths in the
    /// measured area",
    ///      "examples": [
    ///        1862.2
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Infrastructure {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub high_stress_miles: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub low_stress_miles: Option<f64>,
    }

    impl From<&Infrastructure> for Infrastructure {
        fn from(value: &Infrastructure) -> Self {
            value.clone()
        }
    }

    impl Infrastructure {
        pub fn builder() -> builder::Infrastructure {
            Default::default()
        }
    }

    ///Opportunity
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "employment": {
    ///      "description": "BNA category subscore for access to job location
    /// areas",
    ///      "examples": [
    ///        0.0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "higher_education": {
    ///      "description": "BNA category subscore for access to universities
    /// and colleges",
    ///      "examples": [
    ///        84.76
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "k12_education": {
    ///      "description": "BNA category subscore for access to k12 schools",
    ///      "examples": [
    ///        6.63
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "score": {
    ///      "description": "BNA category score for access to education and
    /// jobs\"\"",
    ///      "examples": [
    ///        79.91
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "technical_vocational_college": {
    ///      "description": "BNA category subscore for access to technical and
    /// vocational colleges",
    ///      "examples": [
    ///        81.67
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Opportunity {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub employment: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub higher_education: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub k12_education: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub score: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub technical_vocational_college: Option<f64>,
    }

    impl From<&Opportunity> for Opportunity {
        fn from(value: &Opportunity) -> Self {
            value.clone()
        }
    }

    impl Opportunity {
        pub fn builder() -> builder::Opportunity {
            Default::default()
        }
    }

    ///The URI query parameter caused the error
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The URI query parameter caused the error",
    ///  "examples": [
    ///    "/bnas/analysis/e6aade5a-b343-120b-dbaa-bd916cd99221?"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct Parameter(pub String);
    impl std::ops::Deref for Parameter {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Parameter> for String {
        fn from(value: Parameter) -> Self {
            value.0
        }
    }

    impl From<&Parameter> for Parameter {
        fn from(value: &Parameter) -> Self {
            value.clone()
        }
    }

    impl From<String> for Parameter {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Parameter {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for Parameter {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///People
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "score": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct People {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub score: Option<f64>,
    }

    impl From<&People> for People {
        fn from(value: &People) -> Self {
            value.clone()
        }
    }

    impl People {
        pub fn builder() -> builder::People {
            Default::default()
        }
    }

    ///A JSON Pointer [RFC6901](https://tools.ietf.org/html/rfc6901) to the value in the request document that caused the error [e.g. "/data" for a primary data object, or "/data/attributes/title" for a specific attribute].
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A JSON Pointer [RFC6901](https://tools.ietf.org/html/rfc6901) to the value in the request document that caused the error [e.g. \"/data\" for a primary data object, or \"/data/attributes/title\" for a specific attribute].\n",
    ///  "examples": [
    ///    "/data"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct Pointer(pub String);
    impl std::ops::Deref for Pointer {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Pointer> for String {
        fn from(value: Pointer) -> Self {
            value.0
        }
    }

    impl From<&Pointer> for Pointer {
        fn from(value: &Pointer) -> Self {
            value.clone()
        }
    }

    impl From<String> for Pointer {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Pointer {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for Pointer {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///Recreation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "community_centers": {
    ///      "description": "BNA category subscore for access to community
    /// centers",
    ///      "examples": [
    ///        70.7
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "parks": {
    ///      "description": "BNA category subscore for access to parks",
    ///      "examples": [
    ///        78.49
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "recreation_trails": {
    ///      "description": "BNA category subscore for access to bikeable
    /// trails",
    ///      "examples": [
    ///        94.45
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "score": {
    ///      "description": "BNA total score",
    ///      "examples": [
    ///        77.0
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Recreation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub community_centers: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parks: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub recreation_trails: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub score: Option<f64>,
    }

    impl From<&Recreation> for Recreation {
        fn from(value: &Recreation) -> Self {
            value.clone()
        }
    }

    impl Recreation {
        pub fn builder() -> builder::Recreation {
            Default::default()
        }
    }

    ///Retail
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "score": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Retail {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub score: Option<f64>,
    }

    impl From<&Retail> for Retail {
        fn from(value: &Retail) -> Self {
            value.clone()
        }
    }

    impl Retail {
        pub fn builder() -> builder::Retail {
            Default::default()
        }
    }

    ///An object containing references to the primary source of the error.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An object containing references to the primary source
    /// of the error.",
    ///  "examples": [
    ///    {
    ///      "source": "Parameter
    /// \"/bnas/analysis/e6aade5a-b343-120b-dbaa-bd916cd99221?\""
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/parameter"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/pointer"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/header"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum Source {
        Parameter(Parameter),
        Pointer(Pointer),
        Header(Header),
    }

    impl From<&Source> for Source {
        fn from(value: &Source) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for Source {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::Parameter(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::Pointer(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::Header(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl std::convert::TryFrom<&str> for Source {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Source {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Source {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ToString for Source {
        fn to_string(&self) -> String {
            match self {
                Self::Parameter(x) => x.to_string(),
                Self::Pointer(x) => x.to_string(),
                Self::Header(x) => x.to_string(),
            }
        }
    }

    impl From<Parameter> for Source {
        fn from(value: Parameter) -> Self {
            Self::Parameter(value)
        }
    }

    impl From<Pointer> for Source {
        fn from(value: Pointer) -> Self {
            Self::Pointer(value)
        }
    }

    impl From<Header> for Source {
        fn from(value: Header) -> Self {
            Self::Header(value)
        }
    }

    ///ID of the AWS state machine that was used to run the pipeline
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "ID of the AWS state machine that was used to run the
    /// pipeline",
    ///  "examples": [
    ///    "38f4f54e-98d6-4048-8c0f-99cde05a7e76"
    ///  ],
    ///  "type": "string",
    ///  "format": "uuid"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct StateMachineId(pub uuid::Uuid);
    impl std::ops::Deref for StateMachineId {
        type Target = uuid::Uuid;
        fn deref(&self) -> &uuid::Uuid {
            &self.0
        }
    }

    impl From<StateMachineId> for uuid::Uuid {
        fn from(value: StateMachineId) -> Self {
            value.0
        }
    }

    impl From<&StateMachineId> for StateMachineId {
        fn from(value: &StateMachineId) -> Self {
            value.clone()
        }
    }

    impl From<uuid::Uuid> for StateMachineId {
        fn from(value: uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for StateMachineId {
        type Err = <uuid::Uuid as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for StateMachineId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for StateMachineId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for StateMachineId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for StateMachineId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///Indicate the last step of the pipeline that completed successfully
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Indicate the last step of the pipeline that completed
    /// successfully",
    ///  "examples": [
    ///    "Cleanup"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "SqsMessage",
    ///    "Setup",
    ///    "Analysis",
    ///    "Cleanup"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum Step {
        SqsMessage,
        Setup,
        Analysis,
        Cleanup,
    }

    impl From<&Step> for Step {
        fn from(value: &Step) -> Self {
            value.clone()
        }
    }

    impl ToString for Step {
        fn to_string(&self) -> String {
            match *self {
                Self::SqsMessage => "SqsMessage".to_string(),
                Self::Setup => "Setup".to_string(),
                Self::Analysis => "Analysis".to_string(),
                Self::Cleanup => "Cleanup".to_string(),
            }
        }
    }

    impl std::str::FromStr for Step {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "SqsMessage" => Ok(Self::SqsMessage),
                "Setup" => Ok(Self::Setup),
                "Analysis" => Ok(Self::Analysis),
                "Cleanup" => Ok(Self::Cleanup),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for Step {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Step {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Step {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Submission
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "city": {
    ///      "description": "City name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "consent": {
    ///      "description": "Consent status",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/country"
    ///    },
    ///    "created_at": {
    ///      "description": "Date and time",
    ///      "examples": [
    ///        [
    ///          2023,
    ///          6,
    ///          16,
    ///          22,
    ///          0,
    ///          0,
    ///          0,
    ///          0,
    ///          0
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "email": {
    ///      "description": "Email address",
    ///      "examples": [
    ///        "jane.doe@orgllc.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities\n",
    ///      "examples": [
    ///        "4805000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "first_name": {
    ///      "description": "First name",
    ///      "examples": [
    ///        "Jane"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Submission identifier",
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "last_name": {
    ///      "description": "Last name",
    ///      "examples": [
    ///        "Doe"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "occupation": {
    ///      "description": "Job title or position",
    ///      "examples": [
    ///        "CTO"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "organization": {
    ///      "description": "Name of the organization",
    ///      "examples": [
    ///        "Organization LLC"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "description": "Region name. A region can be a state, a province, a
    /// community, or something similar depending on the country. If a country
    /// does not have this concept, then the country name is used.\n",
    ///      "examples": [
    ///        "Belgium"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "submission_status": {
    ///      "description": "The current status of the submission",
    ///      "examples": [
    ///        "Pending"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Submission {
        ///City name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        ///Consent status
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub consent: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<Country>,
        ///Date and time
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub created_at: Vec<i64>,
        ///Email address
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fips_code: Option<String>,
        ///First name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        ///Submission identifier
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<i64>,
        ///Last name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        ///Job title or position
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub occupation: Option<String>,
        ///Name of the organization
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub organization: Option<String>,
        ///Region name. A region can be a state, a province, a community, or
        /// something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        ///The current status of the submission
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub submission_status: Option<String>,
    }

    impl From<&Submission> for Submission {
        fn from(value: &Submission) -> Self {
            value.clone()
        }
    }

    impl Submission {
        pub fn builder() -> builder::Submission {
            Default::default()
        }
    }

    ///SubmissionPatch
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "city": {
    ///      "description": "City name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "consent": {
    ///      "description": "Consent status",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/country"
    ///    },
    ///    "email": {
    ///      "description": "Email address",
    ///      "examples": [
    ///        "jane.doe@orgllc.com"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities\n",
    ///      "examples": [
    ///        "4805000"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "first_name": {
    ///      "description": "First name",
    ///      "examples": [
    ///        "Jane"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "last_name": {
    ///      "description": "Last name",
    ///      "examples": [
    ///        "Doe"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "occupation": {
    ///      "description": "Job title or position",
    ///      "examples": [
    ///        "CTO"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "organization": {
    ///      "description": "Name of the organization",
    ///      "examples": [
    ///        "Organization LLC"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "region": {
    ///      "description": "Region name. A region can be a state, a province, a
    /// community, or something similar depending on the country. If a country
    /// does not have this concept, then the country name is used.\n",
    ///      "examples": [
    ///        "Belgium"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "submission_status": {
    ///      "description": "The current status of the submission",
    ///      "examples": [
    ///        "Pending"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SubmissionPatch {
        ///City name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        ///Consent status
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub consent: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<Country>,
        ///Email address
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fips_code: Option<String>,
        ///First name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        ///Last name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        ///Job title or position
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub occupation: Option<String>,
        ///Name of the organization
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub organization: Option<String>,
        ///Region name. A region can be a state, a province, a community, or
        /// something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        ///The current status of the submission
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub submission_status: Option<String>,
    }

    impl From<&SubmissionPatch> for SubmissionPatch {
        fn from(value: &SubmissionPatch) -> Self {
            value.clone()
        }
    }

    impl SubmissionPatch {
        pub fn builder() -> builder::SubmissionPatch {
            Default::default()
        }
    }

    ///SubmissionPost
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city",
    ///    "consent",
    ///    "country",
    ///    "email",
    ///    "fips_code",
    ///    "first_name",
    ///    "last_name"
    ///  ],
    ///  "properties": {
    ///    "city": {
    ///      "description": "City name",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "consent": {
    ///      "description": "Consent status",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/country"
    ///    },
    ///    "email": {
    ///      "description": "Email address",
    ///      "examples": [
    ///        "jane.doe@orgllc.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities\n",
    ///      "examples": [
    ///        "4805000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "first_name": {
    ///      "description": "First name",
    ///      "examples": [
    ///        "Jane"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "last_name": {
    ///      "description": "Last name",
    ///      "examples": [
    ///        "Doe"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "occupation": {
    ///      "description": "Job title or position",
    ///      "examples": [
    ///        "CTO"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "organization": {
    ///      "description": "Name of the organization",
    ///      "examples": [
    ///        "Organization LLC"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "region": {
    ///      "description": "Region name. A region can be a state, a province, a
    /// community, or something similar depending on the country. If a country
    /// does not have this concept, then the country name is used.\n",
    ///      "examples": [
    ///        "Belgium"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "submission_status": {
    ///      "description": "The current status of the submission",
    ///      "examples": [
    ///        "Pending"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SubmissionPost {
        ///City name
        pub city: String,
        ///Consent status
        pub consent: bool,
        pub country: Country,
        ///Email address
        pub email: String,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        pub fips_code: String,
        ///First name
        pub first_name: String,
        ///Last name
        pub last_name: String,
        ///Job title or position
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub occupation: Option<String>,
        ///Name of the organization
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub organization: Option<String>,
        ///Region name. A region can be a state, a province, a community, or
        /// something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        ///The current status of the submission
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub submission_status: Option<String>,
    }

    impl From<&SubmissionPost> for SubmissionPost {
        fn from(value: &SubmissionPost) -> Self {
            value.clone()
        }
    }

    impl SubmissionPost {
        pub fn builder() -> builder::SubmissionPost {
            Default::default()
        }
    }

    ///A collection of submissions
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A collection of submissions",
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/submission"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Submissions(pub Vec<Submission>);
    impl std::ops::Deref for Submissions {
        type Target = Vec<Submission>;
        fn deref(&self) -> &Vec<Submission> {
            &self.0
        }
    }

    impl From<Submissions> for Vec<Submission> {
        fn from(value: Submissions) -> Self {
            value.0
        }
    }

    impl From<&Submissions> for Submissions {
        fn from(value: &Submissions) -> Self {
            value.clone()
        }
    }

    impl From<Vec<Submission>> for Submissions {
        fn from(value: Vec<Submission>) -> Self {
            Self(value)
        }
    }

    ///Transit
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "score": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Transit {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub score: Option<f64>,
    }

    impl From<&Transit> for Transit {
        fn from(value: &Transit) -> Self {
            value.clone()
        }
    }

    impl Transit {
        pub fn builder() -> builder::Transit {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Analysis {
            cost: Result<Option<f64>, String>,
            end_time: Result<Option<Vec<i64>>, String>,
            fargate_task_arn: Result<Option<String>, String>,
            results_posted: Result<Option<bool>, String>,
            s3_bucket: Result<Option<String>, String>,
            sqs_message: Result<Option<String>, String>,
            start_time: Result<Option<Vec<i64>>, String>,
            state_machine_id: Result<super::StateMachineId, String>,
            step: Result<Option<super::Step>, String>,
            torn_down: Result<Option<bool>, String>,
        }

        impl Default for Analysis {
            fn default() -> Self {
                Self {
                    cost: Ok(Default::default()),
                    end_time: Ok(Default::default()),
                    fargate_task_arn: Ok(Default::default()),
                    results_posted: Ok(Default::default()),
                    s3_bucket: Ok(Default::default()),
                    sqs_message: Ok(Default::default()),
                    start_time: Ok(Default::default()),
                    state_machine_id: Err("no value supplied for state_machine_id".to_string()),
                    step: Ok(Default::default()),
                    torn_down: Ok(Default::default()),
                }
            }
        }

        impl Analysis {
            pub fn cost<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.cost = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cost: {}", e));
                self
            }
            pub fn end_time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<i64>>>,
                T::Error: std::fmt::Display,
            {
                self.end_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end_time: {}", e));
                self
            }
            pub fn fargate_task_arn<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fargate_task_arn = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_task_arn: {}",
                        e
                    )
                });
                self
            }
            pub fn results_posted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.results_posted = value.try_into().map_err(|e| {
                    format!("error converting supplied value for results_posted: {}", e)
                });
                self
            }
            pub fn s3_bucket<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.s3_bucket = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for s3_bucket: {}", e));
                self
            }
            pub fn sqs_message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.sqs_message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sqs_message: {}", e));
                self
            }
            pub fn start_time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<i64>>>,
                T::Error: std::fmt::Display,
            {
                self.start_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_time: {}", e));
                self
            }
            pub fn state_machine_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::StateMachineId>,
                T::Error: std::fmt::Display,
            {
                self.state_machine_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for state_machine_id: {}",
                        e
                    )
                });
                self
            }
            pub fn step<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Step>>,
                T::Error: std::fmt::Display,
            {
                self.step = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for step: {}", e));
                self
            }
            pub fn torn_down<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.torn_down = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for torn_down: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Analysis> for super::Analysis {
            type Error = super::error::ConversionError;
            fn try_from(value: Analysis) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cost: value.cost?,
                    end_time: value.end_time?,
                    fargate_task_arn: value.fargate_task_arn?,
                    results_posted: value.results_posted?,
                    s3_bucket: value.s3_bucket?,
                    sqs_message: value.sqs_message?,
                    start_time: value.start_time?,
                    state_machine_id: value.state_machine_id?,
                    step: value.step?,
                    torn_down: value.torn_down?,
                })
            }
        }

        impl From<super::Analysis> for Analysis {
            fn from(value: super::Analysis) -> Self {
                Self {
                    cost: Ok(value.cost),
                    end_time: Ok(value.end_time),
                    fargate_task_arn: Ok(value.fargate_task_arn),
                    results_posted: Ok(value.results_posted),
                    s3_bucket: Ok(value.s3_bucket),
                    sqs_message: Ok(value.sqs_message),
                    start_time: Ok(value.start_time),
                    state_machine_id: Ok(value.state_machine_id),
                    step: Ok(value.step),
                    torn_down: Ok(value.torn_down),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AnalysisPatch {
            cost: Result<Option<f64>, String>,
            end_time: Result<Option<Vec<i64>>, String>,
            fargate_task_arn: Result<Option<String>, String>,
            results_posted: Result<Option<bool>, String>,
            s3_bucket: Result<Option<String>, String>,
            sqs_message: Result<Option<String>, String>,
            start_time: Result<Option<Vec<i64>>, String>,
            step: Result<Option<super::Step>, String>,
            torn_down: Result<Option<bool>, String>,
        }

        impl Default for AnalysisPatch {
            fn default() -> Self {
                Self {
                    cost: Ok(Default::default()),
                    end_time: Ok(Default::default()),
                    fargate_task_arn: Ok(Default::default()),
                    results_posted: Ok(Default::default()),
                    s3_bucket: Ok(Default::default()),
                    sqs_message: Ok(Default::default()),
                    start_time: Ok(Default::default()),
                    step: Ok(Default::default()),
                    torn_down: Ok(Default::default()),
                }
            }
        }

        impl AnalysisPatch {
            pub fn cost<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.cost = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cost: {}", e));
                self
            }
            pub fn end_time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<i64>>>,
                T::Error: std::fmt::Display,
            {
                self.end_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end_time: {}", e));
                self
            }
            pub fn fargate_task_arn<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fargate_task_arn = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_task_arn: {}",
                        e
                    )
                });
                self
            }
            pub fn results_posted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.results_posted = value.try_into().map_err(|e| {
                    format!("error converting supplied value for results_posted: {}", e)
                });
                self
            }
            pub fn s3_bucket<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.s3_bucket = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for s3_bucket: {}", e));
                self
            }
            pub fn sqs_message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.sqs_message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sqs_message: {}", e));
                self
            }
            pub fn start_time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<i64>>>,
                T::Error: std::fmt::Display,
            {
                self.start_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_time: {}", e));
                self
            }
            pub fn step<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Step>>,
                T::Error: std::fmt::Display,
            {
                self.step = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for step: {}", e));
                self
            }
            pub fn torn_down<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.torn_down = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for torn_down: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AnalysisPatch> for super::AnalysisPatch {
            type Error = super::error::ConversionError;
            fn try_from(value: AnalysisPatch) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cost: value.cost?,
                    end_time: value.end_time?,
                    fargate_task_arn: value.fargate_task_arn?,
                    results_posted: value.results_posted?,
                    s3_bucket: value.s3_bucket?,
                    sqs_message: value.sqs_message?,
                    start_time: value.start_time?,
                    step: value.step?,
                    torn_down: value.torn_down?,
                })
            }
        }

        impl From<super::AnalysisPatch> for AnalysisPatch {
            fn from(value: super::AnalysisPatch) -> Self {
                Self {
                    cost: Ok(value.cost),
                    end_time: Ok(value.end_time),
                    fargate_task_arn: Ok(value.fargate_task_arn),
                    results_posted: Ok(value.results_posted),
                    s3_bucket: Ok(value.s3_bucket),
                    sqs_message: Ok(value.sqs_message),
                    start_time: Ok(value.start_time),
                    step: Ok(value.step),
                    torn_down: Ok(value.torn_down),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AnalysisPost {
            cost: Result<Option<f64>, String>,
            end_time: Result<Option<Vec<i64>>, String>,
            fargate_task_arn: Result<Option<String>, String>,
            result_posted: Result<Option<bool>, String>,
            s3_bucket: Result<Option<String>, String>,
            sqs_message: Result<Option<String>, String>,
            step: Result<Option<super::Step>, String>,
            torn_down: Result<Option<bool>, String>,
        }

        impl Default for AnalysisPost {
            fn default() -> Self {
                Self {
                    cost: Ok(Default::default()),
                    end_time: Ok(Default::default()),
                    fargate_task_arn: Ok(Default::default()),
                    result_posted: Ok(Default::default()),
                    s3_bucket: Ok(Default::default()),
                    sqs_message: Ok(Default::default()),
                    step: Ok(Default::default()),
                    torn_down: Ok(Default::default()),
                }
            }
        }

        impl AnalysisPost {
            pub fn cost<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.cost = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cost: {}", e));
                self
            }
            pub fn end_time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<i64>>>,
                T::Error: std::fmt::Display,
            {
                self.end_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end_time: {}", e));
                self
            }
            pub fn fargate_task_arn<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fargate_task_arn = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_task_arn: {}",
                        e
                    )
                });
                self
            }
            pub fn result_posted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.result_posted = value.try_into().map_err(|e| {
                    format!("error converting supplied value for result_posted: {}", e)
                });
                self
            }
            pub fn s3_bucket<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.s3_bucket = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for s3_bucket: {}", e));
                self
            }
            pub fn sqs_message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.sqs_message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sqs_message: {}", e));
                self
            }
            pub fn step<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Step>>,
                T::Error: std::fmt::Display,
            {
                self.step = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for step: {}", e));
                self
            }
            pub fn torn_down<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.torn_down = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for torn_down: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AnalysisPost> for super::AnalysisPost {
            type Error = super::error::ConversionError;
            fn try_from(value: AnalysisPost) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cost: value.cost?,
                    end_time: value.end_time?,
                    fargate_task_arn: value.fargate_task_arn?,
                    result_posted: value.result_posted?,
                    s3_bucket: value.s3_bucket?,
                    sqs_message: value.sqs_message?,
                    step: value.step?,
                    torn_down: value.torn_down?,
                })
            }
        }

        impl From<super::AnalysisPost> for AnalysisPost {
            fn from(value: super::AnalysisPost) -> Self {
                Self {
                    cost: Ok(value.cost),
                    end_time: Ok(value.end_time),
                    fargate_task_arn: Ok(value.fargate_task_arn),
                    result_posted: Ok(value.result_posted),
                    s3_bucket: Ok(value.s3_bucket),
                    sqs_message: Ok(value.sqs_message),
                    step: Ok(value.step),
                    torn_down: Ok(value.torn_down),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Bna {
            city_id: Result<uuid::Uuid, String>,
            community_centers: Result<Option<f64>, String>,
            coreservices_score: Result<Option<f64>, String>,
            dentists: Result<Option<f64>, String>,
            doctors: Result<Option<f64>, String>,
            employment: Result<Option<f64>, String>,
            grocery: Result<Option<f64>, String>,
            high_stress_miles: Result<Option<f64>, String>,
            higher_education: Result<Option<f64>, String>,
            hospitals: Result<Option<f64>, String>,
            k12_education: Result<Option<f64>, String>,
            low_stress_miles: Result<Option<f64>, String>,
            opportunity_score: Result<Option<f64>, String>,
            parks: Result<Option<f64>, String>,
            people: Result<Option<f64>, String>,
            pharmacies: Result<Option<f64>, String>,
            rating_id: Result<uuid::Uuid, String>,
            recreation_score: Result<Option<f64>, String>,
            recreation_trails: Result<Option<f64>, String>,
            retail: Result<Option<f64>, String>,
            score: Result<f64, String>,
            social_services: Result<Option<f64>, String>,
            technical_vocational_college: Result<Option<f64>, String>,
            transit: Result<Option<f64>, String>,
            version: Result<String, String>,
        }

        impl Default for Bna {
            fn default() -> Self {
                Self {
                    city_id: Err("no value supplied for city_id".to_string()),
                    community_centers: Ok(Default::default()),
                    coreservices_score: Ok(Default::default()),
                    dentists: Ok(Default::default()),
                    doctors: Ok(Default::default()),
                    employment: Ok(Default::default()),
                    grocery: Ok(Default::default()),
                    high_stress_miles: Ok(Default::default()),
                    higher_education: Ok(Default::default()),
                    hospitals: Ok(Default::default()),
                    k12_education: Ok(Default::default()),
                    low_stress_miles: Ok(Default::default()),
                    opportunity_score: Ok(Default::default()),
                    parks: Ok(Default::default()),
                    people: Ok(Default::default()),
                    pharmacies: Ok(Default::default()),
                    rating_id: Err("no value supplied for rating_id".to_string()),
                    recreation_score: Ok(Default::default()),
                    recreation_trails: Ok(Default::default()),
                    retail: Ok(Default::default()),
                    score: Err("no value supplied for score".to_string()),
                    social_services: Ok(Default::default()),
                    technical_vocational_college: Ok(Default::default()),
                    transit: Ok(Default::default()),
                    version: Err("no value supplied for version".to_string()),
                }
            }
        }

        impl Bna {
            pub fn city_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.city_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city_id: {}", e));
                self
            }
            pub fn community_centers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.community_centers = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for community_centers: {}",
                        e
                    )
                });
                self
            }
            pub fn coreservices_score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.coreservices_score = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for coreservices_score: {}",
                        e
                    )
                });
                self
            }
            pub fn dentists<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.dentists = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for dentists: {}", e));
                self
            }
            pub fn doctors<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.doctors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for doctors: {}", e));
                self
            }
            pub fn employment<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.employment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for employment: {}", e));
                self
            }
            pub fn grocery<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.grocery = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grocery: {}", e));
                self
            }
            pub fn high_stress_miles<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.high_stress_miles = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for high_stress_miles: {}",
                        e
                    )
                });
                self
            }
            pub fn higher_education<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.higher_education = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for higher_education: {}",
                        e
                    )
                });
                self
            }
            pub fn hospitals<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.hospitals = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hospitals: {}", e));
                self
            }
            pub fn k12_education<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.k12_education = value.try_into().map_err(|e| {
                    format!("error converting supplied value for k12_education: {}", e)
                });
                self
            }
            pub fn low_stress_miles<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.low_stress_miles = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for low_stress_miles: {}",
                        e
                    )
                });
                self
            }
            pub fn opportunity_score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.opportunity_score = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for opportunity_score: {}",
                        e
                    )
                });
                self
            }
            pub fn parks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.parks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for parks: {}", e));
                self
            }
            pub fn people<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.people = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for people: {}", e));
                self
            }
            pub fn pharmacies<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.pharmacies = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pharmacies: {}", e));
                self
            }
            pub fn rating_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.rating_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rating_id: {}", e));
                self
            }
            pub fn recreation_score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.recreation_score = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for recreation_score: {}",
                        e
                    )
                });
                self
            }
            pub fn recreation_trails<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.recreation_trails = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for recreation_trails: {}",
                        e
                    )
                });
                self
            }
            pub fn retail<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.retail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for retail: {}", e));
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<f64>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn social_services<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.social_services = value.try_into().map_err(|e| {
                    format!("error converting supplied value for social_services: {}", e)
                });
                self
            }
            pub fn technical_vocational_college<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.technical_vocational_college = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for technical_vocational_college: {}",
                        e
                    )
                });
                self
            }
            pub fn transit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.transit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for transit: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Bna> for super::Bna {
            type Error = super::error::ConversionError;
            fn try_from(value: Bna) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city_id: value.city_id?,
                    community_centers: value.community_centers?,
                    coreservices_score: value.coreservices_score?,
                    dentists: value.dentists?,
                    doctors: value.doctors?,
                    employment: value.employment?,
                    grocery: value.grocery?,
                    high_stress_miles: value.high_stress_miles?,
                    higher_education: value.higher_education?,
                    hospitals: value.hospitals?,
                    k12_education: value.k12_education?,
                    low_stress_miles: value.low_stress_miles?,
                    opportunity_score: value.opportunity_score?,
                    parks: value.parks?,
                    people: value.people?,
                    pharmacies: value.pharmacies?,
                    rating_id: value.rating_id?,
                    recreation_score: value.recreation_score?,
                    recreation_trails: value.recreation_trails?,
                    retail: value.retail?,
                    score: value.score?,
                    social_services: value.social_services?,
                    technical_vocational_college: value.technical_vocational_college?,
                    transit: value.transit?,
                    version: value.version?,
                })
            }
        }

        impl From<super::Bna> for Bna {
            fn from(value: super::Bna) -> Self {
                Self {
                    city_id: Ok(value.city_id),
                    community_centers: Ok(value.community_centers),
                    coreservices_score: Ok(value.coreservices_score),
                    dentists: Ok(value.dentists),
                    doctors: Ok(value.doctors),
                    employment: Ok(value.employment),
                    grocery: Ok(value.grocery),
                    high_stress_miles: Ok(value.high_stress_miles),
                    higher_education: Ok(value.higher_education),
                    hospitals: Ok(value.hospitals),
                    k12_education: Ok(value.k12_education),
                    low_stress_miles: Ok(value.low_stress_miles),
                    opportunity_score: Ok(value.opportunity_score),
                    parks: Ok(value.parks),
                    people: Ok(value.people),
                    pharmacies: Ok(value.pharmacies),
                    rating_id: Ok(value.rating_id),
                    recreation_score: Ok(value.recreation_score),
                    recreation_trails: Ok(value.recreation_trails),
                    retail: Ok(value.retail),
                    score: Ok(value.score),
                    social_services: Ok(value.social_services),
                    technical_vocational_college: Ok(value.technical_vocational_college),
                    transit: Ok(value.transit),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct BnaPost {
            core_services: Result<super::CoreServices, String>,
            features: Result<serde_json::Value, String>,
            infrastructure: Result<super::Infrastructure, String>,
            opportunity: Result<super::Opportunity, String>,
            people: Result<super::People, String>,
            recreation: Result<super::Recreation, String>,
            retail: Result<super::Retail, String>,
            summary: Result<super::BnaSummary, String>,
            transit: Result<super::Transit, String>,
        }

        impl Default for BnaPost {
            fn default() -> Self {
                Self {
                    core_services: Err("no value supplied for core_services".to_string()),
                    features: Err("no value supplied for features".to_string()),
                    infrastructure: Err("no value supplied for infrastructure".to_string()),
                    opportunity: Err("no value supplied for opportunity".to_string()),
                    people: Err("no value supplied for people".to_string()),
                    recreation: Err("no value supplied for recreation".to_string()),
                    retail: Err("no value supplied for retail".to_string()),
                    summary: Err("no value supplied for summary".to_string()),
                    transit: Err("no value supplied for transit".to_string()),
                }
            }
        }

        impl BnaPost {
            pub fn core_services<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::CoreServices>,
                T::Error: std::fmt::Display,
            {
                self.core_services = value.try_into().map_err(|e| {
                    format!("error converting supplied value for core_services: {}", e)
                });
                self
            }
            pub fn features<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<serde_json::Value>,
                T::Error: std::fmt::Display,
            {
                self.features = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for features: {}", e));
                self
            }
            pub fn infrastructure<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Infrastructure>,
                T::Error: std::fmt::Display,
            {
                self.infrastructure = value.try_into().map_err(|e| {
                    format!("error converting supplied value for infrastructure: {}", e)
                });
                self
            }
            pub fn opportunity<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Opportunity>,
                T::Error: std::fmt::Display,
            {
                self.opportunity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for opportunity: {}", e));
                self
            }
            pub fn people<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::People>,
                T::Error: std::fmt::Display,
            {
                self.people = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for people: {}", e));
                self
            }
            pub fn recreation<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Recreation>,
                T::Error: std::fmt::Display,
            {
                self.recreation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recreation: {}", e));
                self
            }
            pub fn retail<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Retail>,
                T::Error: std::fmt::Display,
            {
                self.retail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for retail: {}", e));
                self
            }
            pub fn summary<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::BnaSummary>,
                T::Error: std::fmt::Display,
            {
                self.summary = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for summary: {}", e));
                self
            }
            pub fn transit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Transit>,
                T::Error: std::fmt::Display,
            {
                self.transit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for transit: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<BnaPost> for super::BnaPost {
            type Error = super::error::ConversionError;
            fn try_from(value: BnaPost) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    core_services: value.core_services?,
                    features: value.features?,
                    infrastructure: value.infrastructure?,
                    opportunity: value.opportunity?,
                    people: value.people?,
                    recreation: value.recreation?,
                    retail: value.retail?,
                    summary: value.summary?,
                    transit: value.transit?,
                })
            }
        }

        impl From<super::BnaPost> for BnaPost {
            fn from(value: super::BnaPost) -> Self {
                Self {
                    core_services: Ok(value.core_services),
                    features: Ok(value.features),
                    infrastructure: Ok(value.infrastructure),
                    opportunity: Ok(value.opportunity),
                    people: Ok(value.people),
                    recreation: Ok(value.recreation),
                    retail: Ok(value.retail),
                    summary: Ok(value.summary),
                    transit: Ok(value.transit),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct BnaSummary {
            city_id: Result<uuid::Uuid, String>,
            created_at: Result<Vec<i64>, String>,
            rating_id: Result<uuid::Uuid, String>,
            score: Result<f64, String>,
            version: Result<String, String>,
        }

        impl Default for BnaSummary {
            fn default() -> Self {
                Self {
                    city_id: Err("no value supplied for city_id".to_string()),
                    created_at: Ok(Default::default()),
                    rating_id: Err("no value supplied for rating_id".to_string()),
                    score: Err("no value supplied for score".to_string()),
                    version: Err("no value supplied for version".to_string()),
                }
            }
        }

        impl BnaSummary {
            pub fn city_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.city_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city_id: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<i64>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn rating_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.rating_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rating_id: {}", e));
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<f64>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<BnaSummary> for super::BnaSummary {
            type Error = super::error::ConversionError;
            fn try_from(value: BnaSummary) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city_id: value.city_id?,
                    created_at: value.created_at?,
                    rating_id: value.rating_id?,
                    score: value.score?,
                    version: value.version?,
                })
            }
        }

        impl From<super::BnaSummary> for BnaSummary {
            fn from(value: super::BnaSummary) -> Self {
                Self {
                    city_id: Ok(value.city_id),
                    created_at: Ok(value.created_at),
                    rating_id: Ok(value.rating_id),
                    score: Ok(value.score),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct BnaSummaryWithCityItem {
            city_id: Result<uuid::Uuid, String>,
            country: Result<Option<super::Country>, String>,
            created_at: Result<Vec<i64>, String>,
            latitude: Result<Option<f64>, String>,
            longitude: Result<Option<f64>, String>,
            name: Result<Option<String>, String>,
            rating_id: Result<uuid::Uuid, String>,
            region: Result<Option<String>, String>,
            score: Result<f64, String>,
            speed_limit: Result<Option<f64>, String>,
            state: Result<Option<String>, String>,
            state_abbrev: Result<Option<f64>, String>,
            updated_at: Result<Vec<i64>, String>,
            version: Result<String, String>,
        }

        impl Default for BnaSummaryWithCityItem {
            fn default() -> Self {
                Self {
                    city_id: Err("no value supplied for city_id".to_string()),
                    country: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    latitude: Ok(Default::default()),
                    longitude: Ok(Default::default()),
                    name: Ok(Default::default()),
                    rating_id: Err("no value supplied for rating_id".to_string()),
                    region: Ok(Default::default()),
                    score: Err("no value supplied for score".to_string()),
                    speed_limit: Ok(Default::default()),
                    state: Ok(Default::default()),
                    state_abbrev: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    version: Err("no value supplied for version".to_string()),
                }
            }
        }

        impl BnaSummaryWithCityItem {
            pub fn city_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.city_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city_id: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Country>>,
                T::Error: std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<i64>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn latitude<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.latitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for latitude: {}", e));
                self
            }
            pub fn longitude<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.longitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for longitude: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn rating_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.rating_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rating_id: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<f64>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn speed_limit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.speed_limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for speed_limit: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn state_abbrev<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.state_abbrev = value.try_into().map_err(|e| {
                    format!("error converting supplied value for state_abbrev: {}", e)
                });
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<i64>>,
                T::Error: std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<BnaSummaryWithCityItem> for super::BnaSummaryWithCityItem {
            type Error = super::error::ConversionError;
            fn try_from(
                value: BnaSummaryWithCityItem,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city_id: value.city_id?,
                    country: value.country?,
                    created_at: value.created_at?,
                    latitude: value.latitude?,
                    longitude: value.longitude?,
                    name: value.name?,
                    rating_id: value.rating_id?,
                    region: value.region?,
                    score: value.score?,
                    speed_limit: value.speed_limit?,
                    state: value.state?,
                    state_abbrev: value.state_abbrev?,
                    updated_at: value.updated_at?,
                    version: value.version?,
                })
            }
        }

        impl From<super::BnaSummaryWithCityItem> for BnaSummaryWithCityItem {
            fn from(value: super::BnaSummaryWithCityItem) -> Self {
                Self {
                    city_id: Ok(value.city_id),
                    country: Ok(value.country),
                    created_at: Ok(value.created_at),
                    latitude: Ok(value.latitude),
                    longitude: Ok(value.longitude),
                    name: Ok(value.name),
                    rating_id: Ok(value.rating_id),
                    region: Ok(value.region),
                    score: Ok(value.score),
                    speed_limit: Ok(value.speed_limit),
                    state: Ok(value.state),
                    state_abbrev: Ok(value.state_abbrev),
                    updated_at: Ok(value.updated_at),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Census {
            census_id: Result<Option<i64>, String>,
            city_id: Result<Option<uuid::Uuid>, String>,
            created_at: Result<Vec<i64>, String>,
            fips_code: Result<Option<String>, String>,
            pop_size: Result<Option<super::CensusPopSize>, String>,
            population: Result<Option<i64>, String>,
        }

        impl Default for Census {
            fn default() -> Self {
                Self {
                    census_id: Ok(Default::default()),
                    city_id: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    fips_code: Ok(Default::default()),
                    pop_size: Ok(Default::default()),
                    population: Ok(Default::default()),
                }
            }
        }

        impl Census {
            pub fn census_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.census_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for census_id: {}", e));
                self
            }
            pub fn city_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<uuid::Uuid>>,
                T::Error: std::fmt::Display,
            {
                self.city_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city_id: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<i64>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn pop_size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::CensusPopSize>>,
                T::Error: std::fmt::Display,
            {
                self.pop_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pop_size: {}", e));
                self
            }
            pub fn population<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.population = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for population: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Census> for super::Census {
            type Error = super::error::ConversionError;
            fn try_from(value: Census) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    census_id: value.census_id?,
                    city_id: value.city_id?,
                    created_at: value.created_at?,
                    fips_code: value.fips_code?,
                    pop_size: value.pop_size?,
                    population: value.population?,
                })
            }
        }

        impl From<super::Census> for Census {
            fn from(value: super::Census) -> Self {
                Self {
                    census_id: Ok(value.census_id),
                    city_id: Ok(value.city_id),
                    created_at: Ok(value.created_at),
                    fips_code: Ok(value.fips_code),
                    pop_size: Ok(value.pop_size),
                    population: Ok(value.population),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct City {
            city_id: Result<Option<uuid::Uuid>, String>,
            country: Result<Option<super::Country>, String>,
            created_at: Result<Vec<i64>, String>,
            latitude: Result<Option<f64>, String>,
            longitude: Result<Option<f64>, String>,
            name: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            speed_limit: Result<Option<f64>, String>,
            state: Result<Option<String>, String>,
            state_abbrev: Result<Option<f64>, String>,
            updated_at: Result<Vec<i64>, String>,
        }

        impl Default for City {
            fn default() -> Self {
                Self {
                    city_id: Ok(Default::default()),
                    country: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    latitude: Ok(Default::default()),
                    longitude: Ok(Default::default()),
                    name: Ok(Default::default()),
                    region: Ok(Default::default()),
                    speed_limit: Ok(Default::default()),
                    state: Ok(Default::default()),
                    state_abbrev: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                }
            }
        }

        impl City {
            pub fn city_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<uuid::Uuid>>,
                T::Error: std::fmt::Display,
            {
                self.city_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city_id: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Country>>,
                T::Error: std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<i64>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn latitude<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.latitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for latitude: {}", e));
                self
            }
            pub fn longitude<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.longitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for longitude: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn speed_limit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.speed_limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for speed_limit: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn state_abbrev<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.state_abbrev = value.try_into().map_err(|e| {
                    format!("error converting supplied value for state_abbrev: {}", e)
                });
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<i64>>,
                T::Error: std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<City> for super::City {
            type Error = super::error::ConversionError;
            fn try_from(value: City) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city_id: value.city_id?,
                    country: value.country?,
                    created_at: value.created_at?,
                    latitude: value.latitude?,
                    longitude: value.longitude?,
                    name: value.name?,
                    region: value.region?,
                    speed_limit: value.speed_limit?,
                    state: value.state?,
                    state_abbrev: value.state_abbrev?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl From<super::City> for City {
            fn from(value: super::City) -> Self {
                Self {
                    city_id: Ok(value.city_id),
                    country: Ok(value.country),
                    created_at: Ok(value.created_at),
                    latitude: Ok(value.latitude),
                    longitude: Ok(value.longitude),
                    name: Ok(value.name),
                    region: Ok(value.region),
                    speed_limit: Ok(value.speed_limit),
                    state: Ok(value.state),
                    state_abbrev: Ok(value.state_abbrev),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CityPost {
            country: Result<super::Country, String>,
            latitude: Result<Option<f64>, String>,
            longitude: Result<Option<f64>, String>,
            name: Result<String, String>,
            speed_limit: Result<Option<f64>, String>,
            state: Result<Option<String>, String>,
            state_abbrev: Result<Option<f64>, String>,
        }

        impl Default for CityPost {
            fn default() -> Self {
                Self {
                    country: Err("no value supplied for country".to_string()),
                    latitude: Ok(Default::default()),
                    longitude: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    speed_limit: Ok(Default::default()),
                    state: Ok(Default::default()),
                    state_abbrev: Ok(Default::default()),
                }
            }
        }

        impl CityPost {
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Country>,
                T::Error: std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn latitude<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.latitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for latitude: {}", e));
                self
            }
            pub fn longitude<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.longitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for longitude: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn speed_limit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.speed_limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for speed_limit: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn state_abbrev<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.state_abbrev = value.try_into().map_err(|e| {
                    format!("error converting supplied value for state_abbrev: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<CityPost> for super::CityPost {
            type Error = super::error::ConversionError;
            fn try_from(value: CityPost) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    country: value.country?,
                    latitude: value.latitude?,
                    longitude: value.longitude?,
                    name: value.name?,
                    speed_limit: value.speed_limit?,
                    state: value.state?,
                    state_abbrev: value.state_abbrev?,
                })
            }
        }

        impl From<super::CityPost> for CityPost {
            fn from(value: super::CityPost) -> Self {
                Self {
                    country: Ok(value.country),
                    latitude: Ok(value.latitude),
                    longitude: Ok(value.longitude),
                    name: Ok(value.name),
                    speed_limit: Ok(value.speed_limit),
                    state: Ok(value.state),
                    state_abbrev: Ok(value.state_abbrev),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CoreServices {
            dentists: Result<Option<f64>, String>,
            doctors: Result<Option<f64>, String>,
            grocery: Result<Option<f64>, String>,
            hospitals: Result<Option<f64>, String>,
            pharmacies: Result<Option<f64>, String>,
            score: Result<Option<f64>, String>,
            social_services: Result<Option<f64>, String>,
        }

        impl Default for CoreServices {
            fn default() -> Self {
                Self {
                    dentists: Ok(Default::default()),
                    doctors: Ok(Default::default()),
                    grocery: Ok(Default::default()),
                    hospitals: Ok(Default::default()),
                    pharmacies: Ok(Default::default()),
                    score: Ok(Default::default()),
                    social_services: Ok(Default::default()),
                }
            }
        }

        impl CoreServices {
            pub fn dentists<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.dentists = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for dentists: {}", e));
                self
            }
            pub fn doctors<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.doctors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for doctors: {}", e));
                self
            }
            pub fn grocery<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.grocery = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grocery: {}", e));
                self
            }
            pub fn hospitals<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.hospitals = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hospitals: {}", e));
                self
            }
            pub fn pharmacies<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.pharmacies = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pharmacies: {}", e));
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn social_services<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.social_services = value.try_into().map_err(|e| {
                    format!("error converting supplied value for social_services: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<CoreServices> for super::CoreServices {
            type Error = super::error::ConversionError;
            fn try_from(value: CoreServices) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    dentists: value.dentists?,
                    doctors: value.doctors?,
                    grocery: value.grocery?,
                    hospitals: value.hospitals?,
                    pharmacies: value.pharmacies?,
                    score: value.score?,
                    social_services: value.social_services?,
                })
            }
        }

        impl From<super::CoreServices> for CoreServices {
            fn from(value: super::CoreServices) -> Self {
                Self {
                    dentists: Ok(value.dentists),
                    doctors: Ok(value.doctors),
                    grocery: Ok(value.grocery),
                    hospitals: Ok(value.hospitals),
                    pharmacies: Ok(value.pharmacies),
                    score: Ok(value.score),
                    social_services: Ok(value.social_services),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Enqueue {
            city: Result<Option<String>, String>,
            country: Result<Option<super::Country>, String>,
            fips_code: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
        }

        impl Default for Enqueue {
            fn default() -> Self {
                Self {
                    city: Ok(Default::default()),
                    country: Ok(Default::default()),
                    fips_code: Ok(Default::default()),
                    region: Ok(Default::default()),
                }
            }
        }

        impl Enqueue {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Country>>,
                T::Error: std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Enqueue> for super::Enqueue {
            type Error = super::error::ConversionError;
            fn try_from(value: Enqueue) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city: value.city?,
                    country: value.country?,
                    fips_code: value.fips_code?,
                    region: value.region?,
                })
            }
        }

        impl From<super::Enqueue> for Enqueue {
            fn from(value: super::Enqueue) -> Self {
                Self {
                    city: Ok(value.city),
                    country: Ok(value.country),
                    fips_code: Ok(value.fips_code),
                    region: Ok(value.region),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct EnqueuePost {
            city: Result<Option<String>, String>,
            country: Result<Option<super::Country>, String>,
            fips_code: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
        }

        impl Default for EnqueuePost {
            fn default() -> Self {
                Self {
                    city: Ok(Default::default()),
                    country: Ok(Default::default()),
                    fips_code: Ok(Default::default()),
                    region: Ok(Default::default()),
                }
            }
        }

        impl EnqueuePost {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Country>>,
                T::Error: std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<EnqueuePost> for super::EnqueuePost {
            type Error = super::error::ConversionError;
            fn try_from(value: EnqueuePost) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city: value.city?,
                    country: value.country?,
                    fips_code: value.fips_code?,
                    region: value.region?,
                })
            }
        }

        impl From<super::EnqueuePost> for EnqueuePost {
            fn from(value: super::EnqueuePost) -> Self {
                Self {
                    city: Ok(value.city),
                    country: Ok(value.country),
                    fips_code: Ok(value.fips_code),
                    region: Ok(value.region),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Error {
            details: Result<Option<String>, String>,
            id: Result<Option<super::ApiGatewayId>, String>,
            source: Result<Option<super::Source>, String>,
            status: Result<Option<i64>, String>,
            title: Result<Option<String>, String>,
        }

        impl Default for Error {
            fn default() -> Self {
                Self {
                    details: Ok(Default::default()),
                    id: Ok(Default::default()),
                    source: Ok(Default::default()),
                    status: Ok(Default::default()),
                    title: Ok(Default::default()),
                }
            }
        }

        impl Error {
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::ApiGatewayId>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn source<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Source>>,
                T::Error: std::fmt::Display,
            {
                self.source = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Error> for super::Error {
            type Error = super::error::ConversionError;
            fn try_from(value: Error) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    details: value.details?,
                    id: value.id?,
                    source: value.source?,
                    status: value.status?,
                    title: value.title?,
                })
            }
        }

        impl From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    details: Ok(value.details),
                    id: Ok(value.id),
                    source: Ok(value.source),
                    status: Ok(value.status),
                    title: Ok(value.title),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Features {
            people: Result<Option<f64>, String>,
            retail: Result<Option<f64>, String>,
            transit: Result<Option<f64>, String>,
        }

        impl Default for Features {
            fn default() -> Self {
                Self {
                    people: Ok(Default::default()),
                    retail: Ok(Default::default()),
                    transit: Ok(Default::default()),
                }
            }
        }

        impl Features {
            pub fn people<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.people = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for people: {}", e));
                self
            }
            pub fn retail<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.retail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for retail: {}", e));
                self
            }
            pub fn transit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.transit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for transit: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Features> for super::Features {
            type Error = super::error::ConversionError;
            fn try_from(value: Features) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    people: value.people?,
                    retail: value.retail?,
                    transit: value.transit?,
                })
            }
        }

        impl From<super::Features> for Features {
            fn from(value: super::Features) -> Self {
                Self {
                    people: Ok(value.people),
                    retail: Ok(value.retail),
                    transit: Ok(value.transit),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetCityCensusResponseItem {
            subtype_0: Result<Option<super::City>, String>,
            subtype_1: Result<Option<super::Census>, String>,
        }

        impl Default for GetCityCensusResponseItem {
            fn default() -> Self {
                Self {
                    subtype_0: Ok(Default::default()),
                    subtype_1: Ok(Default::default()),
                }
            }
        }

        impl GetCityCensusResponseItem {
            pub fn subtype_0<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::City>>,
                T::Error: std::fmt::Display,
            {
                self.subtype_0 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
                self
            }
            pub fn subtype_1<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Census>>,
                T::Error: std::fmt::Display,
            {
                self.subtype_1 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetCityCensusResponseItem> for super::GetCityCensusResponseItem {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetCityCensusResponseItem,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    subtype_0: value.subtype_0?,
                    subtype_1: value.subtype_1?,
                })
            }
        }

        impl From<super::GetCityCensusResponseItem> for GetCityCensusResponseItem {
            fn from(value: super::GetCityCensusResponseItem) -> Self {
                Self {
                    subtype_0: Ok(value.subtype_0),
                    subtype_1: Ok(value.subtype_1),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Infrastructure {
            high_stress_miles: Result<Option<f64>, String>,
            low_stress_miles: Result<Option<f64>, String>,
        }

        impl Default for Infrastructure {
            fn default() -> Self {
                Self {
                    high_stress_miles: Ok(Default::default()),
                    low_stress_miles: Ok(Default::default()),
                }
            }
        }

        impl Infrastructure {
            pub fn high_stress_miles<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.high_stress_miles = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for high_stress_miles: {}",
                        e
                    )
                });
                self
            }
            pub fn low_stress_miles<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.low_stress_miles = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for low_stress_miles: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<Infrastructure> for super::Infrastructure {
            type Error = super::error::ConversionError;
            fn try_from(value: Infrastructure) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    high_stress_miles: value.high_stress_miles?,
                    low_stress_miles: value.low_stress_miles?,
                })
            }
        }

        impl From<super::Infrastructure> for Infrastructure {
            fn from(value: super::Infrastructure) -> Self {
                Self {
                    high_stress_miles: Ok(value.high_stress_miles),
                    low_stress_miles: Ok(value.low_stress_miles),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Opportunity {
            employment: Result<Option<f64>, String>,
            higher_education: Result<Option<f64>, String>,
            k12_education: Result<Option<f64>, String>,
            score: Result<Option<f64>, String>,
            technical_vocational_college: Result<Option<f64>, String>,
        }

        impl Default for Opportunity {
            fn default() -> Self {
                Self {
                    employment: Ok(Default::default()),
                    higher_education: Ok(Default::default()),
                    k12_education: Ok(Default::default()),
                    score: Ok(Default::default()),
                    technical_vocational_college: Ok(Default::default()),
                }
            }
        }

        impl Opportunity {
            pub fn employment<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.employment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for employment: {}", e));
                self
            }
            pub fn higher_education<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.higher_education = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for higher_education: {}",
                        e
                    )
                });
                self
            }
            pub fn k12_education<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.k12_education = value.try_into().map_err(|e| {
                    format!("error converting supplied value for k12_education: {}", e)
                });
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn technical_vocational_college<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.technical_vocational_college = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for technical_vocational_college: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<Opportunity> for super::Opportunity {
            type Error = super::error::ConversionError;
            fn try_from(value: Opportunity) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    employment: value.employment?,
                    higher_education: value.higher_education?,
                    k12_education: value.k12_education?,
                    score: value.score?,
                    technical_vocational_college: value.technical_vocational_college?,
                })
            }
        }

        impl From<super::Opportunity> for Opportunity {
            fn from(value: super::Opportunity) -> Self {
                Self {
                    employment: Ok(value.employment),
                    higher_education: Ok(value.higher_education),
                    k12_education: Ok(value.k12_education),
                    score: Ok(value.score),
                    technical_vocational_college: Ok(value.technical_vocational_college),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct People {
            score: Result<Option<f64>, String>,
        }

        impl Default for People {
            fn default() -> Self {
                Self {
                    score: Ok(Default::default()),
                }
            }
        }

        impl People {
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<People> for super::People {
            type Error = super::error::ConversionError;
            fn try_from(value: People) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    score: value.score?,
                })
            }
        }

        impl From<super::People> for People {
            fn from(value: super::People) -> Self {
                Self {
                    score: Ok(value.score),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Recreation {
            community_centers: Result<Option<f64>, String>,
            parks: Result<Option<f64>, String>,
            recreation_trails: Result<Option<f64>, String>,
            score: Result<Option<f64>, String>,
        }

        impl Default for Recreation {
            fn default() -> Self {
                Self {
                    community_centers: Ok(Default::default()),
                    parks: Ok(Default::default()),
                    recreation_trails: Ok(Default::default()),
                    score: Ok(Default::default()),
                }
            }
        }

        impl Recreation {
            pub fn community_centers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.community_centers = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for community_centers: {}",
                        e
                    )
                });
                self
            }
            pub fn parks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.parks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for parks: {}", e));
                self
            }
            pub fn recreation_trails<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.recreation_trails = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for recreation_trails: {}",
                        e
                    )
                });
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Recreation> for super::Recreation {
            type Error = super::error::ConversionError;
            fn try_from(value: Recreation) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    community_centers: value.community_centers?,
                    parks: value.parks?,
                    recreation_trails: value.recreation_trails?,
                    score: value.score?,
                })
            }
        }

        impl From<super::Recreation> for Recreation {
            fn from(value: super::Recreation) -> Self {
                Self {
                    community_centers: Ok(value.community_centers),
                    parks: Ok(value.parks),
                    recreation_trails: Ok(value.recreation_trails),
                    score: Ok(value.score),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Retail {
            score: Result<Option<f64>, String>,
        }

        impl Default for Retail {
            fn default() -> Self {
                Self {
                    score: Ok(Default::default()),
                }
            }
        }

        impl Retail {
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Retail> for super::Retail {
            type Error = super::error::ConversionError;
            fn try_from(value: Retail) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    score: value.score?,
                })
            }
        }

        impl From<super::Retail> for Retail {
            fn from(value: super::Retail) -> Self {
                Self {
                    score: Ok(value.score),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Submission {
            city: Result<Option<String>, String>,
            consent: Result<Option<bool>, String>,
            country: Result<Option<super::Country>, String>,
            created_at: Result<Vec<i64>, String>,
            email: Result<Option<String>, String>,
            fips_code: Result<Option<String>, String>,
            first_name: Result<Option<String>, String>,
            id: Result<Option<i64>, String>,
            last_name: Result<Option<String>, String>,
            occupation: Result<Option<String>, String>,
            organization: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            submission_status: Result<Option<String>, String>,
        }

        impl Default for Submission {
            fn default() -> Self {
                Self {
                    city: Ok(Default::default()),
                    consent: Ok(Default::default()),
                    country: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    email: Ok(Default::default()),
                    fips_code: Ok(Default::default()),
                    first_name: Ok(Default::default()),
                    id: Ok(Default::default()),
                    last_name: Ok(Default::default()),
                    occupation: Ok(Default::default()),
                    organization: Ok(Default::default()),
                    region: Ok(Default::default()),
                    submission_status: Ok(Default::default()),
                }
            }
        }

        impl Submission {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn consent<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.consent = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for consent: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Country>>,
                T::Error: std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<i64>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn first_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.first_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for first_name: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn last_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.last_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_name: {}", e));
                self
            }
            pub fn occupation<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.occupation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for occupation: {}", e));
                self
            }
            pub fn organization<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.organization = value.try_into().map_err(|e| {
                    format!("error converting supplied value for organization: {}", e)
                });
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn submission_status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.submission_status = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for submission_status: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<Submission> for super::Submission {
            type Error = super::error::ConversionError;
            fn try_from(value: Submission) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city: value.city?,
                    consent: value.consent?,
                    country: value.country?,
                    created_at: value.created_at?,
                    email: value.email?,
                    fips_code: value.fips_code?,
                    first_name: value.first_name?,
                    id: value.id?,
                    last_name: value.last_name?,
                    occupation: value.occupation?,
                    organization: value.organization?,
                    region: value.region?,
                    submission_status: value.submission_status?,
                })
            }
        }

        impl From<super::Submission> for Submission {
            fn from(value: super::Submission) -> Self {
                Self {
                    city: Ok(value.city),
                    consent: Ok(value.consent),
                    country: Ok(value.country),
                    created_at: Ok(value.created_at),
                    email: Ok(value.email),
                    fips_code: Ok(value.fips_code),
                    first_name: Ok(value.first_name),
                    id: Ok(value.id),
                    last_name: Ok(value.last_name),
                    occupation: Ok(value.occupation),
                    organization: Ok(value.organization),
                    region: Ok(value.region),
                    submission_status: Ok(value.submission_status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SubmissionPatch {
            city: Result<Option<String>, String>,
            consent: Result<Option<bool>, String>,
            country: Result<Option<super::Country>, String>,
            email: Result<Option<String>, String>,
            fips_code: Result<Option<String>, String>,
            first_name: Result<Option<String>, String>,
            last_name: Result<Option<String>, String>,
            occupation: Result<Option<String>, String>,
            organization: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            submission_status: Result<Option<String>, String>,
        }

        impl Default for SubmissionPatch {
            fn default() -> Self {
                Self {
                    city: Ok(Default::default()),
                    consent: Ok(Default::default()),
                    country: Ok(Default::default()),
                    email: Ok(Default::default()),
                    fips_code: Ok(Default::default()),
                    first_name: Ok(Default::default()),
                    last_name: Ok(Default::default()),
                    occupation: Ok(Default::default()),
                    organization: Ok(Default::default()),
                    region: Ok(Default::default()),
                    submission_status: Ok(Default::default()),
                }
            }
        }

        impl SubmissionPatch {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn consent<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.consent = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for consent: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Country>>,
                T::Error: std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn first_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.first_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for first_name: {}", e));
                self
            }
            pub fn last_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.last_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_name: {}", e));
                self
            }
            pub fn occupation<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.occupation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for occupation: {}", e));
                self
            }
            pub fn organization<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.organization = value.try_into().map_err(|e| {
                    format!("error converting supplied value for organization: {}", e)
                });
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn submission_status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.submission_status = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for submission_status: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<SubmissionPatch> for super::SubmissionPatch {
            type Error = super::error::ConversionError;
            fn try_from(value: SubmissionPatch) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city: value.city?,
                    consent: value.consent?,
                    country: value.country?,
                    email: value.email?,
                    fips_code: value.fips_code?,
                    first_name: value.first_name?,
                    last_name: value.last_name?,
                    occupation: value.occupation?,
                    organization: value.organization?,
                    region: value.region?,
                    submission_status: value.submission_status?,
                })
            }
        }

        impl From<super::SubmissionPatch> for SubmissionPatch {
            fn from(value: super::SubmissionPatch) -> Self {
                Self {
                    city: Ok(value.city),
                    consent: Ok(value.consent),
                    country: Ok(value.country),
                    email: Ok(value.email),
                    fips_code: Ok(value.fips_code),
                    first_name: Ok(value.first_name),
                    last_name: Ok(value.last_name),
                    occupation: Ok(value.occupation),
                    organization: Ok(value.organization),
                    region: Ok(value.region),
                    submission_status: Ok(value.submission_status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SubmissionPost {
            city: Result<String, String>,
            consent: Result<bool, String>,
            country: Result<super::Country, String>,
            email: Result<String, String>,
            fips_code: Result<String, String>,
            first_name: Result<String, String>,
            last_name: Result<String, String>,
            occupation: Result<Option<String>, String>,
            organization: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            submission_status: Result<Option<String>, String>,
        }

        impl Default for SubmissionPost {
            fn default() -> Self {
                Self {
                    city: Err("no value supplied for city".to_string()),
                    consent: Err("no value supplied for consent".to_string()),
                    country: Err("no value supplied for country".to_string()),
                    email: Err("no value supplied for email".to_string()),
                    fips_code: Err("no value supplied for fips_code".to_string()),
                    first_name: Err("no value supplied for first_name".to_string()),
                    last_name: Err("no value supplied for last_name".to_string()),
                    occupation: Ok(Default::default()),
                    organization: Ok(Default::default()),
                    region: Ok(Default::default()),
                    submission_status: Ok(Default::default()),
                }
            }
        }

        impl SubmissionPost {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn consent<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.consent = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for consent: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Country>,
                T::Error: std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn first_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.first_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for first_name: {}", e));
                self
            }
            pub fn last_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.last_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_name: {}", e));
                self
            }
            pub fn occupation<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.occupation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for occupation: {}", e));
                self
            }
            pub fn organization<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.organization = value.try_into().map_err(|e| {
                    format!("error converting supplied value for organization: {}", e)
                });
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn submission_status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.submission_status = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for submission_status: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<SubmissionPost> for super::SubmissionPost {
            type Error = super::error::ConversionError;
            fn try_from(value: SubmissionPost) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city: value.city?,
                    consent: value.consent?,
                    country: value.country?,
                    email: value.email?,
                    fips_code: value.fips_code?,
                    first_name: value.first_name?,
                    last_name: value.last_name?,
                    occupation: value.occupation?,
                    organization: value.organization?,
                    region: value.region?,
                    submission_status: value.submission_status?,
                })
            }
        }

        impl From<super::SubmissionPost> for SubmissionPost {
            fn from(value: super::SubmissionPost) -> Self {
                Self {
                    city: Ok(value.city),
                    consent: Ok(value.consent),
                    country: Ok(value.country),
                    email: Ok(value.email),
                    fips_code: Ok(value.fips_code),
                    first_name: Ok(value.first_name),
                    last_name: Ok(value.last_name),
                    occupation: Ok(value.occupation),
                    organization: Ok(value.organization),
                    region: Ok(value.region),
                    submission_status: Ok(value.submission_status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Transit {
            score: Result<Option<f64>, String>,
        }

        impl Default for Transit {
            fn default() -> Self {
                Self {
                    score: Ok(Default::default()),
                }
            }
        }

        impl Transit {
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Transit> for super::Transit {
            type Error = super::error::ConversionError;
            fn try_from(value: Transit) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    score: value.score?,
                })
            }
        }

        impl From<super::Transit> for Transit {
            fn from(value: super::Transit) -> Self {
                Self {
                    score: Ok(value.score),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for BNA REST API
///
///Provides a way to retrieve the BNA results programmatically.
///
///Version: 1.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0.0"
    }
}

impl Client {
    ///Get city rating summaries
    ///
    ///Get city rating summaries.
    ///
    ///Sends a `GET` request to `/ratings`
    ///
    ///Arguments:
    /// - `page`: Page index (starting at 0)
    /// - `page_size`: The number of items to be returned per page (1..50)
    ///```ignore
    /// let response = client.get_ratings()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_ratings(&self) -> builder::GetRatings {
        builder::GetRatings::new(self)
    }

    ///Create new city rating
    ///
    ///Create a new city rating
    ///
    ///Sends a `POST` request to `/ratings`
    ///
    ///Arguments:
    /// - `body`: Create bna
    ///```ignore
    /// let response = client.post_ratings()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_ratings(&self) -> builder::PostRatings {
        builder::PostRatings::new(self)
    }

    ///Get the city rating analys details
    ///
    ///Get the city rating analys details
    ///
    ///Sends a `GET` request to `/ratings/analyses`
    ///
    ///Arguments:
    /// - `page`: Page index (starting at 0)
    /// - `page_size`: The number of items to be returned per page (1..50)
    ///```ignore
    /// let response = client.get_ratings_analyses()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_ratings_analyses(&self) -> builder::GetRatingsAnalyses {
        builder::GetRatingsAnalyses::new(self)
    }

    ///Submit a new city to analyze
    ///
    ///Submit a new city to analyze
    ///
    ///Sends a `POST` request to `/ratings/analyses`
    ///
    ///Arguments:
    /// - `body`: Create a new analysis performed by the brokenspoke-analyzer
    ///```ignore
    /// let response = client.post_ratings_analyses()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_ratings_analyses(&self) -> builder::PostRatingsAnalyses {
        builder::PostRatingsAnalyses::new(self)
    }

    ///Get the summary of a specific analysis
    ///
    ///Get the summary of a specific analysis .
    ///
    ///Sends a `GET` request to `/ratings/analyses/{analysis_id}`
    ///
    ///Arguments:
    /// - `analysis_id`: State Machine Identifier
    ///```ignore
    /// let response = client.get_analysis()
    ///    .analysis_id(analysis_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_analysis(&self) -> builder::GetAnalysis {
        builder::GetAnalysis::new(self)
    }

    ///Update an analysis
    ///
    ///Update an analysis
    ///
    ///Sends a `PATCH` request to `/ratings/analyses/{analysis_id}`
    ///
    ///Arguments:
    /// - `analysis_id`: State Machine Identifier
    /// - `body`: Update a new analysis performed by the brokenspoke-analyzer
    ///```ignore
    /// let response = client.patch_analysis()
    ///    .analysis_id(analysis_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn patch_analysis(&self) -> builder::PatchAnalysis {
        builder::PatchAnalysis::new(self)
    }

    ///Get a specific city rating summary
    ///
    ///Get a specific city rating summary.
    ///
    ///Sends a `GET` request to `/ratings/{rating_id}`
    ///
    ///Arguments:
    /// - `rating_id`: Analysis identifier
    /// - `component`: Select a component to retrieve alongside the BNA summary.
    ///   If none is specified, all the components are returned.
    ///
    ///```ignore
    /// let response = client.get_rating()
    ///    .rating_id(rating_id)
    ///    .component(component)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_rating(&self) -> builder::GetRating {
        builder::GetRating::new(self)
    }

    ///Get a specific city rating summary and its associated city details
    ///
    ///Get a specific city rating summary and its associated city details.
    ///
    ///Sends a `GET` request to `/ratings/{rating_id}/city`
    ///
    ///Arguments:
    /// - `rating_id`: Analysis identifier
    ///```ignore
    /// let response = client.get_rating_city()
    ///    .rating_id(rating_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_rating_city(&self) -> builder::GetRatingCity {
        builder::GetRatingCity::new(self)
    }

    ///Get city details
    ///
    ///Get the details of all cities where an BNA analysis was performed.
    ///
    ///Sends a `GET` request to `/cities`
    ///
    ///Arguments:
    /// - `page`: Page index (starting at 0)
    /// - `page_size`: The number of items to be returned per page (1..50)
    ///```ignore
    /// let response = client.get_cities()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_cities(&self) -> builder::GetCities {
        builder::GetCities::new(self)
    }

    ///Create a new city
    ///
    ///Create a new city.
    ///
    ///Sends a `POST` request to `/cities`
    ///
    ///Arguments:
    /// - `page`: Page index (starting at 0)
    /// - `page_size`: The number of items to be returned per page (1..50)
    /// - `body`: Create a new city.
    ///```ignore
    /// let response = client.post_city()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_city(&self) -> builder::PostCity {
        builder::PostCity::new(self)
    }

    ///Get the cities that were submitted for analysis
    ///
    ///Get the cities that were submitted for analysis.
    ///
    ///Sends a `GET` request to `/cities/submissions`
    ///
    ///Arguments:
    /// - `page`: Page index (starting at 0)
    /// - `page_size`: The number of items to be returned per page (1..50)
    ///```ignore
    /// let response = client.get_city_submissions()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_city_submissions(&self) -> builder::GetCitySubmissions {
        builder::GetCitySubmissions::new(self)
    }

    ///Submit a new city for analysis
    ///
    ///Submit a new city for analysis.
    ///
    ///Sends a `POST` request to `/cities/submissions`
    ///
    ///Arguments:
    /// - `body`: Create a new analysis to be performed by the
    ///   brokenspoke-analyzer
    ///```ignore
    /// let response = client.post_city_submission()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_city_submission(&self) -> builder::PostCitySubmission {
        builder::PostCitySubmission::new(self)
    }

    ///Get the details of a specific sumission
    ///
    ///Get the details of a specific sumission.
    ///
    ///Sends a `GET` request to `/cities/submissions/{submission_id}`
    ///
    ///Arguments:
    /// - `submission_id`: Submission identifier
    ///```ignore
    /// let response = client.get_city_submission()
    ///    .submission_id(submission_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_city_submission(&self) -> builder::GetCitySubmission {
        builder::GetCitySubmission::new(self)
    }

    ///Update the details of a specific sumission
    ///
    ///Update the details of a specific sumission.
    ///
    ///Sends a `PATCH` request to `/cities/submissions/{submission_id}`
    ///
    ///Arguments:
    /// - `submission_id`: Submission identifier
    /// - `body`: Update the details of a specific sumission.
    ///```ignore
    /// let response = client.patch_city_submissions()
    ///    .submission_id(submission_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn patch_city_submissions(&self) -> builder::PatchCitySubmissions {
        builder::PatchCitySubmissions::new(self)
    }

    ///Get the details of specific city
    ///
    ///Get the details of a specific city where an BNA analysis was computed.
    ///
    ///
    ///Sends a `GET` request to `/cities/{country}/{region}/{name}`
    ///
    ///Arguments:
    /// - `country`: Country name
    /// - `region`: Region name. A region can be a state, a province, a
    ///   community, or something similar depending on the country. If a country
    ///   does not have this concept, then the country name is used.
    ///
    /// - `name`: City name
    ///```ignore
    /// let response = client.get_city()
    ///    .country(country)
    ///    .region(region)
    ///    .name(name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_city(&self) -> builder::GetCity {
        builder::GetCity::new(self)
    }

    ///Get the details of a specific city with all the analysis that were
    /// performed against it
    ///
    ///
    ///Get the details of a specific city with all the analysis that were
    /// performed against it.
    ///
    ///
    ///Sends a `GET` request to `/cities/{country}/{region}/{name}/ratings`
    ///
    ///Arguments:
    /// - `country`: Country name
    /// - `region`: Region name. A region can be a state, a province, a
    ///   community, or something similar depending on the country. If a country
    ///   does not have this concept, then the country name is used.
    ///
    /// - `name`: City name
    ///```ignore
    /// let response = client.get_city_ratings()
    ///    .country(country)
    ///    .region(region)
    ///    .name(name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_city_ratings(&self) -> builder::GetCityRatings {
        builder::GetCityRatings::new(self)
    }

    ///Get the details of a specific city with its associated census
    /// information.
    ///
    ///
    ///Get the details of a specific city with its associated census
    /// information.
    ///
    ///
    ///Sends a `GET` request to `/cities/{country}/{region}/{name}/census`
    ///
    ///Arguments:
    /// - `country`: Country name
    /// - `region`: Region name. A region can be a state, a province, a
    ///   community, or something similar depending on the country. If a country
    ///   does not have this concept, then the country name is used.
    ///
    /// - `name`: City name
    ///```ignore
    /// let response = client.get_city_census()
    ///    .country(country)
    ///    .region(region)
    ///    .name(name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_city_census(&self) -> builder::GetCityCensus {
        builder::GetCityCensus::new(self)
    }

    ///Enqueue a city to process
    ///
    ///Enqueue a city to process.
    ///
    ///Sends a `POST` request to `/ratings/enqueue`
    ///
    ///Arguments:
    /// - `body`: Create a new city to enqueue.
    ///```ignore
    /// let response = client.post_rating_enqueue()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_rating_enqueue(&self) -> builder::PostRatingEnqueue {
        builder::PostRatingEnqueue::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    ///Builder for [`Client::get_ratings`]
    ///
    ///[`Client::get_ratings`]: super::Client::get_ratings
    #[derive(Debug, Clone)]
    pub struct GetRatings<'a> {
        client: &'a super::Client,
        page: Result<Option<i64>, String>,
        page_size: Result<Option<i64>, String>,
    }

    impl<'a> GetRatings<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page failed".to_string());
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page_size = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page_size failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ratings`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::BnaSummary>>, Error<()>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/ratings", client.baseurl,);
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &page {
                query.push(("page", v.to_string()));
            }
            if let Some(v) = &page_size {
                query.push(("page_size", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_ratings`]
    ///
    ///[`Client::post_ratings`]: super::Client::post_ratings
    #[derive(Debug, Clone)]
    pub struct PostRatings<'a> {
        client: &'a super::Client,
        body: Result<types::builder::BnaPost, String>,
    }

    impl<'a> PostRatings<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::BnaPost::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::BnaPost>,
            <V as std::convert::TryInto<types::BnaPost>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `BnaPost` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::BnaPost) -> types::builder::BnaPost,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/ratings`
        pub async fn send(self) -> Result<ResponseValue<types::Bna>, Error<types::Errors>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::BnaPost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/ratings", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_ratings_analyses`]
    ///
    ///[`Client::get_ratings_analyses`]: super::Client::get_ratings_analyses
    #[derive(Debug, Clone)]
    pub struct GetRatingsAnalyses<'a> {
        client: &'a super::Client,
        page: Result<Option<i64>, String>,
        page_size: Result<Option<i64>, String>,
    }

    impl<'a> GetRatingsAnalyses<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page failed".to_string());
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page_size = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page_size failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ratings/analyses`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<Vec<types::Analysis>>, Error<types::Errors>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/ratings/analyses", client.baseurl,);
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &page {
                query.push(("page", v.to_string()));
            }
            if let Some(v) = &page_size {
                query.push(("page_size", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_ratings_analyses`]
    ///
    ///[`Client::post_ratings_analyses`]: super::Client::post_ratings_analyses
    #[derive(Debug, Clone)]
    pub struct PostRatingsAnalyses<'a> {
        client: &'a super::Client,
        body: Result<types::builder::AnalysisPost, String>,
    }

    impl<'a> PostRatingsAnalyses<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::AnalysisPost::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AnalysisPost>,
            <V as std::convert::TryInto<types::AnalysisPost>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `AnalysisPost` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::AnalysisPost) -> types::builder::AnalysisPost,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/ratings/analyses`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<Vec<types::Analysis>>, Error<types::Errors>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::AnalysisPost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/ratings/analyses", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_analysis`]
    ///
    ///[`Client::get_analysis`]: super::Client::get_analysis
    #[derive(Debug, Clone)]
    pub struct GetAnalysis<'a> {
        client: &'a super::Client,
        analysis_id: Result<types::StateMachineId, String>,
    }

    impl<'a> GetAnalysis<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                analysis_id: Err("analysis_id was not initialized".to_string()),
            }
        }

        pub fn analysis_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::StateMachineId>,
        {
            self.analysis_id = value
                .try_into()
                .map_err(|_| "conversion to `StateMachineId` for analysis_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ratings/analyses/{analysis_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Analysis>, Error<types::Errors>> {
            let Self {
                client,
                analysis_id,
            } = self;
            let analysis_id = analysis_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ratings/analyses/{}",
                client.baseurl,
                encode_path(&analysis_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::patch_analysis`]
    ///
    ///[`Client::patch_analysis`]: super::Client::patch_analysis
    #[derive(Debug, Clone)]
    pub struct PatchAnalysis<'a> {
        client: &'a super::Client,
        analysis_id: Result<types::StateMachineId, String>,
        body: Result<types::builder::AnalysisPatch, String>,
    }

    impl<'a> PatchAnalysis<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                analysis_id: Err("analysis_id was not initialized".to_string()),
                body: Ok(types::builder::AnalysisPatch::default()),
            }
        }

        pub fn analysis_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::StateMachineId>,
        {
            self.analysis_id = value
                .try_into()
                .map_err(|_| "conversion to `StateMachineId` for analysis_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AnalysisPatch>,
            <V as std::convert::TryInto<types::AnalysisPatch>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `AnalysisPatch` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::AnalysisPatch) -> types::builder::AnalysisPatch,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PATCH` request to `/ratings/analyses/{analysis_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Analysis>, Error<types::Errors>> {
            let Self {
                client,
                analysis_id,
                body,
            } = self;
            let analysis_id = analysis_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::AnalysisPatch::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ratings/analyses/{}",
                client.baseurl,
                encode_path(&analysis_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .patch(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_rating`]
    ///
    ///[`Client::get_rating`]: super::Client::get_rating
    #[derive(Debug, Clone)]
    pub struct GetRating<'a> {
        client: &'a super::Client,
        rating_id: Result<uuid::Uuid, String>,
        component: Result<Option<types::GetRatingComponent>, String>,
    }

    impl<'a> GetRating<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                rating_id: Err("rating_id was not initialized".to_string()),
                component: Ok(None),
            }
        }

        pub fn rating_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.rating_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for rating_id failed".to_string());
            self
        }

        pub fn component<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetRatingComponent>,
        {
            self.component = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `GetRatingComponent` for component failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ratings/{rating_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Bna>, Error<types::Errors>> {
            let Self {
                client,
                rating_id,
                component,
            } = self;
            let rating_id = rating_id.map_err(Error::InvalidRequest)?;
            let component = component.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ratings/{}",
                client.baseurl,
                encode_path(&rating_id.to_string()),
            );
            let mut query = Vec::with_capacity(1usize);
            if let Some(v) = &component {
                query.push(("component", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_rating_city`]
    ///
    ///[`Client::get_rating_city`]: super::Client::get_rating_city
    #[derive(Debug, Clone)]
    pub struct GetRatingCity<'a> {
        client: &'a super::Client,
        rating_id: Result<uuid::Uuid, String>,
    }

    impl<'a> GetRatingCity<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                rating_id: Err("rating_id was not initialized".to_string()),
            }
        }

        pub fn rating_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.rating_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for rating_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ratings/{rating_id}/city`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<Vec<types::GetRatingCityResponseItem>>, Error<types::Errors>>
        {
            let Self { client, rating_id } = self;
            let rating_id = rating_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ratings/{}/city",
                client.baseurl,
                encode_path(&rating_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_cities`]
    ///
    ///[`Client::get_cities`]: super::Client::get_cities
    #[derive(Debug, Clone)]
    pub struct GetCities<'a> {
        client: &'a super::Client,
        page: Result<Option<i64>, String>,
        page_size: Result<Option<i64>, String>,
    }

    impl<'a> GetCities<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page failed".to_string());
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page_size = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page_size failed".to_string());
            self
        }

        ///Sends a `GET` request to `/cities`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::City>>, Error<()>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/cities", client.baseurl,);
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &page {
                query.push(("page", v.to_string()));
            }
            if let Some(v) = &page_size {
                query.push(("page_size", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_city`]
    ///
    ///[`Client::post_city`]: super::Client::post_city
    #[derive(Debug, Clone)]
    pub struct PostCity<'a> {
        client: &'a super::Client,
        page: Result<Option<i64>, String>,
        page_size: Result<Option<i64>, String>,
        body: Result<types::builder::CityPost, String>,
    }

    impl<'a> PostCity<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
                body: Ok(types::builder::CityPost::default()),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page failed".to_string());
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page_size = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page_size failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CityPost>,
            <V as std::convert::TryInto<types::CityPost>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `CityPost` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::CityPost) -> types::builder::CityPost,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/cities`
        pub async fn send(self) -> Result<ResponseValue<types::City>, Error<types::Errors>> {
            let Self {
                client,
                page,
                page_size,
                body,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::CityPost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/cities", client.baseurl,);
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &page {
                query.push(("page", v.to_string()));
            }
            if let Some(v) = &page_size {
                query.push(("page_size", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_city_submissions`]
    ///
    ///[`Client::get_city_submissions`]: super::Client::get_city_submissions
    #[derive(Debug, Clone)]
    pub struct GetCitySubmissions<'a> {
        client: &'a super::Client,
        page: Result<Option<i64>, String>,
        page_size: Result<Option<i64>, String>,
    }

    impl<'a> GetCitySubmissions<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page failed".to_string());
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.page_size = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for page_size failed".to_string());
            self
        }

        ///Sends a `GET` request to `/cities/submissions`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::Submission>>, Error<()>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/cities/submissions", client.baseurl,);
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &page {
                query.push(("page", v.to_string()));
            }
            if let Some(v) = &page_size {
                query.push(("page_size", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_city_submission`]
    ///
    ///[`Client::post_city_submission`]: super::Client::post_city_submission
    #[derive(Debug, Clone)]
    pub struct PostCitySubmission<'a> {
        client: &'a super::Client,
        body: Result<types::builder::SubmissionPost, String>,
    }

    impl<'a> PostCitySubmission<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::SubmissionPost::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::SubmissionPost>,
            <V as std::convert::TryInto<types::SubmissionPost>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `SubmissionPost` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::SubmissionPost) -> types::builder::SubmissionPost,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/cities/submissions`
        pub async fn send(self) -> Result<ResponseValue<types::Submission>, Error<types::Errors>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::SubmissionPost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/cities/submissions", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_city_submission`]
    ///
    ///[`Client::get_city_submission`]: super::Client::get_city_submission
    #[derive(Debug, Clone)]
    pub struct GetCitySubmission<'a> {
        client: &'a super::Client,
        submission_id: Result<i64, String>,
    }

    impl<'a> GetCitySubmission<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                submission_id: Err("submission_id was not initialized".to_string()),
            }
        }

        pub fn submission_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.submission_id = value
                .try_into()
                .map_err(|_| "conversion to `i64` for submission_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/cities/submissions/{submission_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Submission>, Error<types::Errors>> {
            let Self {
                client,
                submission_id,
            } = self;
            let submission_id = submission_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/cities/submissions/{}",
                client.baseurl,
                encode_path(&submission_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::patch_city_submissions`]
    ///
    ///[`Client::patch_city_submissions`]: super::Client::patch_city_submissions
    #[derive(Debug, Clone)]
    pub struct PatchCitySubmissions<'a> {
        client: &'a super::Client,
        submission_id: Result<i64, String>,
        body: Result<types::builder::SubmissionPatch, String>,
    }

    impl<'a> PatchCitySubmissions<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                submission_id: Err("submission_id was not initialized".to_string()),
                body: Ok(types::builder::SubmissionPatch::default()),
            }
        }

        pub fn submission_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.submission_id = value
                .try_into()
                .map_err(|_| "conversion to `i64` for submission_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::SubmissionPatch>,
            <V as std::convert::TryInto<types::SubmissionPatch>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `SubmissionPatch` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::SubmissionPatch) -> types::builder::SubmissionPatch,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PATCH` request to `/cities/submissions/{submission_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Submission>, Error<types::Errors>> {
            let Self {
                client,
                submission_id,
                body,
            } = self;
            let submission_id = submission_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::SubmissionPatch::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/cities/submissions/{}",
                client.baseurl,
                encode_path(&submission_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .patch(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_city`]
    ///
    ///[`Client::get_city`]: super::Client::get_city
    #[derive(Debug, Clone)]
    pub struct GetCity<'a> {
        client: &'a super::Client,
        country: Result<types::Country, String>,
        region: Result<String, String>,
        name: Result<String, String>,
    }

    impl<'a> GetCity<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                country: Err("country was not initialized".to_string()),
                region: Err("region was not initialized".to_string()),
                name: Err("name was not initialized".to_string()),
            }
        }

        pub fn country<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::Country>,
        {
            self.country = value
                .try_into()
                .map_err(|_| "conversion to `Country` for country failed".to_string());
            self
        }

        pub fn region<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.region = value
                .try_into()
                .map_err(|_| "conversion to `String` for region failed".to_string());
            self
        }

        pub fn name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.name = value
                .try_into()
                .map_err(|_| "conversion to `String` for name failed".to_string());
            self
        }

        ///Sends a `GET` request to `/cities/{country}/{region}/{name}`
        pub async fn send(self) -> Result<ResponseValue<types::City>, Error<types::Errors>> {
            let Self {
                client,
                country,
                region,
                name,
            } = self;
            let country = country.map_err(Error::InvalidRequest)?;
            let region = region.map_err(Error::InvalidRequest)?;
            let name = name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/cities/{}/{}/{}",
                client.baseurl,
                encode_path(&country.to_string()),
                encode_path(&region.to_string()),
                encode_path(&name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_city_ratings`]
    ///
    ///[`Client::get_city_ratings`]: super::Client::get_city_ratings
    #[derive(Debug, Clone)]
    pub struct GetCityRatings<'a> {
        client: &'a super::Client,
        country: Result<types::Country, String>,
        region: Result<String, String>,
        name: Result<String, String>,
    }

    impl<'a> GetCityRatings<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                country: Err("country was not initialized".to_string()),
                region: Err("region was not initialized".to_string()),
                name: Err("name was not initialized".to_string()),
            }
        }

        pub fn country<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::Country>,
        {
            self.country = value
                .try_into()
                .map_err(|_| "conversion to `Country` for country failed".to_string());
            self
        }

        pub fn region<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.region = value
                .try_into()
                .map_err(|_| "conversion to `String` for region failed".to_string());
            self
        }

        pub fn name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.name = value
                .try_into()
                .map_err(|_| "conversion to `String` for name failed".to_string());
            self
        }

        ///Sends a `GET` request to `/cities/{country}/{region}/{name}/ratings`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<Vec<types::GetCityRatingsResponseItem>>, Error<types::Errors>>
        {
            let Self {
                client,
                country,
                region,
                name,
            } = self;
            let country = country.map_err(Error::InvalidRequest)?;
            let region = region.map_err(Error::InvalidRequest)?;
            let name = name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/cities/{}/{}/{}/ratings",
                client.baseurl,
                encode_path(&country.to_string()),
                encode_path(&region.to_string()),
                encode_path(&name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_city_census`]
    ///
    ///[`Client::get_city_census`]: super::Client::get_city_census
    #[derive(Debug, Clone)]
    pub struct GetCityCensus<'a> {
        client: &'a super::Client,
        country: Result<types::Country, String>,
        region: Result<String, String>,
        name: Result<String, String>,
    }

    impl<'a> GetCityCensus<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                country: Err("country was not initialized".to_string()),
                region: Err("region was not initialized".to_string()),
                name: Err("name was not initialized".to_string()),
            }
        }

        pub fn country<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::Country>,
        {
            self.country = value
                .try_into()
                .map_err(|_| "conversion to `Country` for country failed".to_string());
            self
        }

        pub fn region<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.region = value
                .try_into()
                .map_err(|_| "conversion to `String` for region failed".to_string());
            self
        }

        pub fn name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.name = value
                .try_into()
                .map_err(|_| "conversion to `String` for name failed".to_string());
            self
        }

        ///Sends a `GET` request to `/cities/{country}/{region}/{name}/census`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<Vec<types::GetCityCensusResponseItem>>, Error<types::Errors>>
        {
            let Self {
                client,
                country,
                region,
                name,
            } = self;
            let country = country.map_err(Error::InvalidRequest)?;
            let region = region.map_err(Error::InvalidRequest)?;
            let name = name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/cities/{}/{}/{}/census",
                client.baseurl,
                encode_path(&country.to_string()),
                encode_path(&region.to_string()),
                encode_path(&name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_rating_enqueue`]
    ///
    ///[`Client::post_rating_enqueue`]: super::Client::post_rating_enqueue
    #[derive(Debug, Clone)]
    pub struct PostRatingEnqueue<'a> {
        client: &'a super::Client,
        body: Result<types::builder::EnqueuePost, String>,
    }

    impl<'a> PostRatingEnqueue<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::EnqueuePost::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::EnqueuePost>,
            <V as std::convert::TryInto<types::EnqueuePost>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `EnqueuePost` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::EnqueuePost) -> types::builder::EnqueuePost,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/ratings/enqueue`
        pub async fn send(self) -> Result<ResponseValue<types::Enqueue>, Error<types::Errors>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::EnqueuePost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/ratings/enqueue", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}