// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # Symmetric Cryptography (The Bunker)
//!
//! Implements AES-256-GCM operations for encrypting and decrypting data at rest.
//!
//! ## Architectural Invariants
//! - **Authenticity Validation:** GCM mode ensures data is not only hidden but mathematically
//!   guaranteed against tampering.
//! - **Streaming Processing:** Must support chunked processing (e.g., 4 MB chunks) to handle massive
//!   files without exhausting system RAM.
