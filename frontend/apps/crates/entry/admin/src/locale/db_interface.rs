use super::state::DisplayableEntry;
use futures_signals::signal::Mutable;
use shared::{
    api::endpoints,
    domain::locale::{
        Bundle, CreateEntryPath, CreateEntryRequest, DeleteEntryPath, Entry, EntryStatus, ItemKind,
        ListBundlePath, ListEntryGroupBy, ListEntryPath, ListEntryQuery, ListEntryResponse,
        ListItemKindPath, UpdateEntryPath, UpdateEntryRequest,
    },
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use uuid::Uuid;

pub async fn get_bundles() -> Vec<Bundle> {
    endpoints::locale::bundle::List::api_with_auth(ListBundlePath(), None)
        .await
        .unwrap_ji()
        .bundles
}

pub async fn get_item_kind() -> Vec<ItemKind> {
    endpoints::locale::item_kind::List::api_with_auth(ListItemKindPath(), None)
        .await
        .unwrap_ji()
        .item_kinds
}

pub async fn get_entries(bundles: Vec<Uuid>) -> Vec<DisplayableEntry> {
    let query = ListEntryQuery {
        bundles,
        group_by: ListEntryGroupBy::None,
    };
    let res = endpoints::locale::entry::List::api_with_auth(ListEntryPath(), Some(query))
        .await
        .unwrap_ji();

    match res {
        ListEntryResponse::Bundles(_f) => panic!("Not what I need!"),
        ListEntryResponse::List(list) => {
            let list: Vec<Entry> = list;

            let list: Vec<DisplayableEntry> = list.into_iter().map(|e| e.into()).collect();

            list
        }
    }
}

pub async fn clone_entry(entry: &DisplayableEntry) -> DisplayableEntry {
    let body = CreateEntryRequest {
        bundle_id: entry.bundle_id,
        section: entry.section.clone(),
        item_kind_id: None,
        english: Some(entry.english.clone()),
        hebrew: Some(entry.hebrew.clone()),
        status: entry.status,
        zeplin_reference: {
            entry
                .zeplin_reference
                .lock_ref()
                .as_ref()
                .map(|url| url.as_str().to_string())
        },
        comments: Some(entry.comments.clone()),
        in_app: entry.in_app,
        in_element: entry.in_element,
        in_mock: entry.in_mock,
    };

    let res = endpoints::locale::entry::Create::api_with_auth(CreateEntryPath(), Some(body))
        .await
        .unwrap_ji();

    let mut new_entry = entry.clone();
    new_entry.id = res.id;
    new_entry
}

pub async fn create_entry(bundle_id: Uuid) -> DisplayableEntry {
    let body = CreateEntryRequest {
        bundle_id,
        section: None,
        item_kind_id: None,
        english: None,
        hebrew: None,
        status: EntryStatus::Approved,
        zeplin_reference: None,
        comments: None,
        in_app: false,
        in_element: true,
        in_mock: false,
    };

    let res = endpoints::locale::entry::Create::api_with_auth(CreateEntryPath(), Some(body))
        .await
        .unwrap_ji();

    new_entry_with_id(res.id, bundle_id)
}

pub async fn save_entry(entry: &DisplayableEntry) {
    let body: UpdateEntryRequest = entry.clone().into();
    let res = endpoints::locale::entry::Update::api_with_auth_empty(
        UpdateEntryPath(entry.id),
        Some(body),
    )
    .await;
    if res.is_err() {
        panic!();
    }
}

pub async fn delete_entry(entry_id: u32) {
    let res = endpoints::locale::entry::Delete::api_with_auth_empty(
        DeleteEntryPath(entry_id.clone()),
        None,
    )
    .await;
    if res.is_err() {
        panic!();
    }
}

fn new_entry_with_id(id: u32, bundle_id: Uuid) -> DisplayableEntry {
    DisplayableEntry {
        id,
        english: String::new(),
        hebrew: String::new(),
        section: None,
        item_kind_id: None,
        status: EntryStatus::Discuss,
        zeplin_reference: Mutable::new(None),
        comments: String::new(),
        in_app: false,
        in_element: false,
        in_mock: false,
        bundle_id,
    }
}
