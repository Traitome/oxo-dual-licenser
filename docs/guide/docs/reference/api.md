# API Reference

## `oxo-license` crate

### `LicenseConfig`

Project-specific configuration struct. Create a `static` instance.

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

### `LicensePayload`

The signed payload. Field order is the canonical wire format.

### `LicenseFile`

On-disk representation: flattened payload + `signature` field.

### Functions

#### `verify_license(license, config) -> Result<(), LicenseError>`

Verify a license against the config's embedded public key.

#### `verify_license_with_key(license, pubkey_base64, schema) -> Result<(), LicenseError>`

Low-level verification with an explicit public key.

#### `find_license_path(cli_arg, config) -> Option<PathBuf>`

Locate the license file using the priority chain.

#### `load_and_verify(cli_arg, config) -> Result<LicenseFile, LicenseError>`

Load from disk and verify in one call.

### `LicenseError`

| Variant                  | Description                          |
|--------------------------|--------------------------------------|
| `NotFound`               | No license file found                |
| `ReadError`              | File I/O error                       |
| `ParseError`             | JSON parse error                     |
| `InvalidSchema`          | Schema version mismatch              |
| `InvalidSignature`       | Ed25519 signature verification failed|
| `InvalidPublicKey`       | Malformed embedded public key        |
| `InvalidSignatureEncoding` | Bad base64 or wrong length         |
