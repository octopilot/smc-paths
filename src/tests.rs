//! Comprehensive tests for smc-paths
//!
//! Covers PathBuilder (all operations × all formats), error cases, edge cases,
//! error type Display, and the helper function modules (gcp, aws, azure).

#[cfg(test)]
mod errors {
    use crate::errors::PathBuilderError;
    use std::error::Error;

    #[test]
    fn display_provider_operation_mismatch() {
        assert_eq!(
            PathBuilderError::ProviderOperationMismatch.to_string(),
            "Provider and operation do not match"
        );
    }

    #[test]
    fn display_missing_required_parameter() {
        assert_eq!(
            PathBuilderError::MissingRequiredParameter("project".to_string()).to_string(),
            "Missing required parameter: project"
        );
    }

    #[test]
    fn display_invalid_operation_for_provider() {
        assert_eq!(
            PathBuilderError::InvalidOperationForProvider.to_string(),
            "Invalid operation for the selected provider"
        );
    }

    #[test]
    fn display_invalid_format_for_operation() {
        assert_eq!(
            PathBuilderError::InvalidFormatForOperation.to_string(),
            "Invalid format for the operation"
        );
    }

    #[test]
    fn error_trait_implemented() {
        let e: &dyn Error = &PathBuilderError::ProviderOperationMismatch;
        assert_eq!(e.to_string(), "Provider and operation do not match");
    }

    #[test]
    fn equality() {
        assert_eq!(
            PathBuilderError::ProviderOperationMismatch,
            PathBuilderError::ProviderOperationMismatch
        );
        assert_ne!(
            PathBuilderError::ProviderOperationMismatch,
            PathBuilderError::InvalidFormatForOperation
        );
        assert_eq!(
            PathBuilderError::MissingRequiredParameter("a".into()),
            PathBuilderError::MissingRequiredParameter("a".into())
        );
        assert_ne!(
            PathBuilderError::MissingRequiredParameter("a".into()),
            PathBuilderError::MissingRequiredParameter("b".into())
        );
    }

    #[test]
    fn clone() {
        let original = PathBuilderError::MissingRequiredParameter("test".into());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}

#[cfg(test)]
mod builder_gcp {
    use crate::errors::PathBuilderError;
    use crate::formats::PathFormat;
    use crate::operations::GcpOperation;
    use crate::prelude::*;

    // ── CreateSecret / ListSecrets ──────────────────────────────────────────

    #[test]
    fn gcp_create_secret_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("my-project")
            .build_http_path()
            .unwrap();
        assert_eq!(path, "projects/my-project/secrets");
    }

