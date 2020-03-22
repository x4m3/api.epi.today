use crate::v1::data;
use reqwest;
use std::time::Duration;

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

/// Returns the result of a **get** request to the intra
///
/// # Arguments
///
/// * `client` - A reqwest client already built
/// * `path` - A string containing the path of the intra to request
///
/// # Example
///
/// ```
/// use crate::intra_client;
/// let path = format!("/?format=json");
/// let client = intra_client::create_client()?;
/// let res = intra_client::get_path(&client, &path).await?;
/// ```
pub async fn get_path(
    client: &reqwest::Client,
    path: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("https://intra.epitech.eu{}", path);
    Ok(client.get(&url).send().await?)
}

/// Returns the result of a **get** request to the intra with a autologin and path
///
/// # Arguments
///
/// * `client` - A reqwest client already built
/// * `autologin` - A autologin information to make request as user
/// * `path` - A string containing the path of the intra to request
///
/// # Example
///
/// ```
/// use crate::intra_client;
/// let autologin = format!("insert_autologin_here");
/// let path = format!("/user/?format=json");
/// let client = intra_client::create_client()?;
/// let res = intra_client::get_path_auth(&client, &autologin, &path).await?;
/// ```
pub async fn get_path_auth(
    client: &reqwest::Client,
    autologin: &str,
    path: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let final_request = format!("/auth-{}{}", autologin, path);
    Ok(get_path(&client, &final_request).await?)
}

/// Returns the result of a **post** request to the intra
///
/// # Arguments
///
/// * `client` - A reqwest client already built
/// * `autologin` - A autologin information to make request as user
/// * `path` - A string containing the path of the intra to request
pub async fn post_path_auth(
    client: &reqwest::Client,
    autologin: &str,
    path: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("https://intra.epitech.eu/auth-{}{}", autologin, path);
    Ok(client.post(&url).send().await?)
}

/// Returns the result of a **post** request to the intra with token
///
/// # Arguments
///
/// * `client` - A reqwest client already built
/// * `autologin` - A autologin information to make request as user
/// * `path` - A string containing the path of the intra to request
/// * `token` - A structure containing data to be sent as json
pub async fn post_token(
    client: &reqwest::Client,
    autologin: &str,
    path: &str,
    token: &data::PlanningSubmitTokenParams,
) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!(
        "https://intra.epitech.eu/auth-{}{}/token?format=json",
        autologin, path
    );
    Ok(client.post(&url).json(token).send().await?)
}
