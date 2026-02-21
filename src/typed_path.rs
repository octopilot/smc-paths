//! `TypedPath` — a URI path that carries its template and concrete bindings
//! separately so every output format is derived correctly.
//!
//! # Why this exists
//!
//! The previous approach built a concrete URI string first and then tried to
//! *back-replace* the actual values with `{placeholder}` markers using
//! `str::replace`.  This is fragile: a project named `"proj"` would corrupt
//! the literal word `"projects"` in the path, producing `"/v1/pro{project}ects/…"`.
//!
//! `TypedPath` inverts the approach:
//!
//! * The **template** is stored once and is always correct: `"/v1/projects/{project}/secrets"`.
//! * The **bindings** record which concrete value each placeholder receives.
//! * Each output format is derived by either returning the template (routes)
//!   or substituting the bindings into the template (concrete paths).

use crate::formats::PathFormat;

/// A URI path with its template and concrete value bindings held separately.
///
/// # Example
///
/// ```rust
/// use smc_paths::typed_path::TypedPath;
/// use smc_paths::formats::PathFormat;
///
/// let path = TypedPath::new("/v1/projects/{project}/secrets/{secret}")
///     .bind("project", "my-project")
///     .bind("secret",  "my-secret");
///
/// assert_eq!(path.build(PathFormat::Route),        "/v1/projects/{project}/secrets/{secret}");
/// assert_eq!(path.build(PathFormat::PactPath),     "/v1/projects/my-project/secrets/my-secret");
/// assert_eq!(path.build(PathFormat::HttpPath),         "projects/my-project/secrets/my-secret");
/// assert_eq!(path.build(PathFormat::ResponseName),     "projects/my-project/secrets/my-secret");
/// ```
#[derive(Debug, Clone)]
pub struct TypedPath {
    /// URI template with `{placeholder}` markers, **including** the `/v1/` prefix.
    template: &'static str,
    /// Ordered list of `(key, concrete_value)` substitutions.
    bindings: Vec<(&'static str, String)>,
}

impl TypedPath {
    /// Create a new `TypedPath` with the given URI template.
    ///
    /// The template should include the `/v1/` prefix and use `{key}` placeholders,
    /// e.g. `"/v1/projects/{project}/secrets"`.
    #[must_use]
    pub fn new(template: &'static str) -> Self {
        Self {
            template,
            bindings: Vec::new(),
        }
    }

    /// Bind a concrete value to a placeholder key.
    ///
    /// Calling `.bind("project", "my-project")` later produces `my-project`
    /// wherever `{project}` appears in the template.
    #[must_use]
    pub fn bind(mut self, key: &'static str, value: impl Into<String>) -> Self {
        self.bindings.push((key, value.into()));
        self
    }

    /// Build the path in the requested output format.
    #[must_use]
    pub fn build(&self, format: PathFormat) -> String {
        match format {
            // Route: return the template unchanged — placeholders stay as-is.
            PathFormat::Route => self.template.to_string(),

            // HttpPath / ResponseName: substitute concrete values, strip `/v1/` prefix.
            PathFormat::HttpPath | PathFormat::ResponseName => {
                let concrete = self.substitute();
                concrete
                    .strip_prefix("/v1/")
                    .unwrap_or(&concrete)
                    .to_string()
            }

            // PactPath: substitute concrete values, keep `/v1/` prefix.
            PathFormat::PactPath => self.substitute(),
        }
    }

    /// Convenience: build as a route (URI template with placeholders).
    #[must_use]
    pub fn build_route(&self) -> String {
        self.build(PathFormat::Route)
    }

    /// Convenience: build as an HTTP path (concrete values, no `/v1/` prefix).
    #[must_use]
    pub fn build_http_path(&self) -> String {
        self.build(PathFormat::HttpPath)
    }

    /// Convenience: build as a Pact path (concrete values, with `/v1/` prefix).
    #[must_use]
    pub fn build_pact_path(&self) -> String {
        self.build(PathFormat::PactPath)
    }

