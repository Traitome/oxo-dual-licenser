# Quickstart

## 1. Generate a key pair

```bash
oxo-license-issuer generate-keypair --output keys.txt
```

Keep `keys.txt` secret. The public key goes into your application binary.

## 2. Configure your application

```rust
use oxo_license::LicenseConfig;

pub static LICENSE_CONFIG: LicenseConfig = LicenseConfig {
    schema_version: "my-app-license-v1",
    public_key_base64: "YOUR_PUBLIC_KEY_HERE",
    license_env_var: "MY_APP_LICENSE",
    app_qualifier: "io",
    app_org: "myorg",
    app_name: "my-app",
    license_filename: "license.json",
};
```

## 3. Verify at runtime

```rust
use oxo_license::load_and_verify;

fn main() {
    if let Err(e) = load_and_verify(None, &LICENSE_CONFIG) {
        eprintln!("License error: {e}");
        std::process::exit(1);
    }
    // proceed with application logic
}
```

## 4. Issue a license for a customer

```bash
export OXO_LICENSE_PRIVATE_KEY="YOUR_PRIVATE_KEY_SEED"
oxo-license-issuer issue \
  --schema my-app-license-v1 \
  --org "Customer Organization" \
  --email customer@example.com \
  --type commercial \
  --output license.json
```
