#!/bin/bash
set -e

# Issue license for oxo-call using oxo-dual-licenser
#
# Usage:
#   ./issue-license.sh <org_name> <email> <type> <output_file>
#   ./issue-license.sh --help
#
# Examples:
#   ./issue-license.sh "Shixiang Wang" "w_shixiang@163.com" academic ~/Downloads/license.oxo.json
#   ./issue-license.sh "Acme University" "research@acme.edu" commercial ./license.json
#
# Requirements:
#   - OXO_LICENSE_PRIVATE_KEY environment variable must be set
#   - Or pass --private-key <file> option
#
# License types:
#   academic    - Free for academic/research/education use
#   commercial  - Paid commercial license

# Configuration
SCHEMA="oxo-call-license-v1"
ISSUER_BIN="${ISSUER_BIN:-./target/release/oxo-license-issuer}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_usage() {
    cat << EOF
Usage: $0 [OPTIONS] <org_name> <email> <type> <output_file>

Issue a signed license file for oxo-call.

Arguments:
  org_name     Full legal name of the organization or individual
  email        Contact email address (pass "-" to skip)
  type         License type: academic, commercial, or enterprise
  output_file  Path to write the signed license JSON

Options:
  -k, --private-key <FILE>  Path to private key file
  -d, --issued-at <DATE>    Issue date in YYYY-MM-DD format (default: today)
  -h, --help                Show this help message

Environment:
  OXO_LICENSE_PRIVATE_KEY   Base64-encoded 32-byte Ed25519 private key seed
  ISSUER_BIN                Path to oxo-license-issuer binary (default: ./target/release/oxo-license-issuer)

Examples:
  # Basic usage with env var
  export OXO_LICENSE_PRIVATE_KEY="your-base64-private-key"
  $0 "Acme University" "research@acme.edu" academic ~/license.oxo.json

  # Using private key file
  $0 -k ~/.oxo/private.key "Acme University" "research@acme.edu" academic ~/license.oxo.json

  # Without email
  $0 "John Doe" - academic ~/license.oxo.json

  # Commercial license with custom date
  $0 -d 2025-01-01 "Corp Inc" "license@corp.com" commercial ~/license.json
EOF
}

# Parse options
PRIVATE_KEY_OPT=""
ISSUED_AT_OPT=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -k|--private-key)
            PRIVATE_KEY_OPT="$2"
            shift 2
            ;;
        -d|--issued-at)
            ISSUED_AT_OPT="$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        -*)
            echo -e "${RED}Error: Unknown option $1${NC}" >&2
            print_usage
            exit 1
            ;;
        *)
            break
            ;;
    esac
done

# Check arguments
if [ $# -lt 4 ]; then
    echo -e "${RED}Error: Missing required arguments${NC}" >&2
    print_usage
    exit 1
fi

ORG="$1"
EMAIL="$2"
TYPE="$3"
OUTPUT="$4"

# Validate license type
case "$TYPE" in
    academic|commercial|enterprise)
        ;;
    *)
        echo -e "${RED}Error: Invalid license type '$TYPE'${NC}" >&2
        echo "Valid types: academic, commercial, enterprise" >&2
        exit 1
        ;;
esac

# Check issuer binary exists
if [ ! -f "$ISSUER_BIN" ]; then
    echo -e "${YELLOW}Warning: Issuer binary not found at $ISSUER_BIN${NC}"
    echo "Building oxo-license-issuer..."
    cargo build --release -p oxo-license-issuer 2>/dev/null || {
        echo -e "${RED}Error: Failed to build oxo-license-issuer${NC}" >&2
        exit 1
    }
fi

# Check private key availability
if [ -n "$PRIVATE_KEY_OPT" ]; then
    if [ ! -f "$PRIVATE_KEY_OPT" ]; then
        echo -e "${RED}Error: Private key file not found: $PRIVATE_KEY_OPT${NC}" >&2
        exit 1
    fi
    KEY_SOURCE="file: $PRIVATE_KEY_OPT"
elif [ -n "$OXO_LICENSE_PRIVATE_KEY" ]; then
    KEY_SOURCE="environment variable"
else
    echo -e "${RED}Error: No private key provided${NC}" >&2
    echo "Set OXO_LICENSE_PRIVATE_KEY or use --private-key option" >&2
    exit 1
fi

# Build command
CMD="$ISSUER_BIN issue --schema $SCHEMA --org \"$ORG\" --type $TYPE"

if [ -n "$PRIVATE_KEY_OPT" ]; then
    CMD="$CMD --private-key \"$PRIVATE_KEY_OPT\""
fi

if [ -n "$ISSUED_AT_OPT" ]; then
    CMD="$CMD --issued-at \"$ISSUED_AT_OPT\""
fi

if [ "$EMAIL" != "-" ] && [ -n "$EMAIL" ]; then
    CMD="$CMD --email \"$EMAIL\""
fi

CMD="$CMD --output \"$OUTPUT\""

# Print info
echo -e "${GREEN}=== License Issuance ===${NC}"
echo "  Schema    : $SCHEMA"
echo "  Org       : $ORG"
echo "  Email     : ${EMAIL:-<none>}"
echo "  Type      : $TYPE"
echo "  Output    : $OUTPUT"
echo "  Key source: $KEY_SOURCE"
echo ""

# Execute
eval $CMD

# Verify the output
if [ -f "$OUTPUT" ]; then
    echo ""
    echo -e "${GREEN}✓ License file created successfully!${NC}"
    echo ""
    echo "To verify:"
    echo "  oxo-call license verify --license \"$OUTPUT\""
else
    echo -e "${RED}✗ Failed to create license file${NC}" >&2
    exit 1
fi
