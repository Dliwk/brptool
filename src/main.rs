mod brp;
mod cli;
mod consts;
mod error;
mod huffman;
mod session;

use anyhow::Result;
use clap::Parser;
use std::fs::File;

use crate::brp::load_replay;
use crate::cli::CliArgs;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let file = File::open(args.path)?;
    load_replay(file)?;

    Ok(())
}
