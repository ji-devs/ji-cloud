use serde::{Deserialize, Serialize};
use utils::{
    init::user::get_plan_tier, js_object, prelude::is_user_set, storage, unwrap::UnwrapJiExt,
};

const PLAYED_WITHOUT_LOGIN_ALLOWED: u32 = 5;
const PLAYED_FREE_ACCOUNT_ALLOWED_DAILY: u32 = 5;
const PLAYED_WITHOUT_LOGIN_COUNT_KEY: &'static str = "PLAYED_WITHOUT_LOGIN_COUNT";
const PLAYED_TODAY_FREE_KEY: &'static str = "PLAYED_TODAY_FREE";
pub const FREE_ACCOUNT_LIMIT_MESSAGE: &str = const_format::formatcp!(
    "
    Looking to play more than {} JIGs in one day?
    Upgrade now for UNLIMITED daily JIG plays.
",
    PLAYED_FREE_ACCOUNT_ALLOWED_DAILY
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Restricted {
    NoAccountLimit,
    FreeAccountLimit,
}

pub fn play_restricted() -> Option<Restricted> {
    match is_user_set() {
        true => (get_plan_tier().is_free()
            && get_free_played_today_count() >= PLAYED_FREE_ACCOUNT_ALLOWED_DAILY)
            .then(|| Restricted::FreeAccountLimit),
        false => (get_played_without_login_count() >= PLAYED_WITHOUT_LOGIN_ALLOWED)
            .then(|| Restricted::NoAccountLimit),
    }
}

pub fn increase_played_count() {
    match is_user_set() {
        true => {
            if get_plan_tier().is_free() {
                let count = get_free_played_today_count();
                set_free_played_today_count(count + 1);
            }
        }
        false => {
            let count: u32 = get_played_without_login_count();
            set_played_without_login_count(count + 1);
        }
    }
}

fn get_played_without_login_count() -> u32 {
    storage::get_local_storage()
        .unwrap_ji()
        .get_item(PLAYED_WITHOUT_LOGIN_COUNT_KEY)
        .unwrap_ji()
        .map(|v| v.parse().unwrap_ji())
        .unwrap_or(0)
}

fn set_played_without_login_count(count: u32) {
    let _ = storage::get_local_storage()
        .unwrap_ji()
        .set_item(PLAYED_WITHOUT_LOGIN_COUNT_KEY, &count.to_string());
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayedDay {
    day: String,
    count: u32,
}

fn get_free_played_today_count() -> u32 {
    storage::get_local_storage()
        .unwrap_ji()
        .get_item(PLAYED_TODAY_FREE_KEY)
        .unwrap_ji()
        .map(|v| {
            let v: PlayedDay = serde_json::from_str(&v).unwrap();
            if v.day == get_todays_date() {
                v.count
            } else {
                0
            }
        })
        .unwrap_or(0)
}

fn set_free_played_today_count(count: u32) {
    let val = serde_json::to_string(&PlayedDay {
        day: get_todays_date(),
        count,
    })
    .unwrap();
    let _ = storage::get_local_storage()
        .unwrap_ji()
        .set_item(PLAYED_TODAY_FREE_KEY, &val);
}

fn get_todays_date() -> String {
    let date: js_sys::Date = js_sys::Date::new_0();
    date.to_locale_date_string(
        "sv-SE",
        &js_object!({
            "year": "numeric",
            "month": "2-digit",
            "day": "2-digit",
        }),
    )
    .into()
}
