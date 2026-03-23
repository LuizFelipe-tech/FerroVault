// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # Hash & Cracking Operations
//!
//! Implements one-way functions (SHA-256) for integrity checks and password derivation.
//! Features a highly concurrent dictionary attack engine utilizing `rayon` for massive data
//! parallelism across available CPU threads.
