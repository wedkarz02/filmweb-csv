use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, ValueEnum, Clone)]
pub enum FetchType {
    Movies,
    Series,
    Games,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum FetchFrom {
    Rated,
    Watchlist,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about =  None)]
pub struct Args {
    /// Type of resource to fetch
    #[arg(value_enum, long, default_value_t = FetchType::Movies)]
    pub fetch: FetchType,

    /// Fetch from rated or watchlist
    #[arg(value_enum, long, default_value_t = FetchFrom::Rated)]
    pub from: FetchFrom,

    /// Specify the output directory
    #[arg(short, long, default_value = "./exports/")]
    pub output: PathBuf,

    /// Log more details to stdout
    #[arg(short, long, action)]
    pub verbose: bool,
}
