use serde::Serialize;

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