    /// Convenience: build as a response name (same as `HttpPath`).
    #[must_use]
    pub fn build_response_name(&self) -> String {
        self.build(PathFormat::ResponseName)
    }

    /// Return the raw URI template.
    #[must_use]
    pub fn template(&self) -> &'static str {
        self.template
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Substitute all bindings into the template, returning a concrete URI.
    fn substitute(&self) -> String {
        let mut result = self.template.to_string();
        for (key, value) in &self.bindings {
            result = result.replace(&format!("{{{key}}}"), value);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::PathFormat;

    fn make_path() -> TypedPath {
        TypedPath::new("/v1/projects/{project}/secrets/{secret}")
            .bind("project", "my-project")
            .bind("secret", "my-secret")
    }

    #[test]
    fn route_returns_template_unchanged() {
        assert_eq!(
            make_path().build(PathFormat::Route),
            "/v1/projects/{project}/secrets/{secret}"
        );
    }

    #[test]
    fn pact_path_substitutes_and_keeps_v1_prefix() {
        assert_eq!(
            make_path().build(PathFormat::PactPath),
            "/v1/projects/my-project/secrets/my-secret"
        );
    }

    #[test]
    fn http_path_substitutes_and_strips_v1_prefix() {
        assert_eq!(
            make_path().build(PathFormat::HttpPath),
            "projects/my-project/secrets/my-secret"
        );
    }

    #[test]
    fn response_name_same_as_http_path() {
        assert_eq!(
            make_path().build(PathFormat::ResponseName),
            make_path().build(PathFormat::HttpPath),
        );
    }

    #[test]
    fn no_substring_collision_between_value_and_keyword() {
        // Previously, project="proj" would corrupt the literal "projects" in the path.
        // TypedPath substitutes {project} exactly, never touching surrounding text.
        let path = TypedPath::new("/v1/projects/{project}/secrets").bind("project", "proj"); // "proj" is a substring of "projects"

        // Route should still have the placeholder, not a corrupted literal.
        assert_eq!(
            path.build(PathFormat::Route),
            "/v1/projects/{project}/secrets"
        );
        // Concrete path should replace only {project}, not "proj" inside "projects".
        assert_eq!(
            path.build(PathFormat::PactPath),
            "/v1/projects/proj/secrets"
        );
        assert_eq!(path.build(PathFormat::HttpPath), "projects/proj/secrets");
    }

    #[test]
    fn short_values_do_not_corrupt_adjacent_text() {
        let path = TypedPath::new("/v1/projects/{project}/secrets/{secret}/versions/{version}")
            .bind("project", "p")
            .bind("secret", "s")
            .bind("version", "v");

        assert_eq!(
            path.build(PathFormat::PactPath),
            "/v1/projects/p/secrets/s/versions/v"
        );
        assert_eq!(
            path.build(PathFormat::Route),
            "/v1/projects/{project}/secrets/{secret}/versions/{version}"
        );
    }

    #[test]
    fn path_without_v1_prefix_is_unchanged_by_strip() {
        let path = TypedPath::new("/secrets/{secret}").bind("secret", "s");
        // No /v1/ prefix to strip — should be returned as-is for HttpPath.
        assert_eq!(path.build(PathFormat::HttpPath), "/secrets/s");
    }

    #[test]
    fn convenience_methods_match_build() {
        let path = make_path();
        assert_eq!(path.build_route(), path.build(PathFormat::Route));
        assert_eq!(path.build_http_path(), path.build(PathFormat::HttpPath));
        assert_eq!(path.build_pact_path(), path.build(PathFormat::PactPath));
        assert_eq!(
            path.build_response_name(),
            path.build(PathFormat::ResponseName)
        );
    }

    #[test]
    fn template_accessor() {
        let path = TypedPath::new("/v1/projects/{project}/secrets");
        assert_eq!(path.template(), "/v1/projects/{project}/secrets");
    }
}
