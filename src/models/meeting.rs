use chrono::prelude::*;

pub struct Meeting {
    id: String,
    /// Real duration of the meeting
    duration: u16,
    /// Local date of the user
    date_local: DateTime<Local>,
    /// UTC Date
    date_utc: DateTime<Utc>,
}
