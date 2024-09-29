use reqwest::Client;
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};

use crate::util::ToDate;
use crate::Config;
use crate::{error::ApiError, BASE_URL};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct RatingRaw {
    rate: u8,
    entity: u64,
    viewDate: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rating {
    title: String,
    view_date: String,
    rate: u8,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct GeneralInfo {
    title: String,
    originalTitle: String,
    year: u16,
    r#type: String,
    subType: String,
    posterPath: String,
}

pub async fn get_body(response: Response) -> Result<String, ApiError> {
    match response.status() {
        StatusCode::OK => Ok(response.text().await?),
        StatusCode::BAD_REQUEST => Err(ApiError::BadRequest),
        StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized),
        StatusCode::NOT_FOUND => {
            Err(ApiError::NotFound(response.url().to_string()))
        }
        StatusCode::INTERNAL_SERVER_ERROR => Err(ApiError::InternalServerError),
        _ => Err(ApiError::Unrecognized),
    }
}

pub async fn fetch_resource(
    config: &Config,
    client: &Client,
    endpoint: &str,
) -> Result<String, ApiError> {
    let url = format!("{}/{}", BASE_URL, endpoint);

    let response = client
        .get(url.clone())
        .header("Cookie", &config.cookie_header)
        .header("X-Locale", "pl")
        .send()
        .await?;

    return get_body(response).await;
}

pub async fn fetch_movie_ratings(
    config: &Config,
) -> Result<Vec<RatingRaw>, ApiError> {
    let client = Client::new();
    let mut ratings: Vec<RatingRaw> = Vec::new();
    let mut page: u16 = 1;

    loop {
        let endpoint = format!("logged/vote/title/film?page={}", page);
        let body = fetch_resource(config, &client, &endpoint).await?;
        let mut body_json: Vec<RatingRaw> = serde_json::from_str(&body)?;

        if body_json.is_empty() {
            break;
        }

        ratings.append(&mut body_json);
        page += 1;
    }

    Ok(ratings)
}

async fn fetch_general_info(
    config: &Config,
    client: &Client,
    entity: u64,
) -> Result<GeneralInfo, ApiError> {
    let endpoint = format!("title/{}/info", entity);
    let body = fetch_resource(config, &client, &endpoint).await?;
    let general_info: GeneralInfo = serde_json::from_str(&body)?;
    Ok(general_info)
}

pub async fn raw_to_rating(
    config: &Config,
    client: &Client,
    raw: &RatingRaw,
) -> Result<Rating, ApiError> {
    let entity = raw.entity;
    let general_info = fetch_general_info(config, client, entity).await?;
    let view_date = raw
        .viewDate
        .to_date()
        .expect("Filmweb should be setting correct dates");

    Ok(Rating {
        title: general_info.title,
        view_date: view_date.to_string(),
        rate: raw.rate,
    })
}
