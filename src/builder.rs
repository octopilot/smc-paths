//! PathBuilder — type-safe builder for constructing API paths.
//!
//! Internally the builder stores typed parameters ([`ProjectId`], [`SecretName`],
//! etc.) and delegates to the provider-specific path functions that return
//! [`TypedPath`].  This eliminates the previous `str::replace` approach that
//! caused substring collisions (e.g. `"proj"` inside `"projects"`).
//!
//! # Example
//!
//! ```rust
//! use smc_paths::prelude::*;
//!
//! let path = PathBuilder::new()
//!     .gcp_operation(GcpOperation::CreateSecret)
//!     .project("my-project")
//!     .build_http_path();
//! // Returns: Ok("projects/my-project/secrets")
//! ```

use crate::errors::PathBuilderError;
use crate::formats::PathFormat;
use crate::gcp;
use crate::operations::{AwsOperation, AzureOperation, GcpOperation, Operation};
use crate::parameters::{LocationId, ParameterName, ProjectId, SecretName, VersionId, VaultName};
use crate::provider::Provider;

/// Builder for constructing API paths with type safety.
#[derive(Debug, Clone)]
pub struct PathBuilder {
    provider: Option<Provider>,
    operation: Option<Operation>,

    // GCP parameters
    project: Option<ProjectId>,
    location: Option<LocationId>,
    secret: Option<SecretName>,
    parameter: Option<ParameterName>,
    version: Option<VersionId>,

    // AWS parameters (reserved for future use)
    #[allow(dead_code)]
    region: Option<crate::parameters::RegionId>,

    // Azure parameters
    vault_name: Option<VaultName>,

