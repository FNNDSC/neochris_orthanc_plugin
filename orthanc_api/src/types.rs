use crate::dicom::*;
use crate::job::JobInfo;
use compact_str::CompactString;
use nutype::nutype;
use serde::{Deserialize, Serialize};

/// ID of an Orthanc query to a remote modality.
#[nutype(derive(Serialize, Deserialize, Clone, Display, Debug, Eq, PartialEq, Hash))]
pub struct QueryId(String);

impl ResourceId for QueryId {
    type Item = Vec<CompactString>;

    fn uri(&self) -> String {
        format!("/queries/{}", &self)
    }
}

/// ID of an Orthanc job.
#[nutype(derive(Serialize, Deserialize, Clone, Display, Debug, Eq, PartialEq, Hash))]
pub struct JobId(String);

impl ResourceId for JobId {
    type Item = JobInfo;

    fn uri(&self) -> String {
        format!("/jobs/{}", &self)
    }
}

/// ID of a patient in Orthanc.
#[nutype(derive(Serialize, Deserialize, Clone, Display, Debug, Eq, PartialEq, Hash))]
pub struct PatientId(String);

impl ResourceId for PatientId {
    type Item = Patient<Option<()>>;

    fn uri(&self) -> String {
        format!("/patients/{}", &self)
    }
}

impl<T> DicomResourceId<T> for PatientId {
    type Item = Patient<T>;
}

impl AnonymizableId for PatientId {}

impl HierarchalResourceId for PatientId {
    // NOTE: a patient is the "root" in the DICOM hierarchy, it does
    //       not have an ancestor. Orthanc always returns
    //       `{ "RemainingAncestor": null }` from calling `DELETE`
    //       on `/patients/{id}`.
    type Ancestor = PatientId;
}

/// ID of a DICOM study stored by Orthanc.
#[nutype(derive(Serialize, Deserialize, Clone, Display, Debug, Eq, PartialEq, Hash))]
pub struct StudyId(String);

impl ResourceId for StudyId {
    type Item = Option<()>;

    fn uri(&self) -> String {
        format!("/studies/{}", &self)
    }
}

impl<T> DicomResourceId<T> for StudyId {
    type Item = Study<T>;
}

impl HierarchalResourceId for StudyId {
    type Ancestor = PatientId;
}

impl AnonymizableId for StudyId {}

/// ID of a DICOM series stored by Orthanc.
#[nutype(derive(Serialize, Deserialize, Clone, Display, Debug, Eq, PartialEq, Hash))]
pub struct SeriesId(String);

impl ResourceId for SeriesId {
    type Item = Option<()>;

    fn uri(&self) -> String {
        format!("/series/{}", &self)
    }
}

impl<T> DicomResourceId<T> for SeriesId {
    type Item = Series<T>;
}

impl HierarchalResourceId for SeriesId {
    type Ancestor = StudyId;
}

impl AnonymizableId for SeriesId {}

/// ID of a DICOM instance stored by Orthanc.
#[nutype(derive(Serialize, Deserialize, Clone, Display, Debug, Eq, PartialEq, Hash))]
pub struct InstanceId(String);

impl ResourceId for InstanceId {
    type Item = Option<()>;

    fn uri(&self) -> String {
        format!("/instances/{}", &self)
    }
}

impl<T> DicomResourceId<T> for InstanceId {
    type Item = Instance<T>;
}

impl HierarchalResourceId for InstanceId {
    type Ancestor = SeriesId;
}

/// ID of an Orthanc resource.
pub trait ResourceId {
    type Item: serde::de::DeserializeOwned;

    /// Get the API URI of this resource.
    fn uri(&self) -> String;
}

impl<T> ResourceId for &T
where
    T: ResourceId,
{
    type Item = T::Item;

    fn uri(&self) -> String {
        (*self).uri()
    }
}

/// ID of an Orthanc resource that can be anonymized.
pub trait AnonymizableId: ResourceId {
    /// Get the API URI for anonymizing this resource.
    fn anonymize_uri(&self) -> String {
        format!("{}/anonymize", self.uri())
    }
}

impl<T> AnonymizableId for &T
where
    T: AnonymizableId,
{
    fn anonymize_uri(&self) -> String {
        (*self).anonymize_uri()
    }
}

/// ID of an Orthanc DICOM resource, e.g. patient, study, series, instance.
pub trait DicomResourceId<T>: ResourceId {
    type Item: DicomResource<T>;
}

impl<I, T> DicomResourceId<T> for &I
where
    I: DicomResourceId<T>,
{
    type Item = <I as DicomResourceId<T>>::Item;
}

/// ID of an hierarchal DICOM resource.
pub trait HierarchalResourceId: ResourceId {
    type Ancestor: ResourceId;
}

impl<T> HierarchalResourceId for &T
where
    T: HierarchalResourceId,
{
    type Ancestor = T::Ancestor;
}

/// A type for the "RequestedDicomTags" field in Orthanc's JSON response to
/// getting a DICOM patient, study, series, or instance.
pub trait RequestedTags {
    /// DICOM tag names of this type.
    fn names() -> &'static [&'static str];
    // TODO it would be nice to have a derive trait which can figure out the
    // names from its serde serialized names
}

/// Response from deleting a DICOM resource from Orthanc.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteResponse<T> {
    pub remaining_ancestor: Option<IdAndPath<T>>,
}

/// ID and path of an Orthanc resource.
///
/// Note: the Orthanc response typically has `{ "Type": "Patient|Study|Series|Instance" }`,
/// which is missing from this struct because the information is conveyed by the generic type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Hash)]
pub struct IdAndPath<T> {
    #[serde(rename = "ID")]
    pub id: T,
    #[serde(rename = "Path")]
    pub path: String,
}
