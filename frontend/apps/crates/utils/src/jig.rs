use chrono::{DateTime, Utc};
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


const HOUR: u64 = 1000 * 60 * 60;
const DAY: u64 = HOUR * 24;
const WEEK: u64 = DAY * 7;
const YEAR: u64 = DAY * 365;

#[derive(Debug)]
enum PublishedAtUnit {
    Year(u32),
    Week(u32),
    Day(u32),
    Hour(u32),
}
impl PublishedAtUnit {
    pub fn new(millis: u64) -> Self {
        match millis {
            YEAR.. => Self::Year((millis / YEAR) as u32),
            WEEK.. => Self::Week((millis / WEEK) as u32),
            DAY.. => Self::Day((millis / DAY) as u32),
            _ => Self::Hour((millis / HOUR) as u32),
        }
    }
    pub fn get_number(&self) -> u32 {
        match self {
            Self::Year(num) => *num,
            Self::Week(num) => *num,
            Self::Day(num) => *num,
            Self::Hour(num) => *num,
        }
    }
    pub fn to_string_short(&self) -> &'static str {
        match self {
            Self::Year(_) => "Y",
            Self::Week(_) => "W",
            Self::Day(_) => "D",
            Self::Hour(_) => "H",
        }
    }
    pub fn to_string_long(&self) -> &'static str {
        match self {
            Self::Year(_) => "year",
            Self::Week(_) => "week",
            Self::Day(_) => "day",
            Self::Hour(_) => "hour",
        }
    }
}

pub fn published_at_string(time: DateTime<Utc>, short: bool) -> String {
    let millis_since_published = Utc::now().timestamp_millis() - time.timestamp_millis();
    let millis_since_published = millis_since_published as u64;

    let unit = PublishedAtUnit::new(millis_since_published);

    match short {
        true => format!("{} {} ago", unit.get_number(), unit.to_string_short()),
        false => {
            let num = unit.get_number();
            match num {
                1 => format!("1 {} ago", unit.to_string_long()),
                num => format!("{} {}s ago", num, unit.to_string_long()),
            }
        },
    }
}