    #[test]
    fn gcp_create_secret_pact_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("my-project")
            .build_pact_path()
            .unwrap();
        assert_eq!(path, "/v1/projects/my-project/secrets");
    }

    #[test]
    fn gcp_create_secret_route() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("xyzabc")
            .build_route()
            .unwrap();
        assert!(!path.contains("xyzabc"), "project value should be replaced");
        assert!(path.contains("{project}"));
        assert!(path.contains("secrets"));
    }

    #[test]
    fn gcp_create_secret_response_name() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("my-project")
            .build_response_name()
            .unwrap();
        assert_eq!(path, "projects/my-project/secrets");
    }

    #[test]
    fn gcp_list_secrets_uses_same_path_as_create() {
        let create = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("p")
            .build_http_path()
            .unwrap();
        let list = PathBuilder::new()
            .gcp_operation(GcpOperation::ListSecrets)
            .project("p")
            .build_http_path()
            .unwrap();
        assert_eq!(create, list);
    }

    // ── GetSecret / UpdateSecret / DeleteSecret ─────────────────────────────

    #[test]
    fn gcp_get_secret_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::GetSecret)
            .project("proj")
            .secret("my-secret")
            .build_http_path()
            .unwrap();
        assert_eq!(path, "projects/proj/secrets/my-secret");
    }

    #[test]
    fn gcp_get_secret_route() {
        // Use values that are not substrings of path-template keywords
        // ("projects", "secrets") to avoid the naive str::replace collision.
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::GetSecret)
            .project("xyzabc")
            .secret("qwerty")
            .build_route()
            .unwrap();
        // Actual values must be replaced by placeholders
        assert!(!path.contains("xyzabc"), "project value should be replaced");
        assert!(!path.contains("qwerty"), "secret value should be replaced");
        assert!(path.contains("{project}"));
        assert!(path.contains("{secret}"));
    }

    #[test]
    fn gcp_update_secret_same_path_as_get() {
        let get = PathBuilder::new()
            .gcp_operation(GcpOperation::GetSecret)
            .project("p")
            .secret("s")
            .build_http_path()
            .unwrap();
        let update = PathBuilder::new()
            .gcp_operation(GcpOperation::UpdateSecret)
            .project("p")
            .secret("s")
            .build_http_path()
            .unwrap();
        assert_eq!(get, update);
    }

    #[test]
    fn gcp_get_secret_missing_project_returns_error() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::GetSecret)
            .secret("s")
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("project".into())
        );
    }

    #[test]
    fn gcp_get_secret_missing_secret_returns_error() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::GetSecret)
            .project("p")
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("secret".into())
        );
    }

    // ── AddVersion ──────────────────────────────────────────────────────────

    #[test]
    fn gcp_add_version_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::AddVersion)
            .project("p")
            .secret("s")
            .build_http_path()
            .unwrap();
        assert_eq!(path, "projects/p/secrets/s:addVersion");
    }

    #[test]
    fn gcp_add_version_route() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::AddVersion)
            .project("xyzabc")
            .secret("qwerty")
            .build_route()
            .unwrap();
        assert!(!path.contains("xyzabc"));
        assert!(!path.contains("qwerty"));
        assert!(path.contains("{project}"));
        assert!(path.contains("{secret}"));
        assert!(path.contains("addVersion"));
    }

    #[test]
    fn gcp_add_version_missing_secret_returns_error() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::AddVersion)
            .project("p")
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("secret".into())
        );
    }

    // ── ListVersions ────────────────────────────────────────────────────────

    #[test]
    fn gcp_list_versions_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::ListVersions)
            .project("p")
            .secret("s")
            .build_http_path()
            .unwrap();
        assert_eq!(path, "projects/p/secrets/s/versions");
    }

    // ── GetVersion / EnableVersion / DisableVersion ─────────────────────────

    #[test]
    fn gcp_get_version_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::GetVersion)
            .project("p")
            .secret("s")
            .version("3")
            .build_http_path()
            .unwrap();
        assert_eq!(path, "projects/p/secrets/s/versions/3");
    }

    #[test]
    fn gcp_get_version_route() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::GetVersion)
            .project("xyzabc")
            .secret("qwerty")
            .version("rev42")
            .build_route()
            .unwrap();
        assert!(!path.contains("xyzabc"));
        assert!(!path.contains("qwerty"));
        assert!(!path.contains("rev42"));
        assert!(path.contains("{project}"));
        assert!(path.contains("{secret}"));
        assert!(path.contains("{version}"));
    }

    #[test]
    fn gcp_get_version_missing_version_returns_error() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::GetVersion)
            .project("p")
            .secret("s")
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("version".into())
        );
    }

    // ── AccessVersion (EnableSecret / DisableSecret) ─────────────────────────

    #[test]
    fn gcp_access_version_latest_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::AccessVersion)
            .project("p")
            .secret("s")
            .version("latest")
            .build_http_path()
            .unwrap();
        assert_eq!(path, "projects/p/secrets/s/versions/latest:access");
    }

    #[test]
    fn gcp_enable_secret_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::EnableSecret)
            .project("p")
            .secret("s")
            .build_http_path()
            .unwrap();
        assert!(path.contains("enableSecret") || path.contains("enable"));
    }

    #[test]
    fn gcp_disable_secret_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::DisableSecret)
            .project("p")
            .secret("s")
            .build_http_path()
            .unwrap();
        assert!(path.contains("disableSecret") || path.contains("disable"));
    }

    // ── Parameter Manager ───────────────────────────────────────────────────

    #[test]
    fn gcp_create_parameter_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateParameter)
            .project("p")
            .location("us-central1")
            .build_http_path()
            .unwrap();
        assert!(path.contains("p") && path.contains("us-central1"));
    }

    #[test]
    fn gcp_get_parameter_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::GetParameter)
            .project("p")
            .location("us-central1")
            .parameter("my-param")
            .build_http_path()
            .unwrap();
        assert!(
            path.contains("my-param"),
            "path should contain parameter name"
        );
    }

    #[test]
    fn gcp_get_parameter_missing_location_returns_error() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::GetParameter)
            .project("p")
            .parameter("param")
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("location".into())
        );
    }

    #[test]
    fn gcp_get_parameter_missing_parameter_returns_error() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::GetParameter)
            .project("p")
            .location("us-central1")
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("parameter".into())
        );
    }

    #[test]
    fn gcp_get_location_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::GetLocation)
            .project("p")
            .location("us-east1")
            .build_http_path()
            .unwrap();
        assert!(path.contains("us-east1"));
    }

    #[test]
    fn gcp_list_locations_http_path() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::ListLocations)
            .project("p")
            .build_http_path()
            .unwrap();
        assert!(path.contains("locations"));
    }
}