    /// Azure trailing-slash toggle (GET /secrets/{name}/ vs PUT /secrets/{name}).
    use_trailing_slash: bool,
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PathBuilder {
    /// Create a new, empty [`PathBuilder`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            provider: None,
            operation: None,
            project: None,
            location: None,
            secret: None,
            parameter: None,
            version: None,
            region: None,
            vault_name: None,
            use_trailing_slash: false,
        }
    }

    // ── Provider / operation setters ──────────────────────────────────────────

    /// Set the cloud provider explicitly.
    #[must_use]
    pub fn provider(mut self, provider: Provider) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Set an operation (and auto-derive the provider).
    #[must_use]
    pub fn operation(mut self, operation: Operation) -> Self {
        self.operation = Some(operation);
        self.provider = Some(match operation {
            Operation::Gcp(_) => Provider::Gcp,
            Operation::Aws(_) => Provider::Aws,
            Operation::Azure(_) => Provider::Azure,
        });
        self
    }

    /// Convenience: set a GCP operation (also sets provider to GCP).
    #[must_use]
    pub fn gcp_operation(mut self, operation: GcpOperation) -> Self {
        self.provider = Some(Provider::Gcp);
        self.operation = Some(Operation::Gcp(operation));
        self
    }

    /// Convenience: set an AWS operation (also sets provider to AWS).
    #[must_use]
    pub fn aws_operation(mut self, operation: AwsOperation) -> Self {
        self.provider = Some(Provider::Aws);
        self.operation = Some(Operation::Aws(operation));
        self
    }

    /// Convenience: set an Azure operation (also sets provider to Azure).
    #[must_use]
    pub fn azure_operation(mut self, operation: AzureOperation) -> Self {
        self.provider = Some(Provider::Azure);
        self.operation = Some(Operation::Azure(operation));
        self
    }

    // ── Parameter setters ─────────────────────────────────────────────────────

    /// Set the GCP project ID.
    #[must_use]
    pub fn project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(ProjectId::new(project));
        self
    }

    /// Set the GCP location / region.
    #[must_use]
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(LocationId::new(location));
        self
    }

    /// Set the secret name.
    #[must_use]
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(SecretName::new(secret));
        self
    }

    /// Set the parameter name (GCP Parameter Manager).
    #[must_use]
    pub fn parameter(mut self, parameter: impl Into<String>) -> Self {
        self.parameter = Some(ParameterName::new(parameter));
        self
    }

    /// Set the version identifier.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(VersionId::new(version));
        self
    }

    /// Set the AWS region (reserved for future path functions).
    #[must_use]
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(crate::parameters::RegionId::new(region));
        self
    }

    /// Set the Azure Key Vault name.
    #[must_use]
    pub fn vault_name(mut self, vault_name: impl Into<String>) -> Self {
        self.vault_name = Some(VaultName::new(vault_name));
        self
    }

    /// Enable / disable trailing slash for Azure `GetSecret` paths.
    #[must_use]
    pub fn with_trailing_slash(mut self, enabled: bool) -> Self {
        self.use_trailing_slash = enabled;
        self
    }

    // ── Build methods ──────────────────────────────────────────────────────────

    /// Build as an Axum route pattern with `{placeholder}` markers.
    pub fn build_route(&self) -> Result<String, PathBuilderError> {
        self.build(PathFormat::Route)
    }

    /// Build as a concrete HTTP path (no `/v1/` prefix).
    pub fn build_http_path(&self) -> Result<String, PathBuilderError> {
        self.build(PathFormat::HttpPath)
    }

    /// Build as a response name (same as `HttpPath`).
    pub fn build_response_name(&self) -> Result<String, PathBuilderError> {
        self.build(PathFormat::ResponseName)
    }

    /// Build as a Pact contract path (with `/v1/` prefix).
    pub fn build_pact_path(&self) -> Result<String, PathBuilderError> {
        self.build(PathFormat::PactPath)
    }

    /// Return the `x-amz-target` header value for the current AWS operation,
    /// or `None` if the operation does not use a target header.
    pub fn build_aws_header(&self) -> Result<Option<String>, PathBuilderError> {
        self.validate_provider_operation()?;

        match self.operation {
            Some(Operation::Aws(op)) => {
                use crate::aws::secrets_manager;
                let header = match op {
                    AwsOperation::CreateSecret => Some(secrets_manager::CREATE_SECRET),
                    AwsOperation::GetSecretValue => Some(secrets_manager::GET_SECRET_VALUE),
                    AwsOperation::DescribeSecret => Some(secrets_manager::DESCRIBE_SECRET),
                    AwsOperation::PutSecretValue => Some(secrets_manager::PUT_SECRET_VALUE),
                    AwsOperation::UpdateSecret => Some(secrets_manager::UPDATE_SECRET),
                    AwsOperation::DeleteSecret => Some(secrets_manager::DELETE_SECRET),
                    AwsOperation::RestoreSecret => Some(secrets_manager::RESTORE_SECRET),
                    AwsOperation::ListSecrets => Some(secrets_manager::LIST_SECRETS),
                    AwsOperation::ListSecretVersions => {
                        Some(secrets_manager::LIST_SECRET_VERSIONS)
                    }
                    AwsOperation::UpdateSecretVersionStage => {
                        Some(secrets_manager::UPDATE_SECRET_VERSION_STAGE)
                    }
                    AwsOperation::TagResource => Some(secrets_manager::TAG_RESOURCE),
                    AwsOperation::UntagResource => Some(secrets_manager::UNTAG_RESOURCE),
                    AwsOperation::GetResourcePolicy => Some(secrets_manager::GET_RESOURCE_POLICY),
                    _ => None,
                };
                Ok(header.map(std::string::ToString::to_string))
            }
            _ => Ok(None),
        }
    }

    /// Build the path in the requested output format.
    pub fn build(&self, format: PathFormat) -> Result<String, PathBuilderError> {
        self.validate_provider_operation()?;

        match self.operation {
            Some(Operation::Gcp(op)) => Ok(self.build_gcp(op)?.build(format)),
            Some(Operation::Aws(op)) => self.build_aws(op, format),
            Some(Operation::Azure(op)) => Ok(self.build_azure(op)?.build(format)),
            None => Err(PathBuilderError::MissingRequiredParameter(
                "operation".to_string(),
            )),
        }
    }

    // ── Validation ────────────────────────────────────────────────────────────

    fn validate_provider_operation(&self) -> Result<(), PathBuilderError> {
        match (&self.provider, &self.operation) {
            (Some(Provider::Gcp), Some(Operation::Gcp(_)))
            | (Some(Provider::Aws), Some(Operation::Aws(_)))
            | (Some(Provider::Azure), Some(Operation::Azure(_))) => Ok(()),
            (Some(_), Some(_)) => Err(PathBuilderError::ProviderOperationMismatch),
            (None, Some(_)) => Ok(()), // provider auto-set from operation
            (_, None) => Err(PathBuilderError::MissingRequiredParameter(
                "operation".to_string(),
            )),
        }
    }

    // ── Provider-specific builders ─────────────────────────────────────────────

    fn req_project(&self) -> Result<&ProjectId, PathBuilderError> {
        self.project
            .as_ref()
            .ok_or_else(|| PathBuilderError::MissingRequiredParameter("project".into()))
    }

    fn req_secret(&self) -> Result<&SecretName, PathBuilderError> {
        self.secret
            .as_ref()
            .ok_or_else(|| PathBuilderError::MissingRequiredParameter("secret".into()))
    }

    fn req_version(&self) -> Result<&VersionId, PathBuilderError> {
        self.version
            .as_ref()
            .ok_or_else(|| PathBuilderError::MissingRequiredParameter("version".into()))
    }

    fn req_location(&self) -> Result<&LocationId, PathBuilderError> {
        self.location
            .as_ref()
            .ok_or_else(|| PathBuilderError::MissingRequiredParameter("location".into()))
    }

    fn req_parameter(&self) -> Result<&ParameterName, PathBuilderError> {
        self.parameter
            .as_ref()
            .ok_or_else(|| PathBuilderError::MissingRequiredParameter("parameter".into()))
    }

    fn build_gcp(
        &self,
        op: GcpOperation,
    ) -> Result<crate::typed_path::TypedPath, PathBuilderError> {
        use gcp::{parameter_manager as pm, secret_manager as sm};
        let proj = self.req_project()?;

        Ok(match op {
            GcpOperation::CreateSecret | GcpOperation::ListSecrets => sm::create_secret(proj),
            GcpOperation::GetSecret | GcpOperation::UpdateSecret | GcpOperation::DeleteSecret => {
                sm::get_secret_metadata(proj, self.req_secret()?)
            }
            GcpOperation::AddVersion => sm::add_version(proj, self.req_secret()?),
            GcpOperation::EnableSecret => sm::enable_secret(proj, self.req_secret()?),
            GcpOperation::DisableSecret => sm::disable_secret(proj, self.req_secret()?),
            GcpOperation::ListVersions => sm::list_versions(proj, self.req_secret()?),
            GcpOperation::GetVersion
            | GcpOperation::EnableVersion
            | GcpOperation::DisableVersion => {
                sm::get_version(proj, self.req_secret()?, self.req_version()?)
            }
            GcpOperation::AccessVersion => {
                sm::access_version(proj, self.req_secret()?, self.req_version()?)
            }
            GcpOperation::CreateParameter | GcpOperation::ListParameters => {
                pm::create_parameter(proj, self.req_location()?)
            }
            GcpOperation::GetParameter
            | GcpOperation::UpdateParameter
            | GcpOperation::DeleteParameter => {
                pm::get_parameter(proj, self.req_location()?, self.req_parameter()?)
            }
            GcpOperation::CreateParameterVersion | GcpOperation::ListParameterVersions => {
                pm::create_version(proj, self.req_location()?, self.req_parameter()?)
            }
            GcpOperation::GetParameterVersion
            | GcpOperation::UpdateParameterVersion
            | GcpOperation::DeleteParameterVersion => {
                pm::get_version(proj, self.req_location()?, self.req_parameter()?, self.req_version()?)
            }
            GcpOperation::RenderParameterVersion => {
                pm::render_version(proj, self.req_location()?, self.req_parameter()?, self.req_version()?)
            }
            GcpOperation::GetLocation => pm::get_location(proj, self.req_location()?),
            GcpOperation::ListLocations => pm::list_locations(proj),
        })
    }

    fn build_aws(
        &self,
        _op: AwsOperation,
        format: PathFormat,
    ) -> Result<String, PathBuilderError> {
        // AWS uses a single POST endpoint `/` with an `x-amz-target` header.
        match format {
            PathFormat::Route | PathFormat::HttpPath | PathFormat::PactPath => Ok("/".to_string()),
            PathFormat::ResponseName => Err(PathBuilderError::InvalidFormatForOperation),
        }
    }

    fn build_azure(
        &self,
        op: AzureOperation,
    ) -> Result<crate::typed_path::TypedPath, PathBuilderError> {
        use crate::azure::{app_configuration as ac, key_vault as kv};

        Ok(match op {
            AzureOperation::ListSecrets => {
                crate::typed_path::TypedPath::new(kv::LIST_SECRETS)
            }
            AzureOperation::GetSecret => {
                let name = self.req_secret()?;
                if self.use_trailing_slash {
                    kv::get_secret(name)
                } else {
                    kv::set_secret(name) // same path, no trailing slash
                }
            }
            AzureOperation::GetSecretVersion => {
                kv::get_secret_version(self.req_secret()?, self.req_version()?)
            }
            AzureOperation::ListSecretVersions => kv::list_secret_versions(self.req_secret()?),
            AzureOperation::SetSecret => kv::set_secret(self.req_secret()?),
            AzureOperation::DeleteSecret => kv::delete_secret(self.req_secret()?),
            AzureOperation::UpdateSecret => kv::update_secret(self.req_secret()?),
            AzureOperation::BackupSecret => kv::backup_secret(self.req_secret()?),
            AzureOperation::RestoreSecret => {
                crate::typed_path::TypedPath::new(kv::RESTORE_SECRET)
            }
            AzureOperation::GetDeletedSecret => kv::get_deleted_secret(self.req_secret()?),
            AzureOperation::ListDeletedSecrets => {
                crate::typed_path::TypedPath::new(kv::LIST_DELETED_SECRETS)
            }
            AzureOperation::RecoverDeletedSecret => {
                kv::recover_deleted_secret(self.req_secret()?)
            }
            AzureOperation::PurgeDeletedSecret => kv::purge_deleted_secret(self.req_secret()?),
            AzureOperation::GetKeyValue
            | AzureOperation::SetKeyValue
            | AzureOperation::DeleteKeyValue => {
                let key = self
                    .secret
                    .as_ref()
                    .ok_or_else(|| PathBuilderError::MissingRequiredParameter("key".into()))?;
                match op {
                    AzureOperation::GetKeyValue => ac::get_key_value(key),
                    AzureOperation::SetKeyValue => ac::set_key_value(key),
                    AzureOperation::DeleteKeyValue => ac::delete_key_value(key),
                    _ => unreachable!(),
                }
            }
            AzureOperation::ListKeyValues => {
                crate::typed_path::TypedPath::new(ac::LIST_KEY_VALUES)
            }
        })
    }
}
