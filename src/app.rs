use crate::commands::*;
use clap::{ArgEnum, Parser};

#[derive(Parser)]
/// Declares every available command and option of the program.
pub struct Calculator {
    #[clap(subcommand, arg_enum)]
    pub command: Command,
}

/// Help for release using terraform with chart helm
#[derive(Debug, ArgEnum, Parser)]
pub enum Command {
    // TODO :
    // 
    // Faire en sorte que l'enum propose deux possibilité,
    // `Mulitplication` ou `Addition`, prenant chacun en parametre
    // la structure `Options` corespondante

    //* A remplire */,
    //* A remplire */,
}