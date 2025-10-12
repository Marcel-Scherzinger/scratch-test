use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Submissions {
        #[arg(long, required = true)]
        folder: PathBuf,

        #[arg(long, short = 'e')]
        exercise: u8,
    },
    Single {
        #[arg(long, required = true)]
        file: PathBuf,
        #[arg(long, short = 'e')]
        exercise: u8,
        #[command(flatten)]
        part: ExPartSpec,
    },
    FindIdIntersections {
        #[arg(required = true)]
        folder: PathBuf,
        #[arg(long, short = 's')]
        no_ignore_siblings: bool,
        #[arg(long, short = 'm', default_value_t = 0)]
        min_common: usize,
    },
}

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
pub struct ExPartSpec {
    #[arg(long)]
    pub part: Option<char>,
    #[arg(short = 'a')]
    pub a: bool,
    #[arg(short = 'b')]
    pub b: bool,
    #[arg(short = 'c')]
    pub c: bool,
}
