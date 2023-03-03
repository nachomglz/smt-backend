enum MeetingType {
    RETRO,
    DAILY,
}

impl MeetingType {
    fn value(&self) -> &str {
        match *self {
            MeetingType::DAILY => "DAILY",
            MeetingType::RETRO => "RETRO",
        }
    }
}

pub struct MeetingConfig {
    /// Meeting DB Id
    id: String,
    /// Team Id
    team_id: String,
    /// Time in seconds that the meeting should last at maximum
    desired_duration: u16,
    /// Name of the meeting (ex: Pandora Daily)
    meeting_name: String,
    /// Description of the meeting
    description: String,
    /// Type of the meeting (RETRO | DAILY)
    meeting_type: MeetingType,
}
