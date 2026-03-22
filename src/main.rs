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

use std::string::String;
use clap::Parser;
mod cli;
use cli::Cli;

mod laboratorio {
    pub fn abrir_portal() {
        println!("O Mundo Invertido está vazando! 🌑");
    }
}


fn mock_encrypt(text: &str) -> String {
    let reverse_text = text.chars().rev().collect::<String>();
    reverse_text
}

fn main() {
    let args = Cli::parse();

    println!("A forja do FerroVault está acesa. 🦀");
    laboratorio::abrir_portal();
    let typed_text = &args.name;
    let rev_text: String = mock_encrypt(&typed_text);
    println!("Nome invertido: {}", typed_text);
    println!("Nome invertido: {}", rev_text);

    if let Err(e) = cli::handlers::run(args) {
        eprintln!("Erro: {}", e);
        std::process::exit(1);
    }
}