// Copyright (c) 2026-Present Luiz Felipe do Nascimento Melos. All rights reserved.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! # FerroVault CLI Entrypoint
//!
//! Serves as the binary execution bridge, parsing command-line arguments via `clap`
//! and orchestrating high-level logic by delegating all operations to the core library.
//!
//! ## Architectural Invariants
//! - **Logic Decoupling:** Must not implement cryptographic or system-level logic; it acts strictly as a proxy to `lib.rs`.
//! - **Process Integrity:** Responsible for mapping internal library errors to POSIX-compliant exit codes and user-friendly terminal feedback.

use clap::Parser;
use ferro_vault::cli::handlers::run;
use ferro_vault::cli::Cli;
use ferro_vault::cli::MainCommands;
use ferro_vault::error::FerroError;

fn main() -> Result<(), FerroError> {
    let args = Cli::parse();

    println!("A forja do FerroVault está acesa. 🦀");
    run()?;
    execute_command(args.mycommand);
    Ok(())
}

fn execute_command(command: MainCommands) {
    match command {
        MainCommands::Encrypt(_) => println!("🔒 Iniciando criptografia de alto nível..."),
        MainCommands::Decrypt(_) => println!("🔓 Analisando chaves para descriptografia..."),
        MainCommands::Shred => println!("💥 Destruindo arquivos... irrecuperável."),
        MainCommands::Crack => println!("💀 Iniciando força bruta. Aguarde..."),
        MainCommands::Chat => println!("💬 Estabelecendo túnel de comunicação seguro..."),
    }
}
