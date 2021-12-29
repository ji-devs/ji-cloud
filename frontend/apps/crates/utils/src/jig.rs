use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use shared::{
    domain::jig::{AudioBackground, AudioFeedbackNegative, AudioFeedbackPositive, JigPlayerSettings, TextDirection, additional_resource::ResourceContent},
    media::{MediaLibrary, PngImageFile}
};

use crate::path::{audio_lib_url, image_lib_url, pdf_lib_url};

pub trait JigAudioExt {
    fn display_name(&self) -> &'static str;
}

impl JigAudioExt for AudioBackground {
    fn display_name(&self) -> &'static str {
        match self {
            AudioBackground::FunForKids => "Fun for Kids",
            AudioBackground::DancingHappy => "Dancing Happy",
            AudioBackground::Jigzi1 => "Jigzi 1",
            AudioBackground::Jigzi2 => "Jigzi 2",
            AudioBackground::Jigzi3 => "Jigzi 3",
        }
    }
}

impl JigAudioExt for AudioFeedbackPositive {
    fn display_name(&self) -> &'static str {
        match self {
            AudioFeedbackPositive::Correct => "Correct",
            AudioFeedbackPositive::Keys => "Keys",
            AudioFeedbackPositive::Magic => "Magic",
            AudioFeedbackPositive::Notes => "Notes",
            AudioFeedbackPositive::StarPing => "Star Ping",
            AudioFeedbackPositive::Ting => "Ting",
            AudioFeedbackPositive::Trumpet => "Trumpet",
            AudioFeedbackPositive::VoiceAwesome => "Voice Awesome",
            AudioFeedbackPositive::VoicesHurray => "Voices Hurray",
            AudioFeedbackPositive::VoiceYippee => "Voice Yippee",
            AudioFeedbackPositive::Xylophone => "Xylophone",
            AudioFeedbackPositive::Yes => "Yes",
        }
    }
}

impl JigAudioExt for AudioFeedbackNegative {
    fn display_name(&self) -> &'static str {
        match self {
            AudioFeedbackNegative::Bang => "Bang",
            AudioFeedbackNegative::Boing => "Boing",
            AudioFeedbackNegative::Buzz => "Buzz",
            AudioFeedbackNegative::Buzzer => "Buzzer",
            AudioFeedbackNegative::Clang => "Clang",
            AudioFeedbackNegative::Clicks => "Clicks",
            AudioFeedbackNegative::Incorrect => "Incorrect",
            AudioFeedbackNegative::JumpWrong => "Jump Wrong",
            AudioFeedbackNegative::NotRight => "Not Right",
            AudioFeedbackNegative::OhNo => "Oh No",
            AudioFeedbackNegative::ShortClang => "Short Clang",
            AudioFeedbackNegative::Whir => "Whir",
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
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JigPlayerOptions {
    #[serde(default)]
    pub direction: TextDirection,

    #[serde(default)]
    pub display_score: bool,

    #[serde(default)]
    pub track_assessments: bool,

    #[serde(default)]
    pub drag_assist: bool,

    #[serde(default)]
    pub is_student: bool,

    #[serde(default)]
    pub draft: bool,
}

impl Default for JigPlayerOptions {
    fn default() -> Self {
        JigPlayerSettings::default().into()
    }
}

impl From<JigPlayerOptions> for JigPlayerSettings {
    fn from(options: JigPlayerOptions) -> Self {
        Self {
            direction: options.direction,
            display_score: options.display_score,
            track_assessments: options.track_assessments,
            drag_assist: options.drag_assist,
        }
    }
}

impl From<JigPlayerSettings> for JigPlayerOptions {
    fn from(settings: JigPlayerSettings) -> Self {
        Self {
            direction: settings.direction,
            display_score: settings.display_score,
            track_assessments: settings.track_assessments,
            drag_assist: settings.drag_assist,
            is_student: false,
            draft: false,
        }
    }
}


pub trait ResourceContentExt {
    fn get_link(&self) -> String;
}

impl ResourceContentExt for ResourceContent {
    fn get_link(&self) -> String {
        match self {
            ResourceContent::ImageId(image_id) => {
                image_lib_url(MediaLibrary::User, PngImageFile::Original, image_id.clone())
            },
            ResourceContent::AudioId(audio_id) => {
                audio_lib_url(MediaLibrary::User, audio_id.clone())
            },
            ResourceContent::PdfId(pdf_id) => {
                pdf_lib_url(MediaLibrary::User, pdf_id.clone())
            },
            ResourceContent::Link(url) => {
                url.to_string()
            },
        }
    }
}
