// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # CLI Arguments & Parsing
//!
//! Defines the data structures for `clap` to parse user inputs executable commands.
//!
//! ## Responsibilities
//! - Maps subcommands (e.g., `encrypt`, `shred`, `crack`) to internal enums.
//! - Handles environmental variables injection (e.g., `TOOL_AES_KEY`) for CI/CD pipelines.
//! - Enforces strict validation of required flags before passing state to handlers.

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "FerroVault")]
#[command(author = "Luiz Felipe do Nascimento Melos")]
#[command(version = "0.1.0")]
#[command(
    about = "High-performance CLI vault",
    long_about = "FerroVault is a high-performance, security-first command-line interface (CLI) engineered for the orchestration of sensitive data lifecycle management. Built exclusively in Rust, it bridges the gap between low-level system primitives and high-level cryptographic workflows, providing a robust environment for data encryption, digital signatures, and irretrievable destruction.

Core Architectural Pillars
Cryptographic Rigor: FerroVault leverages industry-standard primitives (AES-256-GCM, Ed25519) to ensure not just confidentiality, but absolute data integrity. By enforcing a Zero-Trust model, the system assumes the host environment is hostile, protecting secrets through strict memory isolation and secure pseudo-random number generation (CSPRNG).

Operational Resilience: Designed with the 'Scorched Earth' philosophy, the tool features an advanced file shredding engine. It bypasses standard file-system deletion by executing multiple overwrite passes directly on disk sectors, rendering data recovery impossible even for forensic-grade analysis.

Atomic Process Integrity: To prevent data corruption in mission-critical environments, FerroVault implements Atomic Write protocols. Every operation—from encryption to shredding—is treated as a transactional unit; if a system failure occurs mid-process, the original state remains untouched, preventing the creation of 'corrupted artifacts.'

Headless-First Design: While providing an intuitive interactive mode for developers, FerroVault is optimized for CI/CD pipelines and automated server environments. It offers standardized exit codes, sanitized logging to prevent secret leakage, and high-concurrency processing via data parallelism.

Technical Specifications
Memory Safety: Zeroized memory buffers to prevent 'Cold Boot' attacks and RAM forensics.

Asymmetric Identity: Integrated public-key infrastructure (PKI) for non-repudiable file signing.

Hardware Agnostic: Optimized for minimal overhead, ensuring high-speed processing on both workstations and resource-constrained server nodes."
)]
pub struct Cli {
    #[command(subcommand)]
    pub mycommand: MainCommands,
}

#[derive(Debug, Clone, Args)]
pub struct EncryptionArgs {
    // (Required) The target archive's path.
    #[arg(short, long)]
    input: PathBuf,

    // (Optional) Where to save the result.
    #[arg(short, long)]
    output: Option<PathBuf>,

    // (Optional) An archive containing a cryptographic key.
    #[arg(short, long)]
    key_file: Option<PathBuf>,

    // (Optional) Deletes or not the original file after encryption.
    #[arg(short, long)]
    delete: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum MainCommands {
    Encrypt(EncryptionArgs),
    Decrypt(EncryptionArgs),
    Shred,
    Crack,
    Chat,
}
