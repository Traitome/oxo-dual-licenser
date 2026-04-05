# oxo-dual-licenser

[![CI](https://github.com/Traitome/oxo-dual-licenser/actions/workflows/ci.yml/badge.svg)](https://github.com/Traitome/oxo-dual-licenser/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/oxo-license.svg)](https://crates.io/crates/oxo-license)
[![Docs.rs](https://docs.rs/oxo-license/badge.svg)](https://docs.rs/oxo-license)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Generic dual-license runtime verification library for Traitome Rust projects.**

`oxo-dual-licenser` extracts and generalizes the license system from the
[oxo-call](https://github.com/Traitome/oxo-call) project into a reusable
library and CLI tool.

## Features

- **Ed25519 signatures** — Cryptographically secure license verification
- **Offline verification** — No network calls at runtime
- **Flexible license types** — academic, commercial, enterprise, or custom
- **Configurable discovery** — CLI args, env vars, or platform config dirs
- **oxo-call compatible** — Can issue and verify oxo-call licenses

## Workspace Crates

| Crate | Description |
|-------|-------------|
| [`oxo-license`](crates/oxo-license) | Embeddable Ed25519 license verification library |
| [`oxo-license-issuer`](crates/oxo-license-issuer) | CLI tool for generating key pairs and issuing licenses |

## Quick Start

### Library usage

```toml
[dependencies]
oxo-license = "0.1"
```

```rust
use oxo_license::{LicenseConfig, load_and_verify};

static LICENSE_CONFIG: LicenseConfig = LicenseConfig {
    schema_version: "my-app-license-v1",
    public_key_base64: "YOUR_BASE64_PUBLIC_KEY",
    license_env_var: "MY_APP_LICENSE",
    app_qualifier: "io",
    app_org: "myorg",
    app_name: "my-app",
    license_filename: "license.json",
};

fn main() {
    load_and_verify(None, &LICENSE_CONFIG).expect("valid license required");
}
```

### CLI tool

```bash
# Install
cargo install oxo-license-issuer

# Generate a key pair
oxo-license-issuer generate-keypair --output keys.txt

# Issue a license
export OXO_LICENSE_PRIVATE_KEY="<seed>"
oxo-license-issuer issue \
  --schema my-app-license-v1 \
  --org "Acme University" \
  --type academic \
  --output license.json

# Verify a license
oxo-license-issuer verify \
  --public-key "<pubkey>" \
  --schema my-app-license-v1 \
  license.json
```

### Helper script

```bash
# Issue license with the helper script
./issue-license.sh "Acme University" "research@acme.edu" academic license.json
```

## oxo-call Compatibility

This library can fully replace the license module in oxo-call:

```rust
use oxo_license::LicenseConfig;

pub static OXO_CALL_CONFIG: LicenseConfig = LicenseConfig {
    schema_version: "oxo-call-license-v1",
    public_key_base64: "SOTbyPWS8fSF+XS9dqEg9cFyag0wPO/YMA5LhI4PXw4=",
    license_env_var: "OXO_CALL_LICENSE",
    app_qualifier: "io",
    app_org: "traitome",
    app_name: "oxo-call",
    license_filename: "license.oxo.json",
};
```

### Issue licenses for oxo-call

```bash
# Using the script
./issue-license.sh "Customer Name" "customer@example.com" academic license.oxo.json

# Or using CLI directly
oxo-license-issuer issue \
  --schema oxo-call-license-v1 \
  --org "Customer Name" \
  --type commercial \
  --output license.oxo.json
```

## Documentation

Full documentation is available at [GitHub Pages](https://traitome.github.io/oxo-dual-licenser/).

- [Installation](docs/guide/docs/tutorials/installation.md)
- [Quickstart](docs/guide/docs/tutorials/quickstart.md)
- [Issue a License](docs/guide/docs/how-to/issue-license.md)
- [API Reference](docs/guide/docs/reference/api.md)

## License

This project is available under:
- **MIT License** for open source use (see [LICENSE](LICENSE))
- **Academic License** for non-commercial research (see [LICENSE-ACADEMIC](LICENSE-ACADEMIC))
- **Commercial License** for commercial products (see [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL))

## Citation

If you use this software in academic work, please cite using [CITATION.cff](CITATION.cff).