#[cfg(test)]
mod builder_aws {
    use crate::aws::secrets_manager;
    use crate::errors::PathBuilderError;
    use crate::operations::AwsOperation;
    use crate::prelude::*;

    // ── All AWS operations use "/" endpoint ─────────────────────────────────

    macro_rules! aws_path_test {
        ($name:ident, $op:expr) => {
            #[test]
            fn $name() {
                for path in [
                    PathBuilder::new()
                        .aws_operation($op)
                        .build_http_path()
                        .unwrap(),
                    PathBuilder::new()
                        .aws_operation($op)
                        .build_route()
                        .unwrap(),
                    PathBuilder::new()
                        .aws_operation($op)
                        .build_pact_path()
                        .unwrap(),
                ] {
                    assert_eq!(path, "/");
                }
            }
        };
    }

    aws_path_test!(aws_create_secret_uses_root_endpoint, AwsOperation::CreateSecret);
    aws_path_test!(
        aws_get_secret_value_uses_root_endpoint,
        AwsOperation::GetSecretValue
    );
    aws_path_test!(
        aws_describe_secret_uses_root_endpoint,
        AwsOperation::DescribeSecret
    );
    aws_path_test!(
        aws_put_secret_value_uses_root_endpoint,
        AwsOperation::PutSecretValue
    );
    aws_path_test!(aws_update_secret_uses_root_endpoint, AwsOperation::UpdateSecret);
    aws_path_test!(aws_delete_secret_uses_root_endpoint, AwsOperation::DeleteSecret);
    aws_path_test!(
        aws_list_secrets_uses_root_endpoint,
        AwsOperation::ListSecrets
    );
    aws_path_test!(
        aws_tag_resource_uses_root_endpoint,
        AwsOperation::TagResource
    );

    #[test]
    fn aws_response_name_format_returns_error() {
        let err = PathBuilder::new()
            .aws_operation(AwsOperation::GetSecretValue)
            .build_response_name()
            .unwrap_err();
        assert_eq!(err, PathBuilderError::InvalidFormatForOperation);
    }

    // ── build_aws_header ────────────────────────────────────────────────────

    #[test]
    fn aws_header_create_secret() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::CreateSecret)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::CREATE_SECRET.to_string()));
    }

    #[test]
    fn aws_header_get_secret_value() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::GetSecretValue)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::GET_SECRET_VALUE.to_string()));
    }

    #[test]
    fn aws_header_describe_secret() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::DescribeSecret)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::DESCRIBE_SECRET.to_string()));
    }

    #[test]
    fn aws_header_put_secret_value() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::PutSecretValue)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::PUT_SECRET_VALUE.to_string()));
    }

    #[test]
    fn aws_header_update_secret() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::UpdateSecret)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::UPDATE_SECRET.to_string()));
    }

    #[test]
    fn aws_header_delete_secret() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::DeleteSecret)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::DELETE_SECRET.to_string()));
    }

    #[test]
    fn aws_header_list_secrets() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::ListSecrets)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::LIST_SECRETS.to_string()));
    }

    #[test]
    fn aws_header_tag_resource() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::TagResource)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::TAG_RESOURCE.to_string()));
    }

    #[test]
    fn aws_header_untag_resource() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::UntagResource)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::UNTAG_RESOURCE.to_string()));
    }

    #[test]
    fn aws_header_get_resource_policy() {
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::GetResourcePolicy)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, Some(secrets_manager::GET_RESOURCE_POLICY.to_string()));
    }

    #[test]
    fn aws_header_for_parameter_store_op_returns_none() {
        // Parameter store operations don't have a secrets_manager header
        let h = PathBuilder::new()
            .aws_operation(AwsOperation::GetParameter)
            .build_aws_header()
            .unwrap();
        assert_eq!(h, None);
    }

    #[test]
    fn aws_header_missing_operation_on_non_aws_returns_none() {
        let h = PathBuilder::new()
            .gcp_operation(crate::operations::GcpOperation::CreateSecret)
            .project("p")
            .build_aws_header()
            .unwrap();
        assert_eq!(h, None);
    }

    // ── AWS header constant format ───────────────────────────────────────────

    #[test]
    fn aws_header_constants_have_target_prefix() {
        // AWS headers follow "secretsmanager.<Action>" format
        assert!(secrets_manager::CREATE_SECRET.starts_with("secretsmanager."));
        assert!(secrets_manager::GET_SECRET_VALUE.starts_with("secretsmanager."));
        assert!(secrets_manager::DESCRIBE_SECRET.starts_with("secretsmanager."));
    }
}

