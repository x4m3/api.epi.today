use reqwest;
use std::time::Duration;

pub async fn make_get_request(
    client: &reqwest::Client,
    url: &String,
) -> Result<reqwest::Response, reqwest::Error> {
    Ok(client.get(url).send().await?)
}

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
