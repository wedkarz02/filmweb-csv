use std::{
    fmt::Debug,
    fs::{self, create_dir_all, File},
    path::{self, Path, PathBuf},
    process,
};

use api::ItemData;
use clap::Parser;
use cli::{Args, FetchFrom, FetchType};
use csv::WriterBuilder;
use error::AppError;
use flexi_logger::{Criterion, Duplicate, FileSpec, Logger};
use futures::future::try_join_all;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, LevelFilter};
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

trait LogAndExitOnErr<T> {
    fn log_and_exit_on_err(self, msg: &str) -> anyhow::Result<T>;
}

impl<T, E: std::fmt::Display> LogAndExitOnErr<T> for Result<T, E> {
    fn log_and_exit_on_err(self, msg: &str) -> anyhow::Result<T> {
        self.map_err(|e| {
            error!("{}: {}", msg, e);
            process::exit(1);
        })
    }
}

fn log_fmt(
    write: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    write!(
        write,
        "[{}] {} [{}]: {}",
        record.level(),
        now.format("%Y-%m-%d %H:%M:%S"),
        record.target(),
        record.args()
    )
}

fn setup_logger(args: &Args, home_dir: PathBuf) -> anyhow::Result<Logger> {
    let logs_path = home_dir.join("logs");
    if let Some(parent) = logs_path.parent() {
        create_dir_all(parent)?;
    }

    let stdout_level = match args.verbose {
        true => Duplicate::All,
        false => Duplicate::Error,
    };

    Ok(Logger::with(LevelFilter::Info)
        .log_to_file(FileSpec::default().directory(logs_path))
        .duplicate_to_stdout(stdout_level)
        .rotate(
            Criterion::Size(1024 * 1024),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(5),
        )
        .write_mode(flexi_logger::WriteMode::Direct)
        .format(log_fmt))
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

async fn run_with_config(
    config: &Config,
) -> anyhow::Result<(Vec<ItemData>, String)> {
    let (endpoint_suffix, file_name) = match config.fetch_type {
        cli::FetchType::Movies => ("film", "movies"),
        cli::FetchType::Series => ("serial", "series"),
        cli::FetchType::Games => ("videogame", "games"),
    };

    let items = match config.fetch_from {
        cli::FetchFrom::Rated => {
            get_items::<api::RatingRaw>(
                config,
                &format!("logged/vote/title/{}", endpoint_suffix),
            )
            .await?
        }
        cli::FetchFrom::Watchlist => {
            get_items::<api::WatchlistRaw>(
                config,
                &format!("logged/want2see/{}", endpoint_suffix),
            )
            .await?
        }
    };

    let output_file = format!(
        "{}_{}.csv",
        file_name,
        match config.fetch_from {
            cli::FetchFrom::Rated => "rated",
            cli::FetchFrom::Watchlist => "watchlist",
        }
    );

    Ok((items, output_file))
}

fn read_cookie(args: &Args, home_dir: PathBuf) -> anyhow::Result<String> {
    if let Some(cookie) = &args.cookie {
        return Ok(cookie.clone());
    }

    let cookie_path = home_dir.join("credentials.txt");
    Ok(fs::read_to_string(cookie_path).map(|s| s.trim().to_string())?)
}

fn save_cookie(cookie: &str, home_dir: PathBuf) -> anyhow::Result<()> {
    let cookie_path = home_dir.join("credentials.txt");
    fs::write(&cookie_path, cookie)?;
    info!("Tokens written to: {:?}", path::absolute(cookie_path)?);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let args = cli::Args::parse();
    let home_dir = dirs::home_dir()
        .ok_or(AppError::WithContext("Home directory not found".into()))?
        .join(".filmweb-csv");

    setup_logger(&args, home_dir.clone())
        .log_and_exit_on_err("Failed to setup logger")?
        .start()?;
    info!("Logger initialized");

    let cookie_header = read_cookie(&args, home_dir.clone())
        .log_and_exit_on_err("Cookie header not provided")?;

    if args.save_cookie {
        save_cookie(&cookie_header, home_dir.clone())
            .log_and_exit_on_err("Failed to save cookie header")?;
    }

    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    let config = Config {
        fetch_type: args.fetch,
        fetch_from: args.from,
        cookie_header,
        progress_bar,
    };

    let (items, file_name) = run_with_config(&config)
        .await
        .log_and_exit_on_err("Resource fetching failed")?;

    let mut file_path = args.output;
    file_path.push(file_name);

    item_to_csv(&file_path, &items)
        .log_and_exit_on_err("Saving to file failed")?;

    info!("Data written to: {:?}", path::absolute(file_path)?);
    let elapsed = Instant::now().duration_since(start);
    info!("Total time elapsed: {:.4?}", elapsed);

    Ok(())
}
