use chrono::NaiveDateTime;

/// Checking if provided date is valid
///
/// # Arguments
///
/// * `date` - A string containing a date formatted (yyyy-mm-dd)
///
/// # Example
///
/// ```
/// use crate::date;
/// let good_date = "2020-03-21";
/// let res_ok = date::check_yyyy_mm_dd(&good_date);
/// ```
pub fn check_yyyy_mm_dd(date: &str) -> Option<NaiveDateTime> {
    let full_date = format!("{} 00:00:00", date);

    match NaiveDateTime::parse_from_str(&full_date, "%Y-%m-%d %H:%M:%S") {
        Ok(res) => Some(res),
        Err(_) => return None,
    }
}

/// Extract time from string
///
/// # Arguments
///
/// * `raw_date_time` - A string containing a date and time formatted (yyyy-mm-dd HH:MM:SS)
///
/// # Example
///
/// ```
/// use crate::date;
/// let date_time = "2020-03-21 23:42:00";
/// let res = date::extract_time(&date_time);
/// assert_eq!(res, "23:42");
/// ```
pub fn extract_time(raw_date_time: &str) -> Option<String> {
    match NaiveDateTime::parse_from_str(&raw_date_time, "%Y-%m-%d %H:%M:%S") {
        Ok(date_time) => Some(date_time.format("%H:%M").to_string()),
        Err(_) => return None,
    }
}
