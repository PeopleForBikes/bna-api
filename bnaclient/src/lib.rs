#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
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

    ///Single API Error object as described in <https://jsonapi.org/format/#error-objects>.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Single API Error object as described in <https://jsonapi.org/format/#error-objects>.",
    ///  "type": "object",
    ///  "required": [
    ///    "details",
    ///    "source",
    ///    "status",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "details": {
    ///      "description": "A human-readable explanation specific to this
    /// occurrence of the problem",
    ///      "examples": [
    ///        "the entry was not found"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "A unique identifier for this particular occurrence
    /// of the problem.",
    ///      "examples": [
    ///        "blfwkg8nvHcEJnQ="
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/APIErrorSource"
    ///    },
    ///    "status": {
    ///      "description": "The HTTP status code applicable to this problem,
    /// expressed as a string value.",
    ///      "examples": [
    ///        "404"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "A short, human-readable summary of the problem",
    ///      "examples": [
    ///        "Item Not Found"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ApiError {
        ///A human-readable explanation specific to this occurrence of the
        /// problem
        pub details: ::std::string::String,
        ///A unique identifier for this particular occurrence of the problem.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        pub source: ApiErrorSource,
        ///The HTTP status code applicable to this problem, expressed as a
        /// string value.
        pub status: ::std::string::String,
        ///A short, human-readable summary of the problem
        pub title: ::std::string::String,
    }

    impl ::std::convert::From<&ApiError> for ApiError {
        fn from(value: &ApiError) -> Self {
            value.clone()
        }
    }

    impl ApiError {
        pub fn builder() -> builder::ApiError {
            Default::default()
        }
    }

    ///An object containing references to the primary source of the error.
    ///
    ///It SHOULD include one of the following members or be omitted:
    ///
    ///  - pointer: a JSON Pointer [RFC6901](https://tools.ietf.org/html/rfc6901)
    ///    to the value in the request document that caused the error [e.g.
    ///    "/data" for a primary data object, or "/data/attributes/title" for a
    ///    specific attribute]. This MUST point to a value in the request
    ///    document that exists; if it doesn’t, the client SHOULD simply ignore
    ///    the pointer.
    ///  - parameter: a string indicating which URI query parameter caused the
    ///    error.
    ///  - header: a string indicating the name of a single request header which
    ///    caused the error.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An object containing references to the primary source of the error.\n\nIt SHOULD include one of the following members or be omitted:\n\n  - pointer: a JSON Pointer [RFC6901](https://tools.ietf.org/html/rfc6901) to the\n    value in the request document that caused the error [e.g. \"/data\" for a primary\n    data object, or \"/data/attributes/title\" for a specific attribute].\n    This MUST point to a value in the request document that exists; if it doesn’t,\n    the client SHOULD simply ignore the pointer.\n  - parameter: a string indicating which URI query parameter caused the error.\n  - header: a string indicating the name of a single request header which caused the\n    error.",
    ///  "oneOf": [
    ///    {
    ///      "description": "A JSON Pointer [RFC6901] to the value in the
    /// request document that caused the error.",
    ///      "examples": [
    ///        {
    ///          "pointer": "/data/attributes/title"
    ///        }
    ///      ],
    ///      "type": "object",
    ///      "required": [
    ///        "pointer"
    ///      ],
    ///      "properties": {
    ///        "pointer": {
    ///          "description": "A JSON Pointer [RFC6901] to the value in the
    /// request document that caused the error.",
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "A string indicating which URI query parameter
    /// caused the error.",
    ///      "examples": [
    ///        {
    ///          "parameter": "include"
    ///        }
    ///      ],
    ///      "type": "object",
    ///      "required": [
    ///        "parameter"
    ///      ],
    ///      "properties": {
    ///        "parameter": {
    ///          "description": "A string indicating which URI query parameter
    /// caused the error.",
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "A string indicating the name of a single request
    /// header which caused the error.",
    ///      "examples": [
    ///        {
    ///          "header": "Content-Type"
    ///        }
    ///      ],
    ///      "type": "object",
    ///      "required": [
    ///        "header"
    ///      ],
    ///      "properties": {
    ///        "header": {
    ///          "description": "A string indicating the name of a single
    /// request header which caused the error.",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ApiErrorSource {
        ///A JSON Pointer [RFC6901] to the value in the request document that
        /// caused the error.
        #[serde(rename = "pointer")]
        Pointer(::std::string::String),
        ///A string indicating which URI query parameter caused the error.
        #[serde(rename = "parameter")]
        Parameter(::std::string::String),
        ///A string indicating the name of a single request header which caused
        /// the error.
        #[serde(rename = "header")]
        Header(::std::string::String),
    }

    impl ::std::convert::From<&Self> for ApiErrorSource {
        fn from(value: &ApiErrorSource) -> Self {
            value.clone()
        }
    }

    ///Error objects MUST be returned as an array keyed by errors in the top
    /// level of a JSON:API document.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Error objects MUST be returned as an array keyed by
    /// errors in the top level of a\nJSON:API document.",
    ///  "type": "object",
    ///  "required": [
    ///    "errors"
    ///  ],
    ///  "properties": {
    ///    "errors": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/APIError"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ApiErrors {
        pub errors: ::std::vec::Vec<ApiError>,
    }

    impl ::std::convert::From<&ApiErrors> for ApiErrors {
        fn from(value: &ApiErrors) -> Self {
            value.clone()
        }
    }

    impl ApiErrors {
        pub fn builder() -> builder::ApiErrors {
            Default::default()
        }
    }

    ///`BnaPipeline`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "start_time",
    ///    "state_machine_id",
    ///    "status",
    ///    "step"
    ///  ],
    ///  "properties": {
    ///    "cost": {
    ///      "description": "Cost of an analysis in USD",
    ///      "examples": [
    ///        "6.8941"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "end_time": {
    ///      "description": "End time",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "fargate_price_id": {
    ///      "description": "Fargate price identifier used to compute the cost",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "fargate_task_arn": {
    ///      "description": "ARN of the Fargate task that performed the
    /// analysis",
    ///      "examples": [
    ///        "arn:aws:ecs:us-west-2:123456789012:task/bna/
    /// 29f979fc9fca402d94b014aa23d2f6e0"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "s3_bucket": {
    ///      "description": "Path of the S3 bucket where the results were
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
    ///        {
    ///          "city": "santa rosa",
    ///          "country": "United States",
    ///          "fips_code": "3570670",
    ///          "region": "new mexico"
    ///        }
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "start_time": {
    ///      "description": "Start time",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "state_machine_id": {
    ///      "description": "Pipeline identifier\nThis is the ID of the AWS
    /// state machine that was used to run the pipeline",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/PipelineStatus"
    ///    },
    ///    "step": {
    ///      "$ref": "#/components/schemas/BnaPipelineStep"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BnaPipeline {
        ///Cost of an analysis in USD
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cost: ::std::option::Option<::std::string::String>,
        ///End time
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub end_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Fargate price identifier used to compute the cost
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fargate_price_id: ::std::option::Option<i32>,
        ///ARN of the Fargate task that performed the analysis
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fargate_task_arn: ::std::option::Option<::std::string::String>,
        ///Path of the S3 bucket where the results were stored
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub s3_bucket: ::std::option::Option<::std::string::String>,
        ///Copy of the JSON message that was sent for processing
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sqs_message: ::std::option::Option<::std::string::String>,
        ///Start time
        pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Pipeline identifier
        ///This is the ID of the AWS state machine that was used to run the
        /// pipeline
        pub state_machine_id: ::uuid::Uuid,
        pub status: PipelineStatus,
        pub step: BnaPipelineStep,
    }

    impl ::std::convert::From<&BnaPipeline> for BnaPipeline {
        fn from(value: &BnaPipeline) -> Self {
            value.clone()
        }
    }

    impl BnaPipeline {
        pub fn builder() -> builder::BnaPipeline {
            Default::default()
        }
    }

    ///`BnaPipelinePatch`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "status",
    ///    "step"
    ///  ],
    ///  "properties": {
    ///    "cost": {
    ///      "description": "Cost of an analysis in USD",
    ///      "examples": [
    ///        "6.8941"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "end_time": {
    ///      "description": "End time",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "fargate_price_id": {
    ///      "description": "Fargate price identifier used to compute the cost",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "fargate_task_arn": {
    ///      "description": "ARN of the Fargate task that performed the
    /// analysis",
    ///      "examples": [
    ///        "arn:aws:ecs:us-west-2:123456789012:task/bna/
    /// 29f979fc9fca402d94b014aa23d2f6e0"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "s3_bucket": {
    ///      "description": "Path of the S3 bucket where the results were
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
    ///        {
    ///          "city": "santa rosa",
    ///          "country": "United States",
    ///          "fips_code": "3570670",
    ///          "region": "new mexico"
    ///        }
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "start_time": {
    ///      "description": "Start time",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/PipelineStatus"
    ///    },
    ///    "step": {
    ///      "$ref": "#/components/schemas/BnaPipelineStep"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BnaPipelinePatch {
        ///Cost of an analysis in USD
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cost: ::std::option::Option<::std::string::String>,
        ///End time
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub end_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Fargate price identifier used to compute the cost
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fargate_price_id: ::std::option::Option<i32>,
        ///ARN of the Fargate task that performed the analysis
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fargate_task_arn: ::std::option::Option<::std::string::String>,
        ///Path of the S3 bucket where the results were stored
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub s3_bucket: ::std::option::Option<::std::string::String>,
        ///Copy of the JSON message that was sent for processing
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sqs_message: ::std::option::Option<::std::string::String>,
        ///Start time
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub status: PipelineStatus,
        pub step: BnaPipelineStep,
    }

    impl ::std::convert::From<&BnaPipelinePatch> for BnaPipelinePatch {
        fn from(value: &BnaPipelinePatch) -> Self {
            value.clone()
        }
    }

    impl BnaPipelinePatch {
        pub fn builder() -> builder::BnaPipelinePatch {
            Default::default()
        }
    }

    ///`BnaPipelinePost`
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
    ///        "6.8941"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "end_time": {
    ///      "description": "End time",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "fargate_price_id": {
    ///      "description": "Fargate price identifier used to compute the cost",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "fargate_task_arn": {
    ///      "description": "ARN of the Fargate task that performed the
    /// analysis",
    ///      "examples": [
    ///        "arn:aws:ecs:us-west-2:123456789012:task/bna/
    /// 29f979fc9fca402d94b014aa23d2f6e0"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "s3_bucket": {
    ///      "description": "Path of the S3 bucket where the results were
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
    ///        {
    ///          "city": "santa rosa",
    ///          "country": "United States",
    ///          "fips_code": "3570670",
    ///          "region": "new mexico"
    ///        }
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "start_time": {
    ///      "description": "Start time",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "state_machine_id": {
    ///      "description": "Pipeline identifier\nThis is the ID of the AWS
    /// state machine that was used to run the pipeline",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BnaPipelinePost {
        ///Cost of an analysis in USD
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cost: ::std::option::Option<::std::string::String>,
        ///End time
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub end_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Fargate price identifier used to compute the cost
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fargate_price_id: ::std::option::Option<i32>,
        ///ARN of the Fargate task that performed the analysis
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fargate_task_arn: ::std::option::Option<::std::string::String>,
        ///Path of the S3 bucket where the results were stored
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub s3_bucket: ::std::option::Option<::std::string::String>,
        ///Copy of the JSON message that was sent for processing
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sqs_message: ::std::option::Option<::std::string::String>,
        ///Start time
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Pipeline identifier
        ///This is the ID of the AWS state machine that was used to run the
        /// pipeline
        pub state_machine_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&BnaPipelinePost> for BnaPipelinePost {
        fn from(value: &BnaPipelinePost) -> Self {
            value.clone()
        }
    }

    impl BnaPipelinePost {
        pub fn builder() -> builder::BnaPipelinePost {
            Default::default()
        }
    }

    ///`BnaPipelineStep`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum BnaPipelineStep {
        SqsMessage,
        Setup,
        Analysis,
        Cleanup,
    }

    impl ::std::convert::From<&Self> for BnaPipelineStep {
        fn from(value: &BnaPipelineStep) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for BnaPipelineStep {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SqsMessage => f.write_str("SqsMessage"),
                Self::Setup => f.write_str("Setup"),
                Self::Analysis => f.write_str("Analysis"),
                Self::Cleanup => f.write_str("Cleanup"),
            }
        }
    }

    impl ::std::str::FromStr for BnaPipelineStep {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "SqsMessage" => Ok(Self::SqsMessage),
                "Setup" => Ok(Self::Setup),
                "Analysis" => Ok(Self::Analysis),
                "Cleanup" => Ok(Self::Cleanup),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BnaPipelineStep {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BnaPipelineStep {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BnaPipelineStep {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`BnaPipelines`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/BnaPipeline"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct BnaPipelines(pub ::std::vec::Vec<BnaPipeline>);
    impl ::std::ops::Deref for BnaPipelines {
        type Target = ::std::vec::Vec<BnaPipeline>;
        fn deref(&self) -> &::std::vec::Vec<BnaPipeline> {
            &self.0
        }
    }

    impl ::std::convert::From<BnaPipelines> for ::std::vec::Vec<BnaPipeline> {
        fn from(value: BnaPipelines) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&BnaPipelines> for BnaPipelines {
        fn from(value: &BnaPipelines) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<BnaPipeline>> for BnaPipelines {
        fn from(value: ::std::vec::Vec<BnaPipeline>) -> Self {
            Self(value)
        }
    }

    ///`Cities`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/City"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Cities(pub ::std::vec::Vec<City>);
    impl ::std::ops::Deref for Cities {
        type Target = ::std::vec::Vec<City>;
        fn deref(&self) -> &::std::vec::Vec<City> {
            &self.0
        }
    }

    impl ::std::convert::From<Cities> for ::std::vec::Vec<City> {
        fn from(value: Cities) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Cities> for Cities {
        fn from(value: &Cities) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<City>> for Cities {
        fn from(value: ::std::vec::Vec<City>) -> Self {
            Self(value)
        }
    }

    ///`CitiesWithSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/CityWithSummary"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct CitiesWithSummary(pub ::std::vec::Vec<CityWithSummary>);
    impl ::std::ops::Deref for CitiesWithSummary {
        type Target = ::std::vec::Vec<CityWithSummary>;
        fn deref(&self) -> &::std::vec::Vec<CityWithSummary> {
            &self.0
        }
    }

    impl ::std::convert::From<CitiesWithSummary> for ::std::vec::Vec<CityWithSummary> {
        fn from(value: CitiesWithSummary) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&CitiesWithSummary> for CitiesWithSummary {
        fn from(value: &CitiesWithSummary) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<CityWithSummary>> for CitiesWithSummary {
        fn from(value: ::std::vec::Vec<CityWithSummary>) -> Self {
            Self(value)
        }
    }

    ///Detailed information of a city
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Detailed information of a city",
    ///  "type": "object",
    ///  "required": [
    ///    "country",
    ///    "created_at",
    ///    "id",
    ///    "name",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "country": {
    ///      "$ref": "#/components/schemas/Country"
    ///    },
    ///    "created_at": {
    ///      "description": "Creation date",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "description": "City identifier",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "latitude": {
    ///      "description": "Geographic coordinate that specifies the
    /// north-south position of a point\non the surface of the Earth.",
    ///      "examples": [
    ///        "51.260197"
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "longitude": {
    ///      "description": "Geographic coordinate that specifies the east–west
    /// position of a point\non the surface of the Earth.",
    ///      "examples": [
    ///        "4.402771"
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
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
    /// community, or\nsomething similar depending on the country. If a country
    /// does not have\nthis concept, then the country name is used.",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "residential_speed_limit": {
    ///      "description": "Residential speed limit in kilometer per hour
    /// (km/h).\nOnly use if different from the state speed limit.",
    ///      "examples": [
    ///        "50"
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
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
    /// character long",
    ///      "examples": [
    ///        "VAN"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "updated_at": {
    ///      "description": "Update date",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct City {
        pub country: Country,
        ///Creation date
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///City identifier
        pub id: ::uuid::Uuid,
        ///Geographic coordinate that specifies the north-south position of a
        /// point on the surface of the Earth.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latitude: ::std::option::Option<f64>,
        ///Geographic coordinate that specifies the east–west position of a
        /// point on the surface of the Earth.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub longitude: ::std::option::Option<f64>,
        ///City name
        pub name: ::std::string::String,
        ///Region name. A region can be a state, a province, a community, or
        ///something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
        ///Residential speed limit in kilometer per hour (km/h).
        ///Only use if different from the state speed limit.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub residential_speed_limit: ::std::option::Option<i32>,
        ///State name
        pub state: ::std::string::String,
        ///A short version of the state name, usually 2 or 3 character long
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state_abbrev: ::std::option::Option<::std::string::String>,
        ///Update date
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    }

    impl ::std::convert::From<&City> for City {
        fn from(value: &City) -> Self {
            value.clone()
        }
    }

    impl City {
        pub fn builder() -> builder::City {
            Default::default()
        }
    }

    ///`CityPost`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "country",
    ///    "name",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "country": {
    ///      "$ref": "#/components/schemas/Country"
    ///    },
    ///    "latitude": {
    ///      "description": "Geographic coordinate that specifies the
    /// north-south position of a point\non the surface of the Earth.",
    ///      "examples": [
    ///        "51.260197"
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "longitude": {
    ///      "description": "Geographic coordinate that specifies the east–west
    /// position of a point\non the surface of the Earth.",
    ///      "examples": [
    ///        "4.402771"
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
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
    /// community, or\nsomething similar depending on the country. If a country
    /// does not have\nthis concept, then the country name is used.",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "speed_limit": {
    ///      "description": "Speed limit in kilometer per hour (km/h).",
    ///      "examples": [
    ///        "50"
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "state": {
    ///      "description": "A short version of the state name, usually 2 or 3
    /// character long",
    ///      "examples": [
    ///        "VAN"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "state_abbrev": {
    ///      "description": "A short version of the state name, usually 2 or 3
    /// character long",
    ///      "examples": [
    ///        "VAN"
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CityPost {
        pub country: Country,
        ///Geographic coordinate that specifies the north-south position of a
        /// point on the surface of the Earth.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latitude: ::std::option::Option<f64>,
        ///Geographic coordinate that specifies the east–west position of a
        /// point on the surface of the Earth.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub longitude: ::std::option::Option<f64>,
        ///City name
        pub name: ::std::string::String,
        ///Region name. A region can be a state, a province, a community, or
        ///something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
        ///Speed limit in kilometer per hour (km/h).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub speed_limit: ::std::option::Option<i32>,
        ///A short version of the state name, usually 2 or 3 character long
        pub state: ::std::string::String,
        ///A short version of the state name, usually 2 or 3 character long
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state_abbrev: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&CityPost> for CityPost {
        fn from(value: &CityPost) -> Self {
            value.clone()
        }
    }

    impl CityPost {
        pub fn builder() -> builder::CityPost {
            Default::default()
        }
    }

    ///`CityRatings`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city",
    ///    "ratings"
    ///  ],
    ///  "properties": {
    ///    "city": {
    ///      "$ref": "#/components/schemas/City"
    ///    },
    ///    "ratings": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RatingSummary"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CityRatings {
        pub city: City,
        pub ratings: ::std::vec::Vec<RatingSummary>,
    }

    impl ::std::convert::From<&CityRatings> for CityRatings {
        fn from(value: &CityRatings) -> Self {
            value.clone()
        }
    }

    impl CityRatings {
        pub fn builder() -> builder::CityRatings {
            Default::default()
        }
    }

    ///`CityWithSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city",
    ///    "sumary"
    ///  ],
    ///  "properties": {
    ///    "city": {
    ///      "$ref": "#/components/schemas/City"
    ///    },
    ///    "sumary": {
    ///      "$ref": "#/components/schemas/RatingSummary"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CityWithSummary {
        pub city: City,
        pub sumary: RatingSummary,
    }

    impl ::std::convert::From<&CityWithSummary> for CityWithSummary {
        fn from(value: &CityWithSummary) -> Self {
            value.clone()
        }
    }

    impl CityWithSummary {
        pub fn builder() -> builder::CityWithSummary {
            Default::default()
        }
    }

    ///`CoreServices`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "dentists": {
    ///      "description": "BNA category subscore for access to dentists.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "doctors": {
    ///      "description": "BNA category subscore for access to doctors.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "grocery": {
    ///      "description": "BNA category subscore for access to grocery
    /// stores.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "hospitals": {
    ///      "description": "BNA category subscore for access to hospitals.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "pharmacies": {
    ///      "description": "BNA category subscore for access to pharmacies.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "score": {
    ///      "description": "BNA category score for access to core services.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "social_services": {
    ///      "description": "BNA category subscore for access to social
    /// services.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreServices {
        ///BNA category subscore for access to dentists.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dentists: ::std::option::Option<f64>,
        ///BNA category subscore for access to doctors.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub doctors: ::std::option::Option<f64>,
        ///BNA category subscore for access to grocery stores.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub grocery: ::std::option::Option<f64>,
        ///BNA category subscore for access to hospitals.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub hospitals: ::std::option::Option<f64>,
        ///BNA category subscore for access to pharmacies.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pharmacies: ::std::option::Option<f64>,
        ///BNA category score for access to core services.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub score: ::std::option::Option<f64>,
        ///BNA category subscore for access to social services.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub social_services: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&CoreServices> for CoreServices {
        fn from(value: &CoreServices) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for CoreServices {
        fn default() -> Self {
            Self {
                dentists: Default::default(),
                doctors: Default::default(),
                grocery: Default::default(),
                hospitals: Default::default(),
                pharmacies: Default::default(),
                score: Default::default(),
                social_services: Default::default(),
            }
        }
    }

    impl CoreServices {
        pub fn builder() -> builder::CoreServices {
            Default::default()
        }
    }

    ///`Country`
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
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
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

    impl ::std::convert::From<&Self> for Country {
        fn from(value: &Country) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for Country {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Australia => f.write_str("Australia"),
                Self::Belgium => f.write_str("Belgium"),
                Self::Brazil => f.write_str("Brazil"),
                Self::Canada => f.write_str("Canada"),
                Self::Chile => f.write_str("Chile"),
                Self::Colombia => f.write_str("Colombia"),
                Self::Croatia => f.write_str("Croatia"),
                Self::Cuba => f.write_str("Cuba"),
                Self::England => f.write_str("England"),
                Self::France => f.write_str("France"),
                Self::Germany => f.write_str("Germany"),
                Self::Greece => f.write_str("Greece"),
                Self::Guatemala => f.write_str("Guatemala"),
                Self::Iran => f.write_str("Iran"),
                Self::Iraq => f.write_str("Iraq"),
                Self::Ireland => f.write_str("Ireland"),
                Self::Italy => f.write_str("Italy"),
                Self::Mexico => f.write_str("Mexico"),
                Self::Netherlands => f.write_str("Netherlands"),
                Self::NewZealand => f.write_str("New Zealand"),
                Self::NorthernIreland => f.write_str("Northern Ireland"),
                Self::Portugal => f.write_str("Portugal"),
                Self::Scotland => f.write_str("Scotland"),
                Self::Spain => f.write_str("Spain"),
                Self::UnitedStates => f.write_str("United States"),
                Self::Vietnam => f.write_str("Vietnam"),
                Self::Wales => f.write_str("Wales"),
            }
        }
    }

    impl ::std::str::FromStr for Country {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
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

    impl ::std::convert::TryFrom<&str> for Country {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Country {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Country {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A Fargate price used to estimate the cost of an analysis
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A Fargate price used to estimate the cost of an
    /// analysis",
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "per_second"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "description": "Creation date",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "description": "Identifier of the Fargate Price rate used to
    /// compute the cost of the pipeline run",
    ///      "examples": [
    ///        "1"
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "per_second": {
    ///      "description": "Cost to run Fargate for 1 second",
    ///      "examples": [
    ///        "0.0023"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FargatePrice {
        ///Creation date
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Identifier of the Fargate Price rate used to compute the cost of the
        /// pipeline run
        pub id: i32,
        ///Cost to run Fargate for 1 second
        pub per_second: ::std::string::String,
    }

    impl ::std::convert::From<&FargatePrice> for FargatePrice {
        fn from(value: &FargatePrice) -> Self {
            value.clone()
        }
    }

    impl FargatePrice {
        pub fn builder() -> builder::FargatePrice {
            Default::default()
        }
    }

    ///A collection of Fargate prices.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A collection of Fargate prices.",
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/FargatePrice"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct FargatePrices(pub ::std::vec::Vec<FargatePrice>);
    impl ::std::ops::Deref for FargatePrices {
        type Target = ::std::vec::Vec<FargatePrice>;
        fn deref(&self) -> &::std::vec::Vec<FargatePrice> {
            &self.0
        }
    }

    impl ::std::convert::From<FargatePrices> for ::std::vec::Vec<FargatePrice> {
        fn from(value: FargatePrices) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&FargatePrices> for FargatePrices {
        fn from(value: &FargatePrices) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<FargatePrice>> for FargatePrices {
        fn from(value: ::std::vec::Vec<FargatePrice>) -> Self {
            Self(value)
        }
    }

    ///`Infrastructure`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "high_stress_miles": {
    ///      "description": "Total miles of high-stress streets in the measured
    /// area.",
    ///      "examples": [
    ///        253
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "low_stress_miles": {
    ///      "description": "Total miles of low-stress streets and paths in the
    /// measured area.",
    ///      "examples": [
    ///        127
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Infrastructure {
        ///Total miles of high-stress streets in the measured area.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub high_stress_miles: ::std::option::Option<f64>,
        ///Total miles of low-stress streets and paths in the measured area.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub low_stress_miles: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&Infrastructure> for Infrastructure {
        fn from(value: &Infrastructure) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for Infrastructure {
        fn default() -> Self {
            Self {
                high_stress_miles: Default::default(),
                low_stress_miles: Default::default(),
            }
        }
    }

    impl Infrastructure {
        pub fn builder() -> builder::Infrastructure {
            Default::default()
        }
    }

    ///`Measure`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "buffered_lane": {
    ///      "description": "Miles of buffered bike lanes.",
    ///      "examples": [
    ///        53.859
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "lane": {
    ///      "description": "Miles of bike lanes.",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "path": {
    ///      "description": "Miles of off-street paths.",
    ///      "examples": [
    ///        53.859
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "sharrow": {
    ///      "description": "Miles of sharrows.",
    ///      "examples": [
    ///        53.859
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "track": {
    ///      "description": "Miles of tracks.",
    ///      "examples": [
    ///        53.859
    ///      ],
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Measure {
        ///Miles of buffered bike lanes.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub buffered_lane: ::std::option::Option<f64>,
        ///Miles of bike lanes.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lane: ::std::option::Option<f64>,
        ///Miles of off-street paths.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<f64>,
        ///Miles of sharrows.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sharrow: ::std::option::Option<f64>,
        ///Miles of tracks.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub track: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&Measure> for Measure {
        fn from(value: &Measure) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for Measure {
        fn default() -> Self {
            Self {
                buffered_lane: Default::default(),
                lane: Default::default(),
                path: Default::default(),
                sharrow: Default::default(),
                track: Default::default(),
            }
        }
    }

    impl Measure {
        pub fn builder() -> builder::Measure {
            Default::default()
        }
    }

    ///`Opportunity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "employment": {
    ///      "description": "BNA category subscore for access to job location
    /// areas.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "higher_education": {
    ///      "description": "BNA category subscore for access to universities
    /// and colleges.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "k12_education": {
    ///      "description": "BNA category subscore for access to k12 schools",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "score": {
    ///      "description": "BNA category score for access to education and
    /// jobs.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "technical_vocational_college": {
    ///      "description": "BNA category subscore for access to technical and
    /// vocational colleges.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Opportunity {
        ///BNA category subscore for access to job location areas.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub employment: ::std::option::Option<f64>,
        ///BNA category subscore for access to universities and colleges.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub higher_education: ::std::option::Option<f64>,
        ///BNA category subscore for access to k12 schools
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub k12_education: ::std::option::Option<f64>,
        ///BNA category score for access to education and jobs.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub score: ::std::option::Option<f64>,
        ///BNA category subscore for access to technical and vocational
        /// colleges.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub technical_vocational_college: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&Opportunity> for Opportunity {
        fn from(value: &Opportunity) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for Opportunity {
        fn default() -> Self {
            Self {
                employment: Default::default(),
                higher_education: Default::default(),
                k12_education: Default::default(),
                score: Default::default(),
                technical_vocational_college: Default::default(),
            }
        }
    }

    impl Opportunity {
        pub fn builder() -> builder::Opportunity {
            Default::default()
        }
    }

    ///`People`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "people": {
    ///      "description": "BNA category score for access to residential
    /// areas.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct People {
        ///BNA category score for access to residential areas.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub people: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&People> for People {
        fn from(value: &People) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for People {
        fn default() -> Self {
            Self {
                people: Default::default(),
            }
        }
    }

    impl People {
        pub fn builder() -> builder::People {
            Default::default()
        }
    }

    ///`PipelineStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "Completed",
    ///    "Pending",
    ///    "Processing"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PipelineStatus {
        Completed,
        Pending,
        Processing,
    }

    impl ::std::convert::From<&Self> for PipelineStatus {
        fn from(value: &PipelineStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for PipelineStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Completed => f.write_str("Completed"),
                Self::Pending => f.write_str("Pending"),
                Self::Processing => f.write_str("Processing"),
            }
        }
    }

    impl ::std::str::FromStr for PipelineStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Completed" => Ok(Self::Completed),
                "Pending" => Ok(Self::Pending),
                "Processing" => Ok(Self::Processing),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PipelineStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PipelineStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PipelineStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Rating`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city_id",
    ///    "core_services",
    ///    "id",
    ///    "infrastructure",
    ///    "measure",
    ///    "opportunity",
    ///    "people",
    ///    "recreation",
    ///    "retail",
    ///    "score",
    ///    "transit",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "city_id": {
    ///      "description": "City identifier",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "core_services": {
    ///      "$ref": "#/components/schemas/CoreServices"
    ///    },
    ///    "id": {
    ///      "description": "Rating identifier",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "infrastructure": {
    ///      "$ref": "#/components/schemas/Infrastructure"
    ///    },
    ///    "measure": {
    ///      "$ref": "#/components/schemas/Measure"
    ///    },
    ///    "opportunity": {
    ///      "$ref": "#/components/schemas/Opportunity"
    ///    },
    ///    "people": {
    ///      "$ref": "#/components/schemas/People"
    ///    },
    ///    "recreation": {
    ///      "$ref": "#/components/schemas/Recreation"
    ///    },
    ///    "retail": {
    ///      "$ref": "#/components/schemas/Retail"
    ///    },
    ///    "score": {
    ///      "description": "Total rating score",
    ///      "type": "number",
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "transit": {
    ///      "$ref": "#/components/schemas/Transit"
    ///    },
    ///    "version": {
    ///      "description": "Rating version\nThe format follows the [calver](https://calver.org) specification with\nthe YY.0M[.Minor] scheme.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Rating {
        ///City identifier
        pub city_id: ::uuid::Uuid,
        pub core_services: CoreServices,
        ///Rating identifier
        pub id: ::uuid::Uuid,
        pub infrastructure: Infrastructure,
        pub measure: Measure,
        pub opportunity: Opportunity,
        pub people: People,
        pub recreation: Recreation,
        pub retail: Retail,
        pub score: f64,
        pub transit: Transit,
        ///Rating version
        ///The format follows the [calver](https://calver.org) specification with
        ///the YY.0M[.Minor] scheme.
        pub version: ::std::string::String,
    }

    impl ::std::convert::From<&Rating> for Rating {
        fn from(value: &Rating) -> Self {
            value.clone()
        }
    }

    impl Rating {
        pub fn builder() -> builder::Rating {
            Default::default()
        }
    }

    ///`RatingPost`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city_id",
    ///    "core_services",
    ///    "infrastructure",
    ///    "measure",
    ///    "opportunity",
    ///    "people",
    ///    "pop_size",
    ///    "population",
    ///    "recreation",
    ///    "retail",
    ///    "score",
    ///    "transit",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "city_id": {
    ///      "description": "City identifier",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "core_services": {
    ///      "$ref": "#/components/schemas/CoreServices"
    ///    },
    ///    "infrastructure": {
    ///      "$ref": "#/components/schemas/Infrastructure"
    ///    },
    ///    "measure": {
    ///      "$ref": "#/components/schemas/Measure"
    ///    },
    ///    "opportunity": {
    ///      "$ref": "#/components/schemas/Opportunity"
    ///    },
    ///    "people": {
    ///      "$ref": "#/components/schemas/People"
    ///    },
    ///    "pop_size": {
    ///      "description": "City population size category (small, medium,
    /// large).",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "population": {
    ///      "description": "City population based on the annual U.S. Census
    /// American Community Survey.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "recreation": {
    ///      "$ref": "#/components/schemas/Recreation"
    ///    },
    ///    "retail": {
    ///      "$ref": "#/components/schemas/Retail"
    ///    },
    ///    "score": {
    ///      "description": "City rating score",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "speed_limit_override": {
    ///      "description": "Residential speed limit, if any.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "transit": {
    ///      "$ref": "#/components/schemas/Transit"
    ///    },
    ///    "version": {
    ///      "description": "Rating version\nThe format follows the [calver](https://calver.org) specification with\nthe YY.0M[.Minor] scheme.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RatingPost {
        ///City identifier
        pub city_id: ::uuid::Uuid,
        pub core_services: CoreServices,
        pub infrastructure: Infrastructure,
        pub measure: Measure,
        pub opportunity: Opportunity,
        pub people: People,
        ///City population size category (small, medium, large).
        pub pop_size: i32,
        ///City population based on the annual U.S. Census American Community
        /// Survey.
        pub population: i32,
        pub recreation: Recreation,
        pub retail: Retail,
        pub score: f64,
        ///Residential speed limit, if any.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub speed_limit_override: ::std::option::Option<i32>,
        pub transit: Transit,
        ///Rating version
        ///The format follows the [calver](https://calver.org) specification with
        ///the YY.0M[.Minor] scheme.
        pub version: ::std::string::String,
    }

    impl ::std::convert::From<&RatingPost> for RatingPost {
        fn from(value: &RatingPost) -> Self {
            value.clone()
        }
    }

    impl RatingPost {
        pub fn builder() -> builder::RatingPost {
            Default::default()
        }
    }

    ///`RatingSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city_id",
    ///    "created_at",
    ///    "id",
    ///    "pop_size",
    ///    "population",
    ///    "score",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "city_id": {
    ///      "description": "City identifier",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "created_at": {
    ///      "description": "Creation date",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "description": "Analysis identifier",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "pop_size": {
    ///      "description": "City population size category (small, medium,
    /// large).",
    ///      "examples": [
    ///        "large"
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "population": {
    ///      "description": "City population based on the annual U.S. Census
    /// American Community Survey.",
    ///      "examples": [
    ///        "989252"
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "residential_speed_limit_override": {
    ///      "description": "Residential speed limit override.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "score": {
    ///      "description": "BNA score",
    ///      "examples": [
    ///        "77.0"
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "version": {
    ///      "description": "Analysis version. The format follows the [calver](https://calver.org)\nspecification with the YY.0M[.Minor] scheme.",
    ///      "examples": [
    ///        "23.12"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RatingSummary {
        ///City identifier
        pub city_id: ::uuid::Uuid,
        ///Creation date
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Analysis identifier
        pub id: ::uuid::Uuid,
        ///City population size category (small, medium, large).
        pub pop_size: i32,
        ///City population based on the annual U.S. Census American Community
        /// Survey.
        pub population: i32,
        ///Residential speed limit override.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub residential_speed_limit_override: ::std::option::Option<i32>,
        pub score: f64,
        ///Analysis version. The format follows the [calver](https://calver.org)
        ///specification with the YY.0M[.Minor] scheme.
        pub version: ::std::string::String,
    }

    impl ::std::convert::From<&RatingSummary> for RatingSummary {
        fn from(value: &RatingSummary) -> Self {
            value.clone()
        }
    }

    impl RatingSummary {
        pub fn builder() -> builder::RatingSummary {
            Default::default()
        }
    }

    ///`RatingWithCity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "city",
    ///    "rating"
    ///  ],
    ///  "properties": {
    ///    "city": {
    ///      "$ref": "#/components/schemas/City"
    ///    },
    ///    "rating": {
    ///      "$ref": "#/components/schemas/Rating"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RatingWithCity {
        pub city: City,
        pub rating: Rating,
    }

    impl ::std::convert::From<&RatingWithCity> for RatingWithCity {
        fn from(value: &RatingWithCity) -> Self {
            value.clone()
        }
    }

    impl RatingWithCity {
        pub fn builder() -> builder::RatingWithCity {
            Default::default()
        }
    }

    ///`Ratings`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/Rating"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Ratings(pub ::std::vec::Vec<Rating>);
    impl ::std::ops::Deref for Ratings {
        type Target = ::std::vec::Vec<Rating>;
        fn deref(&self) -> &::std::vec::Vec<Rating> {
            &self.0
        }
    }

    impl ::std::convert::From<Ratings> for ::std::vec::Vec<Rating> {
        fn from(value: Ratings) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Ratings> for Ratings {
        fn from(value: &Ratings) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<Rating>> for Ratings {
        fn from(value: ::std::vec::Vec<Rating>) -> Self {
            Self(value)
        }
    }

    ///`Recreation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "community_centers": {
    ///      "description": "BNA category subscore for access to community
    /// centers.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "parks": {
    ///      "description": "BNA category subscore for access to parks.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "score": {
    ///      "description": "BNA category score for access to recreational
    /// facilities.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    },
    ///    "trails": {
    ///      "description": "BNA category subscore for access to bikeable
    /// trails.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Recreation {
        ///BNA category subscore for access to community centers.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub community_centers: ::std::option::Option<f64>,
        ///BNA category subscore for access to parks.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parks: ::std::option::Option<f64>,
        ///BNA category score for access to recreational facilities.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub score: ::std::option::Option<f64>,
        ///BNA category subscore for access to bikeable trails.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub trails: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&Recreation> for Recreation {
        fn from(value: &Recreation) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for Recreation {
        fn default() -> Self {
            Self {
                community_centers: Default::default(),
                parks: Default::default(),
                score: Default::default(),
                trails: Default::default(),
            }
        }
    }

    impl Recreation {
        pub fn builder() -> builder::Recreation {
            Default::default()
        }
    }

    ///`Retail`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "retail": {
    ///      "description": "BNA category score for access to major retail
    /// centers.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Retail {
        ///BNA category score for access to major retail centers.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub retail: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&Retail> for Retail {
        fn from(value: &Retail) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for Retail {
        fn default() -> Self {
            Self {
                retail: Default::default(),
            }
        }
    }

    impl Retail {
        pub fn builder() -> builder::Retail {
            Default::default()
        }
    }

    ///`Submission`
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
    ///    "created_at",
    ///    "email",
    ///    "fips_code",
    ///    "first_name",
    ///    "id",
    ///    "last_name",
    ///    "status"
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
    ///        "true"
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/Country"
    ///    },
    ///    "created_at": {
    ///      "description": "Creation date",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "email": {
    ///      "description": "email address",
    ///      "examples": [
    ///        "jane.doe@orgllc.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities",
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
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "last_name": {
    ///      "description": "Last name",
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
    ///      "description": "Organization or company",
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
    /// community, or\nsomething similar depending on the country. If a country
    /// does not have\nthis concept, then the country name is used.",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "status": {
    ///      "description": "Submission status, e.g. \"Pending\"",
    ///      "examples": [
    ///        "Pending"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Submission {
        ///City name
        pub city: ::std::string::String,
        ///Consent status
        pub consent: bool,
        pub country: Country,
        ///Creation date
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///email address
        pub email: ::std::string::String,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        pub fips_code: ::std::string::String,
        ///First name
        pub first_name: ::std::string::String,
        ///Submission identifier
        pub id: i32,
        ///Last name
        pub last_name: ::std::string::String,
        ///Job title or position
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub occupation: ::std::option::Option<::std::string::String>,
        ///Organization or company
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub organization: ::std::option::Option<::std::string::String>,
        ///Region name. A region can be a state, a province, a community, or
        ///something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
        ///Submission status, e.g. "Pending"
        pub status: ::std::string::String,
    }

    impl ::std::convert::From<&Submission> for Submission {
        fn from(value: &Submission) -> Self {
            value.clone()
        }
    }

    impl Submission {
        pub fn builder() -> builder::Submission {
            Default::default()
        }
    }

    ///`SubmissionPatch`
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
    ///    "last_name",
    ///    "status"
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
    ///        "true"
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/Country"
    ///    },
    ///    "email": {
    ///      "description": "email address",
    ///      "examples": [
    ///        "jane.doe@orgllc.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities",
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
    ///      "description": "Organization or company",
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
    /// community, or\nsomething similar depending on the country. If a country
    /// does not have\nthis concept, then the country name is used.",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "status": {
    ///      "description": "Submission status, e.g. \"Pending\"",
    ///      "examples": [
    ///        "Pending"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SubmissionPatch {
        ///City name
        pub city: ::std::string::String,
        ///Consent status
        pub consent: bool,
        pub country: Country,
        ///email address
        pub email: ::std::string::String,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        pub fips_code: ::std::string::String,
        ///First name
        pub first_name: ::std::string::String,
        ///Last name
        pub last_name: ::std::string::String,
        ///Job title or position
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub occupation: ::std::option::Option<::std::string::String>,
        ///Organization or company
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub organization: ::std::option::Option<::std::string::String>,
        ///Region name. A region can be a state, a province, a community, or
        ///something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
        ///Submission status, e.g. "Pending"
        pub status: ::std::string::String,
    }

    impl ::std::convert::From<&SubmissionPatch> for SubmissionPatch {
        fn from(value: &SubmissionPatch) -> Self {
            value.clone()
        }
    }

    impl SubmissionPatch {
        pub fn builder() -> builder::SubmissionPatch {
            Default::default()
        }
    }

    ///`SubmissionPost`
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
    ///    "last_name",
    ///    "status"
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
    ///        "true"
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "country": {
    ///      "$ref": "#/components/schemas/Country"
    ///    },
    ///    "email": {
    ///      "description": "email address",
    ///      "examples": [
    ///        "jane.doe@orgllc.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "fips_code": {
    ///      "description": "Numerical city identifier given by the U.S. census,
    /// or 0 for non-US cities",
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
    ///      "description": "Organization or company",
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
    /// community, or\nsomething similar depending on the country. If a country
    /// does not have\nthis concept, then the country name is used.",
    ///      "examples": [
    ///        "Antwerp"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "status": {
    ///      "description": "Submission status, e.g. \"Pending\"",
    ///      "examples": [
    ///        "Pending"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SubmissionPost {
        ///City name
        pub city: ::std::string::String,
        ///Consent status
        pub consent: bool,
        pub country: Country,
        ///email address
        pub email: ::std::string::String,
        ///Numerical city identifier given by the U.S. census, or 0 for non-US
        /// cities
        pub fips_code: ::std::string::String,
        ///First name
        pub first_name: ::std::string::String,
        ///Last name
        pub last_name: ::std::string::String,
        ///Job title or position
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub occupation: ::std::option::Option<::std::string::String>,
        ///Organization or company
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub organization: ::std::option::Option<::std::string::String>,
        ///Region name. A region can be a state, a province, a community, or
        ///something similar depending on the country. If a country does not
        /// have this concept, then the country name is used.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
        ///Submission status, e.g. "Pending"
        pub status: ::std::string::String,
    }

    impl ::std::convert::From<&SubmissionPost> for SubmissionPost {
        fn from(value: &SubmissionPost) -> Self {
            value.clone()
        }
    }

    impl SubmissionPost {
        pub fn builder() -> builder::SubmissionPost {
            Default::default()
        }
    }

    ///`Submissions`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/Submission"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Submissions(pub ::std::vec::Vec<Submission>);
    impl ::std::ops::Deref for Submissions {
        type Target = ::std::vec::Vec<Submission>;
        fn deref(&self) -> &::std::vec::Vec<Submission> {
            &self.0
        }
    }

    impl ::std::convert::From<Submissions> for ::std::vec::Vec<Submission> {
        fn from(value: Submissions) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Submissions> for Submissions {
        fn from(value: &Submissions) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<Submission>> for Submissions {
        fn from(value: ::std::vec::Vec<Submission>) -> Self {
            Self(value)
        }
    }

    ///`Transit`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "transit": {
    ///      "description": "BNA category score for access to major transit
    /// stops.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double",
    ///      "maximum": 100.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Transit {
        ///BNA category score for access to major transit stops.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub transit: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&Transit> for Transit {
        fn from(value: &Transit) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for Transit {
        fn default() -> Self {
            Self {
                transit: Default::default(),
            }
        }
    }

    impl Transit {
        pub fn builder() -> builder::Transit {
            Default::default()
        }
    }

    ///Detailed information of a city
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Detailed information of a city",
    ///  "type": "object",
    ///  "required": [
    ///    "abbrev",
    ///    "fipscode",
    ///    "name",
    ///    "speed_limit"
    ///  ],
    ///  "properties": {
    ///    "abbrev": {
    ///      "description": "Two-letter state abbreviation..",
    ///      "examples": [
    ///        "TX"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "fipscode": {
    ///      "description": "State FIPS code.",
    ///      "examples": [
    ///        "48"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "State name.",
    ///      "examples": [
    ///        "Texas"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "speed_limit": {
    ///      "description": "State speed limit in mph.",
    ///      "examples": [
    ///        "30"
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UsState {
        ///Two-letter state abbreviation..
        pub abbrev: ::std::string::String,
        ///State FIPS code.
        pub fipscode: ::std::string::String,
        ///State name.
        pub name: ::std::string::String,
        ///State speed limit in mph.
        pub speed_limit: i32,
    }

    impl ::std::convert::From<&UsState> for UsState {
        fn from(value: &UsState) -> Self {
            value.clone()
        }
    }

    impl UsState {
        pub fn builder() -> builder::UsState {
            Default::default()
        }
    }

    ///`UsStates`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/UsState"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct UsStates(pub ::std::vec::Vec<UsState>);
    impl ::std::ops::Deref for UsStates {
        type Target = ::std::vec::Vec<UsState>;
        fn deref(&self) -> &::std::vec::Vec<UsState> {
            &self.0
        }
    }

    impl ::std::convert::From<UsStates> for ::std::vec::Vec<UsState> {
        fn from(value: UsStates) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&UsStates> for UsStates {
        fn from(value: &UsStates) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<UsState>> for UsStates {
        fn from(value: ::std::vec::Vec<UsState>) -> Self {
            Self(value)
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct ApiError {
            details: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            source: ::std::result::Result<super::ApiErrorSource, ::std::string::String>,
            status: ::std::result::Result<::std::string::String, ::std::string::String>,
            title: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for ApiError {
            fn default() -> Self {
                Self {
                    details: Err("no value supplied for details".to_string()),
                    id: Ok(Default::default()),
                    source: Err("no value supplied for source".to_string()),
                    status: Err("no value supplied for status".to_string()),
                    title: Err("no value supplied for title".to_string()),
                }
            }
        }

        impl ApiError {
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn source<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ApiErrorSource>,
                T::Error: ::std::fmt::Display,
            {
                self.source = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiError> for super::ApiError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    details: value.details?,
                    id: value.id?,
                    source: value.source?,
                    status: value.status?,
                    title: value.title?,
                })
            }
        }

        impl ::std::convert::From<super::ApiError> for ApiError {
            fn from(value: super::ApiError) -> Self {
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
        pub struct ApiErrors {
            errors: ::std::result::Result<::std::vec::Vec<super::ApiError>, ::std::string::String>,
        }

        impl ::std::default::Default for ApiErrors {
            fn default() -> Self {
                Self {
                    errors: Err("no value supplied for errors".to_string()),
                }
            }
        }

        impl ApiErrors {
            pub fn errors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ApiError>>,
                T::Error: ::std::fmt::Display,
            {
                self.errors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for errors: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiErrors> for super::ApiErrors {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiErrors,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    errors: value.errors?,
                })
            }
        }

        impl ::std::convert::From<super::ApiErrors> for ApiErrors {
            fn from(value: super::ApiErrors) -> Self {
                Self {
                    errors: Ok(value.errors),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct BnaPipeline {
            cost: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            end_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            fargate_price_id:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            fargate_task_arn: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            s3_bucket: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sqs_message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            start_time: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            state_machine_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            status: ::std::result::Result<super::PipelineStatus, ::std::string::String>,
            step: ::std::result::Result<super::BnaPipelineStep, ::std::string::String>,
        }

        impl ::std::default::Default for BnaPipeline {
            fn default() -> Self {
                Self {
                    cost: Ok(Default::default()),
                    end_time: Ok(Default::default()),
                    fargate_price_id: Ok(Default::default()),
                    fargate_task_arn: Ok(Default::default()),
                    s3_bucket: Ok(Default::default()),
                    sqs_message: Ok(Default::default()),
                    start_time: Err("no value supplied for start_time".to_string()),
                    state_machine_id: Err("no value supplied for state_machine_id".to_string()),
                    status: Err("no value supplied for status".to_string()),
                    step: Err("no value supplied for step".to_string()),
                }
            }
        }

        impl BnaPipeline {
            pub fn cost<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cost = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cost: {}", e));
                self
            }
            pub fn end_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.end_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end_time: {}", e));
                self
            }
            pub fn fargate_price_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.fargate_price_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_price_id: {}",
                        e
                    )
                });
                self
            }
            pub fn fargate_task_arn<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.fargate_task_arn = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_task_arn: {}",
                        e
                    )
                });
                self
            }
            pub fn s3_bucket<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.s3_bucket = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for s3_bucket: {}", e));
                self
            }
            pub fn sqs_message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sqs_message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sqs_message: {}", e));
                self
            }
            pub fn start_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.start_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_time: {}", e));
                self
            }
            pub fn state_machine_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.state_machine_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for state_machine_id: {}",
                        e
                    )
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PipelineStatus>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn step<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::BnaPipelineStep>,
                T::Error: ::std::fmt::Display,
            {
                self.step = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for step: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<BnaPipeline> for super::BnaPipeline {
            type Error = super::error::ConversionError;
            fn try_from(
                value: BnaPipeline,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cost: value.cost?,
                    end_time: value.end_time?,
                    fargate_price_id: value.fargate_price_id?,
                    fargate_task_arn: value.fargate_task_arn?,
                    s3_bucket: value.s3_bucket?,
                    sqs_message: value.sqs_message?,
                    start_time: value.start_time?,
                    state_machine_id: value.state_machine_id?,
                    status: value.status?,
                    step: value.step?,
                })
            }
        }

        impl ::std::convert::From<super::BnaPipeline> for BnaPipeline {
            fn from(value: super::BnaPipeline) -> Self {
                Self {
                    cost: Ok(value.cost),
                    end_time: Ok(value.end_time),
                    fargate_price_id: Ok(value.fargate_price_id),
                    fargate_task_arn: Ok(value.fargate_task_arn),
                    s3_bucket: Ok(value.s3_bucket),
                    sqs_message: Ok(value.sqs_message),
                    start_time: Ok(value.start_time),
                    state_machine_id: Ok(value.state_machine_id),
                    status: Ok(value.status),
                    step: Ok(value.step),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct BnaPipelinePatch {
            cost: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            end_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            fargate_price_id:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            fargate_task_arn: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            s3_bucket: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sqs_message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            start_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            status: ::std::result::Result<super::PipelineStatus, ::std::string::String>,
            step: ::std::result::Result<super::BnaPipelineStep, ::std::string::String>,
        }

        impl ::std::default::Default for BnaPipelinePatch {
            fn default() -> Self {
                Self {
                    cost: Ok(Default::default()),
                    end_time: Ok(Default::default()),
                    fargate_price_id: Ok(Default::default()),
                    fargate_task_arn: Ok(Default::default()),
                    s3_bucket: Ok(Default::default()),
                    sqs_message: Ok(Default::default()),
                    start_time: Ok(Default::default()),
                    status: Err("no value supplied for status".to_string()),
                    step: Err("no value supplied for step".to_string()),
                }
            }
        }

        impl BnaPipelinePatch {
            pub fn cost<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cost = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cost: {}", e));
                self
            }
            pub fn end_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.end_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end_time: {}", e));
                self
            }
            pub fn fargate_price_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.fargate_price_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_price_id: {}",
                        e
                    )
                });
                self
            }
            pub fn fargate_task_arn<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.fargate_task_arn = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_task_arn: {}",
                        e
                    )
                });
                self
            }
            pub fn s3_bucket<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.s3_bucket = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for s3_bucket: {}", e));
                self
            }
            pub fn sqs_message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sqs_message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sqs_message: {}", e));
                self
            }
            pub fn start_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.start_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_time: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PipelineStatus>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn step<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::BnaPipelineStep>,
                T::Error: ::std::fmt::Display,
            {
                self.step = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for step: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<BnaPipelinePatch> for super::BnaPipelinePatch {
            type Error = super::error::ConversionError;
            fn try_from(
                value: BnaPipelinePatch,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cost: value.cost?,
                    end_time: value.end_time?,
                    fargate_price_id: value.fargate_price_id?,
                    fargate_task_arn: value.fargate_task_arn?,
                    s3_bucket: value.s3_bucket?,
                    sqs_message: value.sqs_message?,
                    start_time: value.start_time?,
                    status: value.status?,
                    step: value.step?,
                })
            }
        }

        impl ::std::convert::From<super::BnaPipelinePatch> for BnaPipelinePatch {
            fn from(value: super::BnaPipelinePatch) -> Self {
                Self {
                    cost: Ok(value.cost),
                    end_time: Ok(value.end_time),
                    fargate_price_id: Ok(value.fargate_price_id),
                    fargate_task_arn: Ok(value.fargate_task_arn),
                    s3_bucket: Ok(value.s3_bucket),
                    sqs_message: Ok(value.sqs_message),
                    start_time: Ok(value.start_time),
                    status: Ok(value.status),
                    step: Ok(value.step),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct BnaPipelinePost {
            cost: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            end_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            fargate_price_id:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            fargate_task_arn: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            s3_bucket: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sqs_message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            start_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            state_machine_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        }

        impl ::std::default::Default for BnaPipelinePost {
            fn default() -> Self {
                Self {
                    cost: Ok(Default::default()),
                    end_time: Ok(Default::default()),
                    fargate_price_id: Ok(Default::default()),
                    fargate_task_arn: Ok(Default::default()),
                    s3_bucket: Ok(Default::default()),
                    sqs_message: Ok(Default::default()),
                    start_time: Ok(Default::default()),
                    state_machine_id: Err("no value supplied for state_machine_id".to_string()),
                }
            }
        }

        impl BnaPipelinePost {
            pub fn cost<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cost = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cost: {}", e));
                self
            }
            pub fn end_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.end_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end_time: {}", e));
                self
            }
            pub fn fargate_price_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.fargate_price_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_price_id: {}",
                        e
                    )
                });
                self
            }
            pub fn fargate_task_arn<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.fargate_task_arn = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fargate_task_arn: {}",
                        e
                    )
                });
                self
            }
            pub fn s3_bucket<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.s3_bucket = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for s3_bucket: {}", e));
                self
            }
            pub fn sqs_message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sqs_message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sqs_message: {}", e));
                self
            }
            pub fn start_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.start_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_time: {}", e));
                self
            }
            pub fn state_machine_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.state_machine_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for state_machine_id: {}",
                        e
                    )
                });
                self
            }
        }

        impl ::std::convert::TryFrom<BnaPipelinePost> for super::BnaPipelinePost {
            type Error = super::error::ConversionError;
            fn try_from(
                value: BnaPipelinePost,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cost: value.cost?,
                    end_time: value.end_time?,
                    fargate_price_id: value.fargate_price_id?,
                    fargate_task_arn: value.fargate_task_arn?,
                    s3_bucket: value.s3_bucket?,
                    sqs_message: value.sqs_message?,
                    start_time: value.start_time?,
                    state_machine_id: value.state_machine_id?,
                })
            }
        }

        impl ::std::convert::From<super::BnaPipelinePost> for BnaPipelinePost {
            fn from(value: super::BnaPipelinePost) -> Self {
                Self {
                    cost: Ok(value.cost),
                    end_time: Ok(value.end_time),
                    fargate_price_id: Ok(value.fargate_price_id),
                    fargate_task_arn: Ok(value.fargate_task_arn),
                    s3_bucket: Ok(value.s3_bucket),
                    sqs_message: Ok(value.sqs_message),
                    start_time: Ok(value.start_time),
                    state_machine_id: Ok(value.state_machine_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct City {
            country: ::std::result::Result<super::Country, ::std::string::String>,
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            latitude: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            longitude: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            region: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            residential_speed_limit:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            state: ::std::result::Result<::std::string::String, ::std::string::String>,
            state_abbrev: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for City {
            fn default() -> Self {
                Self {
                    country: Err("no value supplied for country".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    latitude: Ok(Default::default()),
                    longitude: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    region: Ok(Default::default()),
                    residential_speed_limit: Ok(Default::default()),
                    state: Err("no value supplied for state".to_string()),
                    state_abbrev: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                }
            }
        }

        impl City {
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Country>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn latitude<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.latitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for latitude: {}", e));
                self
            }
            pub fn longitude<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.longitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for longitude: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn residential_speed_limit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.residential_speed_limit = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for residential_speed_limit: {}",
                        e
                    )
                });
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn state_abbrev<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.state_abbrev = value.try_into().map_err(|e| {
                    format!("error converting supplied value for state_abbrev: {}", e)
                });
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<City> for super::City {
            type Error = super::error::ConversionError;
            fn try_from(value: City) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    country: value.country?,
                    created_at: value.created_at?,
                    id: value.id?,
                    latitude: value.latitude?,
                    longitude: value.longitude?,
                    name: value.name?,
                    region: value.region?,
                    residential_speed_limit: value.residential_speed_limit?,
                    state: value.state?,
                    state_abbrev: value.state_abbrev?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl ::std::convert::From<super::City> for City {
            fn from(value: super::City) -> Self {
                Self {
                    country: Ok(value.country),
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    latitude: Ok(value.latitude),
                    longitude: Ok(value.longitude),
                    name: Ok(value.name),
                    region: Ok(value.region),
                    residential_speed_limit: Ok(value.residential_speed_limit),
                    state: Ok(value.state),
                    state_abbrev: Ok(value.state_abbrev),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CityPost {
            country: ::std::result::Result<super::Country, ::std::string::String>,
            latitude: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            longitude: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            region: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            speed_limit: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            state: ::std::result::Result<::std::string::String, ::std::string::String>,
            state_abbrev: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for CityPost {
            fn default() -> Self {
                Self {
                    country: Err("no value supplied for country".to_string()),
                    latitude: Ok(Default::default()),
                    longitude: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    region: Ok(Default::default()),
                    speed_limit: Ok(Default::default()),
                    state: Err("no value supplied for state".to_string()),
                    state_abbrev: Ok(Default::default()),
                }
            }
        }

        impl CityPost {
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Country>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn latitude<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.latitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for latitude: {}", e));
                self
            }
            pub fn longitude<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.longitude = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for longitude: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn speed_limit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.speed_limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for speed_limit: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn state_abbrev<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.state_abbrev = value.try_into().map_err(|e| {
                    format!("error converting supplied value for state_abbrev: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<CityPost> for super::CityPost {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CityPost,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    country: value.country?,
                    latitude: value.latitude?,
                    longitude: value.longitude?,
                    name: value.name?,
                    region: value.region?,
                    speed_limit: value.speed_limit?,
                    state: value.state?,
                    state_abbrev: value.state_abbrev?,
                })
            }
        }

        impl ::std::convert::From<super::CityPost> for CityPost {
            fn from(value: super::CityPost) -> Self {
                Self {
                    country: Ok(value.country),
                    latitude: Ok(value.latitude),
                    longitude: Ok(value.longitude),
                    name: Ok(value.name),
                    region: Ok(value.region),
                    speed_limit: Ok(value.speed_limit),
                    state: Ok(value.state),
                    state_abbrev: Ok(value.state_abbrev),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CityRatings {
            city: ::std::result::Result<super::City, ::std::string::String>,
            ratings:
                ::std::result::Result<::std::vec::Vec<super::RatingSummary>, ::std::string::String>,
        }

        impl ::std::default::Default for CityRatings {
            fn default() -> Self {
                Self {
                    city: Err("no value supplied for city".to_string()),
                    ratings: Err("no value supplied for ratings".to_string()),
                }
            }
        }

        impl CityRatings {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::City>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn ratings<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::RatingSummary>>,
                T::Error: ::std::fmt::Display,
            {
                self.ratings = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ratings: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<CityRatings> for super::CityRatings {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CityRatings,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city: value.city?,
                    ratings: value.ratings?,
                })
            }
        }

        impl ::std::convert::From<super::CityRatings> for CityRatings {
            fn from(value: super::CityRatings) -> Self {
                Self {
                    city: Ok(value.city),
                    ratings: Ok(value.ratings),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CityWithSummary {
            city: ::std::result::Result<super::City, ::std::string::String>,
            sumary: ::std::result::Result<super::RatingSummary, ::std::string::String>,
        }

        impl ::std::default::Default for CityWithSummary {
            fn default() -> Self {
                Self {
                    city: Err("no value supplied for city".to_string()),
                    sumary: Err("no value supplied for sumary".to_string()),
                }
            }
        }

        impl CityWithSummary {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::City>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn sumary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::RatingSummary>,
                T::Error: ::std::fmt::Display,
            {
                self.sumary = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sumary: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<CityWithSummary> for super::CityWithSummary {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CityWithSummary,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city: value.city?,
                    sumary: value.sumary?,
                })
            }
        }

        impl ::std::convert::From<super::CityWithSummary> for CityWithSummary {
            fn from(value: super::CityWithSummary) -> Self {
                Self {
                    city: Ok(value.city),
                    sumary: Ok(value.sumary),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CoreServices {
            dentists: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            doctors: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            grocery: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            hospitals: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            pharmacies: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            score: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            social_services:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for CoreServices {
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
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.dentists = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for dentists: {}", e));
                self
            }
            pub fn doctors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.doctors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for doctors: {}", e));
                self
            }
            pub fn grocery<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.grocery = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grocery: {}", e));
                self
            }
            pub fn hospitals<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.hospitals = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hospitals: {}", e));
                self
            }
            pub fn pharmacies<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.pharmacies = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pharmacies: {}", e));
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn social_services<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.social_services = value.try_into().map_err(|e| {
                    format!("error converting supplied value for social_services: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<CoreServices> for super::CoreServices {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CoreServices,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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

        impl ::std::convert::From<super::CoreServices> for CoreServices {
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
        pub struct FargatePrice {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<i32, ::std::string::String>,
            per_second: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for FargatePrice {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    per_second: Err("no value supplied for per_second".to_string()),
                }
            }
        }

        impl FargatePrice {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn per_second<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.per_second = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for per_second: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<FargatePrice> for super::FargatePrice {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FargatePrice,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    per_second: value.per_second?,
                })
            }
        }

        impl ::std::convert::From<super::FargatePrice> for FargatePrice {
            fn from(value: super::FargatePrice) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    per_second: Ok(value.per_second),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Infrastructure {
            high_stress_miles:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            low_stress_miles:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for Infrastructure {
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
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
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
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
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

        impl ::std::convert::TryFrom<Infrastructure> for super::Infrastructure {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Infrastructure,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    high_stress_miles: value.high_stress_miles?,
                    low_stress_miles: value.low_stress_miles?,
                })
            }
        }

        impl ::std::convert::From<super::Infrastructure> for Infrastructure {
            fn from(value: super::Infrastructure) -> Self {
                Self {
                    high_stress_miles: Ok(value.high_stress_miles),
                    low_stress_miles: Ok(value.low_stress_miles),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Measure {
            buffered_lane: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            lane: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            path: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            sharrow: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            track: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for Measure {
            fn default() -> Self {
                Self {
                    buffered_lane: Ok(Default::default()),
                    lane: Ok(Default::default()),
                    path: Ok(Default::default()),
                    sharrow: Ok(Default::default()),
                    track: Ok(Default::default()),
                }
            }
        }

        impl Measure {
            pub fn buffered_lane<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.buffered_lane = value.try_into().map_err(|e| {
                    format!("error converting supplied value for buffered_lane: {}", e)
                });
                self
            }
            pub fn lane<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.lane = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lane: {}", e));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn sharrow<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.sharrow = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sharrow: {}", e));
                self
            }
            pub fn track<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.track = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for track: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Measure> for super::Measure {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Measure,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    buffered_lane: value.buffered_lane?,
                    lane: value.lane?,
                    path: value.path?,
                    sharrow: value.sharrow?,
                    track: value.track?,
                })
            }
        }

        impl ::std::convert::From<super::Measure> for Measure {
            fn from(value: super::Measure) -> Self {
                Self {
                    buffered_lane: Ok(value.buffered_lane),
                    lane: Ok(value.lane),
                    path: Ok(value.path),
                    sharrow: Ok(value.sharrow),
                    track: Ok(value.track),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Opportunity {
            employment: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            higher_education:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            k12_education: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            score: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            technical_vocational_college:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for Opportunity {
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
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.employment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for employment: {}", e));
                self
            }
            pub fn higher_education<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
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
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.k12_education = value.try_into().map_err(|e| {
                    format!("error converting supplied value for k12_education: {}", e)
                });
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn technical_vocational_college<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
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

        impl ::std::convert::TryFrom<Opportunity> for super::Opportunity {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Opportunity,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    employment: value.employment?,
                    higher_education: value.higher_education?,
                    k12_education: value.k12_education?,
                    score: value.score?,
                    technical_vocational_college: value.technical_vocational_college?,
                })
            }
        }

        impl ::std::convert::From<super::Opportunity> for Opportunity {
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
            people: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for People {
            fn default() -> Self {
                Self {
                    people: Ok(Default::default()),
                }
            }
        }

        impl People {
            pub fn people<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.people = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for people: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<People> for super::People {
            type Error = super::error::ConversionError;
            fn try_from(
                value: People,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    people: value.people?,
                })
            }
        }

        impl ::std::convert::From<super::People> for People {
            fn from(value: super::People) -> Self {
                Self {
                    people: Ok(value.people),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Rating {
            city_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            core_services: ::std::result::Result<super::CoreServices, ::std::string::String>,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            infrastructure: ::std::result::Result<super::Infrastructure, ::std::string::String>,
            measure: ::std::result::Result<super::Measure, ::std::string::String>,
            opportunity: ::std::result::Result<super::Opportunity, ::std::string::String>,
            people: ::std::result::Result<super::People, ::std::string::String>,
            recreation: ::std::result::Result<super::Recreation, ::std::string::String>,
            retail: ::std::result::Result<super::Retail, ::std::string::String>,
            score: ::std::result::Result<f64, ::std::string::String>,
            transit: ::std::result::Result<super::Transit, ::std::string::String>,
            version: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Rating {
            fn default() -> Self {
                Self {
                    city_id: Err("no value supplied for city_id".to_string()),
                    core_services: Err("no value supplied for core_services".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    infrastructure: Err("no value supplied for infrastructure".to_string()),
                    measure: Err("no value supplied for measure".to_string()),
                    opportunity: Err("no value supplied for opportunity".to_string()),
                    people: Err("no value supplied for people".to_string()),
                    recreation: Err("no value supplied for recreation".to_string()),
                    retail: Err("no value supplied for retail".to_string()),
                    score: Err("no value supplied for score".to_string()),
                    transit: Err("no value supplied for transit".to_string()),
                    version: Err("no value supplied for version".to_string()),
                }
            }
        }

        impl Rating {
            pub fn city_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.city_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city_id: {}", e));
                self
            }
            pub fn core_services<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::CoreServices>,
                T::Error: ::std::fmt::Display,
            {
                self.core_services = value.try_into().map_err(|e| {
                    format!("error converting supplied value for core_services: {}", e)
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn infrastructure<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Infrastructure>,
                T::Error: ::std::fmt::Display,
            {
                self.infrastructure = value.try_into().map_err(|e| {
                    format!("error converting supplied value for infrastructure: {}", e)
                });
                self
            }
            pub fn measure<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Measure>,
                T::Error: ::std::fmt::Display,
            {
                self.measure = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for measure: {}", e));
                self
            }
            pub fn opportunity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Opportunity>,
                T::Error: ::std::fmt::Display,
            {
                self.opportunity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for opportunity: {}", e));
                self
            }
            pub fn people<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::People>,
                T::Error: ::std::fmt::Display,
            {
                self.people = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for people: {}", e));
                self
            }
            pub fn recreation<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Recreation>,
                T::Error: ::std::fmt::Display,
            {
                self.recreation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recreation: {}", e));
                self
            }
            pub fn retail<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Retail>,
                T::Error: ::std::fmt::Display,
            {
                self.retail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for retail: {}", e));
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn transit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Transit>,
                T::Error: ::std::fmt::Display,
            {
                self.transit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for transit: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Rating> for super::Rating {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Rating,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city_id: value.city_id?,
                    core_services: value.core_services?,
                    id: value.id?,
                    infrastructure: value.infrastructure?,
                    measure: value.measure?,
                    opportunity: value.opportunity?,
                    people: value.people?,
                    recreation: value.recreation?,
                    retail: value.retail?,
                    score: value.score?,
                    transit: value.transit?,
                    version: value.version?,
                })
            }
        }

        impl ::std::convert::From<super::Rating> for Rating {
            fn from(value: super::Rating) -> Self {
                Self {
                    city_id: Ok(value.city_id),
                    core_services: Ok(value.core_services),
                    id: Ok(value.id),
                    infrastructure: Ok(value.infrastructure),
                    measure: Ok(value.measure),
                    opportunity: Ok(value.opportunity),
                    people: Ok(value.people),
                    recreation: Ok(value.recreation),
                    retail: Ok(value.retail),
                    score: Ok(value.score),
                    transit: Ok(value.transit),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RatingPost {
            city_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            core_services: ::std::result::Result<super::CoreServices, ::std::string::String>,
            infrastructure: ::std::result::Result<super::Infrastructure, ::std::string::String>,
            measure: ::std::result::Result<super::Measure, ::std::string::String>,
            opportunity: ::std::result::Result<super::Opportunity, ::std::string::String>,
            people: ::std::result::Result<super::People, ::std::string::String>,
            pop_size: ::std::result::Result<i32, ::std::string::String>,
            population: ::std::result::Result<i32, ::std::string::String>,
            recreation: ::std::result::Result<super::Recreation, ::std::string::String>,
            retail: ::std::result::Result<super::Retail, ::std::string::String>,
            score: ::std::result::Result<f64, ::std::string::String>,
            speed_limit_override:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            transit: ::std::result::Result<super::Transit, ::std::string::String>,
            version: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for RatingPost {
            fn default() -> Self {
                Self {
                    city_id: Err("no value supplied for city_id".to_string()),
                    core_services: Err("no value supplied for core_services".to_string()),
                    infrastructure: Err("no value supplied for infrastructure".to_string()),
                    measure: Err("no value supplied for measure".to_string()),
                    opportunity: Err("no value supplied for opportunity".to_string()),
                    people: Err("no value supplied for people".to_string()),
                    pop_size: Err("no value supplied for pop_size".to_string()),
                    population: Err("no value supplied for population".to_string()),
                    recreation: Err("no value supplied for recreation".to_string()),
                    retail: Err("no value supplied for retail".to_string()),
                    score: Err("no value supplied for score".to_string()),
                    speed_limit_override: Ok(Default::default()),
                    transit: Err("no value supplied for transit".to_string()),
                    version: Err("no value supplied for version".to_string()),
                }
            }
        }

        impl RatingPost {
            pub fn city_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.city_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city_id: {}", e));
                self
            }
            pub fn core_services<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::CoreServices>,
                T::Error: ::std::fmt::Display,
            {
                self.core_services = value.try_into().map_err(|e| {
                    format!("error converting supplied value for core_services: {}", e)
                });
                self
            }
            pub fn infrastructure<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Infrastructure>,
                T::Error: ::std::fmt::Display,
            {
                self.infrastructure = value.try_into().map_err(|e| {
                    format!("error converting supplied value for infrastructure: {}", e)
                });
                self
            }
            pub fn measure<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Measure>,
                T::Error: ::std::fmt::Display,
            {
                self.measure = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for measure: {}", e));
                self
            }
            pub fn opportunity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Opportunity>,
                T::Error: ::std::fmt::Display,
            {
                self.opportunity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for opportunity: {}", e));
                self
            }
            pub fn people<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::People>,
                T::Error: ::std::fmt::Display,
            {
                self.people = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for people: {}", e));
                self
            }
            pub fn pop_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.pop_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pop_size: {}", e));
                self
            }
            pub fn population<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.population = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for population: {}", e));
                self
            }
            pub fn recreation<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Recreation>,
                T::Error: ::std::fmt::Display,
            {
                self.recreation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recreation: {}", e));
                self
            }
            pub fn retail<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Retail>,
                T::Error: ::std::fmt::Display,
            {
                self.retail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for retail: {}", e));
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn speed_limit_override<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.speed_limit_override = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for speed_limit_override: {}",
                        e
                    )
                });
                self
            }
            pub fn transit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Transit>,
                T::Error: ::std::fmt::Display,
            {
                self.transit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for transit: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RatingPost> for super::RatingPost {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RatingPost,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city_id: value.city_id?,
                    core_services: value.core_services?,
                    infrastructure: value.infrastructure?,
                    measure: value.measure?,
                    opportunity: value.opportunity?,
                    people: value.people?,
                    pop_size: value.pop_size?,
                    population: value.population?,
                    recreation: value.recreation?,
                    retail: value.retail?,
                    score: value.score?,
                    speed_limit_override: value.speed_limit_override?,
                    transit: value.transit?,
                    version: value.version?,
                })
            }
        }

        impl ::std::convert::From<super::RatingPost> for RatingPost {
            fn from(value: super::RatingPost) -> Self {
                Self {
                    city_id: Ok(value.city_id),
                    core_services: Ok(value.core_services),
                    infrastructure: Ok(value.infrastructure),
                    measure: Ok(value.measure),
                    opportunity: Ok(value.opportunity),
                    people: Ok(value.people),
                    pop_size: Ok(value.pop_size),
                    population: Ok(value.population),
                    recreation: Ok(value.recreation),
                    retail: Ok(value.retail),
                    score: Ok(value.score),
                    speed_limit_override: Ok(value.speed_limit_override),
                    transit: Ok(value.transit),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RatingSummary {
            city_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            pop_size: ::std::result::Result<i32, ::std::string::String>,
            population: ::std::result::Result<i32, ::std::string::String>,
            residential_speed_limit_override:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            score: ::std::result::Result<f64, ::std::string::String>,
            version: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for RatingSummary {
            fn default() -> Self {
                Self {
                    city_id: Err("no value supplied for city_id".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    pop_size: Err("no value supplied for pop_size".to_string()),
                    population: Err("no value supplied for population".to_string()),
                    residential_speed_limit_override: Ok(Default::default()),
                    score: Err("no value supplied for score".to_string()),
                    version: Err("no value supplied for version".to_string()),
                }
            }
        }

        impl RatingSummary {
            pub fn city_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.city_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city_id: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn pop_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.pop_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pop_size: {}", e));
                self
            }
            pub fn population<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.population = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for population: {}", e));
                self
            }
            pub fn residential_speed_limit_override<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.residential_speed_limit_override = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for residential_speed_limit_override: {}",
                        e
                    )
                });
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<f64>,
                T::Error: ::std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RatingSummary> for super::RatingSummary {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RatingSummary,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city_id: value.city_id?,
                    created_at: value.created_at?,
                    id: value.id?,
                    pop_size: value.pop_size?,
                    population: value.population?,
                    residential_speed_limit_override: value.residential_speed_limit_override?,
                    score: value.score?,
                    version: value.version?,
                })
            }
        }

        impl ::std::convert::From<super::RatingSummary> for RatingSummary {
            fn from(value: super::RatingSummary) -> Self {
                Self {
                    city_id: Ok(value.city_id),
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    pop_size: Ok(value.pop_size),
                    population: Ok(value.population),
                    residential_speed_limit_override: Ok(value.residential_speed_limit_override),
                    score: Ok(value.score),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RatingWithCity {
            city: ::std::result::Result<super::City, ::std::string::String>,
            rating: ::std::result::Result<super::Rating, ::std::string::String>,
        }

        impl ::std::default::Default for RatingWithCity {
            fn default() -> Self {
                Self {
                    city: Err("no value supplied for city".to_string()),
                    rating: Err("no value supplied for rating".to_string()),
                }
            }
        }

        impl RatingWithCity {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::City>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn rating<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Rating>,
                T::Error: ::std::fmt::Display,
            {
                self.rating = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rating: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RatingWithCity> for super::RatingWithCity {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RatingWithCity,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    city: value.city?,
                    rating: value.rating?,
                })
            }
        }

        impl ::std::convert::From<super::RatingWithCity> for RatingWithCity {
            fn from(value: super::RatingWithCity) -> Self {
                Self {
                    city: Ok(value.city),
                    rating: Ok(value.rating),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Recreation {
            community_centers:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            parks: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            score: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            trails: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for Recreation {
            fn default() -> Self {
                Self {
                    community_centers: Ok(Default::default()),
                    parks: Ok(Default::default()),
                    score: Ok(Default::default()),
                    trails: Ok(Default::default()),
                }
            }
        }

        impl Recreation {
            pub fn community_centers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
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
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.parks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for parks: {}", e));
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.score = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for score: {}", e));
                self
            }
            pub fn trails<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trails = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trails: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Recreation> for super::Recreation {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Recreation,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    community_centers: value.community_centers?,
                    parks: value.parks?,
                    score: value.score?,
                    trails: value.trails?,
                })
            }
        }

        impl ::std::convert::From<super::Recreation> for Recreation {
            fn from(value: super::Recreation) -> Self {
                Self {
                    community_centers: Ok(value.community_centers),
                    parks: Ok(value.parks),
                    score: Ok(value.score),
                    trails: Ok(value.trails),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Retail {
            retail: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for Retail {
            fn default() -> Self {
                Self {
                    retail: Ok(Default::default()),
                }
            }
        }

        impl Retail {
            pub fn retail<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.retail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for retail: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Retail> for super::Retail {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Retail,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    retail: value.retail?,
                })
            }
        }

        impl ::std::convert::From<super::Retail> for Retail {
            fn from(value: super::Retail) -> Self {
                Self {
                    retail: Ok(value.retail),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Submission {
            city: ::std::result::Result<::std::string::String, ::std::string::String>,
            consent: ::std::result::Result<bool, ::std::string::String>,
            country: ::std::result::Result<super::Country, ::std::string::String>,
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            fips_code: ::std::result::Result<::std::string::String, ::std::string::String>,
            first_name: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<i32, ::std::string::String>,
            last_name: ::std::result::Result<::std::string::String, ::std::string::String>,
            occupation: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            organization: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            region: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            status: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Submission {
            fn default() -> Self {
                Self {
                    city: Err("no value supplied for city".to_string()),
                    consent: Err("no value supplied for consent".to_string()),
                    country: Err("no value supplied for country".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    email: Err("no value supplied for email".to_string()),
                    fips_code: Err("no value supplied for fips_code".to_string()),
                    first_name: Err("no value supplied for first_name".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    last_name: Err("no value supplied for last_name".to_string()),
                    occupation: Ok(Default::default()),
                    organization: Ok(Default::default()),
                    region: Ok(Default::default()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl Submission {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn consent<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.consent = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for consent: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Country>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn first_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.first_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for first_name: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn last_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.last_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_name: {}", e));
                self
            }
            pub fn occupation<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.occupation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for occupation: {}", e));
                self
            }
            pub fn organization<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.organization = value.try_into().map_err(|e| {
                    format!("error converting supplied value for organization: {}", e)
                });
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Submission> for super::Submission {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Submission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::Submission> for Submission {
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
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SubmissionPatch {
            city: ::std::result::Result<::std::string::String, ::std::string::String>,
            consent: ::std::result::Result<bool, ::std::string::String>,
            country: ::std::result::Result<super::Country, ::std::string::String>,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            fips_code: ::std::result::Result<::std::string::String, ::std::string::String>,
            first_name: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_name: ::std::result::Result<::std::string::String, ::std::string::String>,
            occupation: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            organization: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            region: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            status: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for SubmissionPatch {
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
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl SubmissionPatch {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn consent<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.consent = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for consent: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Country>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn first_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.first_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for first_name: {}", e));
                self
            }
            pub fn last_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.last_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_name: {}", e));
                self
            }
            pub fn occupation<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.occupation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for occupation: {}", e));
                self
            }
            pub fn organization<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.organization = value.try_into().map_err(|e| {
                    format!("error converting supplied value for organization: {}", e)
                });
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<SubmissionPatch> for super::SubmissionPatch {
            type Error = super::error::ConversionError;
            fn try_from(
                value: SubmissionPatch,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::SubmissionPatch> for SubmissionPatch {
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
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SubmissionPost {
            city: ::std::result::Result<::std::string::String, ::std::string::String>,
            consent: ::std::result::Result<bool, ::std::string::String>,
            country: ::std::result::Result<super::Country, ::std::string::String>,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            fips_code: ::std::result::Result<::std::string::String, ::std::string::String>,
            first_name: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_name: ::std::result::Result<::std::string::String, ::std::string::String>,
            occupation: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            organization: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            region: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            status: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for SubmissionPost {
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
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl SubmissionPost {
            pub fn city<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.city = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for city: {}", e));
                self
            }
            pub fn consent<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.consent = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for consent: {}", e));
                self
            }
            pub fn country<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Country>,
                T::Error: ::std::fmt::Display,
            {
                self.country = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for country: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn fips_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.fips_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fips_code: {}", e));
                self
            }
            pub fn first_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.first_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for first_name: {}", e));
                self
            }
            pub fn last_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.last_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_name: {}", e));
                self
            }
            pub fn occupation<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.occupation = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for occupation: {}", e));
                self
            }
            pub fn organization<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.organization = value.try_into().map_err(|e| {
                    format!("error converting supplied value for organization: {}", e)
                });
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<SubmissionPost> for super::SubmissionPost {
            type Error = super::error::ConversionError;
            fn try_from(
                value: SubmissionPost,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::SubmissionPost> for SubmissionPost {
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
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Transit {
            transit: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for Transit {
            fn default() -> Self {
                Self {
                    transit: Ok(Default::default()),
                }
            }
        }

        impl Transit {
            pub fn transit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.transit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for transit: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Transit> for super::Transit {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Transit,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    transit: value.transit?,
                })
            }
        }

        impl ::std::convert::From<super::Transit> for Transit {
            fn from(value: super::Transit) -> Self {
                Self {
                    transit: Ok(value.transit),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UsState {
            abbrev: ::std::result::Result<::std::string::String, ::std::string::String>,
            fipscode: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            speed_limit: ::std::result::Result<i32, ::std::string::String>,
        }

        impl ::std::default::Default for UsState {
            fn default() -> Self {
                Self {
                    abbrev: Err("no value supplied for abbrev".to_string()),
                    fipscode: Err("no value supplied for fipscode".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    speed_limit: Err("no value supplied for speed_limit".to_string()),
                }
            }
        }

        impl UsState {
            pub fn abbrev<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.abbrev = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for abbrev: {}", e));
                self
            }
            pub fn fipscode<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.fipscode = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fipscode: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn speed_limit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.speed_limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for speed_limit: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<UsState> for super::UsState {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UsState,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    abbrev: value.abbrev?,
                    fipscode: value.fipscode?,
                    name: value.name?,
                    speed_limit: value.speed_limit?,
                })
            }
        }

        impl ::std::convert::From<super::UsState> for UsState {
            fn from(value: super::UsState) -> Self {
                Self {
                    abbrev: Ok(value.abbrev),
                    fipscode: Ok(value.fipscode),
                    name: Ok(value.name),
                    speed_limit: Ok(value.speed_limit),
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
            let dur = ::std::time::Duration::from_secs(15u64);
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
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
impl Client {
    ///Get the details of all cities where an BNA analysis was performed.
    ///
    ///Sends a `GET` request to `/cities`
    ///
    ///Arguments:
    /// - `page`: The result page being returned
    /// - `page_size`: The number of items per page
    ///```ignore
    /// let response = client.get_cities()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_cities(&self) -> builder::GetCities<'_> {
        builder::GetCities::new(self)
    }

    ///Create a new city.
    ///
    ///Sends a `POST` request to `/cities`
    ///
    ///```ignore
    /// let response = client.post_city()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_city(&self) -> builder::PostCity<'_> {
        builder::PostCity::new(self)
    }

    ///Get the submissions details.
    ///
    ///Sends a `GET` request to `/cities/submissions`
    ///
    ///Arguments:
    /// - `page`: The result page being returned
    /// - `page_size`: The number of items per page
    /// - `status`: Filter for the submission status
    ///```ignore
    /// let response = client.get_cities_submissions()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .status(status)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_cities_submissions(&self) -> builder::GetCitiesSubmissions<'_> {
        builder::GetCitiesSubmissions::new(self)
    }

    ///Create a new city submission.
    ///
    ///Sends a `POST` request to `/cities/submissions`
    ///
    ///```ignore
    /// let response = client.post_cities_submission()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_cities_submission(&self) -> builder::PostCitiesSubmission<'_> {
        builder::PostCitiesSubmission::new(self)
    }

    ///Get the details of a specific sumission.
    ///
    ///Sends a `GET` request to `/cities/submissions/{submission_id}`
    ///
    ///Arguments:
    /// - `submission_id`: Submission identifier
    /// - `status`: Filter for the submission status
    ///```ignore
    /// let response = client.get_cities_submission()
    ///    .submission_id(submission_id)
    ///    .status(status)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_cities_submission(&self) -> builder::GetCitiesSubmission<'_> {
        builder::GetCitiesSubmission::new(self)
    }

    ///Update a city submission.
    ///
    ///Sends a `PATCH` request to `/cities/submissions/{submission_id}`
    ///
    ///Arguments:
    /// - `submission_id`: Submission identifier
    /// - `body`
    ///```ignore
    /// let response = client.patch_cities_submission()
    ///    .submission_id(submission_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn patch_cities_submission(&self) -> builder::PatchCitiesSubmission<'_> {
        builder::PatchCitiesSubmission::new(self)
    }

    ///Get the top N cities for a specific year.
    ///
    ///Sends a `GET` request to `/cities/top/{year}/{count}`
    ///
    ///Arguments:
    /// - `year`: The year to collect the top cities for
    /// - `count`: The number of top cities to collect
    ///```ignore
    /// let response = client.get_top_cities()
    ///    .year(year)
    ///    .count(count)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_top_cities(&self) -> builder::GetTopCities<'_> {
        builder::GetTopCities::new(self)
    }

    ///Get the details of a specific city where an BNA analysis was computed.
    ///
    ///Sends a `GET` request to `/cities/{country}/{region}/{name}`
    ///
    ///Arguments:
    /// - `country`: Country name
    /// - `region`: Region name. A region can be a state, a province, a
    ///   community, or
    ///something similar depending on the country. If a country does not have
    ///this concept, then the country name is used.
    /// - `name`: City name
    ///```ignore
    /// let response = client.get_city()
    ///    .country(country)
    ///    .region(region)
    ///    .name(name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_city(&self) -> builder::GetCity<'_> {
        builder::GetCity::new(self)
    }

    ///Get the details of a specific city with all the analysis that were
    /// performed against it.
    ///
    ///Sends a `GET` request to `/cities/{country}/{region}/{name}/ratings`
    ///
    ///Arguments:
    /// - `country`: Country name
    /// - `region`: Region name. A region can be a state, a province, a
    ///   community, or
    ///something similar depending on the country. If a country does not have
    ///this concept, then the country name is used.
    /// - `name`: City name
    /// - `page`: The result page being returned
    /// - `page_size`: The number of items per page
    ///```ignore
    /// let response = client.get_city_ratings()
    ///    .country(country)
    ///    .region(region)
    ///    .name(name)
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_city_ratings(&self) -> builder::GetCityRatings<'_> {
        builder::GetCityRatings::new(self)
    }

    ///Get the details of all BNA pipelines
    ///
    ///Sends a `GET` request to `/pipelines/bna`
    ///
    ///Arguments:
    /// - `page`: The result page being returned
    /// - `page_size`: The number of items per page
    ///```ignore
    /// let response = client.get_pipelines_bnas()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_pipelines_bnas(&self) -> builder::GetPipelinesBnas<'_> {
        builder::GetPipelinesBnas::new(self)
    }

    ///Create a new BNA pipeline
    ///
    ///Sends a `POST` request to `/pipelines/bna`
    ///
    ///```ignore
    /// let response = client.post_pipelines_bna()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_pipelines_bna(&self) -> builder::PostPipelinesBna<'_> {
        builder::PostPipelinesBna::new(self)
    }

    ///Get the details of a specific BNA pipeline
    ///
    ///Sends a `GET` request to `/pipelines/bna/{pipeline_id}`
    ///
    ///Arguments:
    /// - `pipeline_id`: Pipeline identifier
    ///```ignore
    /// let response = client.get_pipelines_bna()
    ///    .pipeline_id(pipeline_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_pipelines_bna(&self) -> builder::GetPipelinesBna<'_> {
        builder::GetPipelinesBna::new(self)
    }

    ///Update the details of a specific BNA pipeline
    ///
    ///Sends a `PATCH` request to `/pipelines/bna/{pipeline_id}`
    ///
    ///Arguments:
    /// - `pipeline_id`: Pipeline identifier
    /// - `body`
    ///```ignore
    /// let response = client.patch_pipelines_bna()
    ///    .pipeline_id(pipeline_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn patch_pipelines_bna(&self) -> builder::PatchPipelinesBna<'_> {
        builder::PatchPipelinesBna::new(self)
    }

    ///Get all the AWS Fargate prices used to compute analysis costs.
    ///
    ///Sends a `GET` request to `/prices/fargate`
    ///
    ///Arguments:
    /// - `page`: The result page being returned
    /// - `page_size`: The number of items per page
    ///```ignore
    /// let response = client.get_prices_fargate()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_prices_fargate(&self) -> builder::GetPricesFargate<'_> {
        builder::GetPricesFargate::new(self)
    }

    ///Get a specific AWS Fargate price used to compute the cost of analysis
    /// cost.
    ///
    ///Sends a `GET` request to `/prices/fargate/{price_id}`
    ///
    ///Arguments:
    /// - `price_id`: Identifier of a Fargate price
    ///```ignore
    /// let response = client.get_price_fargate()
    ///    .price_id(price_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_price_fargate(&self) -> builder::GetPriceFargate<'_> {
        builder::GetPriceFargate::new(self)
    }

    ///Get city ratings
    ///
    ///Sends a `GET` request to `/ratings`
    ///
    ///Arguments:
    /// - `page`: The result page being returned
    /// - `page_size`: The number of items per page
    ///```ignore
    /// let response = client.get_ratings()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_ratings(&self) -> builder::GetRatings<'_> {
        builder::GetRatings::new(self)
    }

    ///Create a new city rating
    ///
    ///Sends a `POST` request to `/ratings`
    ///
    ///```ignore
    /// let response = client.post_rating()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn post_rating(&self) -> builder::PostRating<'_> {
        builder::PostRating::new(self)
    }

    ///Get the details of a specific city rating
    ///
    ///Sends a `GET` request to `/ratings/{rating_id}`
    ///
    ///Arguments:
    /// - `rating_id`: Rating identifier
    ///```ignore
    /// let response = client.get_rating()
    ///    .rating_id(rating_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_rating(&self) -> builder::GetRating<'_> {
        builder::GetRating::new(self)
    }

    ///Get a city rating and its associated city details
    ///
    ///Sends a `GET` request to `/ratings/{rating_id}/city`
    ///
    ///Arguments:
    /// - `rating_id`: Rating identifier
    ///```ignore
    /// let response = client.get_ratings_city()
    ///    .rating_id(rating_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_ratings_city(&self) -> builder::GetRatingsCity<'_> {
        builder::GetRatingsCity::new(self)
    }

    ///Retrieve all rating reports.
    ///
    ///Sends a `GET` request to `/reports`
    ///
    ///```ignore
    /// let response = client.get_reports()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_reports(&self) -> builder::GetReports<'_> {
        builder::GetReports::new(self)
    }

    ///Retrieve the latest rating reports for a specific year.
    ///
    ///Sends a `GET` request to `/reports/{year}`
    ///
    ///Arguments:
    /// - `year`: Year to retrieve the reports for
    ///```ignore
    /// let response = client.get_reports_year()
    ///    .year(year)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_reports_year(&self) -> builder::GetReportsYear<'_> {
        builder::GetReportsYear::new(self)
    }

    ///Get the details of all US states.
    ///
    ///Sends a `GET` request to `/usstates`
    ///
    ///Arguments:
    /// - `page`: The result page being returned
    /// - `page_size`: The number of items per page
    ///```ignore
    /// let response = client.get_us_states()
    ///    .page(page)
    ///    .page_size(page_size)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_us_states(&self) -> builder::GetUsStates<'_> {
        builder::GetUsStates::new(self)
    }

    ///Get the details of a specific US state.
    ///
    ///Sends a `GET` request to `/usstates/{name}`
    ///
    ///Arguments:
    /// - `name`: Full name of a US state
    ///```ignore
    /// let response = client.get_us_state()
    ///    .name(name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_us_state(&self) -> builder::GetUsState<'_> {
        builder::GetUsState::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, RequestBuilderExt,
        ResponseValue,
    };
    ///Builder for [`Client::get_cities`]
    ///
    ///[`Client::get_cities`]: super::Client::get_cities
    #[derive(Debug, Clone)]
    pub struct GetCities<'a> {
        client: &'a super::Client,
        page: Result<Option<::std::num::NonZeroU64>, String>,
        page_size: Result<Option<::std::num::NonZeroU64>, String>,
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
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page_size = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page_size failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/cities`
        pub async fn send(self) -> Result<ResponseValue<types::Cities>, Error<()>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/cities", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("page_size", &page_size))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_cities",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        body: Result<types::builder::CityPost, String>,
    }

    impl<'a> PostCity<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
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
        pub async fn send(self) -> Result<ResponseValue<types::City>, Error<types::ApiErrors>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::CityPost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/cities", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_city",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_cities_submissions`]
    ///
    ///[`Client::get_cities_submissions`]: super::Client::get_cities_submissions
    #[derive(Debug, Clone)]
    pub struct GetCitiesSubmissions<'a> {
        client: &'a super::Client,
        page: Result<Option<::std::num::NonZeroU64>, String>,
        page_size: Result<Option<::std::num::NonZeroU64>, String>,
        status: Result<Option<::std::string::String>, String>,
    }

    impl<'a> GetCitiesSubmissions<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
                status: Ok(None),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page_size = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page_size failed".to_string()
            });
            self
        }

        pub fn status<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.status = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for status failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/cities/submissions`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::Submissions>, Error<types::ApiErrors>> {
            let Self {
                client,
                page,
                page_size,
                status,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let status = status.map_err(Error::InvalidRequest)?;
            let url = format!("{}/cities/submissions", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("page_size", &page_size))
                .query(&progenitor_client::QueryParam::new("status", &status))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_cities_submissions",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::post_cities_submission`]
    ///
    ///[`Client::post_cities_submission`]: super::Client::post_cities_submission
    #[derive(Debug, Clone)]
    pub struct PostCitiesSubmission<'a> {
        client: &'a super::Client,
        body: Result<types::builder::SubmissionPost, String>,
    }

    impl<'a> PostCitiesSubmission<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
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
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::Submission>, Error<types::ApiErrors>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::SubmissionPost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/cities/submissions", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_cities_submission",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_cities_submission`]
    ///
    ///[`Client::get_cities_submission`]: super::Client::get_cities_submission
    #[derive(Debug, Clone)]
    pub struct GetCitiesSubmission<'a> {
        client: &'a super::Client,
        submission_id: Result<i32, String>,
        status: Result<Option<::std::string::String>, String>,
    }

    impl<'a> GetCitiesSubmission<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                submission_id: Err("submission_id was not initialized".to_string()),
                status: Ok(None),
            }
        }

        pub fn submission_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.submission_id = value
                .try_into()
                .map_err(|_| "conversion to `i32` for submission_id failed".to_string());
            self
        }

        pub fn status<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.status = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for status failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/cities/submissions/{submission_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::Submission>, Error<types::ApiErrors>> {
            let Self {
                client,
                submission_id,
                status,
            } = self;
            let submission_id = submission_id.map_err(Error::InvalidRequest)?;
            let status = status.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/cities/submissions/{}",
                client.baseurl,
                encode_path(&submission_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("status", &status))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_cities_submission",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::patch_cities_submission`]
    ///
    ///[`Client::patch_cities_submission`]: super::Client::patch_cities_submission
    #[derive(Debug, Clone)]
    pub struct PatchCitiesSubmission<'a> {
        client: &'a super::Client,
        submission_id: Result<i32, String>,
        body: Result<types::builder::SubmissionPatch, String>,
    }

    impl<'a> PatchCitiesSubmission<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                submission_id: Err("submission_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn submission_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.submission_id = value
                .try_into()
                .map_err(|_| "conversion to `i32` for submission_id failed".to_string());
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
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::Submission>, Error<types::ApiErrors>> {
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .patch(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "patch_cities_submission",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_top_cities`]
    ///
    ///[`Client::get_top_cities`]: super::Client::get_top_cities
    #[derive(Debug, Clone)]
    pub struct GetTopCities<'a> {
        client: &'a super::Client,
        year: Result<i32, String>,
        count: Result<::std::num::NonZeroU32, String>,
    }

    impl<'a> GetTopCities<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                year: Err("year was not initialized".to_string()),
                count: Err("count was not initialized".to_string()),
            }
        }

        pub fn year<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.year = value
                .try_into()
                .map_err(|_| "conversion to `i32` for year failed".to_string());
            self
        }

        pub fn count<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU32>,
        {
            self.count = value.try_into().map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU32` for count failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/cities/top/{year}/{count}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CitiesWithSummary>, Error<types::ApiErrors>> {
            let Self {
                client,
                year,
                count,
            } = self;
            let year = year.map_err(Error::InvalidRequest)?;
            let count = count.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/cities/top/{}/{}",
                client.baseurl,
                encode_path(&year.to_string()),
                encode_path(&count.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_top_cities",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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
        region: Result<::std::string::String, String>,
        name: Result<::std::string::String, String>,
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
            V: std::convert::TryInto<::std::string::String>,
        {
            self.region = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for region failed".to_string()
            });
            self
        }

        pub fn name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.name = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for name failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/cities/{country}/{region}/{name}`
        pub async fn send(self) -> Result<ResponseValue<types::City>, Error<types::ApiErrors>> {
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_city",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_city_ratings`]
    ///
    ///[`Client::get_city_ratings`]: super::Client::get_city_ratings
    #[derive(Debug, Clone)]
    pub struct GetCityRatings<'a> {
        client: &'a super::Client,
        country: Result<types::Country, String>,
        region: Result<::std::string::String, String>,
        name: Result<::std::string::String, String>,
        page: Result<Option<::std::num::NonZeroU64>, String>,
        page_size: Result<Option<::std::num::NonZeroU64>, String>,
    }

    impl<'a> GetCityRatings<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                country: Err("country was not initialized".to_string()),
                region: Err("region was not initialized".to_string()),
                name: Err("name was not initialized".to_string()),
                page: Ok(None),
                page_size: Ok(None),
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
            V: std::convert::TryInto<::std::string::String>,
        {
            self.region = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for region failed".to_string()
            });
            self
        }

        pub fn name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.name = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for name failed".to_string()
            });
            self
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page_size = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page_size failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/cities/{country}/{region}/{name}/ratings`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CityRatings>, Error<types::ApiErrors>> {
            let Self {
                client,
                country,
                region,
                name,
                page,
                page_size,
            } = self;
            let country = country.map_err(Error::InvalidRequest)?;
            let region = region.map_err(Error::InvalidRequest)?;
            let name = name.map_err(Error::InvalidRequest)?;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/cities/{}/{}/{}/ratings",
                client.baseurl,
                encode_path(&country.to_string()),
                encode_path(&region.to_string()),
                encode_path(&name.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("page_size", &page_size))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_city_ratings",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_pipelines_bnas`]
    ///
    ///[`Client::get_pipelines_bnas`]: super::Client::get_pipelines_bnas
    #[derive(Debug, Clone)]
    pub struct GetPipelinesBnas<'a> {
        client: &'a super::Client,
        page: Result<Option<::std::num::NonZeroU64>, String>,
        page_size: Result<Option<::std::num::NonZeroU64>, String>,
    }

    impl<'a> GetPipelinesBnas<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page_size = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page_size failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/pipelines/bna`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::BnaPipelines>, Error<types::ApiErrors>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/pipelines/bna", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("page_size", &page_size))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_pipelines_bnas",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::post_pipelines_bna`]
    ///
    ///[`Client::post_pipelines_bna`]: super::Client::post_pipelines_bna
    #[derive(Debug, Clone)]
    pub struct PostPipelinesBna<'a> {
        client: &'a super::Client,
        body: Result<types::builder::BnaPipelinePost, String>,
    }

    impl<'a> PostPipelinesBna<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::BnaPipelinePost>,
            <V as std::convert::TryInto<types::BnaPipelinePost>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `BnaPipelinePost` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::BnaPipelinePost) -> types::builder::BnaPipelinePost,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/pipelines/bna`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::BnaPipeline>, Error<types::ApiErrors>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::BnaPipelinePost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/pipelines/bna", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_pipelines_bna",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_pipelines_bna`]
    ///
    ///[`Client::get_pipelines_bna`]: super::Client::get_pipelines_bna
    #[derive(Debug, Clone)]
    pub struct GetPipelinesBna<'a> {
        client: &'a super::Client,
        pipeline_id: Result<::uuid::Uuid, String>,
    }

    impl<'a> GetPipelinesBna<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                pipeline_id: Err("pipeline_id was not initialized".to_string()),
            }
        }

        pub fn pipeline_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.pipeline_id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for pipeline_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/pipelines/bna/{pipeline_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::BnaPipeline>, Error<types::ApiErrors>> {
            let Self {
                client,
                pipeline_id,
            } = self;
            let pipeline_id = pipeline_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/pipelines/bna/{}",
                client.baseurl,
                encode_path(&pipeline_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_pipelines_bna",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::patch_pipelines_bna`]
    ///
    ///[`Client::patch_pipelines_bna`]: super::Client::patch_pipelines_bna
    #[derive(Debug, Clone)]
    pub struct PatchPipelinesBna<'a> {
        client: &'a super::Client,
        pipeline_id: Result<::uuid::Uuid, String>,
        body: Result<types::builder::BnaPipelinePatch, String>,
    }

    impl<'a> PatchPipelinesBna<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                pipeline_id: Err("pipeline_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn pipeline_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.pipeline_id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for pipeline_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::BnaPipelinePatch>,
            <V as std::convert::TryInto<types::BnaPipelinePatch>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `BnaPipelinePatch` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::BnaPipelinePatch,
            ) -> types::builder::BnaPipelinePatch,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PATCH` request to `/pipelines/bna/{pipeline_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::BnaPipeline>, Error<types::ApiErrors>> {
            let Self {
                client,
                pipeline_id,
                body,
            } = self;
            let pipeline_id = pipeline_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::BnaPipelinePatch::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/pipelines/bna/{}",
                client.baseurl,
                encode_path(&pipeline_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .patch(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "patch_pipelines_bna",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_prices_fargate`]
    ///
    ///[`Client::get_prices_fargate`]: super::Client::get_prices_fargate
    #[derive(Debug, Clone)]
    pub struct GetPricesFargate<'a> {
        client: &'a super::Client,
        page: Result<Option<::std::num::NonZeroU64>, String>,
        page_size: Result<Option<::std::num::NonZeroU64>, String>,
    }

    impl<'a> GetPricesFargate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page_size = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page_size failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/prices/fargate`
        pub async fn send(self) -> Result<ResponseValue<types::FargatePrices>, Error<()>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/prices/fargate", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("page_size", &page_size))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_prices_fargate",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_price_fargate`]
    ///
    ///[`Client::get_price_fargate`]: super::Client::get_price_fargate
    #[derive(Debug, Clone)]
    pub struct GetPriceFargate<'a> {
        client: &'a super::Client,
        price_id: Result<i32, String>,
    }

    impl<'a> GetPriceFargate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                price_id: Err("price_id was not initialized".to_string()),
            }
        }

        pub fn price_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.price_id = value
                .try_into()
                .map_err(|_| "conversion to `i32` for price_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/prices/fargate/{price_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::FargatePrice>, Error<types::ApiErrors>> {
            let Self { client, price_id } = self;
            let price_id = price_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/prices/fargate/{}",
                client.baseurl,
                encode_path(&price_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_price_fargate",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_ratings`]
    ///
    ///[`Client::get_ratings`]: super::Client::get_ratings
    #[derive(Debug, Clone)]
    pub struct GetRatings<'a> {
        client: &'a super::Client,
        page: Result<Option<::std::num::NonZeroU64>, String>,
        page_size: Result<Option<::std::num::NonZeroU64>, String>,
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
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page_size = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page_size failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/ratings`
        pub async fn send(self) -> Result<ResponseValue<types::Ratings>, Error<()>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/ratings", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("page_size", &page_size))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_ratings",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::post_rating`]
    ///
    ///[`Client::post_rating`]: super::Client::post_rating
    #[derive(Debug, Clone)]
    pub struct PostRating<'a> {
        client: &'a super::Client,
        body: Result<types::builder::RatingPost, String>,
    }

    impl<'a> PostRating<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RatingPost>,
            <V as std::convert::TryInto<types::RatingPost>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `RatingPost` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::RatingPost) -> types::builder::RatingPost,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/ratings`
        pub async fn send(self) -> Result<ResponseValue<types::Rating>, Error<types::ApiErrors>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::RatingPost::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/ratings", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "post_rating",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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
        rating_id: Result<::uuid::Uuid, String>,
    }

    impl<'a> GetRating<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                rating_id: Err("rating_id was not initialized".to_string()),
            }
        }

        pub fn rating_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.rating_id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for rating_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ratings/{rating_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Rating>, Error<types::ApiErrors>> {
            let Self { client, rating_id } = self;
            let rating_id = rating_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ratings/{}",
                client.baseurl,
                encode_path(&rating_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_rating",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_ratings_city`]
    ///
    ///[`Client::get_ratings_city`]: super::Client::get_ratings_city
    #[derive(Debug, Clone)]
    pub struct GetRatingsCity<'a> {
        client: &'a super::Client,
        rating_id: Result<::uuid::Uuid, String>,
    }

    impl<'a> GetRatingsCity<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                rating_id: Err("rating_id was not initialized".to_string()),
            }
        }

        pub fn rating_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.rating_id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for rating_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ratings/{rating_id}/city`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::RatingWithCity>, Error<types::ApiErrors>> {
            let Self { client, rating_id } = self;
            let rating_id = rating_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ratings/{}/city",
                client.baseurl,
                encode_path(&rating_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_ratings_city",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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

    ///Builder for [`Client::get_reports`]
    ///
    ///[`Client::get_reports`]: super::Client::get_reports
    #[derive(Debug, Clone)]
    pub struct GetReports<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetReports<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/reports`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/reports", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.get(url).headers(header_map).build()?;
            let info = OperationInfo {
                operation_id: "get_reports",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_reports_year`]
    ///
    ///[`Client::get_reports_year`]: super::Client::get_reports_year
    #[derive(Debug, Clone)]
    pub struct GetReportsYear<'a> {
        client: &'a super::Client,
        year: Result<i32, String>,
    }

    impl<'a> GetReportsYear<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                year: Err("year was not initialized".to_string()),
            }
        }

        pub fn year<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.year = value
                .try_into()
                .map_err(|_| "conversion to `i32` for year failed".to_string());
            self
        }

        ///Sends a `GET` request to `/reports/{year}`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self { client, year } = self;
            let year = year.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/reports/{}",
                client.baseurl,
                encode_path(&year.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.get(url).headers(header_map).build()?;
            let info = OperationInfo {
                operation_id: "get_reports_year",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_us_states`]
    ///
    ///[`Client::get_us_states`]: super::Client::get_us_states
    #[derive(Debug, Clone)]
    pub struct GetUsStates<'a> {
        client: &'a super::Client,
        page: Result<Option<::std::num::NonZeroU64>, String>,
        page_size: Result<Option<::std::num::NonZeroU64>, String>,
    }

    impl<'a> GetUsStates<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                page: Ok(None),
                page_size: Ok(None),
            }
        }

        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page failed".to_string()
            });
            self
        }

        pub fn page_size<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU64>,
        {
            self.page_size = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU64` for page_size failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/usstates`
        pub async fn send(self) -> Result<ResponseValue<types::UsStates>, Error<()>> {
            let Self {
                client,
                page,
                page_size,
            } = self;
            let page = page.map_err(Error::InvalidRequest)?;
            let page_size = page_size.map_err(Error::InvalidRequest)?;
            let url = format!("{}/usstates", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("page_size", &page_size))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_us_states",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_us_state`]
    ///
    ///[`Client::get_us_state`]: super::Client::get_us_state
    #[derive(Debug, Clone)]
    pub struct GetUsState<'a> {
        client: &'a super::Client,
        name: Result<::std::string::String, String>,
    }

    impl<'a> GetUsState<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                name: Err("name was not initialized".to_string()),
            }
        }

        pub fn name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.name = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for name failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/usstates/{name}`
        pub async fn send(self) -> Result<ResponseValue<types::UsState>, Error<types::ApiErrors>> {
            let Self { client, name } = self;
            let name = name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/usstates/{}",
                client.baseurl,
                encode_path(&name.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_us_state",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
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
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
