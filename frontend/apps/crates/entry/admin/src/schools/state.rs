use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, Signal};
use shared::domain::Page;
use strum_macros::Display;
use utils::routes::AdminSchoolsRoute;

pub struct Schools {
    pub route: Mutable<AdminSchoolsRoute>,
    pub search_filters: SearchFilters,
    pub loader: AsyncLoader,
}

impl Schools {
    pub fn new(route: AdminSchoolsRoute) -> Rc<Self> {
        Rc::new(Self {
            route: Mutable::new(route),
            search_filters: SearchFilters::default(),
            loader: AsyncLoader::new(),
        })
    }
}

#[derive(Default)]
pub struct SearchFilters {
    pub q: Mutable<String>,
    pub verified: Mutable<VerifiedFilter>,
    pub active_page: Mutable<Page>,
    events: Mutable<()>,
}

impl SearchFilters {
    pub fn set_query(&self, q: String) {
        self.q.set(q);
        self.active_page.set(0.into());
        self.events.set(());
    }

    pub fn set_verified(&self, verified: VerifiedFilter) {
        self.verified.set(verified);
        self.active_page.set(0.into());
        self.events.set(());
    }

    pub fn set_active_page(&self, active_page: Page) {
        self.active_page.set(active_page);
        self.events.set(());
    }

    pub fn change_signal(&self) -> impl Signal<Item = ()> {
        self.events.signal()
    }
}

#[derive(Display, Debug, Clone, Eq, PartialEq)]
pub enum VerifiedFilter {
    #[strum(serialize = "All schools")]
    All,
    Verified,
    Unverified,
}

impl Default for VerifiedFilter {
    fn default() -> Self {
        Self::Unverified
    }
}

impl VerifiedFilter {
    pub fn as_value(&self) -> Option<bool> {
        match self {
            Self::All => None,
            Self::Verified => Some(true),
            Self::Unverified => Some(false),
        }
    }
}