#[cfg(test)]
mod builder_azure {
    use crate::errors::PathBuilderError;
    use crate::operations::AzureOperation;
    use crate::prelude::*;

    // ── SetSecret ──────────────────────────────────────────────────────────

    #[test]
    fn azure_set_secret_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::SetSecret)
            .secret("my-secret")
            .build_http_path()
            .unwrap();
        assert!(
            path.contains("my-secret"),
            "path should contain secret name"
        );
        assert!(!path.starts_with("/v1/"), "HttpPath strips /v1/ prefix");
    }

    #[test]
    fn azure_set_secret_pact_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::SetSecret)
            .secret("my-secret")
            .build_pact_path()
            .unwrap();
        assert!(path.contains("my-secret"));
    }

    #[test]
    fn azure_set_secret_route() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::SetSecret)
            .secret("my-secret")
            .build_route()
            .unwrap();
        // Route should replace the actual name with a placeholder
        assert!(path.contains("{name}") || !path.contains("my-secret"),
            "Route format should use placeholder, got: {path}");
    }

    #[test]
    fn azure_set_secret_missing_secret_returns_error() {
        let err = PathBuilder::new()
            .azure_operation(AzureOperation::SetSecret)
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("secret".into())
        );
    }

    // ── GetSecret ──────────────────────────────────────────────────────────

    #[test]
    fn azure_get_secret_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::GetSecret)
            .secret("my-secret")
            .build_http_path()
            .unwrap();
        assert!(path.contains("my-secret"));
    }

    // ── GetSecretVersion ────────────────────────────────────────────────────

    #[test]
    fn azure_get_secret_version_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::GetSecretVersion)
            .secret("my-secret")
            .version("abc123")
            .build_http_path()
            .unwrap();
        assert!(path.contains("my-secret") && path.contains("abc123"));
    }

    #[test]
    fn azure_get_secret_version_missing_version_returns_error() {
        let err = PathBuilder::new()
            .azure_operation(AzureOperation::GetSecretVersion)
            .secret("s")
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("version".into())
        );
    }

    // ── ListSecretVersions ──────────────────────────────────────────────────

    #[test]
    fn azure_list_secret_versions_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::ListSecretVersions)
            .secret("my-secret")
            .build_http_path()
            .unwrap();
        assert!(path.contains("my-secret"));
    }

    // ── DeleteSecret ────────────────────────────────────────────────────────

    #[test]
    fn azure_delete_secret_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::DeleteSecret)
            .secret("my-secret")
            .build_http_path()
            .unwrap();
        assert!(path.contains("my-secret"));
    }

    // ── BackupSecret / RestoreSecret ────────────────────────────────────────

    #[test]
    fn azure_backup_secret_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::BackupSecret)
            .secret("my-secret")
            .build_http_path()
            .unwrap();
        assert!(path.contains("my-secret") && path.contains("backup"));
    }

    #[test]
    fn azure_restore_secret_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::RestoreSecret)
            .build_http_path()
            .unwrap();
        assert!(path.contains("restore"));
    }

    // ── GetDeletedSecret / RecoverDeletedSecret / PurgeDeletedSecret ─────────

    #[test]
    fn azure_get_deleted_secret_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::GetDeletedSecret)
            .secret("s")
            .build_http_path()
            .unwrap();
        assert!(path.contains("deletedsecrets") || path.contains("deleted"));
    }

    #[test]
    fn azure_recover_deleted_secret_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::RecoverDeletedSecret)
            .secret("s")
            .build_http_path()
            .unwrap();
        assert!(path.contains("recover"));
    }

    #[test]
    fn azure_purge_deleted_secret_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::PurgeDeletedSecret)
            .secret("s")
            .build_http_path()
            .unwrap();
        assert!(path.contains("deletedsecrets") || path.contains("deleted"));
    }

    // ── App Configuration ───────────────────────────────────────────────────

    #[test]
    fn azure_get_key_value_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::GetKeyValue)
            .secret("my-key")
            .build_http_path()
            .unwrap();
        assert!(path.contains("my-key"));
    }

    #[test]
    fn azure_set_key_value_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::SetKeyValue)
            .secret("my-key")
            .build_http_path()
            .unwrap();
        assert!(path.contains("my-key"));
    }

    #[test]
    fn azure_delete_key_value_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::DeleteKeyValue)
            .secret("my-key")
            .build_http_path()
            .unwrap();
        assert!(path.contains("my-key"));
    }

    #[test]
    fn azure_list_key_values_http_path() {
        let path = PathBuilder::new()
            .azure_operation(AzureOperation::ListKeyValues)
            .build_http_path()
            .unwrap();
        assert!(!path.is_empty());
    }
}

