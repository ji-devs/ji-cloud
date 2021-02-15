use std::collections::HashSet;
use dominator_helpers::futures::AsyncLoader;
use std::collections::HashMap;
use super::db_interface;
use url::Url;
use web_sys::HtmlDialogElement;
use std::rc::Rc;
use std::clone::Clone;
use serde_derive::{Deserialize, Serialize};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use strum_macros::{EnumString, Display, EnumIter};
use strum::IntoEnumIterator;


pub struct State {
    pub entries: HashMap<String, bool>,
    pub translations: MutableVec<Rc<Mutable<Translation>>>,
    pub sections: MutableVec<Section>,
    pub item_kinds: MutableVec<ItemKind>,
    pub visible_columns: MutableVec<String>,
    pub hidden_columns: MutableVec<String>,
    pub dialog_ref: Mutable<Option<HtmlDialogElement>>,
    pub loader: Rc<AsyncLoader>,


    pub sort: Mutable<Sort>,
    pub filters: Rc<Mutable<Filters>>,
}

impl State {
    pub fn new() -> State {
        let entries: HashMap<String, bool> = db_interface::get_entries()
            .iter()
            .map(|entry| (entry.clone(), true))
            .collect();

        // this should probably react to a signal update
        let visible_entries: Vec<&String> = entries
            .iter()
            .filter(|entry| *entry.1)
            .map(|entry| entry.0)
            .collect();
        let translations = db_interface::get_translations(&visible_entries);
        let sections_vec = Self::generate_sections(&translations);
        let sections = MutableVec::new_with_values(sections_vec.clone());
        let item_kinds_vec = Self::generate_item_kinds(&translations);
        let item_kinds = MutableVec::new_with_values(item_kinds_vec.clone());

        let translations = translations.iter().map(|i| Rc::new(Mutable::new(i.clone()))).collect();
        let translations = MutableVec::new_with_values(translations);


        let visible_columns = vec![
            "ID".to_string(),
            "Section".to_string(),
            "Translation Kind".to_string(),
            "English".to_string(),
            "Status".to_string(),
            "Zeplin reference".to_string(),
            "Comments".to_string(),
        ];
        let hidden_columns = vec![
            "App".to_string(),
            "Element".to_string(),
            "Mock".to_string(),
        ];
        let visible_columns = MutableVec::new_with_values(visible_columns);
        let hidden_columns = MutableVec::new_with_values(hidden_columns);
        Self {
            entries,
            translations,
            sections,
            item_kinds,
            visible_columns,
            hidden_columns,
            dialog_ref: Mutable::new(None),
            loader: Rc::new(AsyncLoader::new()),



            sort: Mutable::new(Sort {
                column: SortKind::ItemKind,
                order: SortOrder::Asc,
            }),
            filters: Rc::new(Mutable::new(Filters {
                section: sections_vec.iter().map(|s| s.clone()).collect(),
                item_kind: item_kinds_vec.iter().map(|ik| ik.clone()).collect(),
                status: TranslationStatus::iter().collect(),
            })),
        }
    }

    pub async fn add_translation(&self) {
        let translation = db_interface::create_translation().await;
        let mut vec = self.translations.lock_mut();
        super::temp_utils::log(&translation);
        vec.push_cloned(Rc::new(Mutable::new(translation)));
    }

    pub async fn clone_translation(&self, translation: &Translation) {
        let translation = db_interface::clone_translation(&translation).await;
        let mut vec = self.translations.lock_mut();
        vec.push_cloned(Rc::new(Mutable::new(translation)));
    }

    pub fn remove_translation(&self, id: &str) {
        let mut vec = self.translations.lock_mut();
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

    fn generate_sections(translation_vec: &Vec<Translation>) -> Vec<String> {
        translation_vec.iter().filter(|t| t.section.is_some()).map(|s| s.section.clone().unwrap()).collect()
    }

    fn generate_item_kinds(translation_vec: &Vec<Translation>) -> Vec<String> {
        translation_vec.iter().filter(|t| t.item_kind.is_some()).map(|s| s.item_kind.clone().unwrap()).collect()
    }

}


#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, EnumIter, PartialEq, Eq, Hash)]
pub enum TranslationStatus {
    Approved,
    Discuss,
    #[strum(serialize = "On Hold")]
    OnHold,
}

pub type Section = String;
pub type ItemKind = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Translation {
    pub id: String,
    pub section: Option<Section>,
    pub item_kind: Option<ItemKind>,
    pub english: String,
    pub hebrew: String,
    pub status: TranslationStatus,
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

#[derive(Clone)]
pub struct Filters {
    pub section: HashSet<Section>,
    pub item_kind: HashSet<ItemKind>,
    pub status: HashSet<TranslationStatus>,
}

#[derive(Clone, PartialEq, Serialize)]
pub enum SortOrder {
    Asc,
    Desc
}
