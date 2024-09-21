use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MovieRating {
    rate: u8,
    entity: u64,
    viewDate: u64,
    timestamp: u64,
}

pub async fn fetch_movies(
    base_url: &str,
    cookie_header: &str,
) -> Result<Vec<MovieRating>, ApiError> {
    let mut movies: Vec<MovieRating> = Vec::new();
    let mut page: u16 = 1;

    let client = Client::new();

    loop {
        let url = format!("{}/vote/title/film?page={}", base_url, page);

        let response = client
            .get(url.clone())
            .header("Cookie", cookie_header)
            .send()
            .await?;

        println!("{:#?}", response);

        let body = match response.status() {
            StatusCode::OK => response.text().await?,
            StatusCode::BAD_REQUEST => return Err(ApiError::BadRequest),
            StatusCode::UNAUTHORIZED => return Err(ApiError::Unauthorized),
            StatusCode::NOT_FOUND => return Err(ApiError::NotFound(url)),
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err(ApiError::InternalServerError)
            }
            _ => return Err(ApiError::Unrecognized),
        };

        let mut body_json: Vec<MovieRating> = serde_json::from_str(&body)?;

        if body_json.is_empty() {
            break;
        }

        movies.append(&mut body_json);
        page += 1;
    }

    Ok(movies)
}
