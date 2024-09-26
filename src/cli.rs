use clap::{Parser, ValueEnum};

#[derive(Debug, ValueEnum, Clone)]
pub enum FetchType {
    Movies,
    Series,
    Games,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about =  None)]
pub struct Args {
    /// Type of resource to fetch
    #[arg(value_enum, short, long, default_value_t = FetchType::Movies)]
    pub fetch: FetchType,
}
