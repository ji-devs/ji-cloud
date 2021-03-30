use dominator::{html, clone, Dom};
use shared::domain::jig::{JigId, ModuleId, module::ModuleKind};
use utils::prelude::*;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use std::cell::RefCell;
use super::actions;

pub struct IframeDom {
}

impl IframeDom {
    pub fn render(jig_id: JigId, module_id: ModuleId) -> Dom {
        let loader = AsyncLoader::new();
        let module_kind:Rc<RefCell<Option<ModuleKind>>> = Rc::new(RefCell::new(None));


        loader.load(clone!(jig_id, module_id, module_kind => async move {
            actions::load_module_kind(jig_id, module_id, module_kind).await;
        }));

        Dom::with_state(loader, clone!(jig_id, module_id, module_kind => move |loader| {
            html!("iframe" => web_sys::HtmlIFrameElement, {
                .property("slot", "main")
                .style("width", "100%")
                .style("height", "100%")
                .property_signal("src", loader.is_loading().map(clone!(jig_id, module_id, module_kind => move |loading| {

                    if loading {
                        "".to_string()
                    } else {
                        let module_kind = module_kind.borrow_mut().take().unwrap_ji();
                        let route:String = Route::Module(ModuleRoute::Edit(module_kind, jig_id, module_id)).into();
                        let url = unsafe {
                            SETTINGS.get_unchecked()
                                .remote_target
                                .spa_iframe(&route)
                        };

                        url
                    }
                })))
            })
        }))

    }
}
