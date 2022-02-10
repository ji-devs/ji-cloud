use std::{rc::Rc, collections::HashSet, cell::RefCell, pin::Pin};

use futures_signals::signal::{Mutable, Signal, Broadcaster};
use shared::domain::category::{Category, CategoryId};
use web_sys::HtmlElement;

pub struct CategoriesInput {
    pub input: Mutable<String>,
    pub focused: Mutable<bool>,
    pub all_categories: Vec<Category>,
    pub selected_categories: Mutable<HashSet<CategoryId>>,
    pub(super) placeholder: String,
    pub(super) value: Broadcaster<Pin<Box<dyn Signal<Item = String>>>>,
    pub(super) overlay_content_elem: Rc<RefCell<Option<HtmlElement>>>,
}

impl CategoriesInput {
    pub fn new(
        value: Pin<Box<dyn Signal<Item = String>>>,
        placeholder: String,
        categories: Vec<Category>,
        selected_categories: Mutable<HashSet<CategoryId>>
    ) -> Rc<Self> {

        let mut all_categories = vec![];
        get_categories_labels(categories, &mut all_categories, None);

        Rc::new(Self {
            input: Mutable::new(String::new()),
            focused: Mutable::new(false),
            all_categories,
            selected_categories,
            placeholder,
            value: Broadcaster::new(value),
            overlay_content_elem: Rc::new(RefCell::new(None)),
        })
    }
}

fn get_categories_labels(nested_categories: Vec<Category>, flat_categories: &mut Vec<Category>, parent_name: Option<String>) {
    for mut category in nested_categories {
        let children = category.children;
        category.children = vec![];
        if let Some(parent_name) = &parent_name {
            category.name.insert_str(0, parent_name);
        };
        let current_name = category.name.clone() + " ";
        flat_categories.push(category);
        get_categories_labels(children, flat_categories, Some(current_name));
    }
}
