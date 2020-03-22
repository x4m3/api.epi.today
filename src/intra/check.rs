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
/// use crate::intra::check;
/// let good_date = "2020-03-21";
/// let res_ok = check::yyyy_mm_dd(&good_date);
/// ```
pub fn yyyy_mm_dd(date: &str) -> Option<NaiveDateTime> {
    let full_date = format!("{} 00:00:00", date);

    match NaiveDateTime::parse_from_str(&full_date, "%Y-%m-%d %H:%M:%S") {
        Ok(res) => Some(res),
        Err(_) => return None,
    }
}

/// Internal check module
mod check {
    use regex::Regex;

    /// Check if module is syntactically correct
    pub fn module(module: &str) -> Option<bool> {
        let re = match Regex::new("^([A-Z])-([A-Z]*)-([0-9]*)$") {
            Ok(re) => re,
            Err(_) => return None,
        };
        Some(re.is_match(module))
    }

    /// Check if instance is syntactically correct
    pub fn instance(instance: &str) -> Option<bool> {
        let re = match Regex::new("^([A-Z]*)-([0-9]*)-([0-9])$") {
            Ok(re) => re,
            Err(_) => return None,
        };
        Some(re.is_match(instance))
    }

    /// Check if activity is syntactically correct
    pub fn activity(activity: &str) -> Option<bool> {
        let re = match Regex::new("^(acti-\\d*)$") {
            Ok(re) => re,
            Err(_) => return None,
        };
        Some(re.is_match(activity))
    }

    /// Check if event is syntactically correct
    pub fn event(event: &str) -> Option<bool> {
        let re = match Regex::new("^(event-[0-9]*)$") {
            Ok(re) => re,
            Err(_) => return None,
        };
        Some(re.is_match(event))
    }

    /// Check if email is syntactically correct
    pub fn email(email: &str) -> Option<bool> {
        let re = match Regex::new("^([A-Z0-9a-z.-]+@epitech.eu)$") {
            Ok(re) => re,
            Err(_) => return None,
        };
        Some(re.is_match(email))
    }
}

/// Check input values of a planning event
pub fn planning_event(module: &str, instance: &str, activity: &str, event: &str) -> Option<String> {
    // TODO: find a cleaner way to do this
    match check::module(module) {
        Some(res) => {
            if res == false {
                return Some(String::from("field `module` is invalid"));
            } else {
                ()
            }
        }
        None => return Some(String::from("field `module` is invalid")),
    };

    match check::instance(instance) {
        Some(res) => {
            if res == false {
                return Some(String::from("field `instance` is invalid"));
            } else {
                ()
            }
        }
        None => return Some(String::from("field `instance` is invalid")),
    };

    match check::activity(activity) {
        Some(res) => {
            if res == false {
                return Some(String::from("field `activity` is invalid"));
            } else {
                ()
            }
        }
        None => return Some(String::from("field `activity` is invalid")),
    };

    match check::event(event) {
        Some(res) => {
            if res == false {
                return Some(String::from("field `event` is invalid"));
            } else {
                ()
            }
        }
        None => return Some(String::from("field `event` is invalid")),
    };

    // Everything is good syntactically
    None
}

/// Check input values of a planning rdv
pub fn planning_rdv(module: &str, instance: &str, activity: &str, email: &str) -> Option<String> {
    // TODO: find a cleaner way to do this
    match check::module(module) {
        Some(res) => {
            if res == false {
                return Some(String::from("field `module` is invalid"));
            } else {
                ()
            }
        }
        None => return Some(String::from("field `module` is invalid")),
    };

    match check::instance(instance) {
        Some(res) => {
            if res == false {
                return Some(String::from("field `instance` is invalid"));
            } else {
                ()
            }
        }
        None => return Some(String::from("field `instance` is invalid")),
    };

    match check::activity(activity) {
        Some(res) => {
            if res == false {
                return Some(String::from("field `activity` is invalid"));
            } else {
                ()
            }
        }
        None => return Some(String::from("field `activity` is invalid")),
    };

    match check::email(email) {
        Some(res) => {
            if res == false {
                return Some(String::from("field `email` is invalid"));
            } else {
                ()
            }
        }
        None => return Some(String::from("field `email` is invalid")),
    };

    // Everything is good syntactically
    None
}
