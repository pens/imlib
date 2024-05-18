//! This is a program for organizing my photo library, acting as a wrapper around 'exiftool'.
//!
//! Copyright 2023-4 Seth Pendergrass. See LICENSE.

use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

mod catalog;
mod commands;
mod setup;

#[derive(Parser)]
struct Args {
    /// Directory of photo library. Updates default in XDG_CONFIG_HOME.
    #[arg(short, global = true)]
    library: Option<PathBuf>,

    /// Verbosity level. Max: 2.
    #[arg(short, action = ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

// TODO flag for live photo video deletion
#[derive(Subcommand)]
enum Commands {
    /// Clean library.
    Clean,
    /// Import photos from path into library.
    Import { path: PathBuf },
}

fn main() {
    let args = Args::parse();
    setup::configure_logging(args.verbose);
    let library = match setup::get_or_update_library(args.library) {
        Ok(path) => path,
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1);
        }
    };

    match args.command {
        Commands::Clean => commands::clean(&library),
        Commands::Import { path } => commands::import(&library, &path),
    }
}
