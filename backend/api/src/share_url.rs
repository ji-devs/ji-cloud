//! Share URL generation utilities.

use ji_core::{settings::RuntimeSettings, url_signature::sign_url};
use serde::Serialize;
use shared::domain::{
    asset::DraftOrLive,
    course::{CourseId, CourseResponse},
    jig::{JigId, JigPlayerSettings, JigResponse, JigShareUrlRequest, TextDirection},
    module::ModuleId,
    playlist::{PlaylistId, PlaylistResponse},
};

pub struct JigShareUrlOptions<'a> {
    pub module_id: Option<ModuleId>,
    pub draft_or_live: DraftOrLive,
    pub play_token: Option<&'a str>,
    pub players_name: Option<&'a str>,
    pub is_student: bool,
    pub quota: bool,
    pub direction: Option<TextDirection>,
    pub scoring: Option<bool>,
    pub drag_assist: Option<bool>,
}

#[derive(Serialize)]
struct JigShareUrlQuery<'a> {
    draft_or_live: DraftOrLive,
    #[serde(skip_serializing_if = "Option::is_none")]
    play_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    players_name: Option<&'a str>,
    is_student: bool,
    quota: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<TextDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scoring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drag_assist: Option<bool>,
}

#[derive(Serialize)]
struct PlaylistShareUrlQuery {
    draft_or_live: DraftOrLive,
    is_student: bool,
}

#[derive(Serialize)]
struct CourseShareUrlQuery {
    draft_or_live: DraftOrLive,
    is_student: bool,
}

/// Generates a signed share URL for a JIG.
pub fn generate_jig_share_url(
    settings: &RuntimeSettings,
    jig_id: JigId,
    options: JigShareUrlOptions<'_>,
) -> String {
    let base_url = settings.remote_target().pages_url();
    let path = jig_share_path(jig_id, options.module_id);
    let query = serde_urlencoded::to_string(JigShareUrlQuery {
        draft_or_live: options.draft_or_live,
        play_token: options.play_token,
        players_name: options.players_name,
        is_student: options.is_student,
        quota: options.quota,
        direction: options.direction,
        scoring: options.scoring,
        drag_assist: options.drag_assist,
    })
    .expect("valid JIG share URL query");

    sign_url(
        &base_url,
        &format!("{path}?{query}"),
        &settings.token_secret,
    )
}

/// Generates a normal public share URL for a JIG.
pub fn generate_public_jig_share_url(
    settings: &RuntimeSettings,
    jig_id: JigId,
    player_settings: &JigPlayerSettings,
    req: &JigShareUrlRequest,
) -> String {
    generate_jig_share_url(
        settings,
        jig_id,
        JigShareUrlOptions {
            module_id: None,
            draft_or_live: DraftOrLive::Live,
            play_token: None,
            players_name: None,
            is_student: false,
            quota: true,
            direction: Some(req.direction.unwrap_or(player_settings.direction)),
            scoring: Some(req.scoring.unwrap_or(player_settings.scoring)),
            drag_assist: Some(player_settings.drag_assist),
        },
    )
}

/// Generates a student share URL for a JIG.
pub fn generate_student_jig_share_url(
    settings: &RuntimeSettings,
    jig_id: JigId,
    player_settings: &JigPlayerSettings,
    req: &JigShareUrlRequest,
) -> String {
    generate_jig_share_url(
        settings,
        jig_id,
        JigShareUrlOptions {
            module_id: None,
            draft_or_live: DraftOrLive::Live,
            play_token: None,
            players_name: None,
            is_student: true,
            quota: false,
            direction: Some(req.direction.unwrap_or(player_settings.direction)),
            scoring: Some(req.scoring.unwrap_or(player_settings.scoring)),
            drag_assist: Some(player_settings.drag_assist),
        },
    )
}

/// Generates a signed share URL for a playlist.
pub fn generate_playlist_share_url(
    settings: &RuntimeSettings,
    playlist_id: PlaylistId,
    is_student: bool,
) -> String {
    let base_url = settings.remote_target().pages_url();
    let query = serde_urlencoded::to_string(PlaylistShareUrlQuery {
        draft_or_live: DraftOrLive::Live,
        is_student,
    })
    .expect("valid playlist share URL query");
    let path = format!("/asset/play/playlist/{}?{}", playlist_id.0, query);

    sign_url(&base_url, &path, &settings.token_secret)
}

/// Generates a signed share URL for a course.
pub fn generate_course_share_url(
    settings: &RuntimeSettings,
    course_id: CourseId,
    is_student: bool,
) -> String {
    let base_url = settings.remote_target().pages_url();
    let query = serde_urlencoded::to_string(CourseShareUrlQuery {
        draft_or_live: DraftOrLive::Live,
        is_student,
    })
    .expect("valid course share URL query");
    let path = format!("/asset/play/course/{}?{}", course_id.0, query);

    sign_url(&base_url, &path, &settings.token_secret)
}

/// Adds a signed share URL to a JigResponse.
pub fn add_share_url_to_jig(settings: &RuntimeSettings, jig: &mut JigResponse) {
    let req = JigShareUrlRequest {
        direction: None,
        scoring: None,
    };
    jig.share_url = generate_public_jig_share_url(
        settings,
        jig.id,
        &jig.jig_data.default_player_settings,
        &req,
    );
    jig.student_share_url = generate_student_jig_share_url(
        settings,
        jig.id,
        &jig.jig_data.default_player_settings,
        &req,
    );
}

/// Adds a signed share URL to a PlaylistResponse.
pub fn add_share_url_to_playlist(settings: &RuntimeSettings, playlist: &mut PlaylistResponse) {
    playlist.share_url = generate_playlist_share_url(settings, playlist.id, false);
    playlist.student_share_url = generate_playlist_share_url(settings, playlist.id, true);
}

/// Adds a signed share URL to a CourseResponse.
pub fn add_share_url_to_course(settings: &RuntimeSettings, course: &mut CourseResponse) {
    course.share_url = generate_course_share_url(settings, course.id, false);
    course.student_share_url = generate_course_share_url(settings, course.id, true);
}

fn jig_share_path(jig_id: JigId, module_id: Option<ModuleId>) -> String {
    match module_id {
        Some(module_id) => format!("/asset/play/jig/{}/{}", jig_id.0, module_id.0),
        None => format!("/asset/play/jig/{}", jig_id.0),
    }
}
