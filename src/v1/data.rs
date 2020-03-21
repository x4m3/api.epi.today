use serde::{Deserialize, Serialize};

/// Default data type
///
/// Used for generic output, such as errors
#[derive(Serialize)]
pub struct Default {
    /// Generic message
    pub msg: String,
}

/// User data type
///
/// Used for storing user information
#[derive(Serialize)]
pub struct User {
    /// Full name (firstname and lastname)
    pub name: String,

    /// Email address
    pub email: String,

    /// First group user belongs to (should be city of enrolment)
    pub city: String,

    /// Student year
    pub year: u64,

    /// Current semester
    pub semester: u64,

    /// Credits obtained
    pub credits: u64,

    /// Current G.P.A
    pub gpa: String,

    /// Weekly log in hours (continuous)
    pub log: f64,
}

/// Custom planning list data type
///
/// Used for storing list of custom plannings
#[derive(Serialize)]
pub struct CustomPlanningList {
    /// Custom planning ID
    pub id: u64,

    /// Custom planning name
    pub name: String,
}

/// Custom planning event input data type
///
/// Used for custom planning event requests
#[derive(Deserialize)]
pub struct CustomPlanningEventInput {
    /// Custom planning ID
    pub calendar_id: u64,

    /// Requested date
    pub date: String,
}

/// Custom planning event result data type
///
/// Used for custom planning event responses
#[derive(Serialize)]
pub struct CustomPlanningEventResult {
    /// Custom planning ID
    pub calendar_id: u64,

    /// Event ID
    pub event_id: u64,

    /// Event title
    pub title: String,

    /// Event room
    pub room: String,

    /// Event start
    pub time_start: String,

    /// Event end
    pub time_end: String,

    /// Event teacher
    pub teacher: String,

    /// Registration status
    pub registration_status: bool,
}
