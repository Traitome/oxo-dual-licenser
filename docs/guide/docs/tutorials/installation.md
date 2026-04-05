# Installation

## Adding `oxo-license` to your project

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oxo-license = "0.1"
```

## Installing `oxo-license-issuer` CLI

### From crates.io

```bash
cargo install oxo-license-issuer
```

### From source

```bash
git clone https://github.com/Traitome/oxo-dual-licenser
cd oxo-dual-licenser
cargo install --path crates/oxo-license-issuer
```

### Pre-built binaries

Download from the [GitHub Releases](https://github.com/Traitome/oxo-dual-licenser/releases) page.

## Verifying the installation

```bash
oxo-license-issuer --help
```

Output:

```
CLI maintenance tool for issuing and managing oxo-license signed licenses

Usage: oxo-license-issuer <COMMAND>

Commands:
  generate-keypair  Generate a fresh Ed25519 key pair (use once per deployment)
  issue             Issue (sign) a new license file
  verify            Verify a signed license file against a public key
  help              Print this message or the help of the given subcommand(s)
```

## Using the issue-license.sh script

For convenience, a helper script is provided in the repository:

```bash
# Clone and make executable
git clone https://github.com/Traitome/oxo-dual-licenser
cd oxo-dual-licenser
chmod +x issue-license.sh

# View help
./issue-license.sh --help
```

This script provides a simplified interface for issuing licenses with colored output and validation.