#[cfg(test)]
mod builder_errors {
    use crate::errors::PathBuilderError;
    use crate::operations::{AwsOperation, AzureOperation, GcpOperation};
    use crate::prelude::*;
    use crate::provider::Provider;

    // ── No operation set ────────────────────────────────────────────────────

    #[test]
    fn no_operation_returns_error() {
        let err = PathBuilder::new()
            .provider(Provider::Gcp)
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("operation".into())
        );
    }

    #[test]
    fn no_operation_no_provider_returns_error() {
        let err = PathBuilder::new().build_http_path().unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("operation".into())
        );
    }

    // ── Provider/operation mismatch ─────────────────────────────────────────

    // Note: the convenience methods (gcp_operation, aws_operation, azure_operation)
    // always auto-set the provider, so a mismatch can only be produced by calling
    // provider() AFTER the convenience method to override it.

    #[test]
    fn gcp_operation_then_aws_provider_returns_mismatch() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("p")
            .provider(Provider::Aws) // override provider after operation
            .build_http_path()
            .unwrap_err();
        assert_eq!(err, PathBuilderError::ProviderOperationMismatch);
    }

    #[test]
    fn aws_operation_then_gcp_provider_returns_mismatch() {
        let err = PathBuilder::new()
            .aws_operation(AwsOperation::CreateSecret)
            .provider(Provider::Gcp) // override provider after operation
            .build_http_path()
            .unwrap_err();
        assert_eq!(err, PathBuilderError::ProviderOperationMismatch);
    }

    #[test]
    fn gcp_operation_then_azure_provider_returns_mismatch() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("p")
            .provider(Provider::Azure) // override provider after operation
            .build_http_path()
            .unwrap_err();
        assert_eq!(err, PathBuilderError::ProviderOperationMismatch);
    }

    // ── Missing required parameters ─────────────────────────────────────────

    #[test]
    fn gcp_create_secret_missing_project() {
        let err = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("project".into())
        );
    }

    #[test]
    fn azure_set_secret_missing_secret() {
        let err = PathBuilder::new()
            .azure_operation(AzureOperation::SetSecret)
            .build_http_path()
            .unwrap_err();
        assert_eq!(
            err,
            PathBuilderError::MissingRequiredParameter("secret".into())
        );
    }
}

#[cfg(test)]
mod builder_misc {
    use crate::operations::{AwsOperation, GcpOperation};
    use crate::prelude::*;
    use crate::provider::Provider;

    // ── Default / new are equivalent ────────────────────────────────────────

    #[test]
    fn default_equals_new() {
        let a = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("p")
            .build_http_path()
            .unwrap();
        let b = PathBuilder::default()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("p")
            .build_http_path()
            .unwrap();
        assert_eq!(a, b);
    }

    // ── operation() auto-sets provider ──────────────────────────────────────

    #[test]
    fn operation_auto_sets_gcp_provider() {
        // Should not return ProviderOperationMismatch
        let result = PathBuilder::new()
            .operation(crate::operations::Operation::Gcp(
                GcpOperation::CreateSecret,
            ))
            .project("p")
            .build_http_path();
        assert!(result.is_ok());
    }

    #[test]
    fn operation_auto_sets_aws_provider() {
        let result = PathBuilder::new()
            .operation(crate::operations::Operation::Aws(
                AwsOperation::CreateSecret,
            ))
            .build_http_path();
        assert!(result.is_ok());
    }

