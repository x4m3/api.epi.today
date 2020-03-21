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
