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
