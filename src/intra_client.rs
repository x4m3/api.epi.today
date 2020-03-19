use reqwest;
use std::time::Duration;

/// Returns the result of a request to the intra
///
/// # Arguments
///
/// * `client` - A reqwest client already built
/// * `url` - A string containing the url of the request to execute
///
/// # Example
///
/// ```
/// use crate::intra_client;
/// let request_url = format!("https://intra.epitech.eu/?format=json");
/// let client = intra_client::create_client()?;
/// let res = client.get(&request_url).send().await?;
/// ```
pub async fn make_get_request(
    client: &reqwest::Client,
    url: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    Ok(client.get(url).send().await?)
}

/// Returns a client ready to be used for making requests to the intra
///
/// # Example
///
/// ```
/// use crate::intra_client;
/// let client = intra_client::create_client()?;
/// ```
pub fn create_client() -> Result<reqwest::Client, reqwest::Error> {
    let timeout = Duration::new(5, 0);

    let client = reqwest::Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .timeout(timeout)
        .build()?;

    Ok(client)
}
