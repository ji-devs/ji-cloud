use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::module::body::{Sprite, Transform};
use super::state::*;
use components::{
    transform::dom::TransformDom,
    text_editor::dom::render_wysiwyg,
};


pub struct TextDom {}
impl TextDom {
    pub fn render(state:Rc<State>, index: ReadOnlyMutable<Option<usize>>, text: Rc<Text>) -> Dom {


        text.transform.size.set(Some((300.0, 300.0)));
        //sticker.transform.lock_mut().scale.0 = [0.5, 0.5, 0.5];
        html!("empty-fragment", {
            .children_signal_vec(
                state.renderables.selected_signal(index.clone()).map(clone!(state, text=> move |selected| {
                    let mut children:Vec<Dom> = Vec::new();
                  
                    //just for local testing
                    let selected = false;

                    let mock = true;

                    if selected {
                        children.push(
                            html!("div", {
                                .style("display", "block")
                                .style("border", "green dashed 1px")
                                .style("box-sizing", "border-box")
                                .style("align-self", "baseline")
                                .style("position", "absolute")
                                .style_signal("transform", text.transform.rotation_matrix_string_signal())
                                .style_signal("top", text.transform.y_px_signal().map(|x| format!("{}px", x)))
                                .style_signal("left", text.transform.x_px_signal().map(|x| format!("{}px", x)))
                                .style_signal("width", text.transform.width_px_signal().map(|x| format!("{}px", x)))
                                .style_signal("height", text.transform.height_px_signal().map(|x| format!("{}px", x)))
                                .child(render_wysiwyg(state.text_editor.clone()))
                            })
                        );
                        children.push(TransformDom::render(text.transform.clone()));
                    } else {
                        children.push(
                            if mock {
                                html!("div", {
                                    .text("Hello World!!!")
                                    .style_signal("transform", text.transform.rotation_matrix_string_signal())
                                    .style("display", "block")
                                    .style("position", "absolute")
                                    .style("background-color", "red")
                                    .style("text-align", "center")
                                    .style_signal("top", text.transform.y_px_signal().map(|x| format!("{}px", x)))
                                    .style_signal("left", text.transform.x_px_signal().map(|x| format!("{}px", x)))
                                    .style_signal("width", text.transform.width_px_signal().map(|x| format!("{}px", x)))
                                    .style_signal("height", text.transform.height_px_signal().map(|x| format!("{}px", x)))
                                })
                            } else {
                                html!("wysiwyg-output-renderer", {
                                    .property_signal("valueAsString", text.value.signal_cloned())
                                    .style("position", "absolute")
                                    .style_signal("transform", text.transform.rotation_matrix_string_signal())
                                    .style_signal("top", text.transform.y_px_signal().map(|x| format!("{}px", x)))
                                    .style_signal("left", text.transform.x_px_signal().map(|x| format!("{}px", x)))
                                    .style_signal("width", text.transform.width_px_signal().map(|x| format!("{}px", x)))
                                    .style_signal("height", text.transform.height_px_signal().map(|x| format!("{}px", x)))
                                    //replace this with text measure?
                                    .event(clone!(text=> move |evt:events::ImageLoad| {
                                        text.transform.size.set(Some(evt.size()));
                                    }))
                                })
                            }
                        );
                       
                        //just for testing
                        children.push(TransformDom::render(text.transform.clone()));
                    }

                    children
                }))
                .to_signal_vec()
            )
        })
    }
}
