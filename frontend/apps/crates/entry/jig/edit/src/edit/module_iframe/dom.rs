use dominator::{clone, html, Dom};

use shared::domain::asset::AssetId;
use utils::prelude::*;

use super::ModuleIframe;
use futures_signals::signal::SignalExt;
use std::rc::Rc;

impl ModuleIframe {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_module_kind();

        html!("iframe" => web_sys::HtmlIFrameElement, {
            .property("allow", "autoplay; fullscreen")
            .property("slot", "main")
            .style("width", "100%")
            .style("height", "100%")
            .style("border", "none")
            .property_signal("src", state.module_kind.signal().map(clone!(state => move |module_kind| {
                match module_kind {
                    None => String::new(),
                    Some(module_kind) => {
                        let path = match state.asset_id {
                            AssetId::JigId(jig_id) => {
                                Route::Module(ModuleRoute::Edit(module_kind, jig_id, state.module_id)).to_string()
                            },
                            AssetId::CourseId(_course_id) => {
                                todo!()
                            },
                        };
                        let url = unsafe {
                            SETTINGS.get_unchecked()
                                .remote_target
                                .spa_iframe(&path)
                        };
                        url
                    },
                }
            })))
        })
    }
}
