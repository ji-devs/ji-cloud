use crate::{
    prelude::*,
    pages::{index}
};

use utils::routes::{Route, DevRoute};
use components::module::page::*;

pub struct Router {
    loader: AsyncLoader,
    page: RefCell<Option<PageKind>>
}

enum PageKind {
    Index(Rc<index::Page>),
    //Grid(module_grid::dom::Page), //?page=grid
    //Renderer(renderer_demo::page::Page), //?page=renderer
}

impl Router {
    pub fn render() {

        let _self = Rc::new(Self {
            loader: AsyncLoader::new(),
            page: RefCell::new(None)
        });

        _self.clone().loader.load(
            dominator::routing::url()
                .signal_ref(|url| Route::from_url(&url))
                .for_each(clone!(_self => move |route| {
                    *_self.page.borrow_mut() =
                        page_str(route)
                            .and_then(|page| match page.as_ref() {
                                //DEPRECATED (fix module page stuff): 
                                //"grid" => Some(PageKind::Grid(module_grid::dom::render())),
                                //"renderer" => Some(PageKind::Renderer(renderer_demo::page::Page::render())),
                                "" | _ => Some(PageKind::Index(index::Page::render()))
                            });

                    async {}
                }))
        );
    }
}

fn page_str(route:Route) -> Option<String> {
    match route {
        Route::Dev(route) => match route {
            DevRoute::Showcase(_, page) => {
                Some(page)
            },
            _ => None
        },
        _ => None
    }
}
