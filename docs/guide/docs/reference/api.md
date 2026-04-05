# API Reference

Complete API documentation for the `oxo-license` crate.

## Core Types

### `LicenseConfig`

Project-specific configuration struct. Create a `static` instance in your application.

```rust
pub struct LicenseConfig {
    pub schema_version: &'static str,
    pub public_key_base64: &'static str,
    pub license_env_var: &'static str,
    pub app_qualifier: &'static str,
    pub app_org: &'static str,
    pub app_name: &'static str,
    pub license_filename: &'static str,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `schema_version` | `&'static str` | Schema identifier (e.g., `"my-app-license-v1"`) |
| `public_key_base64` | `&'static str` | Base64-encoded 32-byte Ed25519 public key |
| `license_env_var` | `&'static str` | Environment variable name for license path |
| `app_qualifier` | `&'static str` | App qualifier for config directory |
| `app_org` | `&'static str` | Organization name |
| `app_name` | `&'static str` | Application name |
| `license_filename` | `&'static str` | License filename |

### `LicensePayload`

The signed payload containing license information. Field order is the canonical wire format.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    pub schema: String,
    pub license_id: String,
    pub issued_to_org: String,
    pub contact_email: Option<String>,
    pub license_type: String,
    pub scope: String,
    pub perpetual: bool,
    pub issued_at: String,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `schema` | `String` | Schema version identifier |
| `license_id` | `String` | UUID v4 license identifier |
| `issued_to_org` | `String` | Organization or individual name |
| `contact_email` | `Option<String>` | Contact email address |
| `license_type` | `String` | License type (e.g., `"academic"`, `"commercial"`) |
| `scope` | `String` | License scope (default: `"org"`) |
| `perpetual` | `bool` | Whether license is perpetual |
| `issued_at` | `String` | Issue date in YYYY-MM-DD format |

### `LicenseFile`

On-disk representation: flattened payload with signature.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseFile {
    pub schema: String,
    pub license_id: String,
    pub issued_to_org: String,
    pub contact_email: Option<String>,
    pub license_type: String,
    pub scope: String,
    pub perpetual: bool,
    pub issued_at: String,
    pub signature: String,
    #[serde(skip)]
    pub payload: LicensePayload,
}
```

## Functions

### `verify_license`

Verify a license against the config's embedded public key.

```rust
pub fn verify_license(
    license: &LicenseFile,
    config: &LicenseConfig,
) -> Result<(), LicenseError>
```

#### Parameters

- `license`: The license file to verify
- `config`: Configuration containing public key and schema

#### Returns

- `Ok(())` if verification succeeds
- `Err(LicenseError)` if verification fails

### `verify_license_with_key`

Low-level verification with an explicit public key.

```rust
pub fn verify_license_with_key(
    license: &LicenseFile,
    pubkey_base64: &str,
    schema_version: &str,
) -> Result<(), LicenseError>
```

#### Parameters

- `license`: The license file to verify
- `pubkey_base64`: Base64-encoded 32-byte Ed25519 public key
- `schema_version`: Expected schema version

#### Returns

- `Ok(())` if verification succeeds
- `Err(LicenseError)` if verification fails

### `find_license_path`

Locate the license file using the priority chain.

```rust
pub fn find_license_path(
    cli_arg: Option<&Path>,
    config: &LicenseConfig,
) -> Option<PathBuf>
```

#### Search Priority

1. CLI argument (`--license /path/to/license.json`)
2. Environment variable (`MY_APP_LICENSE=/path/to/license.json`)
3. Platform config directory:
   - macOS: `~/Library/Application Support/{qualifier}.{org}.{app}/{filename}`
   - Linux: `~/.config/{app}/{filename}`
   - Windows: `%APPDATA%\{app}\{filename}`
4. Legacy Unix: `~/.config/{app}/{filename}`

### `load_and_verify`

Load from disk and verify in one call.

```rust
pub fn load_and_verify(
    cli_arg: Option<&Path>,
    config: &LicenseConfig,
) -> Result<LicenseFile, LicenseError>
```

#### Parameters

- `cli_arg`: Optional CLI argument path
- `config`: Configuration for verification

#### Returns

- `Ok(LicenseFile)` with the loaded and verified license
- `Err(LicenseError)` if loading or verification fails

## Error Handling

### `LicenseError`

```rust
pub enum LicenseError {
    NotFound { searched_paths: Vec<String> },
    ReadError { path: String, source: std::io::Error },
    ParseError { source: serde_json::Error },
    InvalidSchema { expected: String, found: String },
    InvalidSignature { message: String },
    InvalidPublicKey { message: String },
    InvalidSignatureEncoding { message: String },
}
```

| Variant | Description |
|---------|-------------|
| `NotFound` | No license file found in any search location |
| `ReadError` | File I/O error when reading license |
| `ParseError` | JSON parse error |
| `InvalidSchema` | Schema version mismatch |
| `InvalidSignature` | Ed25519 signature verification failed |
| `InvalidPublicKey` | Malformed embedded public key |
| `InvalidSignatureEncoding` | Bad base64 or wrong signature length |

### Example Error Handling

```rust
use oxo_license::{load_and_verify, LicenseError};

match load_and_verify(None, &LICENSE_CONFIG) {
    Ok(license) => {
        println!("Licensed to: {}", license.payload.issued_to_org);
    }
    Err(LicenseError::NotFound { searched_paths }) => {
        eprintln!("License not found. Searched:");
        for path in searched_paths {
            eprintln!("  - {}", path);
        }
    }
    Err(LicenseError::InvalidSignature { message }) => {
        eprintln!("Invalid license signature: {}", message);
    }
    Err(e) => {
        eprintln!("License error: {}", e);
    }
}
```

## CLI Tool

### `oxo-license-issuer`

#### `generate-keypair`

Generate a fresh Ed25519 key pair.

```bash
oxo-license-issuer generate-keypair --output <FILE>
```

#### `issue`

Issue (sign) a new license file.

```bash
oxo-license-issuer issue [OPTIONS] --org <ORG> --schema <SCHEMA>

Options:
      --private-key <FILE>  Path to private key file
      --schema <SCHEMA>     Schema version string
      --org <ORG>           Organization or individual name
      --email <EMAIL>       Contact email address
      --type <TYPE>         License type [default: commercial]
      --issued-at <DATE>    Issue date (YYYY-MM-DD)
  -o, --output <FILE>       Output file path
```

#### `verify`

Verify a signed license file.

```bash
oxo-license-issuer verify [OPTIONS] <LICENSE_FILE>

Options:
      --public-key <KEY>  Base64-encoded public key
      --schema <SCHEMA>   Expected schema version
```
