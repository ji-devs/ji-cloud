use dominator::{Dom, html, clone};
use shared::domain::jig::module::body::legacy::{ModuleData, design::*};
use utils::{prelude::*, path};
use super::styles;
use crate::base::state::Base;
use std::rc::Rc;

impl Base {
    pub fn render_design(self: Rc<Self>) -> Dom {
        html!("empty-fragment", {
            .apply_if(self.raw.design.bg.is_some(), |dom| {
                let url = &path::legacy::layers_url(
                    &self.raw.base_id,
                    &self.raw.id,
                    self.raw.design.bg.as_ref().unwrap_ji()
                );

                dom.child(html!("img", {
                    .class(&*styles::BG)
                    .attribute("src", &url)
                }))
            })
        })
    }
}