use serde::{Deserialize, Serialize};

/// Default data type
///
/// Used for generic output, such as errors or general information
#[derive(Serialize)]
pub struct Default {
    /// Generic message
    pub msg: String,
}

/// User data type
///
/// Used for storing user information
/// Used only for outputs
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
/// Used for getting list of custom plannings
/// Used only for outputs
#[derive(Serialize)]
pub struct CustomPlanningList {
    /// Custom planning ID
    pub id: u64,

    /// Custom planning name
    pub name: String,
}

/// Custom planning event input data type
///
/// Used for custom planning event
/// Used only for input
#[derive(Deserialize)]
pub struct CustomPlanningEventInput {
    /// Custom planning ID
    pub calendar_id: u64,

    /// Requested date
    pub date: String,
}

/// Custom planning event result data type
///
/// Used for custom planning events
/// Used only for outputs
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

/// Custom planning event params data type
///
/// Used for registering or un-registering to custom planning events
/// Used only for input
#[derive(Deserialize)]
pub struct CustomPlanningEventParams {
    /// Custom planning ID
    pub calendar_id: u64,

    /// Event ID
    pub event_id: u64,
}

/// Planning event params data type
///
/// Used for registering or un-registering to planning events
/// Used only for input
#[derive(Deserialize)]
pub struct PlanningEventParams {
    // School year of event
    pub year: u64,

    // Code of module
    pub code_module: String,

    // Code of module instance
    pub code_instance: String,

    // Code of activity
    pub code_acti: String,

    // Code of event
    pub code_event: String,
}

/// Planning token params data type
///
/// Used for registering a token to an event
/// Used only for input
#[derive(Deserialize)]
pub struct PlanningTokenParams {
    // School year of event
    pub year: u64,

    // Code of module
    pub code_module: String,

    // Code of module instance
    pub code_instance: String,

    // Code of activity
    pub code_acti: String,

    // Code of event
    pub code_event: String,

    // Token to register
    pub token: u64,
}

/// Send a token
///
/// Used for sending a token to the intra
#[derive(Serialize)]
pub struct PlanningSubmitTokenParams {
    /// Token to send
    pub token: String,
}

/// Planning rdv params data type
///
/// Used for getting information about rdv
/// Used only for input
#[derive(Serialize, Deserialize)] // Serialize because it's used in `/planning/day`
pub struct PlanningRdvParams {
    // School year of event
    pub year: u64,

    // Code of module
    pub code_module: String,

    // Code of module instance
    pub code_instance: String,

    // Code of activity
    pub code_acti: String,

    // Student email address
    pub email: String,
}

/// Planning rdv result data type
///
/// Used for getting information about rdv
/// Used only for output
#[derive(Serialize, Deserialize)] // Deserialize because it's used in `/planning/day`
pub struct PlanningRdvResult {
    // Rdv title
    pub title: String,

    // Rdv start
    pub time_start: String,

    // Rdv end
    pub time_end: String,
}

/// Planning day input data type
///
/// Used for planning day
/// Used only for input
#[derive(Deserialize)]
pub struct PlanningDayInput {
    /// Requested date
    pub date: String,

    // User's current semester
    pub current_semester: u64,

    // Student email address
    pub email: String,
}

/// Planning day result data type
///
/// Used for planning day
/// Used only for output
#[derive(Serialize)]
pub struct PlanningDayResult {
    //
    // Fields below are details to events
    // Meant for making requests related to events
    //

    // Is event from a custom planning
    pub is_custom: bool,

    // Is event a rdv
    pub is_rdv: bool,

    // Is event a regular event
    pub is_regular: bool,

    // School year of event
    pub year: u64,

    // Code of module
    pub code_module: String,

    // Code of module instance
    pub code_instance: String,

    // Code of activity
    pub code_acti: String,

    // Code of event
    pub code_event: String,

    // Semester of event
    pub semester: u64,

    // ID of custom calendar
    pub custom_calendar_id: u64,

    // ID of event in custom calendar
    pub custom_event_id: u64,

    //
    // Fields below are common to *all* events
    // Meant to be displayed to the end user
    //

    // Event title
    pub title: String,

    // Module name (or custom planning)
    pub module: String,

    // Event location
    pub room: String,

    // Event teacher
    pub teacher: String,

    // Event start
    pub time_start: String,

    // Event end
    pub time_end: String,

    // Registration status of event
    pub registration_status: bool,
}
