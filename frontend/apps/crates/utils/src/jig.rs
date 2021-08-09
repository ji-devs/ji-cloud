use shared::domain::jig::{AudioBackground, AudioFeedbackPositive, AudioFeedbackNegative};


pub trait AudioBackgroundExt {
    fn display_name(&self) -> &'static str;
}


impl AudioBackgroundExt for AudioBackground {
    fn display_name(&self) -> &'static str {
        match self {
            AudioBackground::Placeholder0 => "Placeholder background 0",
            AudioBackground::Placeholder1 => "Placeholder background 1",
            AudioBackground::Placeholder2 => "Placeholder background 2",
            AudioBackground::Placeholder3 => "Placeholder background 3",
            AudioBackground::Placeholder4 => "Placeholder background 4",
        }
    }
}


pub trait AudioFeedbackPositiveExt {
    fn display_name(&self) -> &'static str;
}


impl AudioFeedbackPositiveExt for AudioFeedbackPositive {
    fn display_name(&self) -> &'static str {
        match self {
            AudioFeedbackPositive::Placeholder0 => "Placeholder positive 0",
            AudioFeedbackPositive::Placeholder1 => "Placeholder positive 1",
            AudioFeedbackPositive::Placeholder2 => "Placeholder positive 2",
            AudioFeedbackPositive::Placeholder3 => "Placeholder positive 3",
            AudioFeedbackPositive::Placeholder4 => "Placeholder positive 4",
        }
    }
}


pub trait AudioFeedbackNegativeExt {
    fn display_name(&self) -> &'static str;
}


impl AudioFeedbackNegativeExt for AudioFeedbackNegative {
    fn display_name(&self) -> &'static str {
        match self {
            AudioFeedbackNegative::Placeholder0 => "Placeholder negative 0",
            AudioFeedbackNegative::Placeholder1 => "Placeholder negative 1",
            AudioFeedbackNegative::Placeholder2 => "Placeholder negative 2",
            AudioFeedbackNegative::Placeholder3 => "Placeholder negative 3",
            AudioFeedbackNegative::Placeholder4 => "Placeholder negative 4",
        }
    }
}
