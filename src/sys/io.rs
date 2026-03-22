// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # Fail-Safe I/O Operations
//!
//! Manages file reading and writing with catastrophic failure in mind.
//!
//! ## Architectural Invariants
//! - **Atomic Writes:** Uses temporary hidden files (`.tmp`) during processing. The original file
//!   is only replaced via an atomic OS-level rename if and only if the process reaches 100%.
//! - **Pre-flight Checks:** Validates available disk space before initiating I/O to prevent mid-operation
//!   crashes due to storage exhaustion.