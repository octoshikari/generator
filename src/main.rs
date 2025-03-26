mod generator;
mod schema;

use crate::generator::generate;
use clap::{Arg, Command};
use human_panic::setup_panic;
use log::error;

fn main() {
    setup_panic!();
    let matches = Command::new("fts-codegen")
        .version("0.1.0")
        .about("Does awesome things")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .about("Generate types depends on target programming language")
                .arg(
                    Arg::new("target")
                        .alias("-t")
                        .value_parser(["typescript", "csharp", "python"])
                        .default_value("typescript"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            match generate(){
                Ok(result) => {},
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
