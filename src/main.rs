use std::{
    env,
    fmt::Debug,
    fs::{create_dir_all, File},
    path::{self, Path},
};

use anyhow::Context;
use api::ItemData;
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

#[derive(Clone)]
struct Config {
    fetch_type: FetchType,
    fetch_from: FetchFrom,
    cookie_header: String,
    progress_bar: ProgressBar,
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("fetch_type", &self.fetch_type)
            .field("fetch_from", &self.fetch_from)
            .field("cookie_header", &"...")
            .field("progress_bar", &self.progress_bar)
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

async fn execute_futures<T>(
    config: &Config,
    raw_items: &[T],
) -> anyhow::Result<Vec<ItemData>>
where
    T: api::RawEntity,
{
    config.progress_bar.inc(0);
    let movies = try_join_all(raw_items.iter().map(|raw| {
        let pb = config.progress_bar.clone();
        let cfg = config.clone();
        let client = Client::new();
        async move {
            let item = api::raw_to_item(&cfg, &client, raw).await;
            pb.inc(1);
            item
        }
    }))
    .await?;
    config.progress_bar.finish();
    Ok(movies)
}

async fn get_items<T>(
    config: &Config,
    endpoint: &str,
) -> anyhow::Result<Vec<ItemData>>
where
    T: api::RawEntity + serde::de::DeserializeOwned,
{
    let items: Vec<T> = api::fetch_pages(config, endpoint).await?;
    config.progress_bar.set_length(items.len() as u64);

    execute_futures(config, &items).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    dotenvy::dotenv().context(".env file not found")?;

    let args = cli::Args::parse();

    let cookie_header =
        env::var("COOKIE_HEADER").expect("COOKIE_HEADER should be set");

    let pb = ProgressBar::new(0);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    let config = Config {
        fetch_type: args.fetch,
        fetch_from: args.from,
        cookie_header,
        progress_bar: pb,
    };

    println!("[INFO]: Fetching from the API...");

    let (items, file_name) = match (&config.fetch_type, &config.fetch_from) {
        (cli::FetchType::Movies, cli::FetchFrom::Rated) => (
            get_items::<api::RatingRaw>(&config, "logged/vote/title/film")
                .await?,
            "movies_rated.csv",
        ),
        (cli::FetchType::Movies, cli::FetchFrom::Watchlist) => (
            get_items::<api::WatchlistRaw>(&config, "logged/want2see/film")
                .await?,
            "movies_watchlist.csv",
        ),
        (cli::FetchType::Series, cli::FetchFrom::Rated) => (
            get_items::<api::RatingRaw>(&config, "logged/vote/title/serial")
                .await?,
            "series_rated.csv",
        ),
        (cli::FetchType::Series, cli::FetchFrom::Watchlist) => (
            get_items::<api::WatchlistRaw>(&config, "logged/want2see/serial")
                .await?,
            "series_watchlist.csv",
        ),
        (cli::FetchType::Games, cli::FetchFrom::Rated) => (
            get_items::<api::RatingRaw>(&config, "logged/vote/title/videogame")
                .await?,
            "games_rated.csv",
        ),
        (cli::FetchType::Games, cli::FetchFrom::Watchlist) => (
            get_items::<api::WatchlistRaw>(
                &config,
                "logged/want2see/videogame",
            )
            .await?,
            "games_watchlist.csv",
        ),
    };

    let mut file_path = args.output;
    file_path.push(file_name);

    item_to_csv(&file_path, &items)?;
    println!("[INFO]: Data saved to: {:?}", path::absolute(file_path)?);

    let elapsed = Instant::now().duration_since(start);
    println!("[INFO]: Total time elapsed: {:.4?}", elapsed);

    Ok(())
}
