use reqwest::Client;
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, util::ToDate, BASE_URL};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct RatingEntity {
    rate: u8,
    entity: u64,
    viewDate: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieRating {
    title: String,
    view_date: String,
    rate: u8,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct MovieInfo {
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

pub async fn fetch_ratings(
    cookie_header: &str,
) -> Result<Vec<RatingEntity>, ApiError> {
    let mut ratings: Vec<RatingEntity> = Vec::new();
    let mut page: u16 = 1;

    let client = Client::new();

    loop {
        let url = format!("{}/logged/vote/title/film?page={}", BASE_URL, page);

        let response = client
            .get(url.clone())
            .header("Cookie", cookie_header)
            .send()
            .await?;

        let body = get_body(response).await?;
        let mut body_json: Vec<RatingEntity> = serde_json::from_str(&body)?;

        if body_json.is_empty() {
            break;
        }

        ratings.append(&mut body_json);
        page += 1;
    }

    Ok(ratings)
}

async fn fetch_movie_info(entity: u64) -> Result<MovieInfo, ApiError> {
    let url = format!("{}/title/{}/info", BASE_URL, entity);
    let client = Client::new();

    let response =
        client.get(url.clone()).header("X-Locale", "pl_PL").send().await?;

    let body = get_body(response).await?;
    let movie_info: MovieInfo = serde_json::from_str(&body)?;

    Ok(movie_info)
}

pub async fn entity_to_movie(
    rating_entity: &RatingEntity,
) -> Result<MovieRating, ApiError> {
    let entity = rating_entity.entity;
    let movie_info = fetch_movie_info(entity).await?;
    let view_date = rating_entity
        .viewDate
        .to_date()
        .expect("Filmweb should be setting correct dates");

    Ok(MovieRating {
        title: movie_info.title,
        view_date: view_date.to_string(),
        rate: rating_entity.rate,
    })
}
