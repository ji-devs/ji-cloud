use crate::student_code::state::StudentCode;

use dominator::{html, Dom};
use futures_signals::signal::Signal;
use utils::routes::{KidsRoute, Route};

pub struct Router {}

impl Router {
    pub fn render() -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal())
        })
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        dominator::routing::url().signal_ref(|url| {
            let route = Route::from_url(url);
            match route {
                Route::Kids(route) => match route {
                    KidsRoute::StudentCode(code) => Some(StudentCode::new().render(code)),
                },
                _ => None,
            }
        })
    }
}
