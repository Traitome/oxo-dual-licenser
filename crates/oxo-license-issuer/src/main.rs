//! CLI maintenance tool for issuing and managing oxo-license signed licenses.
//!
//! This tool is intended for **Traitome maintainers** to:
//!
//! 1. Generate Ed25519 key pairs for new projects
//! 2. Issue signed license files for customers
//! 3. Verify existing license files
//!
//! # Usage
//!
//! ## Generate a new Ed25519 key pair
//! ```bash
//! oxo-license-issuer generate-keypair --output keys.txt
//! ```
//!
//! ## Issue a license
//! ```bash
//! export OXO_LICENSE_PRIVATE_KEY="<base64-seed>"
//! oxo-license-issuer issue \
//!   --schema my-app-license-v1 \
//!   --org "Acme University" \
//!   --email research@acme.edu \
//!   --type academic \
//!   --output license.json
//! ```
//!
//! ## Verify a license
//! ```bash
//! oxo-license-issuer verify \
//!   --public-key "<base64-pubkey>" \
//!   --schema my-app-license-v1 \
//!   license.json
//! ```

use base64::{Engine as _, engine::general_purpose::STANDARD};
use chrono::Local;
use clap::{Parser, Subcommand};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── CLI ───────────────────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
#[command(
    name = "oxo-license-issuer",
    about = "CLI maintenance tool for issuing and managing oxo-license signed licenses",
    long_about = "Signs license files with an Ed25519 private key.\n\
                  Keep the private key secret — never commit it to the repository."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a fresh Ed25519 key pair (use once per deployment)
    GenerateKeypair {
        /// Write the key pair to this file instead of stdout
        #[arg(long, short)]
        output: Option<std::path::PathBuf>,
    },

    /// Issue (sign) a new license file
    Issue {
        /// License schema version (e.g. "my-app-license-v1")
        #[arg(long, default_value = "oxo-license-v1")]
        schema: String,

        /// Path to private key file (Base64-encoded 32-byte seed, first line).
        /// Alternatively set OXO_LICENSE_PRIVATE_KEY environment variable.
        #[arg(long)]
        private_key: Option<std::path::PathBuf>,

        /// Full legal name of the organization or individual
        #[arg(long)]
        org: String,

        /// Contact e-mail address (optional)
        #[arg(long)]
        email: Option<String>,

        /// License type (e.g. "academic", "commercial", "enterprise")
        #[arg(long, default_value = "commercial")]
        r#type: String,

        /// Issue date in YYYY-MM-DD format (defaults to today)
        #[arg(long)]
        issued_at: Option<String>,

        /// Write the signed license JSON to this file (defaults to stdout)
        #[arg(long, short)]
        output: Option<std::path::PathBuf>,
    },

    /// Verify a signed license file against a public key
    Verify {
        /// Base64-encoded Ed25519 public key (32 bytes)
        #[arg(long)]
        public_key: String,

        /// Expected schema version
        #[arg(long, default_value = "oxo-license-v1")]
        schema: String,

        /// Path to the license JSON file
        license_file: std::path::PathBuf,
    },
}

// ── License structures ────────────────────────────────────────────────────────

/// Mirrors LicensePayload in oxo-license lib.
/// Field order is part of the wire format — must match exactly.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LicensePayload {
    schema: String,
    license_id: String,
    issued_to_org: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    contact_email: Option<String>,
    license_type: String,
    scope: String,
    perpetual: bool,
    issued_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LicenseFile {
    #[serde(flatten)]
    payload: LicensePayload,
    signature: String,
}

// ── Key helpers ───────────────────────────────────────────────────────────────

