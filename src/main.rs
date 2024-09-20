use std::env;

use anyhow::Ok;
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct MovieRating {
    rate: u8,
    entity: u64,
    viewDate: u64,
    timestamp: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let cookie_str = env::var("COOKIE_STR").expect("COOKIE_STR should be set");

    let url = "https://www.filmweb.pl/api/v1/logged/vote/title/film?page=1";
    let client = Client::new();

    let response = client
        .get(url)
        .header("Cookie", cookie_str)
        .send()
        .await?
        .text()
        .await?;

    let ratings: Vec<MovieRating> = serde_json::from_str(&response)?;
    println!("{:#?}", ratings);

    Ok(())
}
