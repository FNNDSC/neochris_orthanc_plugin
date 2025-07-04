//! Type definitions to use the [Find] trait to search for the
//! ID of series for each study by StudyInstanceUID.

use orthanc_sdk::api::{
    Find,
    types::{SeriesId, StudyId},
};
use orthanc_sdk::openapi::ToolsFindPostRequest;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// List of series for a study.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SeriesOfStudy {
    /// Study ID
    #[serde(rename = "ID")]
    pub id: StudyId,
    /// Series IDs
    pub series: Vec<SeriesId>,
}

/// A request to find series of a study by StudyInstanceUID.
pub(crate) struct FindSeriesByStudy(pub String);

impl From<FindSeriesByStudy> for ToolsFindPostRequest {
    fn from(value: FindSeriesByStudy) -> Self {
        ToolsFindPostRequest {
            case_sensitive: Some(false),
            expand: Some(true),
            level: Some("Study".to_string()),
            response_content: Some(vec!["Children".to_string()]),
            query: Some(json!({"StudyInstanceUID": value.0})),
            ..Default::default()
        }
    }
}

impl Find for FindSeriesByStudy {
    type Item = SeriesOfStudy;
}
