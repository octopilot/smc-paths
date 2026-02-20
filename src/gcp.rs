//! GCP Secret Manager and Parameter Manager API Paths
//!
//! Path functions now accept typed parameters ([`ProjectId`], [`SecretName`], etc.)
//! and return [`TypedPath`] so every output format is derived without `str::replace`.

use crate::parameters::{LocationId, ParameterName, ProjectId, SecretName, VersionId};
use crate::typed_path::TypedPath;

/// GCP Secret Manager API paths
pub mod secret_manager {
    use super::*;

    /// Base route template for Secret Manager operations.
    pub const BASE: &str = "/v1/projects/{project}/secrets";

    /// `POST /v1/projects/{project}/secrets`
    pub fn create_secret(project: &ProjectId) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets").bind("project", project.as_str())
    }

    /// `GET /v1/projects/{project}/secrets`
    pub fn list_secrets(project: &ProjectId) -> TypedPath {
        create_secret(project) // same path, different HTTP method
    }

    /// `GET /v1/projects/{project}/secrets/{secret}`
    pub fn get_secret_metadata(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `PATCH /v1/projects/{project}/secrets/{secret}`
    pub fn update_secret_metadata(project: &ProjectId, secret: &SecretName) -> TypedPath {
        get_secret_metadata(project, secret)
    }

    /// `DELETE /v1/projects/{project}/secrets/{secret}`
    pub fn delete_secret(project: &ProjectId, secret: &SecretName) -> TypedPath {
        get_secret_metadata(project, secret)
    }

    /// `POST /v1/projects/{project}/secrets/{secret}:addVersion`
    pub fn add_version(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}:addVersion")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `POST /v1/projects/{project}/secrets/{secret}:enable`
    pub fn enable_secret(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}:enable")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `POST /v1/projects/{project}/secrets/{secret}:disable`
    pub fn disable_secret(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}:disable")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `GET /v1/projects/{project}/secrets/{secret}/versions`
    pub fn list_versions(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}/versions")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `GET /v1/projects/{project}/secrets/{secret}/versions/{version}`
    pub fn get_version(
        project: &ProjectId,
        secret: &SecretName,
        version: &VersionId,
    ) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}/versions/{version}")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
            .bind("version", version.as_str())
    }

    /// `GET /v1/projects/{project}/secrets/{secret}/versions/latest:access`
    pub fn access_latest_version(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}/versions/latest:access")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `GET /v1/projects/{project}/secrets/{secret}/versions/{version}:access`
    pub fn access_version(
        project: &ProjectId,
        secret: &SecretName,
        version: &VersionId,
    ) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}/versions/{version}:access")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
            .bind("version", version.as_str())
    }

    // ── Controller-style helpers (no /v1/ prefix) ─────────────────────────────

    /// `projects/{project}/secrets/{secret}` (no `/v1/` prefix, controller style)
    pub fn secret_path(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("projects/{project}/secrets/{secret}")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `projects/{project}/secrets` (no `/v1/` prefix, controller style)
    pub fn secrets_base(project: &ProjectId) -> TypedPath {
        TypedPath::new("projects/{project}/secrets").bind("project", project.as_str())
    }

    /// `projects/{project}/secrets/{secret}:addVersion` (no `/v1/` prefix)
    pub fn add_version_path(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("projects/{project}/secrets/{secret}:addVersion")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `projects/{project}/secrets/{secret}/versions/latest:access` (no `/v1/` prefix)
    pub fn access_latest_version_path(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("projects/{project}/secrets/{secret}/versions/latest:access")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `projects/{project}/secrets/{secret}:enable` (no `/v1/` prefix)
    pub fn enable_secret_path(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("projects/{project}/secrets/{secret}:enable")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }

    /// `projects/{project}/secrets/{secret}:disable` (no `/v1/` prefix)
    pub fn disable_secret_path(project: &ProjectId, secret: &SecretName) -> TypedPath {
        TypedPath::new("projects/{project}/secrets/{secret}:disable")
            .bind("project", project.as_str())
            .bind("secret", secret.as_str())
    }
}

/// GCP Parameter Manager API paths
pub mod parameter_manager {
    use super::*;

    /// `POST /v1/projects/{project}/locations/{location}/parameters`
    pub fn create_parameter(project: &ProjectId, location: &LocationId) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/locations/{location}/parameters")
            .bind("project", project.as_str())
            .bind("location", location.as_str())
    }

    /// `GET /v1/projects/{project}/locations/{location}/parameters`
    pub fn list_parameters(project: &ProjectId, location: &LocationId) -> TypedPath {
        create_parameter(project, location)
    }

    /// `GET /v1/projects/{project}/locations/{location}/parameters/{parameter}`
    pub fn get_parameter(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
    ) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/locations/{location}/parameters/{parameter}")
            .bind("project", project.as_str())
            .bind("location", location.as_str())
            .bind("parameter", parameter.as_str())
    }

    /// `PATCH /v1/projects/{project}/locations/{location}/parameters/{parameter}`
    pub fn update_parameter(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
    ) -> TypedPath {
        get_parameter(project, location, parameter)
    }

    /// `DELETE /v1/projects/{project}/locations/{location}/parameters/{parameter}`
    pub fn delete_parameter(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
    ) -> TypedPath {
        get_parameter(project, location, parameter)
    }

    /// `POST /v1/projects/{project}/locations/{location}/parameters/{parameter}/versions`
    pub fn create_version(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
    ) -> TypedPath {
        TypedPath::new(
            "/v1/projects/{project}/locations/{location}/parameters/{parameter}/versions",
        )
        .bind("project", project.as_str())
        .bind("location", location.as_str())
        .bind("parameter", parameter.as_str())
    }

    /// `GET /v1/projects/{project}/locations/{location}/parameters/{parameter}/versions`
    pub fn list_versions(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
    ) -> TypedPath {
        create_version(project, location, parameter)
    }

    /// `GET /v1/…/parameters/{parameter}/versions/{version}`
    pub fn get_version(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
        version: &VersionId,
    ) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/locations/{location}/parameters/{parameter}/versions/{version}")
            .bind("project",   project.as_str())
            .bind("location",  location.as_str())
            .bind("parameter", parameter.as_str())
            .bind("version",   version.as_str())
    }

    /// `PATCH /v1/…/parameters/{parameter}/versions/{version}`
    pub fn update_version(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
        version: &VersionId,
    ) -> TypedPath {
        get_version(project, location, parameter, version)
    }

    /// `DELETE /v1/…/parameters/{parameter}/versions/{version}`
    pub fn delete_version(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
        version: &VersionId,
    ) -> TypedPath {
        get_version(project, location, parameter, version)
    }

    /// `GET /v1/…/parameters/{parameter}/versions/{version}:render`
    pub fn render_version(
        project: &ProjectId,
        location: &LocationId,
        parameter: &ParameterName,
        version: &VersionId,
    ) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/locations/{location}/parameters/{parameter}/versions/{version}:render")
            .bind("project",   project.as_str())
            .bind("location",  location.as_str())
            .bind("parameter", parameter.as_str())
            .bind("version",   version.as_str())
    }

    /// `GET /v1/projects/{project}/locations/{location}`
    pub fn get_location(project: &ProjectId, location: &LocationId) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/locations/{location}")
            .bind("project", project.as_str())
            .bind("location", location.as_str())
    }

    /// `GET /v1/projects/{project}/locations`
    pub fn list_locations(project: &ProjectId) -> TypedPath {
        TypedPath::new("/v1/projects/{project}/locations").bind("project", project.as_str())
    }
}

