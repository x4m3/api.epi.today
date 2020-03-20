use actix_web::HttpRequest;
use regex::Regex;

/// Returns a string containing a autologin in a HTTP Header
///
pub fn get_from_header<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("autologin")?.to_str().ok()
}

/// Returns whether a autologin is syntactically valid or not
///
/// # Arguments
///
/// * `autologin` - A string containing a autologin
///
/// # Example
///
/// ```
/// use crate::intra_client;
/// let good_autologin = format!("abcdefghijklmnopqrstuvwxyz1234567890abcd");
/// let res_ok = intra_autologin::autologin(&good_autologin);
/// assert_eq!(res_ok, true);
///
/// let bad_autologin = format!("abcdef");
/// let res_ko = intra_autologin::autologin(&bad_autologin);
/// assert_eq!(res_ko, false);
/// ```
pub fn check(autologin: &str) -> Option<bool> {
    let re = match Regex::new("^([a-z0-9]{40})$") {
        Ok(re) => re,
        Err(_) => return None,
    };
    Some(re.is_match(autologin))
}
