use std::collections::{BTreeMap, HashMap};
use dominator_helpers::make_custom_event_serde;
use serde_derive::Deserialize;
use wasm_bindgen::prelude::*;


#[derive(Deserialize, Debug)]
pub struct AddEntryData {}

make_custom_event_serde!("add-entry", AddEntryEvent, AddEntryData);




pub type SelectedBundleChangeData = HashMap<String, bool>;

make_custom_event_serde!("selected-bundle-change", SelectedBundleChangeEvent, SelectedBundleChangeData);

impl SelectedBundleChangeEvent {
    pub fn bundles(&self) -> HashMap<String, bool> {
        self.data()
    }
}




#[derive(Deserialize, Debug)]
pub struct SortData {}

make_custom_event_serde!("sort", SortEvent, SortData);




pub type FilterData = BTreeMap<String, bool>;

make_custom_event_serde!("filter", FilterEvent, FilterData);

impl FilterEvent {
    pub fn options(&self) -> BTreeMap<String, bool> {
        self.data()
    }
}
