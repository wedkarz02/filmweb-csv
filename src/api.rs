use std::fmt::Debug;

use log::info;
use reqwest::Client;
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};

use crate::util::ToDate;
use crate::Config;
use crate::{error::ApiError, error::AppError, BASE_URL};

pub trait RawEntity {
    fn entity(&self) -> u64;
    fn timestamp(&self) -> u64;
    fn rate(&self) -> u8;
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct RatingRaw {
    rate: u8,
    entity: u64,
    viewDate: u64,
    timestamp: u64,
}

impl RawEntity for RatingRaw {
    fn entity(&self) -> u64 {
        self.entity
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn rate(&self) -> u8 {
        self.rate
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WatchlistRaw {
    entity: u64,
    timestamp: u64,
    level: u8,
    followMask: Option<u8>,
}

impl RawEntity for WatchlistRaw {
    fn entity(&self) -> u64 {
        self.entity
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn rate(&self) -> u8 {
        0
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct GeneralInfo {
    title: String,
    originalTitle: String,
    year: u16,
    r#type: String,
    subType: Option<String>,
    posterPath: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemData {
    pub title: String,
    pub original_title: String,
    pub year: u16,
    pub date: String,
    pub rate: u8,
}

pub async fn get_body(response: Response) -> Result<String, AppError> {
    match response.status() {
        StatusCode::OK => Ok(response.text().await?),
        StatusCode::BAD_REQUEST => Err(ApiError::BadRequest.into()),
        StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized.into()),
        StatusCode::NOT_FOUND => {
            Err(ApiError::NotFound(response.url().to_string()).into())
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            Err(ApiError::InternalServerError.into())
        }
        _ => Err(ApiError::Unrecognized.into()),
    }
}

async fn fetch_general_info(
    config: &Config,
    client: &Client,
    entity: u64,
) -> Result<GeneralInfo, AppError> {
    let endpoint = format!("title/{}/info", entity);
    let body = fetch_resource(config, client, &endpoint).await?;
    let general_info: GeneralInfo = serde_json::from_str(&body)?;
    Ok(general_info)
}

pub async fn raw_to_item<T>(
    config: &Config,
    client: &Client,
    raw: &T,
) -> Result<ItemData, AppError>
where
    T: RawEntity,
{
    let general_info = fetch_general_info(config, client, raw.entity()).await?;
    let date = raw.timestamp().to_date_from_timestamp().unwrap_or_default();

    Ok(ItemData {
        title: general_info.title,
        original_title: general_info.originalTitle,
        year: general_info.year,
        date: date.format("%Y-%m-%d").to_string(),
        rate: raw.rate(),
    })
}

pub async fn fetch_resource(
    config: &Config,
    client: &Client,
    endpoint: &str,
) -> Result<String, AppError> {
    let url = format!("{}/{}", BASE_URL, endpoint);
    info!("Fetching from: {}", url);

    let response = client
        .get(url.clone())
        .header("Cookie", &config.cookie_header)
        .header("X-Locale", "pl")
        .send()
        .await?;

    get_body(response).await
}

pub async fn fetch_pages<T>(
    config: &Config,
    endpoint: &str,
) -> Result<Vec<T>, AppError>
where
    T: serde::de::DeserializeOwned,
{
    let client = Client::new();
    let mut deserialized: Vec<T> = Vec::new();
    let mut page: u16 = 1;

    loop {
        let endpoint = format!("{}?page={}", endpoint, page);
        let body = fetch_resource(config, &client, &endpoint).await?;
        let mut body_json: Vec<T> = serde_json::from_str(&body)?;

        if body_json.is_empty() {
            break;
        }

        deserialized.append(&mut body_json);
        page += 1;
    }

    Ok(deserialized)
}
