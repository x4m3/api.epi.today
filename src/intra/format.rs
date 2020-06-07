use chrono::{Duration, NaiveDateTime};
use regex::Regex;
use serde_json::Value;

/// Prettifies a raw room format into easily-readable room name
///
/// # Arguments
///
/// * `raw_room` - A string containing a raw room format
///
/// # Example
///
/// ```
/// use crate::intra::format;
///
/// let raw_room = "FR/REN/Epitech/Bureau-De-Laurene";
/// let res = format::room(&raw_room).unwrap();
///
/// assert_eq!(res, "Epitech → Bureau De Laurene");
/// ```
pub fn room(raw_room: &str) -> Option<String> {
    // Raw room format: "Country/City/Location/Room-Name"
    let mut room = String::from(raw_room.clone());

    // Start by finding if there is Country and City in the room string
    let re = match Regex::new("^([a-zA-Z]+/[a-zA-Z]+/)") {
        Ok(re) => re,
        Err(_) => return None,
    };
    // Remove them if they are present
    room = re.replace(&room, "").to_string();

    // Replace the `/` by arrows for prettiness
    room = room.replace("/", " → ");

    // Replace the `-` by spaces for room name
    room = room.replace("-", " ");

    // We are done, return the freshly formatted room
    Some(room)
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
/// use crate::intra::format;
/// let date_time = "2020-03-21 23:42:00";
/// let res = format::time(&date_time);
/// assert_eq!(res, "23:42");
/// ```
pub fn time(raw_date_time: &str) -> Option<String> {
    match NaiveDateTime::parse_from_str(&raw_date_time, "%Y-%m-%d %H:%M:%S") {
        Ok(date_time) => Some(date_time.format("%H:%M").to_string()),
        Err(_) => None,
    }
}

/// Extract time from rdv start time
///
/// # Arguments
///
/// * `raw_object` - An object with start time
///
pub fn rdv_time_start(raw_object: &Value) -> Option<String> {
    // Try to get string from raw_object `date`
    let raw_date = match raw_object.as_str() {
        Some(raw_date) => raw_date,
        None => return None,
    };

    // Try to extract time
    match time(raw_date) {
        Some(start) => Some(start),
        None => None,
    }
}

/// Extract time from rdv start time with rdv duration
///
/// # Arguments
///
/// * `raw_object` - An object with start time and duration
///
pub fn rdv_time_end(raw_object: &Value) -> Option<String> {
    // Try to get string from raw_object `date`
    let raw_start = match raw_object["date"].as_str() {
        Some(raw_start) => raw_start,
        None => return None,
    };

    // Convert start time in date data
    let start = match NaiveDateTime::parse_from_str(&raw_start, "%Y-%m-%d %H:%M:%S") {
        Ok(start) => start,
        Err(_) => return None,
    };

    // Try to get number from raw_object `duration`
    let duration_mins = match raw_object["duration"].as_i64() {
        Some(duration_mins) => duration_mins,
        None => return None,
    };

    // Add duration to start date
    let end = start + Duration::minutes(duration_mins);

    // Format end time as string
    Some(end.format("%H:%M").to_string())
}
