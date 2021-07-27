use dominator::{html, clone, Dom};
use shared::domain::jig::{JigId, module::ModuleId, module::ModuleKind};
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
        let is_loading = Mutable::new(true);
        let module_kind:Rc<RefCell<Option<ModuleKind>>> = Rc::new(RefCell::new(None));

        html!("iframe" => web_sys::HtmlIFrameElement, {
            .property("allow", "autoplay; fullscreen")
            .property("slot", "main")
            .future(clone!(jig_id, module_id, module_kind, is_loading => async move { 
                actions::load_module_kind(jig_id, module_id, module_kind).await;
                is_loading.set_neq(false);
            }))
            .style("width", "100%")
            .style("height", "100%")
            .property_signal("src", is_loading.signal().map(clone!(jig_id, module_id, module_kind => move |loading| {

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

    }
}
