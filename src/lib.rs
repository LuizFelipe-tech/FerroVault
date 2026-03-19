// Copyright (c) 2026 Luiz Felipe. All rights reserved.
// Licensed under the Apache License, Version 2.0.
// See the LICENSE file in the project root for full license information.

//! # FerroVault Core Library
//!
//! The central engine of the project. It exposes a modular API for symmetric/asymmetric
//! cryptography, secure file deletion, and multithreaded auditing utilities.
//!
//! ## Architectural Invariants
//! - **Encapsulation:** Only stable and vetted modules are exposed to the public API surface to prevent unauthorized access to internal primitives.
//! - **State Sovereignty:** Core cryptographic functions must remain platform-agnostic, delegating OS-specific operations to the `sys` module.
pub mod cli;
pub mod crypto;
pub mod sys;
pub mod utils;