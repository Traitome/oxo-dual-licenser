# oxo-dual-licenser

[![CI](https://github.com/Traitome/oxo-dual-licenser/actions/workflows/ci.yml/badge.svg)](https://github.com/Traitome/oxo-dual-licenser/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/oxo-license.svg)](https://crates.io/crates/oxo-license)
[![Docs.rs](https://docs.rs/oxo-license/badge.svg)](https://docs.rs/oxo-license)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Generic dual-license runtime verification library for Traitome Rust projects.**

`oxo-dual-licenser` extracts and generalizes the license system from the
[oxo-call](https://github.com/Traitome/oxo-call) project into a reusable
library and CLI tool.

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

## License

This project is available under:
- **MIT License** for open source use (see [LICENSE](LICENSE))
- **Academic License** for non-commercial research (see [LICENSE-ACADEMIC](LICENSE-ACADEMIC))
- **Commercial License** for commercial products (see [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL))

## Citation

If you use this software in academic work, please cite using [CITATION.cff](CITATION.cff).
