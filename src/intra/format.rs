use regex::Regex;

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

/// Module for checking input strings
pub mod check {

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
    pub fn planning_event(
        module: &str,
        instance: &str,
        activity: &str,
        event: &str,
    ) -> Option<String> {
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
}
