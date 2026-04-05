# oxo-dual-licenser

**Generic dual-license runtime verification library for Traitome Rust projects.**

`oxo-dual-licenser` provides an Ed25519-based license verification system
that can be embedded into any Rust application. It consists of two crates:

- **`oxo-license`** — embeddable library for runtime license verification
- **`oxo-license-issuer`** — CLI tool for generating key pairs and issuing licenses

## Features

- **Any license types** — strings like `"academic"`, `"commercial"`, `"enterprise"`
- **Offline verification** — no network calls at runtime
- **Configurable discovery** — env vars, platform config dirs, CLI args
- **Shared key infrastructure** — one key pair per organization
- **oxo-call compatible** — can issue licenses for oxo-call

## Quick Links

- [Installation](tutorials/installation.md)
- [Quickstart](tutorials/quickstart.md)
- [Issue a License](how-to/issue-license.md)
- [API Reference](reference/api.md)

## Compatibility with oxo-call

This library is designed as a generalization of the license system from
[oxo-call](https://github.com/Traitome/oxo-call). It can:

1. **Verify oxo-call licenses** — Pass the oxo-call public key and schema
2. **Issue oxo-call licenses** — Use the same private key to sign licenses
3. **Replace oxo-call's license module** — Configure `LicenseConfig` with oxo-call parameters

### oxo-call Configuration Example

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
