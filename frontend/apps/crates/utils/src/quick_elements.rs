pub use const_format::formatcp;

#[macro_export]
macro_rules! dialog {
    ( $($methods:tt)* ) => {{
        use dominator::{html, with_node};
        // Wrapper contains a backdrop overlay and the dialog as siblings.
        // Uses show() instead of showModal() so the dialog stays out of the
        // browser's top layer, allowing overlays like Google Maps Autocomplete
        // (.pac-container) to render above it.
        html!("div", {
            .style("position", "fixed")
            .style("inset", "0")
            .style("z-index", "10000")
            .children(&mut [
                // Backdrop overlay — dispatches "cancel" on the sibling
                // dialog when clicked, so callers can handle it.
                html!("div" => web_sys::HtmlElement, {
                    .with_node!(backdrop => {
                        .style("position", "fixed")
                        .style("inset", "0")
                        .style("background-color", "#d8e7facc")
                        .event(move |_: dominator::events::Click| {
                            if let Some(sibling) = backdrop.next_element_sibling() {
                                let _ = sibling.dispatch_event(
                                    &web_sys::Event::new("cancel").unwrap()
                                );
                            }
                        })
                    })
                }),
                // Dialog content
                html!("dialog" => web_sys::HtmlDialogElement, {
                    .after_inserted(|dialog: web_sys::HtmlDialogElement| {
                        let _ = dialog.show();
                    })
                    .style("border", "none")
                    .style("padding", "0")
                    .style("border-radius", "16px")
                    .style("box-shadow", "0 3px 6px 0 rgba(0, 0, 0, 0.16)")
                    .style("position", "fixed")
                    .style("top", "50%")
                    .style("left", "50%")
                    .style("transform", "translate(-50%, -50%)")
                    .style("z-index", "1")
                    $($methods)*
                }),
            ])
        })
    }};
}

#[macro_export]
macro_rules! gap {
    ($size:expr) => {
        {
            const SIZE_U32: u32 = $size; // needed to assert type
            const SIZE_STR: &str = $crate::quick_elements::formatcp!("{}px", SIZE_U32);
            html!("div", {
                .style("height", SIZE_STR)
                .style("width", SIZE_STR)
            })
        }
    };
}

#[macro_export]
macro_rules! icon {
    ($icon:expr) => {
        {
            html!("fa-icon", {
                .prop("icon", $icon)
            })
        }
    };
    ($icon:expr, { $($methods:tt)* }) => {
        {
            html!("fa-icon", {
                .prop("icon", $icon)
                $($methods)*
            })
        }
    };
}
