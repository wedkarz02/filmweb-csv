use std::env;

use anyhow::Context;
use api::entity_to_movie;

mod api;
mod error;
mod util;

static BASE_URL: &str = "https://www.filmweb.pl/api/v1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context(".env file not found")?;

    let cookie_header =
        env::var("COOKIE_HEADER").expect("COOKIE_HEADER should be set");

    let ratings = api::fetch_ratings(&cookie_header).await?;

    for rating in ratings {
        let movie_info = entity_to_movie(&rating).await?;
        println!("{:#?}", movie_info);
    }

    Ok(())
}
