//! Newtype wrappers for path parameters.
//!
//! Each wrapper represents a distinct semantic concept so the compiler prevents
//! accidental argument swaps (e.g. passing a `SecretName` where `ProjectId` is
//! required).  All types accept `&str` / `String` via [`From`] / [`Into`] so
//! callers can still write `.project("my-project")` without boilerplate.

use std::fmt;

macro_rules! path_param {
    (
        $(#[$meta:meta])*
        $name:ident,
        $placeholder:literal $(,)?
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(pub(crate) String);

        impl $name {
            /// Create a new parameter value.
            pub fn new(val: impl Into<String>) -> Self {
                Self(val.into())
            }

            /// Return the underlying string value.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Return the route placeholder string (e.g. `"{project}"`).
            #[must_use]
            pub fn placeholder() -> &'static str {
                concat!("{", $placeholder, "}")
            }

            /// Return the raw placeholder key (e.g. `"project"`).
            #[must_use]
            pub fn key() -> &'static str {
                $placeholder
            }
        }

        impl<S: Into<String>> From<S> for $name {
            fn from(val: S) -> Self {
                Self(val.into())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

path_param!(
    /// GCP project ID (e.g. `"my-gcp-project"`).
    ProjectId,
    "project"
);

path_param!(
    /// Secret name within a project (e.g. `"database-password"`).
    SecretName,
    "secret"
);

path_param!(
    /// Version identifier within a secret (e.g. `"latest"` or `"3"`).
    VersionId,
    "version"
);

path_param!(
    /// GCP region / location identifier (e.g. `"us-central1"`).
    LocationId,
    "location"
);

path_param!(
    /// Parameter Manager parameter name (e.g. `"my-parameter"`).
    ParameterName,
    "parameter"
);

path_param!(
    /// Azure Key Vault name (e.g. `"my-vault"`).
    VaultName,
    "vault"
);

path_param!(
    /// AWS region identifier (e.g. `"us-east-1"`).
    RegionId,
    "region"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_id_from_str() {
        let p = ProjectId::from("my-project");
        assert_eq!(p.as_str(), "my-project");
    }

    #[test]
    fn placeholder_is_correct() {
        assert_eq!(ProjectId::placeholder(), "{project}");
        assert_eq!(SecretName::placeholder(), "{secret}");
        assert_eq!(VersionId::placeholder(), "{version}");
        assert_eq!(LocationId::placeholder(), "{location}");
        assert_eq!(ParameterName::placeholder(), "{parameter}");
        assert_eq!(VaultName::placeholder(), "{vault}");
    }

    #[test]
    fn key_is_correct() {
        assert_eq!(ProjectId::key(), "project");
        assert_eq!(SecretName::key(), "secret");
    }

    #[test]
    fn display() {
        assert_eq!(ProjectId::new("p").to_string(), "p");
    }

    #[test]
    fn equality() {
        assert_eq!(ProjectId::new("a"), ProjectId::new("a"));
        assert_ne!(ProjectId::new("a"), ProjectId::new("b"));
    }

    #[test]
    fn clone() {
        let a = SecretName::new("x");
        assert_eq!(a.clone(), a);
    }
}
