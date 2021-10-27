use super::actions::EnumOptionsExt;
use super::{actions::AsStringExt, db_interface};
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use serde_derive::{Deserialize, Serialize};
use shared::domain::locale::{Bundle, Entry, EntryStatus, ItemKind, UpdateEntryRequest};
use std::clone::Clone;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use strum_macros::Display;
use url::Url;
use uuid::Uuid;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlDialogElement, HtmlOptionElement, HtmlOptionsCollection};

pub struct LoaderState {
    pub loader: AsyncLoader,
    pub inner: Mutable<Option<Rc<State>>>,
}

impl LoaderState {
    pub fn new() -> Self {
        let loader = AsyncLoader::new();
        let inner = Mutable::new(None);

        loader.load(clone!(inner => async move {
            inner.set(Some(Rc::new(State::new().await)));
        }));

        Self { loader, inner }
    }
}

pub struct State {
    pub bundles: Mutable<HashMap<Bundle, bool>>,
    pub entries: MutableVec<Rc<Mutable<DisplayableEntry>>>,
    pub visible_columns: Rc<MutableVec<Column>>,
    pub hidden_columns: Rc<MutableVec<Column>>,
    pub dialog_ref: Mutable<Option<HtmlDialogElement>>,
    pub loader: Rc<AsyncLoader>,
    pub saving_loader: Rc<AsyncLoader>,

    pub item_kind_filter: Mutable<HashMap<Option<Uuid>, bool>>,
    pub item_kind_options: Vec<ItemKind>,
    pub section_options: Mutable<BTreeMap<Section, bool>>,
    pub status_options: Mutable<BTreeMap<EntryStatus, bool>>,

    pub sort: Mutable<Sort>,
}

impl State {
    pub async fn new() -> State {
        let bundles: HashMap<Bundle, bool> = db_interface::get_bundles()
            .await
            .iter()
            .map(|bundle| (bundle.clone(), true))
            .collect();

        let item_kind_options: Vec<ItemKind> = db_interface::get_item_kind().await;

        let mut item_kind_filter: HashMap<Option<Uuid>, bool> = item_kind_options
            .iter()
            .map(|item_kind| (Some(item_kind.id), true))
            .collect();
        item_kind_filter.insert(None, true);

        let visible_bundles: Vec<Uuid> = bundles
            .iter()
            .filter(|bundle| *bundle.1)
            .map(|bundle| bundle.0.id)
            .collect();
        let entries = db_interface::get_entries(visible_bundles).await;

        let section_options =
            Self::generate_options(&entries, |t| t.section.clone().unwrap_or_default());
        let status_options = EntryStatus::options()
            .into_iter()
            .map(|s| (s, true))
            .collect::<BTreeMap<EntryStatus, bool>>();

        let entries = entries
            .iter()
            .map(|i| Rc::new(Mutable::new(i.clone())))
            .collect();
        let entries = MutableVec::new_with_values(entries);

        let mut columns = BTreeMap::new();
        columns.insert("ID".to_string(), true);
        columns.insert("Section".to_string(), true);
        columns.insert("Item Kind".to_string(), true);
        columns.insert("English".to_string(), true);
        columns.insert("Hebrew".to_string(), false);
        columns.insert("Status".to_string(), true);
        columns.insert("Zeplin reference".to_string(), true);
        columns.insert("Comments".to_string(), true);
        columns.insert("App".to_string(), true);
        columns.insert("Element".to_string(), true);
        columns.insert("Mock".to_string(), true);
        columns.insert("Actions".to_string(), true);

        let visible_columns = Rc::new(MutableVec::new_with_values(vec![
            Column::ID,
            Column::Section,
            Column::ItemKind,
            Column::English,
            Column::Status,
            Column::ZeplinReference,
            Column::Comments,
            Column::App,
            Column::Element,
            Column::Mock,
            Column::Actions,
        ]));
        let hidden_columns = Rc::new(MutableVec::new_with_values(vec![
            Column::Hebrew,
            Column::Bundle,
        ]));

        Self {
            bundles: Mutable::new(bundles),
            entries,
            visible_columns,
            hidden_columns,
            dialog_ref: Mutable::new(None),
            loader: Rc::new(AsyncLoader::new()),
            saving_loader: Rc::new(AsyncLoader::new()),

            item_kind_options,
            item_kind_filter: Mutable::new(item_kind_filter),
            section_options: Mutable::new(section_options),
            status_options: Mutable::new(status_options),

            sort: Mutable::new(Sort {
                column: SortKind::ItemKind,
                order: SortOrder::Asc,
            }),
        }
    }

