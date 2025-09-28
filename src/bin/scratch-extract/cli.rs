use std::path::PathBuf;

use clap::Parser;
use derive_more::Display;

#[derive(Debug, Clone, Display, Copy, clap::ValueEnum)]
pub enum Data {
    Json,
    Model,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(flatten)]
    pub input: InputGroup,

    /// If absent use stdout, if used without parameter replace only extension
    ///
    /// Extension replacement is not available when reading from stdin
    #[arg(short = 'o', long)]
    pub output_file: Option<Option<PathBuf>>,

    /// Never overwrite existing files
    #[arg(short = 'W', long, default_value_t = false)]
    pub no_overwrite: bool,

    /// default: json or based on extension
    #[arg(short = 's', long)]
    pub select: Option<Data>,
}

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
pub struct InputGroup {
    /// Read input file from provided path
    #[arg(required = false, group = "input")]
    pub input_file: Option<PathBuf>,

    /// Read input file (binary) content via stdin
    #[arg(long, required = false, group = "input")]
    pub stdin: bool,
}
