// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # File Shredder (Scorched Earth Protocol)
//!
//! Executes irretrievable data destruction by bypassing standard OS file deletion.
//!
//! ## Mechanism
//! Opens file descriptors in low-level write mode to inject multiple passes of raw bytes
//! (0×00, 0xFF, and CSPRNG noise) directly onto the physical disk sectors before unlinking
//! the reference from the file system.
