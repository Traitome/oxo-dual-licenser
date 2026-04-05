# How to Issue a License

## Prerequisites

- `oxo-license-issuer` installed
- Your private key seed (from `generate-keypair`)

## Steps

### 1. Set the private key

```bash
export OXO_LICENSE_PRIVATE_KEY="<base64-encoded-32-byte-seed>"
```

Or use a key file:

```bash
oxo-license-issuer issue --private-key keys.txt ...
```

### 2. Issue the license

```bash
oxo-license-issuer issue \
  --schema my-app-license-v1 \
  --org "Acme University" \
  --email research@acme.edu \
  --type academic \
  --issued-at 2025-01-01 \
  --output license.json
```

### 3. Verify the issued license

```bash
oxo-license-issuer verify \
  --public-key "<base64-pubkey>" \
  --schema my-app-license-v1 \
  license.json
```

## License Types

The `--type` flag accepts any string. Common values:

| Type         | Description                          |
|--------------|--------------------------------------|
| `academic`   | Non-commercial research/education    |
| `commercial` | Standard commercial use              |
| `enterprise` | Enterprise with extended rights      |
| `trial`      | Time-limited evaluation              |
