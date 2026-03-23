// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # Paranoid Memory Management
//!
//! Enforces the active destruction of sensitive data in RAM. Uses tools like `zeroize`
//! to ensure that cryptographic keys, passwords and plaintext buffers are physically overwritten
//! with zeros the exact microsecond they fall out of scope.
