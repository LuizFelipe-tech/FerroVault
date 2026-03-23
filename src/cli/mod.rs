// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # Command Line Interface Module
//!
//! Encapsulates the user-facing terminal experience and headless server execution modes.
//! Responsible for argument parsing, user prompting, and providing visual feedback without
//! polluting the core cryptographic logic.

pub mod args;
pub mod handlers;
pub use args::Cli;
pub use args::MainCommands;
