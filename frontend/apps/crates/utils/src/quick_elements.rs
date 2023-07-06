pub use const_format::formatcp;

#[macro_export]
macro_rules! dialog {
    ( $($methods:tt)* ) => {{
        use dominator::{pseudo, html, class};
        html!("dialog" => web_sys::HtmlDialogElement, {
            .after_inserted(|dialog: web_sys::HtmlDialogElement| {
                let _ = dialog.show_modal();
            })
            .style("border", "none")
            .style("padding", "0")
            .style("border-radius", "16px")
            .style("box-shadow", "0 3px 6px 0 rgba(0, 0, 0, 0.16)")
            .child(html!("style", {
                .text(r#"
                    dialog::backdrop {
                        background-color: #d8e7fade;
                    }
                "#)
            }))
            // this would add a class to the global stylesheet, and won't work in shadow root.
            // .class(class! {
            //     .pseudo!("::backdrop", {
            //         .style("background-color", "var(--light-blue-3)")
            //     })
            // })
            $($methods)*
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
