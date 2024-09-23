use std::{env, fs::File, path::Path};

use anyhow::Context;
use api::{entity_to_movie, MovieRating};
use csv::WriterBuilder;
use futures::future::try_join_all;

mod api;
mod error;
mod util;

static BASE_URL: &str = "https://www.filmweb.pl/api/v1";

fn movies_to_csv(
    file_path: &Path,
    movies: &[MovieRating],
) -> anyhow::Result<()> {
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
    dotenvy::dotenv().context(".env file not found")?;

    let cookie_header =
        env::var("COOKIE_HEADER").expect("COOKIE_HEADER should be set");

    let ratings = api::fetch_ratings(&cookie_header).await?;
    let movies = try_join_all(ratings.iter().map(entity_to_movie)).await?;

    movies_to_csv(Path::new("exports.csv"), &movies)?;

    Ok(())
}
