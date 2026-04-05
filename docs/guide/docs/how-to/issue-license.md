# How to Issue a License

This guide covers various scenarios for issuing licenses.

## Prerequisites

- `oxo-license-issuer` installed
- Your private key seed (from `generate-keypair`)

## Methods

### Method 1: Using the helper script (Recommended)

The `issue-license.sh` script provides a user-friendly interface:

```bash
# Set environment variable
export OXO_LICENSE_PRIVATE_KEY="<base64-encoded-32-byte-seed>"

# Issue license
./issue-license.sh "Acme University" "research@acme.edu" academic ~/license.json
```

#### Script Options

```bash
./issue-license.sh [OPTIONS] <org_name> <email> <type> <output_file>

Options:
  -k, --private-key <FILE>  Path to private key file
  -d, --issued-at <DATE>    Issue date in YYYY-MM-DD format
  -h, --help                Show help message
```

#### Examples

```bash
# Using private key file
./issue-license.sh -k ~/.oxo/private.key "Acme University" "research@acme.edu" academic ~/license.json

# Without email (pass "-")
./issue-license.sh "John Doe" - academic ~/license.json

# Commercial license with custom date
./issue-license.sh -d 2025-01-01 "Corp Inc" "license@corp.com" commercial ~/license.json

# Enterprise license
./issue-license.sh "Big Corp" "enterprise@bigcorp.com" enterprise ~/license.json
```

### Method 2: Using the CLI directly

```bash
# Set private key via environment variable
export OXO_LICENSE_PRIVATE_KEY="<base64-encoded-32-byte-seed>"

# Or use a key file
oxo-license-issuer issue --private-key keys.txt ...

# Issue the license
oxo-license-issuer issue \
  --schema my-app-license-v1 \
  --org "Acme University" \
  --email research@acme.edu \
  --type academic \
  --issued-at 2025-01-01 \
  --output license.json
```

### Method 3: Issuing for oxo-call

To issue licenses compatible with oxo-call:

```bash
export OXO_LICENSE_PRIVATE_KEY="EpLj2jI715ogs5yPFI4nbrOtl8guL+ZNG9B09jb48yI="

oxo-license-issuer issue \
  --schema oxo-call-license-v1 \
  --org "Customer Name" \
  --email customer@example.com \
  --type academic \
  --output license.oxo.json
```

Or using the script:

```bash
./issue-license.sh "Customer Name" "customer@example.com" academic license.oxo.json
```

## Verification

Always verify the issued license before distributing:

```bash
# Verify with oxo-license-issuer
oxo-license-issuer verify \
  --public-key "<your-public-key>" \
  --schema my-app-license-v1 \
  license.json

# Verify with oxo-call (for oxo-call licenses)
oxo-call license verify --license license.oxo.json
```

## License Types

The `--type` flag accepts any string. Common values:

| Type         | Description                          | Use Case |
|--------------|--------------------------------------|----------|
| `academic`   | Non-commercial research/education    | Universities, research labs |
| `commercial` | Standard commercial use              | Small to medium businesses |
| `enterprise` | Enterprise with extended rights      | Large organizations |
| `trial`      | Time-limited evaluation              | Prospective customers |

## Troubleshooting

### "No private key provided"

Ensure `OXO_LICENSE_PRIVATE_KEY` is set or use `--private-key`:

```bash
# Check if environment variable is set
echo $OXO_LICENSE_PRIVATE_KEY

# Or use file
oxo-license-issuer issue --private-key keys.txt ...
```

### "Invalid signature encoding"

The private key must be a valid base64-encoded 32-byte seed. Regenerate if needed:

```bash
oxo-license-issuer generate-keypair --output keys.txt
```

### "Schema version mismatch"

Ensure the `--schema` matches what's configured in your application's `LicenseConfig`.
