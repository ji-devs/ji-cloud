use futures_signals::signal::Mutable;
use super::temp_utils::get_random_string;
use super::temp_utils::log;
use super::state::{Entry, EntryStatus, Bundle};

// make async
pub fn get_bundles() -> Vec<String> {
    vec![
        "JIG".to_string(),
        "Memory game".to_string(),
        "Publish".to_string(),
        "Poster".to_string(),
    ]
}


// make async
pub fn get_entries(bundles: &Vec<&Bundle>) -> Vec<Entry> {
    println!("{:?}", bundles);
    let json = r#"
        [
            {
                "id": "simple",
                "english": "Hello world",
                "hebrew": "כגד",
                "in_app": false,
                "in_element": false,
                "in_mock": false,
                "comments": "fdsa",
                "zeplin_reference": null,
                "item_kind": "Button",
                "status": "Discuss",
                "section": "sec1"
            },
            {
                "id": "simple",
                "english": "Hello world",
                "hebrew": "כגד",
                "in_app": false,
                "in_element": false,
                "in_mock": false,
                "comments": "fdsa",
                "zeplin_reference": null,
                "item_kind": "hay",
                "status": "OnHold",
                "section": "sec2"
            },
            {
                "id": "simple",
                "english": "Hello world",
                "hebrew": "כגד",
                "in_app": false,
                "in_element": false,
                "in_mock": false,
                "comments": "fdsa",
                "zeplin_reference": null,
                "item_kind": "Button",
                "status": "Approved",
                "section": "sec3"
            },
            {
                "id": "complex",
                "english": "{$userName} {$photoCount ->\n    [one] added a new photo\n   *[other] added {$photoCount} new photos\n} to {$userGender ->\n    [male] his stream\n    [female] her stream\n   *[other] their stream\n}.\n",
                "hebrew": "כגלםממך",
                "in_app": false,
                "in_element": false,
                "in_mock": false,
                "comments": "fdsa",
                "zeplin_reference": "https://google.com",
                "item_kind": "Subheading",
                "status": "Approved",
                "section": "sec4"
            }
        ]
    "#;
    let vec: Vec<Entry> = serde_json::from_str(&json).unwrap();
    vec
}

pub async fn clone_entry(entry: &Entry) -> Entry {
    let mut entry = entry.clone();
    entry.id = get_random_string(10);
    log(&entry);
    entry
}

pub async fn create_entry() -> Entry {
    Entry {
        id: get_random_string(10),
        english: String::new(),
        hebrew: String::new(),
        section: Some("sec2".to_string()),
        item_kind: Some("Subheading".to_string()),
        status: EntryStatus::Discuss,
        zeplin_reference: Mutable::new(None),
        comments: String::new(),
        in_app: false,
        in_element: false,
        in_mock: false,
    }
}

pub async fn save_entry(entry:Entry) ->Entry {
    log(&entry);
    entry
}