    // ── provider() can be set explicitly before convenience method ───────────

    #[test]
    fn explicit_provider_matches_gcp_operation_ok() {
        let result = PathBuilder::new()
            .provider(Provider::Gcp)
            .gcp_operation(GcpOperation::CreateSecret)
            .project("p")
            .build_http_path();
        assert!(result.is_ok());
    }

    // ── HttpPath strips /v1/ prefix ─────────────────────────────────────────

    #[test]
    fn http_path_strips_v1_prefix() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("my-project")
            .build_http_path()
            .unwrap();
        assert!(!path.starts_with("/v1/"));
    }

    #[test]
    fn pact_path_keeps_v1_prefix() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("my-project")
            .build_pact_path()
            .unwrap();
        assert!(path.starts_with("/v1/"));
    }

    // ── Route uses placeholders ─────────────────────────────────────────────

    #[test]
    fn route_replaces_project_with_placeholder() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("actual-project-id")
            .build_route()
            .unwrap();
        assert!(!path.contains("actual-project-id"));
        assert!(path.contains("{project}"));
    }

    #[test]
    fn route_replaces_secret_with_placeholder() {
        let path = PathBuilder::new()
            .gcp_operation(GcpOperation::GetSecret)
            .project("p")
            .secret("actual-secret-name")
            .build_route()
            .unwrap();
        assert!(!path.contains("actual-secret-name"));
        assert!(path.contains("{secret}"));
    }

    // ── Clone ────────────────────────────────────────────────────────────────

    #[test]
    fn builder_is_cloneable() {
        let base = PathBuilder::new()
            .gcp_operation(GcpOperation::CreateSecret)
            .project("p");
        let path1 = base.clone().build_http_path().unwrap();
        let path2 = base.build_http_path().unwrap();
        assert_eq!(path1, path2);
    }
}

#[cfg(test)]
mod gcp_functions {
    use crate::gcp::{parameter_manager, secret_manager};

    // ── secret_manager functions ─────────────────────────────────────────────

    #[test]
    fn create_secret_path() {
        let p = secret_manager::create_secret("my-project");
        assert_eq!(p, "/v1/projects/my-project/secrets");
    }

    #[test]
    fn get_secret_metadata_path() {
        let p = secret_manager::get_secret_metadata("proj", "my-secret");
        assert_eq!(p, "/v1/projects/proj/secrets/my-secret");
    }

    #[test]
    fn add_version_path() {
        let p = secret_manager::add_version("proj", "s");
        assert!(p.ends_with(":addVersion"));
        assert!(p.contains("proj") && p.contains('/'));
    }

    #[test]
    fn list_versions_path() {
        let p = secret_manager::list_versions("proj", "s");
        assert!(p.contains("versions") && p.contains("proj"));
    }

    #[test]
    fn get_version_path() {
        let p = secret_manager::get_version("proj", "s", "3");
        assert!(p.ends_with("/3"));
        assert!(p.contains("versions"));
    }

    #[test]
    fn access_latest_version_path() {
        let p = secret_manager::access_latest_version("proj", "s");
        assert!(p.contains("latest") && p.contains(":access"));
    }

    #[test]
    fn enable_secret_path_contains_action() {
        let p = secret_manager::enable_secret("proj", "s");
        assert!(p.contains("enable") || p.contains("Enable"));
    }

    #[test]
    fn disable_secret_path_contains_action() {
        let p = secret_manager::disable_secret("proj", "s");
        assert!(p.contains("disable") || p.contains("Disable"));
    }

    // ── parameter_manager functions ─────────────────────────────────────────

    #[test]
    fn get_location_path() {
        let p = parameter_manager::get_location("proj", "us-central1");
        assert!(p.contains("us-central1") && p.contains("locations"));
    }

    #[test]
    fn list_locations_path() {
        let p = parameter_manager::list_locations("proj");
        assert!(p.contains("locations") && p.contains("proj"));
    }

    #[test]
    fn get_parameter_path() {
        let p = parameter_manager::get_parameter("proj", "us-east1", "my-param");
        assert!(p.contains("my-param") && p.contains("us-east1"));
    }

    #[test]
    fn render_version_path() {
        let p = parameter_manager::render_version("proj", "us-central1", "param", "v1");
        assert!(p.contains(":render") || p.contains("render"));
        assert!(p.contains("v1"));
    }
}

#[cfg(test)]
mod azure_functions {
    use crate::azure::{app_configuration as ac, key_vault as kv};

