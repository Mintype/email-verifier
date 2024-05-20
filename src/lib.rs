use reqwest::blocking::Client;
use reqwest::Error;
use serde::Deserialize;
use std::error::Error as StdError;

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub email_address: String,
    pub domain: String,
    pub valid_syntax: bool,
    pub disposable: bool,
    pub webmail: bool,
    pub deliverable: bool,
    pub catch_all: bool,
    pub gibberish: bool,
    pub spam: bool,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub status: String,
    pub data: ResponseData,
}

pub fn fetch_email_data(email: &str) -> Result<ApiResponse, Box<dyn StdError>> {
    let url = format!("https://api.eva.pingutil.com/email?email={}", email);

    // Create a client with disabled SSL verification
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client.get(&url).send()?;

    if response.status().is_success() {
        let body = response.text()?;
        let api_response: ApiResponse = serde_json::from_str(&body)?;
        Ok(api_response)
    } else {
        Err(Box::from(format!("HTTP request failed with status: {}", response.status())))
    }
}
