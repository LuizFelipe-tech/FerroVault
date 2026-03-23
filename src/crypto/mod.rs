// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # Cryptography Engine
//!
//! Orchestrates all mathematical operations. Strictly relies on industry-standard, audited
//! crates (like `ring` or `aes-gcm`).
//! **Invariant:** Never implements custom cryptographic math.

mod asymmetric;
mod csprng;
mod hash;
mod symmetric;
