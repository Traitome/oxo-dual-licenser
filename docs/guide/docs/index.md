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

## Quick Links

- [Installation](tutorials/installation.md)
- [Quickstart](tutorials/quickstart.md)
- [Issue a License](how-to/issue-license.md)
- [API Reference](reference/api.md)
