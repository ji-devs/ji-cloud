use super::button::dom::render_button;
use super::state::*;
use dominator::{html, Dom, DomBuilder};
use std::rc::Rc;
use web_sys::HtmlElement;

pub fn render_settings(state: Rc<ModuleSettings>) -> Dom {
    _render_mixin(
        state,
        None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>,
    )
}

pub fn render_button_mixin<F>(state: Rc<ModuleSettings>, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    _render_mixin(state, Some(mixin))
}

fn _render_mixin<F>(state: Rc<ModuleSettings>, _mixin: Option<F>) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("module-settings-container", {
        .children(
            state.lines.iter().map(|(line_kind, buttons)| {
              html!("module-settings-line", {
                  .property("kind", line_kind.as_str_id())
                    .children(buttons.iter().map(|button| {
                        match button {
                            Some(button) => render_button(button.clone()),
                            None => html!("span"),
                        }
                    }))
              })
            })
        )
    })
}
