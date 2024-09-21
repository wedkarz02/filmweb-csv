use std::env;

use anyhow::Ok;

mod api;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let base_url = "https://www.filmweb.pl/api/v1/logged";
    let cookie_header =
        env::var("COOKIE_HEADER").expect("COOKIE_HEADER should be set");

    let ratings = api::fetch_movies(base_url, &cookie_header).await?;

    println!("{:#?}", ratings);

    Ok(())
}
