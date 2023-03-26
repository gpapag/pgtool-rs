//! cli
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pg_data")]
#[command(author = "George Papageorgiou")]
#[command(version = "0.1.0")]
#[command(about = "Provides cli tools to interact with Postgresql data stores")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Path to .pgpass
    #[arg(short, long)]
    pub pass_file: std::path::PathBuf,

    /// Instance postgres db
    #[arg(short, long)]
    pub env: String,
}

impl Args {
    pub fn parse_args() -> Args {
        Args::parse()
    }
}
