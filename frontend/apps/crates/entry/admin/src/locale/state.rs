use std::collections::BTreeMap;
use std::str::FromStr;
use dominator_helpers::futures::AsyncLoader;
use std::collections::HashMap;
use super::db_interface;
use url::Url;
use web_sys::{HtmlDialogElement, HtmlOptionElement, HtmlOptionsCollection};
use std::rc::Rc;
use std::clone::Clone;
use serde_derive::{Deserialize, Serialize};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use strum_macros::{EnumString, Display, EnumIter};
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use std::cmp::Ord;


pub struct State {
    pub bundles: HashMap<Bundle, bool>,
    pub entries: MutableVec<Rc<Mutable<Entry>>>,
    pub visible_columns: MutableVec<String>,
    pub hidden_columns: MutableVec<String>,
    pub dialog_ref: Mutable<Option<HtmlDialogElement>>,
    pub loader: Rc<AsyncLoader>,
    pub saving_loader: Rc<AsyncLoader>,

    pub section_options: Mutable<BTreeMap<Section, bool>>,
    pub item_kind_options: Mutable<BTreeMap<ItemKind, bool>>,
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

        let visible_bundles: Vec<&Bundle> = bundles
            .iter()
            .filter(|bundle| *bundle.1)
            .map(|bundle| bundle.0)
            .collect();
        let entries = db_interface::get_entries(&visible_bundles).await;

        let section_options = Self::generate_options(&entries, |t| t.section.clone().unwrap());
        let item_kind_options = Self::generate_options(&entries, |t| t.item_kind.clone().unwrap());
        let status_options = EntryStatus::iter().map(|s| (s, true)).collect::<BTreeMap<EntryStatus, bool>>();

        let entries = entries.iter().map(|i| Rc::new(Mutable::new(i.clone()))).collect();
        let entries = MutableVec::new_with_values(entries);


        let visible_columns = MutableVec::new_with_values(vec![
            "ID".to_string(),
            "Section".to_string(),
            "Item Kind".to_string(),
            "English".to_string(),
            "Status".to_string(),
            "Zeplin reference".to_string(),
            "Comments".to_string(),
        ]);
        let hidden_columns = MutableVec::new_with_values(vec![
            "App".to_string(),
            "Element".to_string(),
            "Mock".to_string(),
        ]);
        Self {
            bundles,
            entries,
            visible_columns,
            hidden_columns,
            dialog_ref: Mutable::new(None),
            loader: Rc::new(AsyncLoader::new()),
            saving_loader: Rc::new(AsyncLoader::new()),

            section_options: Mutable::new(section_options),
            item_kind_options: Mutable::new(item_kind_options),
            status_options: Mutable::new(status_options),

            sort: Mutable::new(Sort {
                column: SortKind::ItemKind,
                order: SortOrder::Asc,
            }),
        }
    }

    pub async fn add_entry(&self) {
        let entry = db_interface::create_entry().await;
        let mut vec = self.entries.lock_mut();
        log::info!("{:?}", entry);
        vec.push_cloned(Rc::new(Mutable::new(entry)));
    }

    pub async fn clone_entry(&self, entry: &Entry) {
        let entry = db_interface::clone_entry(&entry).await;
        let mut vec = self.entries.lock_mut();
        vec.push_cloned(Rc::new(Mutable::new(entry)));
    }

    pub async fn save_entry(&self, entry: &Entry) {
        let _entry: Entry = db_interface::save_entry(&entry).await;
    }

    pub fn remove_entry(&self, id: &str) {
        let mut vec = self.entries.lock_mut();
        let index = vec.iter().position(|i| i.lock_ref().id == id).unwrap();
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

    pub fn filter_change<T>(options: &HtmlOptionsCollection, map: &mut BTreeMap<T, bool> ) where T: FromStr + Ord {
        for i in 0..options.length() {
            let option: HtmlOptionElement = options.get_with_index(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();

            let parsed = T::from_str(&option.value()).unwrap_or_else(|_| panic!("Invalid option in select"));

            map.insert(parsed, option.selected());
        }
    }

    pub async fn selected_bundles_change(&self, options: &HtmlOptionsCollection) {
        let mut visible_bundles = Vec::new();
        for i in 0..options.length() {
            let option: HtmlOptionElement = options.get_with_index(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
            if option.selected() {
                visible_bundles.push(option.value());
            }
        };
        let entries: Vec<Rc<Mutable<Entry>>> = db_interface::get_entries(&visible_bundles.iter().map(|s| s).collect())
            .await
            .into_iter()
            .map(|e| Rc::new(Mutable::new(e)))
            .collect();
        self.entries.lock_mut().replace_cloned(entries);
    }

    pub fn regenerate_section_options(&self) {
        let entries: Vec<Entry> = self.entries.lock_ref().iter().map(|t| t.lock_ref().clone()).collect();
        let section_options = Self::generate_options(&entries, |t| t.section.clone().unwrap());
        let mut lock = self.section_options.lock_mut();
        *lock = section_options;
    }

    pub fn regenerate_item_kind_options(&self) {
        let entries: Vec<Entry> = self.entries.lock_ref().iter().map(|t| t.lock_ref().clone()).collect();
        let item_kind_options = Self::generate_options(&entries, |t| t.item_kind.clone().unwrap());
        let mut lock = self.item_kind_options.lock_mut();
        *lock = item_kind_options;
    }

    fn generate_options<T>(entry_vec: &Vec<Entry>, f: fn(&Entry) -> T) -> BTreeMap<T, bool>
        where T: Ord
    {
        entry_vec.iter().map(|t| (f(t), true)).collect()
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryStatus {
    Approved,
    Discuss,
    #[strum(serialize = "On Hold")]
    OnHold,
}

pub type Section = String;
pub type ItemKind = String;
pub type Bundle = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entry {
    pub id: String,
    pub section: Option<Section>,
    pub item_kind: Option<ItemKind>,
    pub english: String,
    pub hebrew: String,
    pub status: EntryStatus,
    pub zeplin_reference: Mutable<Option<Url>>,
    pub comments: String,
    pub in_app: bool,
    pub in_element: bool,
    pub in_mock: bool,
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
    Status,
    Comments,
}

#[derive(Clone, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc
}
