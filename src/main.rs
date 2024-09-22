use std::env;

use anyhow::Context;
use api::entity_to_movie;

mod api;
mod error;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context(".env file not found")?;

    let base_url = "https://www.filmweb.pl/api/v1";
    let cookie_header =
        env::var("COOKIE_HEADER").expect("COOKIE_HEADER should be set");

    let ratings = api::fetch_ratings(base_url, &cookie_header).await?;

    for rating in ratings {
        let movie_info = entity_to_movie(base_url, &rating).await?;
        println!("{:#?}", movie_info);
    }

    Ok(())
}
