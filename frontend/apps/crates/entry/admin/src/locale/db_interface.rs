use super::state::DisplayableEntry;
use futures_signals::signal::Mutable;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::locale::{
        Bundle, CreateEntryRequest, CreateEntryResponse, Entry, EntryStatus, ItemKind,
        ListBundleResponse, ListEntryGroupBy, ListEntryQuery, ListEntryResponse,
        ListItemKindResponse, UpdateEntryRequest,
    },
    error::EmptyError,
};
use utils::{
    fetch::{api_with_auth, api_with_auth_empty},
    unwrap::UnwrapJiExt,
};
use uuid::Uuid;

pub async fn get_bundles() -> Vec<Bundle> {
    api_with_auth::<ListBundleResponse, EmptyError, ()>(
        endpoints::locale::bundle::List::PATH,
        endpoints::locale::bundle::List::METHOD,
        None,
    )
    .await
    .unwrap_ji()
    .bundles
}

pub async fn get_item_kind() -> Vec<ItemKind> {
    api_with_auth::<ListItemKindResponse, EmptyError, ()>(
        endpoints::locale::item_kind::List::PATH,
        endpoints::locale::item_kind::List::METHOD,
        None,
    )
    .await
    .unwrap_ji()
    .item_kinds
}

pub async fn get_entries(bundles: Vec<Uuid>) -> Vec<DisplayableEntry> {
    let query = ListEntryQuery {
        bundles,
        group_by: ListEntryGroupBy::None,
    };
    let res = api_with_auth::<ListEntryResponse, EmptyError, ListEntryQuery>(
        endpoints::locale::entry::List::PATH,
        endpoints::locale::entry::List::METHOD,
        Some(query),
    )
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

    let res = api_with_auth::<CreateEntryResponse, EmptyError, CreateEntryRequest>(
        endpoints::locale::entry::Create::PATH,
        endpoints::locale::entry::Create::METHOD,
        Some(body),
    )
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

    let res = api_with_auth::<CreateEntryResponse, EmptyError, CreateEntryRequest>(
        endpoints::locale::entry::Create::PATH,
        endpoints::locale::entry::Create::METHOD,
        Some(body),
    )
    .await
    .unwrap_ji();

    new_entry_with_id(res.id, bundle_id)
}

pub async fn save_entry(entry: &DisplayableEntry) {
    let path = endpoints::locale::entry::Update::PATH.replace("{id}", &entry.id.to_string());
    let body: UpdateEntryRequest = entry.clone().into();
    let res = api_with_auth_empty::<EmptyError, UpdateEntryRequest>(
        &path,
        endpoints::locale::entry::Update::METHOD,
        Some(body),
    )
    .await;
    if res.is_err() {
        panic!();
    }
}

pub async fn delete_entry(entry_id: u32) {
    let path = endpoints::locale::entry::Delete::PATH.replace("{id}", &entry_id.to_string());
    let res = api_with_auth_empty::<(), UpdateEntryRequest>(
        &path,
        endpoints::locale::entry::Delete::METHOD,
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
