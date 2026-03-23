// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # Sanitized Logging Engine
//!
//! Handles application telemetry and user feedback.
//! **Invariant:** Must actively blind attackers by sanitizing outputs. Logs must NEVER leak
//! plaintext data, private keys, or sensitive internal states to `stdout`/`stderr`.
