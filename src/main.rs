use std::{
    env,
    fs::{create_dir_all, File},
    path::{self, Path},
};

use anyhow::Context;
use api::{entity_to_movie, MovieRating};
use clap::Parser;
use csv::WriterBuilder;
use futures::future::try_join_all;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::time::Instant;

mod api;
mod cli;
mod error;
mod util;

static BASE_URL: &str = "https://www.filmweb.pl/api/v1";

fn movies_to_csv(
    file_path: &Path,
    movies: &[MovieRating],
) -> anyhow::Result<()> {
    if let Some(parent) = file_path.parent() {
        create_dir_all(parent)?;
    }

    let out = File::create(file_path)?;
    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(out);

    for movie in movies {
        writer.serialize(movie)?;
    }

    writer.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    dotenvy::dotenv().context(".env file not found")?;

    let args = cli::Args::parse();

    match args.fetch {
        cli::FetchType::Movies => println!("movies set"),
        cli::FetchType::Games => println!("games set"),
        cli::FetchType::Series => println!("series set"),
    }

    let cookie_header =
        env::var("COOKIE_HEADER").expect("COOKIE_HEADER should be set");

    println!("[INFO]: Fetching from the API...");
    let ratings = api::fetch_ratings(&cookie_header).await?;

    let pb = ProgressBar::new(ratings.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    pb.inc(0);
    let movies = try_join_all(ratings.iter().map(|rating| {
        let pb = pb.clone();
        async move {
            let movie = entity_to_movie(rating).await;
            pb.inc(1);
            movie
        }
    }))
    .await?;

    pb.finish();

    let file_path = Path::new("exports/exports.csv");
    movies_to_csv(file_path, &movies)?;
    let abs_path = path::absolute(file_path)?;
    println!("[INFO]: Data saved to: {:?}", abs_path);

    let elapsed = Instant::now().duration_since(start);
    println!("[INFO]: Total time elapsed: {:?}", elapsed);

    Ok(())
}
