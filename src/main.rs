// Copyright (c) 2026 Luiz Felipe. All rights reserved.
// Licensed under the Apache License, Version 2.0.
// See the LICENSE file in the project root for full license information.

//! # FerroVault CLI Entrypoint
//!
//! Serves as the binary execution bridge, parsing command-line arguments via `clap`
//! and orchestrating high-level logic by delegating all operations to the core library.
//!
//! ## Architectural Invariants
//! - **Logic Decoupling:** Must not implement cryptographic or system-level logic; it acts strictly as a proxy to `lib.rs`.
//! - **Process Integrity:** Responsible for mapping internal library errors to POSIX-compliant exit codes and user-friendly terminal feedback.

use clap::Parser;

mod laboratorio {
    pub fn abrir_portal() {
        println!("O Mundo Invertido está vazando! 🌑");
    }
}
#[derive(Parser)]
struct Cli {
    main_num: i32,
    secondary_num: i32,
}
fn somar(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    let args = Cli::parse();
    println!("A forja do FerroVault está acesa. 🦀");
    laboratorio::abrir_portal();
    println!("A soma entre {args.main_num} e {args.secondary_menu}");
}