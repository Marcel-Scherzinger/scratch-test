use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(long, required = true)]
    pub folder: PathBuf,

    #[arg(long, short = 'e')]
    pub exercise: u8,
}
