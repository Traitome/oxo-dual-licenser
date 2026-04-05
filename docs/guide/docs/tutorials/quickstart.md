# Quickstart

This guide walks you through setting up license verification in a Rust application.

## 1. Generate a key pair

```bash
oxo-license-issuer generate-keypair --output keys.txt
```

Output:

```
─── Ed25519 key pair generated ───────────────────────────────────────
Public key  (embed in your binary): AOIRnS4aD6gxubJ3/EMyRU0mYaH8ijA/8igKOxLN8Ls=
Private key (keep secret, never commit): EpLj2jI715ogs5yPFI4nbrOtl8guL+ZNG9B09jb48yI=
──────────────────────────────────────────────────────────────────────
Keys written to 'keys.txt'
```

!!! warning "Security"
    Keep `keys.txt` secret and never commit it to the repository. The private key is used to sign licenses; the public key is embedded in your application binary.

## 2. Configure your application

Create a `LicenseConfig` in your application:

```rust
use oxo_license::LicenseConfig;

pub static LICENSE_CONFIG: LicenseConfig = LicenseConfig {
    schema_version: "my-app-license-v1",
    public_key_base64: "AOIRnS4aD6gxubJ3/EMyRU0mYaH8ijA/8igKOxLN8Ls=",
    license_env_var: "MY_APP_LICENSE",
    app_qualifier: "io",
    app_org: "myorg",
    app_name: "my-app",
    license_filename: "license.json",
};
```

### Configuration Fields

| Field | Description | Example |
|-------|-------------|---------|
| `schema_version` | Schema identifier for your app | `"my-app-license-v1"` |
| `public_key_base64` | Base64-encoded 32-byte Ed25519 public key | From `generate-keypair` |
| `license_env_var` | Environment variable for license path | `"MY_APP_LICENSE"` |
| `app_qualifier` | App qualifier for config dir | `"io"` |
| `app_org` | Organization name | `"myorg"` |
| `app_name` | Application name | `"my-app"` |
| `license_filename` | License filename | `"license.json"` |

## 3. Verify at runtime

```rust
use oxo_license::{load_and_verify, LicenseError};

fn check_license() -> Result<(), LicenseError> {
    let license = load_and_verify(None, &LICENSE_CONFIG)?;
    
    // Access license details
    println!("Licensed to: {}", license.payload.issued_to_org);
    println!("License type: {}", license.payload.license_type);
    
    // Check license type if needed
    match license.payload.license_type.as_str() {
        "academic" => println!("Academic license - non-commercial use only"),
        "commercial" => println!("Commercial license"),
        "enterprise" => println!("Enterprise license"),
        other => println!("License type: {}", other),
    }
    
    Ok(())
}

fn main() {
    if let Err(e) = check_license() {
        eprintln!("License error: {e}");
        std::process::exit(1);
    }
    
    // Application logic continues...
}
```

## 4. Issue a license

### Using the CLI

```bash
export OXO_LICENSE_PRIVATE_KEY="EpLj2jI715ogs5yPFI4nbrOtl8guL+ZNG9B09jb48yI="

oxo-license-issuer issue \
  --schema my-app-license-v1 \
  --org "Acme University" \
  --email research@acme.edu \
  --type academic \
  --output license.json
```

### Using the helper script

```bash
./issue-license.sh "Acme University" "research@acme.edu" academic license.json
```

## 5. Deploy the license

Users place the `license.json` file in one of these locations:

1. **CLI argument**: `--license /path/to/license.json`
2. **Environment variable**: `MY_APP_LICENSE=/path/to/license.json`
3. **Platform config directory**:
   - macOS: `~/Library/Application Support/io.myorg.my-app/license.json`
   - Linux: `~/.config/my-app/license.json`
   - Windows: `%APPDATA%\my-app\license.json`
4. **Legacy Unix**: `~/.config/my-app/license.json`

## License File Format

The generated license file is a JSON object:

```json
{
  "schema": "my-app-license-v1",
  "license_id": "550e8400-e29b-41d4-a716-446655440000",
  "issued_to_org": "Acme University",
  "contact_email": "research@acme.edu",
  "license_type": "academic",
  "scope": "org",
  "perpetual": true,
  "issued_at": "2025-01-01",
  "signature": "BASE64_ENCODED_64_BYTE_ED25519_SIGNATURE"
}
```

!!! note "Field Order"
    Field order in JSON is the canonical wire format. The signature covers `serde_json::to_vec(&payload)` with fields in the declared order. Do not reorder struct fields.
