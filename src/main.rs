#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    nonstandard_style
)]

//! calculator
use crate::app::*;
use crate::commands::*;
use clap::Parser;

mod app;
mod commands;

fn main() {
    env_logger::try_init().ok();
    let app: Calculator = Calculator::parse();


    match app.command {
        // TODO :
        //
        // Faire en sorte de match l'enum `Command` avec la fonction corespondante
        //
        // SYNTAXE :
        //
        // MATCH => fichier::fonction(params),
        //
        // DOC: https://doc.rust-lang.org/rust-by-example/flow_control/match.html

        //* A remplire */ => /* A remplire */,
        //* A remplire */ => /* A remplire */,
    };
}