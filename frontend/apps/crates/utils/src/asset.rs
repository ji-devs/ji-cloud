use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use shared::{
    api::endpoints,
    domain::{
        additional_resource::ResourceContent,
        asset::{AssetId, DraftOrLive},
        course::unit::CourseUnitValue,
        course::{CourseCreatePath, CourseCreateRequest},
        jig::{
            AudioBackground, AudioFeedbackNegative, AudioFeedbackPositive, JigCreatePath,
            JigCreateRequest, JigPlayerSettings, TextDirection,
        },
        module::{ModuleBody, ModuleCreatePath, ModuleCreateRequest, ModuleKind},
        playlist::{PlaylistCreatePath, PlaylistCreateRequest},
        resource::{ResourceCreatePath, ResourceCreateRequest},
    },
    media::{MediaLibrary, PngImageFile},
};
use wasm_bindgen_futures::spawn_local;

use crate::{
    path::{audio_lib_url, image_lib_url, pdf_lib_url},
    paywall,
    prelude::get_user_mutable,
    prelude::ApiEndpointExt,
    routes::{
        AssetEditRoute, AssetRoute, CourseEditRoute, JigEditRoute, PlaylistEditRoute,
        ResourceEditRoute, Route,
    },
    unwrap::UnwrapJiExt,
};

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
            AudioBackground::Awestruck => "Awestruck",
            AudioBackground::BayBounce => "Bay Bounce",
            AudioBackground::CalmAndReflective => "Calm and Reflective",
            AudioBackground::DayWithoutRain => "Day Without Rain",
            AudioBackground::DestinationFreedom => "Destination Freedom",
            AudioBackground::FutureMemories => "Future Memories",
            AudioBackground::HappyInstrumental => "Happy Instrumental",
            AudioBackground::HappyWhistle => "Happy Whistle",
            AudioBackground::KidsInstrumental => "Kids Instrumental",
            AudioBackground::PartyKids => "Party Kids",
            AudioBackground::RhythmKids => "Rhythm Kids",
            AudioBackground::SunKissed => "Sun Kissed",
            AudioBackground::LegacyCuckooToYou => "?",
            AudioBackground::LegacyFirstEtude => "?",
            AudioBackground::LegacyHanerotHalalu => "?",
            AudioBackground::LegacyIslandRomp => "?",
            AudioBackground::LegacyJiTap => "?",
            AudioBackground::LegacyMaozTzur => "?",
            AudioBackground::LegacyModehAni => "?",
            AudioBackground::LegacyMonkeyBars => "?",
            AudioBackground::LegacyMorningZoo => "?",
            AudioBackground::LegacyNapTime => "?",
            AudioBackground::LegacyPlaylandMarch => "?",
            AudioBackground::LegacyShehechiyanu => "?",
            AudioBackground::LegacySunAndNoClouds => "?",
            AudioBackground::LegacyTeddysBear => "?",
            AudioBackground::LegacyWanderingWalrus => "?",
            AudioBackground::LegacyWindupLullaby => "?",
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
    pub draft_or_live: DraftOrLive,
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
            draft_or_live: DraftOrLive::Live,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PlaylistPlayerOptions {
    #[serde(default)]
    pub draft_or_live: DraftOrLive,

    #[serde(default)]
    pub is_student: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CoursePlayerOptions {
    #[serde(default)]
    pub draft_or_live: DraftOrLive,

    #[serde(default)]
    pub is_student: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AssetPlayerOptions {
    Jig(JigPlayerOptions),
    Playlist(PlaylistPlayerOptions),
    Course(CoursePlayerOptions),
}

impl AssetPlayerOptions {
    pub fn is_draft(&self) -> bool {
        match self {
            Self::Jig(options) => options.draft_or_live.is_draft(),
            Self::Playlist(options) => options.draft_or_live.is_draft(),
            Self::Course(options) => options.draft_or_live.is_draft(),
        }
    }

    pub fn default_from_id(asset_id: &AssetId) -> Self {
        match asset_id {
            AssetId::JigId(_) => Self::Jig(Default::default()),
            AssetId::PlaylistId(_) => Self::Playlist(Default::default()),
            AssetId::CourseId(_) => Self::Course(Default::default()),
            AssetId::ResourceId(_) => unimplemented!(),
        }
    }
}

impl From<JigPlayerOptions> for AssetPlayerOptions {
    fn from(player_option: JigPlayerOptions) -> Self {
        AssetPlayerOptions::Jig(player_option)
    }
}

impl From<PlaylistPlayerOptions> for AssetPlayerOptions {
    fn from(player_option: PlaylistPlayerOptions) -> Self {
        AssetPlayerOptions::Playlist(player_option)
    }
}

impl From<CoursePlayerOptions> for AssetPlayerOptions {
    fn from(player_option: CoursePlayerOptions) -> Self {
        AssetPlayerOptions::Course(player_option)
    }
}

pub trait ResourceContentExt {
    fn get_link(&self) -> String;
}

pub trait CourseUnitValueExt {
    fn get_link(&self) -> String;
}

impl ResourceContentExt for ResourceContent {
    fn get_link(&self) -> String {
        match self {
            ResourceContent::ImageId(image_id) => {
                image_lib_url(MediaLibrary::User, PngImageFile::Original, *image_id)
            }
            ResourceContent::AudioId(audio_id) => audio_lib_url(MediaLibrary::User, *audio_id),
            ResourceContent::PdfId(pdf_id) => pdf_lib_url(MediaLibrary::User, *pdf_id),
            ResourceContent::Link(url) => url.to_string(),
        }
    }
}

impl CourseUnitValueExt for CourseUnitValue {
    fn get_link(&self) -> String {
        match self {
            CourseUnitValue::ImageId(image_id) => {
                image_lib_url(MediaLibrary::User, PngImageFile::Original, *image_id)
            }
            CourseUnitValue::AudioId(audio_id) => audio_lib_url(MediaLibrary::User, *audio_id),
            CourseUnitValue::PdfId(pdf_id) => pdf_lib_url(MediaLibrary::User, *pdf_id),
            CourseUnitValue::Link(url) => url.to_string(),
            CourseUnitValue::Video(_) => todo!(),
        }
    }
}

/// centralized create-asset functions
pub fn create_jig() {
    let total_existing = get_user_mutable()
        .lock_ref()
        .as_ref()
        .map(|user| user.jig_count)
        .unwrap_or_default();
    if !paywall::can_create_jig(total_existing) {
        paywall::dialog_limit(
            "
                Looking to create more than 5 JIGs?
                Upgrade now for UNLIMITED access to Jigzi Studio.
            ",
        );
        return;
    }
    spawn_local(async move {
        let req = JigCreateRequest::default();

        let resp = endpoints::jig::Create::api_with_auth(JigCreatePath(), Some(req))
            .await
            .unwrap_ji();
        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
            resp.id,
            JigEditRoute::Landing,
        )))
        .into();
        dominator::routing::go_to_url(&url);
    });
}
pub fn create_resource() {
    let total_existing = get_user_mutable()
        .lock_ref()
        .as_ref()
        .map(|user| user.resource_count)
        .unwrap_or_default();
    if !paywall::can_create_jig(total_existing) {
        paywall::dialog_limit(
            "
                Looking to add your own resources?
                Upgrade now for UNLIMITED access to Jigzi Studio.
            ",
        );
        return;
    }
    spawn_local(async move {
        let req = ResourceCreateRequest::default();

        let resp = endpoints::resource::Create::api_with_auth(ResourceCreatePath(), Some(req))
            .await
            .unwrap_ji();
        add_playlist_or_resource_cover(resp.id.into()).await;
        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
            resp.id,
            ResourceEditRoute::Landing,
        )))
        .into();
        dominator::routing::go_to_url(&url);
    });
}
pub fn create_playlist() {
    let total_existing = get_user_mutable()
        .lock_ref()
        .as_ref()
        .map(|user| user.playlist_count)
        .unwrap_or_default();
    if !paywall::can_create_jig(total_existing) {
        paywall::dialog_limit(
            "
                Looking to create a playlist?
                Upgrade now for UNLIMITED access to Jigzi Studio.
            ",
        );
        return;
    }
    spawn_local(async move {
        let req = PlaylistCreateRequest::default();

        let resp = endpoints::playlist::Create::api_with_auth(PlaylistCreatePath(), Some(req))
            .await
            .unwrap_ji();
        add_playlist_or_resource_cover(resp.id.into()).await;
        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
            resp.id,
            PlaylistEditRoute::Landing,
        )))
        .into();
        dominator::routing::go_to_url(&url);
    });
}
pub fn create_course() {
    let total_existing = get_user_mutable()
        .lock_ref()
        .as_ref()
        .map(|user| user.course_count)
        .unwrap_or_default();
    if !paywall::can_create_jig(total_existing) {
        paywall::dialog_limit(
            "
                Looking to create a course?
                Upgrade now for UNLIMITED access to Jigzi Studio.
            ",
        );
        return;
    }
    spawn_local(async move {
        let req = CourseCreateRequest::default();

        let resp = endpoints::course::Create::api_with_auth(CourseCreatePath(), Some(req))
            .await
            .unwrap_ji();
        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
            resp.id,
            CourseEditRoute::Landing,
        )))
        .into();
        dominator::routing::go_to_url(&url);
    });
}

async fn add_playlist_or_resource_cover(asset_id: AssetId) {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: asset_id,
    };

    endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req))
        .await
        .unwrap_ji();
}
