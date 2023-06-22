/*
    all this trait does it add a shadow dom with styles, maybe just add a macro that does this similar to `html!`
    ```
        host!("./styles.css", {
            .child(...)
        })
    ```
*/

use std::cell::RefCell;
use std::{any::TypeId, collections::HashMap};

use dominator::{html, shadow_root, with_node, Dom, DomBuilder, ShadowRootMode};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CssStyleSheet;
use web_sys::{HtmlElement, ShadowRoot};

thread_local! {
    static STYLESHEETS: RefCell<HashMap<TypeId, CssStyleSheet>> = Default::default();
}

// using a T to get around orphan rules when implementing Component in dependents of this crate,
// not entirely sure that this is the right way of doing things though
pub trait Component<T: 'static> {
    const ROOT_ELEMENT: &'static str = "div";
    const SHADOW_ROOT_MODE: ShadowRootMode = ShadowRootMode::Open;

    fn styles() -> &'static str;

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot>;

    fn render(&self) -> Dom {
        let styles = STYLESHEETS.with(|stylesheets| {
            let id = TypeId::of::<T>();
            let mut stylesheets = stylesheets.borrow_mut();
            let sheet = match stylesheets.get(&id) {
                Some(sheet) => sheet,
                None => {
                    let sheet = create_style_sheet(Self::styles());
                    stylesheets.insert(id, sheet);
                    stylesheets.get(&id).unwrap()
                }
            };
            return sheet.clone();
        });
        html!(Self::ROOT_ELEMENT, {
            .apply(|dom| {
                self.apply_on_host(dom)
            })
            .shadow_root!(Self::SHADOW_ROOT_MODE => {
                .with_node!(shadow => {
                    .apply(|dom| {
                        add_styles_to_shadow(&shadow, &styles);
                        self.dom(dom)
                    })
                })
            })
        })
    }

    fn apply_on_host(&self, host: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        host
    }
}

// web_sys doesn't have yet CssStyleSheet::new()
#[wasm_bindgen(inline_js = r#"
    export function _create_style_sheet(s) {
        const sheet = new CSSStyleSheet;
        sheet.replaceSync(s);
        return sheet;
    }
"#)]
extern "C" {
    fn _create_style_sheet(s: &str) -> CssStyleSheet;
}
pub fn create_style_sheet(s: &str) -> CssStyleSheet {
    _create_style_sheet(s)
}

// web_sys doesn't have yet ShadowRoot::adopted_style_sheets
#[wasm_bindgen(inline_js = r#"
    export function _add_styles_to_shadow(shadow, styles) {
        shadow.adoptedStyleSheets = [...shadow.adoptedStyleSheets, styles];
    }
"#)]
extern "C" {
    fn _add_styles_to_shadow(shadow: &ShadowRoot, styles: &CssStyleSheet);
}
pub fn add_styles_to_shadow(shadow: &ShadowRoot, styles: &CssStyleSheet) {
    _add_styles_to_shadow(shadow, styles)
}
