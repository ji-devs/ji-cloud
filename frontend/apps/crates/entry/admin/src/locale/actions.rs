use std::collections::HashMap;

use shared::domain::locale::{EntryStatus, ItemKind};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;

pub trait AsStringExt {
    fn to_string(&self) -> String;
    fn from_str(s: &str) -> Self;
}
impl AsStringExt for EntryStatus {
    fn to_string(&self) -> String {
        match self {
            EntryStatus::Approved => String::from("Approved"),
            EntryStatus::Discuss => String::from("Discuss"),
            EntryStatus::OnHold => String::from("On Hold"),
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "Approved" => EntryStatus::Approved,
            "Discuss" => EntryStatus::Discuss,
            "On Hold" => EntryStatus::OnHold,
            _ => panic!("Invalid EntryStatus: {}", s),
        }
    }
}

pub trait EnumOptionsExt {
    fn options() -> Vec<EntryStatus>;
}
impl EnumOptionsExt for EntryStatus {
    fn options() -> Vec<EntryStatus> {
        vec![
            EntryStatus::Approved,
            EntryStatus::Discuss,
            EntryStatus::OnHold,
        ]
    }
}
