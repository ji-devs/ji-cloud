use dominator::{Dom, html};
use futures_signals::signal::Signal;
use std::rc::Rc;
use utils::routes::{KidsRoute, Route};
use super::student_code;


pub struct Router {
}

impl Router {
    pub fn render() -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal())
        })
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        dominator::routing::url()
            .signal_ref(|url| {
                let route = Route::from_url(&url);
                match route {
                    Route::Kids(route) => {
                        match route {
                            KidsRoute::StudentCode => {
                                Some(student_code::dom::render(Rc::new(student_code::state::State::new())))
                            },
                        }
                    }
                    _ => None
                }
            })
    }
}
