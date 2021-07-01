use dominator::{DomBuilder, Dom, html, clone};
use std::rc::Rc;
use super::state::*;
use web_sys::HtmlElement;
use super::button::dom::render_button;

pub fn render_settings(state: Rc<ModuleSettings>) -> Dom {
    _render_mixin(state, None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>) 
}

pub fn render_button_mixin<F>(state: Rc<ModuleSettings>, mixin: F) -> Dom 
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,

{
    _render_mixin(state, Some(mixin))
}

fn _render_mixin<F>(state: Rc<ModuleSettings>, mixin: Option<F>) -> Dom 
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,

{
    html!("module-settings-container", {
        .children(
            state.lines.iter().map(|(line_kind, buttons)| {
              html!("module-settings-line", {
                  .property("kind", line_kind.as_str_id())
                    .children(buttons.iter().map(|button| {
                        render_button(button.clone())
                    }))
              })
            })
        )
    })
}
