// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # Cryptographically Secure Pseudo-Random Number Generator (The Forge)
//!
//! Provides mathematically chaotic generation for encryption keys, initialization vectors (IVs),
//! and secure passwords. Strictly avoids standard OS-level random number generators
//! that exhibit predictable entropy patterns.