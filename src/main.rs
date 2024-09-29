use std::{
    env,
    fmt::Debug,
    fs::{create_dir_all, File},
    path::{self, Path},
};

use anyhow::Context;
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

fn item_to_csv(
    file_path: &Path,
    items: &[api::ItemData],
) -> anyhow::Result<()> {
    if let Some(parent) = file_path.parent() {
        create_dir_all(parent)?;
    }

    let out = File::create(file_path)?;
    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(out);

    for item in items {
        writer.serialize(item)?;
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
    // let wishlisted_movies_raw: Vec<api::WishlistedRaw> =
    //     api::fetch_pages(&config, "logged/want2see/film").await?;
    let ratings: Vec<api::RatingRaw> =
        api::fetch_pages(&config, "logged/vote/title/film").await?;

    // let pb = ProgressBar::new(wishlisted_movies_raw.len() as u64);
    let pb = ProgressBar::new(ratings.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    pb.inc(0);
    // let movies = try_join_all(wishlisted_movies_raw.iter().map(|raw| {
    let movies = try_join_all(ratings.iter().map(|raw| {
        let pb = pb.clone();
        let cfg = config.clone();
        let client = Client::new();
        async move {
            let item = api::raw_to_item(&cfg, &client, raw).await;
            pb.inc(1);
            item
        }
    }))
    .await?;

    pb.finish();

    let file_path = Path::new("exports/exports.csv");
    item_to_csv(file_path, &movies)?;
    println!("[INFO]: Data saved to: {:?}", path::absolute(file_path)?);

    let elapsed = Instant::now().duration_since(start);
    println!("[INFO]: Total time elapsed: {:.4?}", elapsed);

    Ok(())
}