/// Route constants (URI templates) for Axum — unchanged from before.
pub mod routes {
    pub mod secret_manager {
        pub const CREATE_SECRET: &str = "/v1/projects/{project}/secrets";
        pub const SECRET: &str = "/v1/projects/{project}/secrets/{secret}";
        pub const SECRET_VERSIONS: &str = "/v1/projects/{project}/secrets/{secret}/versions";
        pub const SECRET_VERSION: &str =
            "/v1/projects/{project}/secrets/{secret}/versions/{version}";
    }
    pub mod parameter_manager {
        pub const CREATE_PARAMETER: &str =
            "/v1/projects/{project}/locations/{location}/parameters";
        pub const PARAMETER: &str =
            "/v1/projects/{project}/locations/{location}/parameters/{parameter}";
        pub const PARAMETER_VERSIONS: &str =
            "/v1/projects/{project}/locations/{location}/parameters/{parameter}/versions";
        pub const PARAMETER_VERSION: &str =
            "/v1/projects/{project}/locations/{location}/parameters/{parameter}/versions/{version}";
    }
    pub mod locations {
        pub const LOCATION: &str = "/v1/projects/{project}/locations/{location}";
        pub const LIST_LOCATIONS: &str = "/v1/projects/{project}/locations";
    }
}

#[cfg(test)]
mod route_validation {
    use super::routes;
    use crate::builder::PathBuilder;
    use crate::operations::GcpOperation;

    #[test]
    fn validate_secret_manager_routes() {
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::CreateSecret)
                .project("test-project")
                .build_route()
                .unwrap(),
            routes::secret_manager::CREATE_SECRET
        );
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::GetSecret)
                .project("test-project")
                .secret("test-secret")
                .build_route()
                .unwrap(),
            routes::secret_manager::SECRET
        );
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::ListVersions)
                .project("test-project")
                .secret("test-secret")
                .build_route()
                .unwrap(),
            routes::secret_manager::SECRET_VERSIONS
        );
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::GetVersion)
                .project("test-project")
                .secret("test-secret")
                .version("123")
                .build_route()
                .unwrap(),
            routes::secret_manager::SECRET_VERSION
        );
    }

    #[test]
    fn validate_parameter_manager_routes() {
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::CreateParameter)
                .project("test-project")
                .location("us-central1")
                .build_route()
                .unwrap(),
            routes::parameter_manager::CREATE_PARAMETER
        );
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::GetParameter)
                .project("test-project")
                .location("us-central1")
                .parameter("test-parameter")
                .build_route()
                .unwrap(),
            routes::parameter_manager::PARAMETER
        );
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::ListParameterVersions)
                .project("test-project")
                .location("us-central1")
                .parameter("test-parameter")
                .build_route()
                .unwrap(),
            routes::parameter_manager::PARAMETER_VERSIONS
        );
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::GetParameterVersion)
                .project("test-project")
                .location("us-central1")
                .parameter("test-parameter")
                .version("123")
                .build_route()
                .unwrap(),
            routes::parameter_manager::PARAMETER_VERSION
        );
    }

    #[test]
    fn validate_location_routes() {
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::GetLocation)
                .project("test-project")
                .location("us-central1")
                .build_route()
                .unwrap(),
            routes::locations::LOCATION
        );
        assert_eq!(
            PathBuilder::new()
                .gcp_operation(GcpOperation::ListLocations)
                .project("test-project")
                .build_route()
                .unwrap(),
            routes::locations::LIST_LOCATIONS
        );
    }
}