    pub async fn add_entry(state: Rc<State>) {
        let bundle_id = state
            .bundles
            .lock_ref()
            .iter()
            .find(|(_, selected)| **selected)
            .unwrap_throw()
            .0
            .id;
        let entry = db_interface::create_entry(bundle_id).await;
        let mut vec = state.entries.lock_mut();
        vec.push_cloned(Rc::new(Mutable::new(entry)));
    }

    pub async fn clone_entry(&self, entry: &DisplayableEntry) {
        let entry = db_interface::clone_entry(entry).await;
        let mut vec = self.entries.lock_mut();
        vec.push_cloned(Rc::new(Mutable::new(entry)));
    }

    pub async fn save_entry(&self, entry: &DisplayableEntry) {
        db_interface::save_entry(entry).await;
    }

    pub async fn remove_entry(&self, entry_id: u32) {
        db_interface::delete_entry(entry_id).await;
        let mut vec = self.entries.lock_mut();
        let index = vec
            .iter()
            .position(|i| i.lock_ref().id == entry_id)
            .unwrap();
        vec.remove(index);
    }

    pub fn sort_clicked(&self, sort_kind: SortKind) {
        let mut sort = self.sort.lock_mut();
        if sort.column == sort_kind {
            sort.order = match sort.order {
                SortOrder::Asc => SortOrder::Desc,
                SortOrder::Desc => SortOrder::Asc,
            }
        } else {
            sort.column = sort_kind;
            sort.order = SortOrder::Asc;
        }
    }

    // filter change might be combined
    pub fn filter_change<T>(options: &HtmlOptionsCollection, map: &mut BTreeMap<T, bool>)
    where
        T: FromStr + Ord,
    {
        for i in 0..options.length() {
            let option: HtmlOptionElement = options
                .get_with_index(i)
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();

            let parsed =
                T::from_str(&option.value()).unwrap_or_else(|_| panic!("Invalid option in select"));

            map.insert(parsed, option.selected());
        }
    }

    pub fn filter_change_str_ext<T>(options: &HtmlOptionsCollection, map: &mut BTreeMap<T, bool>)
    where
        T: AsStringExt + Ord,
    {
        for i in 0..options.length() {
            let option: HtmlOptionElement = options
                .get_with_index(i)
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();

            let parsed = T::from_str(&option.value());

            map.insert(parsed, option.selected());
        }
    }

    pub async fn selected_bundles_change(&self, options: &HtmlOptionsCollection) {
        let mut visible_bundles = Vec::new();
        for i in 0..options.length() {
            let option: HtmlOptionElement = options
                .get_with_index(i)
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            let uuid = Uuid::parse_str(&option.value()).unwrap_throw();
            let selected = option.selected();
            let mut bundles = self.bundles.lock_mut();
            let bundle = bundles
                .iter()
                .find(|(bundle, _)| bundle.id == uuid)
                .unwrap_throw()
                .0
                .clone();
            bundles.insert(bundle, selected);
            if selected {
                visible_bundles.push(uuid);
            }
        }

        let entries: Vec<Rc<Mutable<DisplayableEntry>>> =
            db_interface::get_entries(visible_bundles)
                .await
                .into_iter()
                .map(|e| Rc::new(Mutable::new(e)))
                .collect();
        self.entries.lock_mut().replace_cloned(entries);
    }

    pub fn regenerate_section_options(&self) {
        let entries: Vec<DisplayableEntry> = self
            .entries
            .lock_ref()
            .iter()
            .map(|t| t.lock_ref().clone())
            .collect();
        let section_options =
            Self::generate_options(&entries, |t| t.section.clone().unwrap_or_default());
        let mut lock = self.section_options.lock_mut();
        *lock = section_options;
    }

