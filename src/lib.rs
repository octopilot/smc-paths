//! Type-safe API path builder for GCP Secret Manager, AWS Secrets Manager, and Azure Key Vault.
//!
//! This crate is the single source of truth for all API paths used by the
//! secret-manager-controller and its Pact mock servers, ensuring exact consistency
//! between production API calls and contract tests.
//!
//! ## Quick Start
//!
//! ```rust
//! use smc_paths::prelude::*;
//!
//! let path = PathBuilder::new()
//!     .gcp_operation(GcpOperation::CreateSecret)
//!     .project("my-project")
//!     .build_http_path()
//!     .unwrap();
//! ```
//!
//! ## PathBuilder
//!
//! The `PathBuilder` provides a type-safe, builder-pattern API for constructing
//! API paths with different output formats (routes, HTTP paths, response names, etc.).
//!
//! ## Route Constants
//!
//! Route constants are provided for Axum routes, which require static string literals.
//! These constants are validated against PathBuilder output in tests.

pub mod aws;
pub mod azure;
pub mod gcp;

// Core PathBuilder components
pub mod builder;
pub mod errors;
pub mod formats;
pub mod operations;
pub mod prelude;
pub mod provider;
