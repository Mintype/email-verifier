use reqwest::blocking::Client;
use reqwest::Error;
use serde::Deserialize;
use std::error::Error as StdError;

/// Represents the data structure of the response from the email validation API.
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

/// Represents the API response structure.
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub status: String,
    pub data: ResponseData,
}

/// Fetches email validation data from the given API for a specified email address.
///
/// # Arguments
///
/// * `email` - A string slice that holds the email address to validate.
///
/// # Returns
///
/// * `Ok(ApiResponse)` if the request is successful and the response can be parsed.
/// * `Err(Box<dyn StdError>)` if there is an error with the request or parsing the response.
///
/// # Example
///
/// ```
/// let email = "example@example.com";
/// match fetch_email_data(email) {
///     Ok(api_response) => println!("{:?}", api_response),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
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

/// Extracts the domain from an email address.
///
/// # Arguments
///
/// * `email` - A string slice that holds the email address.
///
/// # Returns
///
/// * `Some(&str)` containing the domain if the email contains a valid domain.
/// * `None` if the email does not contain a valid domain.
///
/// # Example
///
/// ```
/// let email = "example@example.com";
/// if let Some(domain) = get_domain_from_email(email) {
///     println!("Domain: {}", domain);
/// } else {
///     println!("Invalid email address");
/// }
/// ```
pub fn get_domain_from_email(email: &str) -> Option<&str> {
    if let Some(domain) = email.split('@').nth(1) {
        if domain.is_empty() {
            None
        } else {
            Some(domain)
        }
    } else {
        None
    }
}
