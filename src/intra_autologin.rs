use actix_web::HttpRequest;
use regex::Regex;

pub fn get_from_header<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("autologin")?.to_str.ok()
}

pub fn check(autologin: &str) -> bool {
    let re = Regex::new("^([a-z0-9]{40})$").unwrap();
    re.is_match(autologin)
}