fn load_signing_key(path: Option<&std::path::Path>) -> anyhow::Result<SigningKey> {
    let seed_b64 = if let Some(p) = path {
        let contents = std::fs::read_to_string(p)
            .map_err(|e| anyhow::anyhow!("Cannot read private key file '{}': {e}", p.display()))?;
        contents
            .lines()
            .find(|l| l.starts_with("PRIVATE_KEY_SEED="))
            .map(|l| l.trim_start_matches("PRIVATE_KEY_SEED=").to_string())
            .unwrap_or_else(|| contents.trim().to_string())
    } else if let Ok(val) = std::env::var("OXO_LICENSE_PRIVATE_KEY") {
        val.trim().to_string()
    } else {
        anyhow::bail!(
            "No private key provided.\n\
             Use --private-key <file> or set OXO_LICENSE_PRIVATE_KEY=<base64-seed>"
        );
    };

    let seed_bytes = STANDARD
        .decode(&seed_b64)
        .map_err(|e| anyhow::anyhow!("Failed to Base64-decode private key: {e}"))?;
    let seed_array: [u8; 32] = seed_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Private key must be exactly 32 bytes"))?;
    Ok(SigningKey::from_bytes(&seed_array))
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKeypair { output } => {
            let signing_key = SigningKey::generate(&mut OsRng);
            let private_b64 = STANDARD.encode(signing_key.as_bytes());
            let public_b64 = STANDARD.encode(signing_key.verifying_key().as_bytes());

            let text = format!("PRIVATE_KEY_SEED={private_b64}\nPUBLIC_KEY={public_b64}\n");

            eprintln!("─── Ed25519 key pair generated ───────────────────────────────────────");
            eprintln!("Public key  (embed in your binary): {public_b64}");
            eprintln!("Private key (keep secret, never commit): {private_b64}");
            eprintln!("──────────────────────────────────────────────────────────────────────");

            if let Some(path) = output {
                std::fs::write(&path, &text)?;
                eprintln!("Keys written to '{}'", path.display());
            } else {
                print!("{text}");
            }
        }

        Commands::Issue {
            schema,
            private_key,
            org,
            email,
            r#type,
            issued_at,
            output,
        } => {
            let signing_key = load_signing_key(private_key.as_deref())?;

            let issued_at_str =
                issued_at.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

            let payload = LicensePayload {
                schema,
                license_id: Uuid::new_v4().to_string(),
                issued_to_org: org,
                contact_email: email,
                license_type: r#type,
                scope: "org".to_string(),
                perpetual: true,
                issued_at: issued_at_str,
            };

            let payload_bytes = serde_json::to_vec(&payload)?;
            let signature = signing_key.sign(&payload_bytes);
            let signature_b64 = STANDARD.encode(signature.to_bytes());

            let license_file = LicenseFile {
                payload,
                signature: signature_b64,
            };

            let json = serde_json::to_string_pretty(&license_file)?;

            if let Some(path) = output {
                std::fs::write(&path, &json)?;
                eprintln!(
                    "✓ License written to '{}'\n  Issued to : {}\n  Type      : {}\n  Schema    : {}",
                    path.display(),
                    license_file.payload.issued_to_org,
                    license_file.payload.license_type,
                    license_file.payload.schema,
                );
            } else {
                println!("{json}");
            }
        }

        Commands::Verify {
            public_key,
            schema,
            license_file,
        } => {
            let contents = std::fs::read_to_string(&license_file)
                .map_err(|e| anyhow::anyhow!("Cannot read license file: {e}"))?;

            let license: LicenseFile = serde_json::from_str(&contents)
                .map_err(|e| anyhow::anyhow!("Failed to parse license JSON: {e}"))?;

            // Schema check
            if license.payload.schema != schema {
                anyhow::bail!(
                    "Schema mismatch: expected '{}', found '{}'",
                    schema,
                    license.payload.schema
                );
            }

            // Decode public key
            let pubkey_bytes = STANDARD
                .decode(&public_key)
                .map_err(|e| anyhow::anyhow!("Invalid public key encoding: {e}"))?;
            let pubkey_array: [u8; 32] = pubkey_bytes
                .try_into()
                .map_err(|_| anyhow::anyhow!("Public key must be exactly 32 bytes"))?;
            let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
                .map_err(|e| anyhow::anyhow!("Invalid public key: {e}"))?;

            // Decode signature
            let sig_bytes = STANDARD
                .decode(&license.signature)
                .map_err(|e| anyhow::anyhow!("Invalid signature encoding: {e}"))?;
            let sig_array: [u8; 64] = sig_bytes
                .try_into()
                .map_err(|_| anyhow::anyhow!("Signature must be exactly 64 bytes"))?;
            let signature = ed25519_dalek::Signature::from_bytes(&sig_array);

            // Verify
            let payload_bytes = serde_json::to_vec(&license.payload)?;
            verifying_key
                .verify(&payload_bytes, &signature)
                .map_err(|_| {
                    anyhow::anyhow!(
                        "Signature verification FAILED — license is invalid or tampered"
                    )
                })?;

            eprintln!("✓ License signature is valid");
            eprintln!("  Schema    : {}", license.payload.schema);
            eprintln!("  ID        : {}", license.payload.license_id);
            eprintln!("  Org       : {}", license.payload.issued_to_org);
            eprintln!("  Type      : {}", license.payload.license_type);
            eprintln!("  Issued    : {}", license.payload.issued_at);
            if let Some(email) = &license.payload.contact_email {
                eprintln!("  Email     : {email}");
            }
        }
    }

    Ok(())
}