    fn generate_options<T>(
        entry_vec: &Vec<DisplayableEntry>,
        f: fn(&DisplayableEntry) -> T,
    ) -> BTreeMap<T, bool>
    where
        T: Ord,
    {
        entry_vec.iter().map(|t| (f(t), true)).collect()
    }
}

pub type Section = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DisplayableEntry {
    pub id: u32,
    pub section: Option<Section>,
    pub item_kind_id: Option<Uuid>,
    pub english: String,
    pub hebrew: String,
    pub status: EntryStatus,
    pub zeplin_reference: Mutable<Option<Url>>,
    pub comments: String,
    pub in_app: bool,
    pub in_element: bool,
    pub in_mock: bool,
    pub bundle_id: Uuid,
}

impl From<Entry> for DisplayableEntry {
    fn from(entry: Entry) -> Self {
        DisplayableEntry {
            id: entry.id,
            section: entry.section.clone(),
            item_kind_id: entry.item_kind_id,
            english: entry.english.clone().unwrap_or_default(),
            hebrew: entry.hebrew.clone().unwrap_or_default(),
            status: entry.status,
            zeplin_reference: {
                let v = entry
                    .zeplin_reference
                    .map(|url| Url::parse(&url).unwrap_throw());
                Mutable::new(v)
            },
            comments: entry.comments.clone().unwrap_or_default(),
            in_app: entry.in_app,
            in_element: entry.in_element,
            in_mock: entry.in_mock,
            bundle_id: entry.bundle_id,
        }
    }
}

impl From<DisplayableEntry> for Entry {
    fn from(displayable_entry: DisplayableEntry) -> Self {
        Entry {
            id: displayable_entry.id,
            section: displayable_entry.section.clone(),
            item_kind_id: displayable_entry.item_kind_id,
            english: Some(displayable_entry.english.clone()),
            hebrew: Some(displayable_entry.hebrew.clone()),
            status: displayable_entry.status,
            zeplin_reference: {
                displayable_entry
                    .zeplin_reference
                    .lock_ref()
                    .as_ref()
                    .map(|url| url.to_string())
            },
            comments: Some(displayable_entry.comments.clone()),
            in_app: displayable_entry.in_app,
            in_element: displayable_entry.in_element,
            in_mock: displayable_entry.in_mock,
            bundle_id: displayable_entry.bundle_id,
        }
    }
}

impl From<DisplayableEntry> for UpdateEntryRequest {
    fn from(entry: DisplayableEntry) -> Self {
        UpdateEntryRequest {
            bundle_id: Some(entry.bundle_id),
            section: Some(entry.section.clone()),
            item_kind_id: entry.item_kind_id,
            english: Some(entry.english.clone()),
            hebrew: Some(entry.hebrew.clone()),
            status: Some(entry.status),
            zeplin_reference: {
                let v = entry
                    .zeplin_reference
                    .lock_ref()
                    .as_ref()
                    .map(|url| url.to_string());
                Some(v)
            },
            comments: Some(Some(entry.comments.clone())),
            in_app: Some(entry.in_app),
            in_element: Some(entry.in_element),
            in_mock: Some(entry.in_mock),
        }
    }
}

#[derive(Clone)]
pub struct Sort {
    pub order: SortOrder,
    pub column: SortKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortKind {
    Section,
    ItemKind,
    English,
    Hebrew,
    Status,
    Comments,
}

#[derive(Clone, PartialEq, Display)]
pub enum SortOrder {
    #[strum(serialize = "asc")]
    Asc,

    #[strum(serialize = "desc")]
    Desc,
}

#[derive(Clone, PartialEq, Display)]
pub enum Column {
    ID,
    Section,
    #[strum(serialize = "Item Kind")]
    ItemKind,
    English,
    Hebrew,
    Status,
    #[strum(serialize = "Zeplin Reference")]
    ZeplinReference,
    Comments,
    App,
    Element,
    Mock,
    Actions,
    Bundle,
}
