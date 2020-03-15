use reqwest;
use std::time::Duration;

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
