use std::{
    env,
    fmt::Debug,
    fs::{create_dir_all, File},
    path::{self, Path},
};

use anyhow::Context;
use api::Rating;
use clap::Parser;
use cli::{FetchFrom, FetchType};
use csv::WriterBuilder;
use futures::future::try_join_all;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use tokio::time::Instant;

mod api;
mod cli;
mod error;
mod util;

static BASE_URL: &str = "https://www.filmweb.pl/api/v1";

#[allow(unused)]
#[derive(Clone)]
struct Config {
    fetch_type: FetchType,
    fetch_from: FetchFrom,
    cookie_header: String,
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("fetch_type", &self.fetch_type)
            .field("fetch_from", &self.fetch_from)
            .field("cookie_header", &"...")
            .finish()
    }
}

fn ratings_to_csv(file_path: &Path, ratings: &[Rating]) -> anyhow::Result<()> {
    if let Some(parent) = file_path.parent() {
        create_dir_all(parent)?;
    }

    let out = File::create(file_path)?;
    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(out);

    for rating in ratings {
        writer.serialize(rating)?;
    }

    writer.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    dotenvy::dotenv().context(".env file not found")?;

    let cookie_header =
        env::var("COOKIE_HEADER").expect("COOKIE_HEADER should be set");

    let args = cli::Args::parse();

    let config = Config {
        fetch_type: args.fetch,
        fetch_from: args.from,
        cookie_header,
    };

    println!("config: {:#?}", config);

    println!("[INFO]: Fetching from the API...");
    let ratings = api::fetch_movie_ratings(&config).await?;

    let pb = ProgressBar::new(ratings.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    pb.inc(0);
    let movies = try_join_all(ratings.iter().map(|raw| {
        let pb = pb.clone();
        let cfg = config.clone();
        let client = Client::new();
        async move {
            let movie = api::raw_to_rating(&cfg, &client, raw).await;
            pb.inc(1);
            movie
        }
    }))
    .await?;

    pb.finish();

    let file_path = Path::new("exports/exports.csv");
    ratings_to_csv(file_path, &movies)?;
    let abs_path = path::absolute(file_path)?;
    println!("[INFO]: Data saved to: {:?}", abs_path);

    let elapsed = Instant::now().duration_since(start);
    println!("[INFO]: Total time elapsed: {:.4?}", elapsed);

    Ok(())
}
