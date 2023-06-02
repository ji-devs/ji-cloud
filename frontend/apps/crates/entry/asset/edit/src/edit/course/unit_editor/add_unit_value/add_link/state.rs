use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use url::Url;

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddLink {
    pub url: Mutable<Option<Url>>,
    pub url_str: Mutable<String>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
    pub loader: AsyncLoader,
}

impl AddLink {
    pub fn new(add_unit_value_state: Rc<AddUnitValueState>, url: &Option<Url>) -> Rc<Self> {
        let url_str = if let Some(url) = url {
            url.to_string()
        } else {
            "".to_string()
        };

        Rc::new(Self {
            url: Mutable::new(url.clone()),
            url_str: Mutable::new(url_str.clone()),
            add_unit_value_state,
            loader: AsyncLoader::new(),
        })
    }
}
