// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # CLI Arguments & Parsing
//!
//! Defines the data structures for `clap` to parse user inputs executable commands.
//!
//! ## Responsibilities
//! - Maps subcommands (e.g., `encrypt`, `shred`, `crack`) to internal enums.
//! - Handles environmental variables injection (e.g., `TOOL_AES_KEY`) for CI/CD pipelines.
//! - Enforces strict validation of required flags before passing state to handlers.