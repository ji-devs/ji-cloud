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
