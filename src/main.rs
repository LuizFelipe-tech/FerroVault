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
mod cli;
use cli::Cli;
use cli::handlers::run;
use cli::MainCommands;

mod laboratorio {
    pub fn abrir_portal() {
        println!("O Mundo Invertido está vazando! 🌑");
    }
}

fn main() {
    let args = Cli::parse();

    println!("A forja do FerroVault está acesa. 🦀");
    let start = run(&args).expect("Nenhum argumento encontrado");
    laboratorio::abrir_portal();
    execute_command(args.mycommand)
}

fn execute_command(command: MainCommands) {
    match command {
        MainCommands::Encrypt => println!("🔒 Iniciando criptografia de alto nível..."),
        MainCommands::Decrypt => println!("🔓 Analisando chaves para descriptografia..."),
        MainCommands::Shred => println!("💥 Destruindo arquivos... irrecuperável."),
        MainCommands::Crack => println!("💀 Iniciando força bruta. Aguarde..."),
        MainCommands::Chat => println!("💬 Estabelecendo túnel de comunicação seguro...")
    }
}