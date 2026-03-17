# 🦀 FerroVault

> **⚠️ STATUS: WORK IN PROGRESS (WIP)**
> *This project is currently under active development. The architectural foundation is being laid out, and core cryptographic modules are being implemented incrementally. Not all features listed below are fully operational yet. Expect breaking changes.*

**FerroVault** is a high-performance, fail-safe cryptographic toolkit built entirely in Rust. Designed for zero-trust environments and automated server pipelines, it provides a comprehensive suite of security tools ranging from symmetric encryption to bare-metal file shredding and multithreaded hash cracking.

## 🎯 The Architectural Thesis
This is not just a wrapper around encryption libraries. FerroVault is built on three uncompromising principles:
1. **Memory Paranoia:** Sensitive data (keys, plaintexts) floating in RAM are actively zeroed out immediately after use to prevent memory-dump leaks.
2. **Fail-Safe Execution:** If an operation fails (e.g., I/O error mid-encryption), the system aborts cleanly, ensuring the original data is never silently corrupted.
3. **Interface Agnosticism:** Core cryptographic logic is strictly decoupled from the CLI, paving the way for headless server execution or future GUI integrations.

## 🛡️ The Arsenal (Core Features)

* 🔐 **The Bunker (Symmetric Encryption):** AES-256-GCM authenticated encryption for files and data streams.
* 📜 **The Diplomat (Asymmetric Cryptography):** Ed25519 elliptic curve key-pair generation for secure digital signatures and identity verification.
* ☢️ **Scorched Earth (File Shredder):** Low-level multi-pass disk overwriting (0x00, 0xFF, random) to ensure deleted files are cryptographically unrecoverable by forensic tools.
* 🦋 **The Infiltrator (Hash Cracker):** High-speed, dictionary-based SHA-256 hash cracking, utilizing aggressive data parallelism across all CPU cores.
* 🎲 **The Forge (CSPRNG):** Cryptographically secure pseudo-random number generation for unbreakable passwords and salts.

## ⚙️ The Tech Stack
* **Language:** Rust (Strict memory safety and zero-cost abstractions).
* **Cryptography:** `ring` and `aes-gcm` (Audited, government-standard cryptographic primitives).
* **Concurrency:** `rayon` (Fearless data parallelism).
* **Memory Management:** `zeroize` (Secure memory wiping).
* **CLI Interface:** `clap` (Robust, POSIX-compliant command-line parsing).

## 🚀 Getting Started (Coming Soon)

Once the initial modules are stabilized, you will be able to run FerroVault via Cargo:

```bash
# Clone the repository
git clone [https://github.com/yourusername/ferrovault.git](https://github.com/yourusername/ferrovault.git)
cd ferrovault

# Build for release (optimized)
cargo build --release

# Access the CLI help menu
./target/release/ferrovault --help
⚖️ Security Disclaimer & License
Disclaimer: This tool is developed for educational purposes, portfolio demonstration, and personal use. While it implements standard cryptographic libraries, it has not undergone independent professional security auditing. Use at your own risk.

License: Licensed under the Apache License 2.0.