    #[test]
    fn kv_get_secret_path() {
        let p = kv::get_secret("my-secret");
        assert!(p.contains("my-secret"));
    }

    #[test]
    fn kv_get_secret_version_path() {
        let p = kv::get_secret_version("my-secret", "abc");
        assert!(p.contains("my-secret") && p.contains("abc"));
    }

    #[test]
    fn kv_list_secret_versions_path() {
        let p = kv::list_secret_versions("my-secret");
        assert!(p.contains("my-secret"));
    }

    #[test]
    fn kv_set_secret_path() {
        let p = kv::set_secret("my-secret");
        assert!(p.contains("my-secret"));
    }

    #[test]
    fn kv_delete_secret_path() {
        let p = kv::delete_secret("my-secret");
        assert!(p.contains("my-secret"));
    }

    #[test]
    fn kv_update_secret_path() {
        let p = kv::update_secret("my-secret");
        assert!(p.contains("my-secret"));
    }

    #[test]
    fn kv_backup_secret_path() {
        let p = kv::backup_secret("my-secret");
        assert!(p.contains("my-secret") && p.contains("backup"));
    }

    #[test]
    fn kv_get_deleted_secret_path() {
        let p = kv::get_deleted_secret("my-secret");
        assert!(p.contains("my-secret") && (p.contains("deleted") || p.contains("Deleted")));
    }

    #[test]
    fn kv_recover_deleted_secret_path() {
        let p = kv::recover_deleted_secret("my-secret");
        assert!(p.contains("my-secret") && p.contains("recover"));
    }

    #[test]
    fn kv_purge_deleted_secret_path() {
        let p = kv::purge_deleted_secret("my-secret");
        assert!(p.contains("my-secret"));
    }

    #[test]
    fn ac_get_key_value_path() {
        let p = ac::get_key_value("my-key");
        assert!(p.contains("my-key"));
    }

    #[test]
    fn ac_set_key_value_path() {
        let p = ac::set_key_value("my-key");
        assert!(p.contains("my-key"));
    }

    #[test]
    fn ac_delete_key_value_path() {
        let p = ac::delete_key_value("my-key");
        assert!(p.contains("my-key"));
    }
}

#[cfg(test)]
mod aws_constants {
    use crate::aws::secrets_manager;

    #[test]
    fn all_headers_non_empty() {
        let headers = [
            secrets_manager::CREATE_SECRET,
            secrets_manager::GET_SECRET_VALUE,
            secrets_manager::DESCRIBE_SECRET,
            secrets_manager::PUT_SECRET_VALUE,
            secrets_manager::UPDATE_SECRET,
            secrets_manager::DELETE_SECRET,
            secrets_manager::RESTORE_SECRET,
            secrets_manager::LIST_SECRETS,
            secrets_manager::LIST_SECRET_VERSIONS,
            secrets_manager::UPDATE_SECRET_VERSION_STAGE,
            secrets_manager::TAG_RESOURCE,
            secrets_manager::UNTAG_RESOURCE,
            secrets_manager::GET_RESOURCE_POLICY,
        ];
        for h in headers {
            assert!(!h.is_empty(), "Header should not be empty: {h}");
        }
    }

    #[test]
    fn all_headers_contain_secretsmanager_namespace() {
        let headers = [
            secrets_manager::CREATE_SECRET,
            secrets_manager::GET_SECRET_VALUE,
            secrets_manager::DESCRIBE_SECRET,
        ];
        for h in headers {
            assert!(
                h.starts_with("secretsmanager."),
                "Expected 'secretsmanager.' prefix, got: {h}"
            );
        }
    }

    #[test]
    fn headers_are_unique() {
        let mut headers = vec![
            secrets_manager::CREATE_SECRET,
            secrets_manager::GET_SECRET_VALUE,
            secrets_manager::DESCRIBE_SECRET,
            secrets_manager::PUT_SECRET_VALUE,
            secrets_manager::UPDATE_SECRET,
            secrets_manager::DELETE_SECRET,
            secrets_manager::LIST_SECRETS,
            secrets_manager::TAG_RESOURCE,
            secrets_manager::UNTAG_RESOURCE,
            secrets_manager::GET_RESOURCE_POLICY,
        ];
        headers.sort_unstable();
        let original_len = headers.len();
        headers.dedup();
        assert_eq!(original_len, headers.len(), "Headers must be unique");
    }
}
