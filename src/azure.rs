//! Azure Key Vault and App Configuration API Paths

use crate::parameters::{SecretName, VersionId};
use crate::typed_path::TypedPath;

/// Azure Key Vault Secrets API paths
pub mod key_vault {
    use super::*;

    pub const BASE: &str = "/";
    pub const LIST_SECRETS: &str = "/secrets";
    pub const RESTORE_SECRET: &str = "/secrets/restore";
    pub const LIST_DELETED_SECRETS: &str = "/deletedsecrets";

    /// `GET /secrets/{name}/` (trailing slash — Azure Key Vault GET convention)
    pub fn get_secret(name: &SecretName) -> TypedPath {
        TypedPath::new("/secrets/{secret}/").bind("secret", name.as_str())
    }

    /// `GET /secrets/{name}/{version}`
    pub fn get_secret_version(name: &SecretName, version: &VersionId) -> TypedPath {
        TypedPath::new("/secrets/{secret}/{version}")
            .bind("secret", name.as_str())
            .bind("version", version.as_str())
    }

    /// `GET /secrets/{name}/versions`
    pub fn list_secret_versions(name: &SecretName) -> TypedPath {
        TypedPath::new("/secrets/{secret}/versions").bind("secret", name.as_str())
    }

    /// `PUT /secrets/{name}` (no trailing slash)
    pub fn set_secret(name: &SecretName) -> TypedPath {
        TypedPath::new("/secrets/{secret}").bind("secret", name.as_str())
    }

    /// `DELETE /secrets/{name}`
    pub fn delete_secret(name: &SecretName) -> TypedPath {
        set_secret(name)
    }

    /// `PATCH /secrets/{name}`
    pub fn update_secret(name: &SecretName) -> TypedPath {
        set_secret(name)
    }

    /// `POST /secrets/{name}/backup`
    pub fn backup_secret(name: &SecretName) -> TypedPath {
        TypedPath::new("/secrets/{secret}/backup").bind("secret", name.as_str())
    }

    /// `GET /deletedsecrets/{name}`
    pub fn get_deleted_secret(name: &SecretName) -> TypedPath {
        TypedPath::new("/deletedsecrets/{secret}").bind("secret", name.as_str())
    }

    /// `POST /deletedsecrets/{name}/recover`
    pub fn recover_deleted_secret(name: &SecretName) -> TypedPath {
        TypedPath::new("/deletedsecrets/{secret}/recover").bind("secret", name.as_str())
    }

    /// `DELETE /deletedsecrets/{name}`
    pub fn purge_deleted_secret(name: &SecretName) -> TypedPath {
        TypedPath::new("/deletedsecrets/{secret}").bind("secret", name.as_str())
    }
}

/// Azure App Configuration API paths
pub mod app_configuration {
    use crate::parameters::SecretName;
    use crate::typed_path::TypedPath;

    pub const BASE: &str = "/";
    pub const LIST_KEY_VALUES: &str = "/kv";

    /// `GET /kv/{key}`
    pub fn get_key_value(key: &SecretName) -> TypedPath {
        TypedPath::new("/kv/{secret}").bind("secret", key.as_str())
    }

    /// `PUT /kv/{key}`
    pub fn set_key_value(key: &SecretName) -> TypedPath {
        get_key_value(key)
    }

    /// `DELETE /kv/{key}`
    pub fn delete_key_value(key: &SecretName) -> TypedPath {
        get_key_value(key)
    }
}

/// Route constants (URI templates) for Axum — unchanged from before.
pub mod routes {
    pub mod key_vault {
        pub const LIST_SECRETS: &str = "/secrets";
        pub const GET_SECRET: &str = "/secrets/{name}/";
        pub const GET_SECRET_VERSION: &str = "/secrets/{name}/{version}";
        pub const LIST_SECRET_VERSIONS: &str = "/secrets/{name}/versions";
        pub const SET_SECRET: &str = "/secrets/{name}";
        pub const DELETE_SECRET: &str = "/secrets/{name}";
        pub const UPDATE_SECRET: &str = "/secrets/{name}";
        pub const BACKUP_SECRET: &str = "/secrets/{name}/backup";
        pub const RESTORE_SECRET: &str = "/secrets/restore";
        pub const GET_DELETED_SECRET: &str = "/deletedsecrets/{name}";
        pub const LIST_DELETED_SECRETS: &str = "/deletedsecrets";
        pub const RECOVER_DELETED_SECRET: &str = "/deletedsecrets/{name}/recover";
        pub const PURGE_DELETED_SECRET: &str = "/deletedsecrets/{name}";
    }
    pub mod app_configuration {
        pub const LIST_KEY_VALUES: &str = "/kv";
        pub const GET_KEY_VALUE: &str = "/kv/{key}";
        pub const SET_KEY_VALUE: &str = "/kv/{key}";
        pub const DELETE_KEY_VALUE: &str = "/kv/{key}";
    }
}
