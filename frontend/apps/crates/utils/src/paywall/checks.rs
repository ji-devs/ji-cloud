use crate::prelude::get_plan_tier;
use shared::domain::billing::PlanTier;

pub fn can_create_jig(total_existing: u64) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => total_existing < 5,
        PlanTier::Free => total_existing < 5,
    }
}
pub fn can_create_playlist(_total_existing: u64) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => true,
        PlanTier::Free => false,
    }
}
pub fn can_create_resource(total_existing: u64) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => total_existing < 5,
        PlanTier::Free => false,
    }
}
pub fn can_create_course(_total_existing: u64) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => false,
        PlanTier::Free => false,
    }
}
pub fn can_play_jig(is_premium: bool) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => true,
        PlanTier::Free => !is_premium,
    }
}
pub fn can_play_playlist(is_premium: bool) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => true,
        PlanTier::Free => !is_premium,
    }
}
pub fn can_play_resource(is_premium: bool) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => true,
        PlanTier::Free => !is_premium,
    }
}
pub fn can_play_course(is_premium: bool) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => true,
        PlanTier::Free => !is_premium,
    }
}
pub fn can_print() -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => true,
        PlanTier::Free => false,
    }
}
pub fn can_use_image(is_premium: bool) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => !is_premium,
        PlanTier::Free => !is_premium,
    }
}
pub fn can_use_theme(is_premium: bool) -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => !is_premium,
        PlanTier::Free => !is_premium,
    }
}
pub fn can_create_circle() -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => true,
        PlanTier::Free => false,
    }
}
pub fn can_create_codes() -> bool {
    match get_plan_tier() {
        PlanTier::Pro => true,
        PlanTier::Basic => true,
        PlanTier::Free => false,
    }
}
