use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Path to .brp file
    #[arg(short, long)]
    pub path: PathBuf,
}
