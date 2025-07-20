#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

#[cfg(feature = "client")]
pub mod apis;

/// Orthanc OpenAPI request and response models.
pub mod models;
