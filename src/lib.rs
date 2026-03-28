// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # FerroVault Core Library
//!
//! The central nervous system of the FerroVault tool. This library strictly implements
//! the Interface Agnosticism principle, remaining entirely unaware of whether it is being
//! invoked by a CLI, a GUI, or a background daemon.
//!
//! ## Architectural Invariants
//! - **Domain Isolation:** Exposes public traits and high-level orchestrators for cryptography,
//!   system operations, and memory management.
//! - **Fail-Safe Propagation:** Never panics. All internal errors are caught and returned as
//!   structured `Result<T, CustomError>` to be handled by the presentation layer.
pub mod cli;
pub mod crypto;
pub mod error;
pub mod sys;
pub mod utils;
