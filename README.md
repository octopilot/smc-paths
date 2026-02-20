# smc-paths

[![License: PolyForm Shield 1.0.0](https://img.shields.io/badge/License-PolyForm%20Shield%201.0.0-blue.svg)](https://polyformproject.org/licenses/shield/1.0.0)

Type-safe API path builder for **GCP Secret Manager**, **AWS Secrets Manager**, and **Azure Key Vault**.

This crate is the single source of truth for all API paths used by
[secret-manager-controller](https://github.com/octopilot/secret-manager-controller)
and its Pact mock servers, ensuring exact consistency between production API calls and
contract tests.

## Usage

```toml
# Cargo.toml
[dependencies]
smc-paths = { git = "https://github.com/octopilot/smc-paths", tag = "v0.1.0" }
```

```rust
use smc_paths::prelude::*;

// GCP Secret Manager
let path = PathBuilder::new()
    .gcp_operation(GcpOperation::AccessVersion)
    .project("my-project")
    .secret("my-secret")
    .version("latest")
    .build_http_path()
    .unwrap();

// AWS Secrets Manager
let path = PathBuilder::new()
    .aws_operation(AwsOperation::GetSecretValue)
    .build_http_path()
    .unwrap();

// Azure Key Vault
let path = PathBuilder::new()
    .azure_operation(AzureOperation::GetSecret)
    .vault("my-vault")
    .secret("my-secret")
    .build_http_path()
    .unwrap();
```

## Local development

If you are also working on `secret-manager-controller`, add a `[patch]` override to the
workspace `Cargo.toml` so that consumers resolve to your local checkout instead of fetching
from GitHub:

```toml
[patch.'https://github.com/octopilot/smc-paths']
smc-paths = { path = "crates/smc-paths" }
```

## License

[PolyForm Shield License 1.0.0](LICENSE) — free for any use that does not compete with Octopilot.
