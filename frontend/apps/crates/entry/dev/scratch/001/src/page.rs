use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, always},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id};
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use components::{
    // image_search::{self, state::ImageSearchOptions},
    audio_input::{self, options::AudioInputOptions, state::State as AudioState},
    color_select,
    text_editor,
};
use shared::domain::audio::AudioId;
use std::pin::Pin;
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTemplateElement, DocumentFragment, Document};
use utils::{prelude::*, themes::ThemeId};

pub struct Page { }

impl Page {
    pub fn render() -> Dom {
        render_text()
    }
}

struct State {
    pub current_step: Mutable<u32>
}

impl State {
    pub fn new (initial_step:u32) -> Rc<Self> {
        Rc::new(Self {
            current_step: Mutable::new(initial_step)
        })
    }
}

pub fn render_steps() -> Dom {
    let state = State::new(1);

    html!("steps-nav", {
        .property("steps", 4)
        .children(vec![
            render_button(1, "This", state.clone()),
            render_button(2, "Is", state.clone()),
            render_button(3, "A", state.clone()),
            render_button(4, "Test", state.clone()),
        ])
    })
}

fn render_button(step:u32, label:&str, state:Rc<State>) -> Dom {
    html!("circle-button", {
        .property("text", format!("{}", step))
        .property("label", label)
        .property("slot", format!("btn-{}", step))
        .property_signal("active", state.current_step.signal().map(move |current_step| {
            if current_step == step {
                true
            } else {
                false
            }
        }))
        .event(clone!(step, state => move |evt: events::Click| {
            state.current_step.set(step);
        }))
    })
}

// pub fn render_image_search() -> Dom {
//     let opts = ImageSearchOptions {
//             background_only: Some(false),
//             upload: Some(()),
//             filters: Some(()),
//             value: Mutable::new(None),
//     };

//     html!("div", {
//         .style("padding", "30px")
//         .child(image_search::dom::render(opts, None))
//     })
// }


pub fn render_audio_input() -> Dom {
    let opts:AudioInputOptions = AudioInputOptions {
        //ummm... this is a lie... I guess... but w/e
        //in the usual case of supplying a Some the real type is inferred
        // on_change: None,
        on_change: Some(Box::new(|h| {
            log::info!("Audio chagne: {:?}", h);
        })),
        audio_id: None,
    };

    let state = Rc::new(AudioState::new(opts));

    html!("div", {
        .style("padding", "30px")
        .child(audio_input::dom::render(state.clone(), None))
        .child(html!("button", {
            .text("Set to Some")
            .event(clone!(state => move |_: events::Click| {
                let uuid = uuid::Uuid::parse_str("30a326ec-991f-11eb-afad-7717bd5f5028").unwrap_ji();
                state.set_audio_id_ext(Some(AudioId(uuid)));
            }))
        }))
        .child(html!("button", {
            .text("Set to None")
            .event(clone!(state => move |_: events::Click| {
                state.set_audio_id_ext(None);
            }))
        }))
    })
}


pub fn render_color_select() -> Dom {
    let state = color_select::state::State::new(
        Some(ThemeId::HappyBrush),
        Rc::new(Mutable::new(None))
        );
    html!("div", {
        .style("padding", "30px")
        .child(color_select::dom::render(
            Rc::new(state),
            None
        ))
    })
}


pub fn render_text_editor_controls(state: Rc<text_editor::state::State>) -> Dom {
    html!("div", {
        .style("grid-row", "1 / -1")
        .style("padding", "10px")
        .style("width", "492px")
        .child(text_editor::dom::render_controls(state.clone()))
        .child(html!("br"))
        .child(html!("br"))
        .child(html!("br"))
        .child(html!("dl", {
            .children(&mut [
                html!("dt", {.text("Font")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| controls.font.to_string()))}),

                html!("dt", {.text("Element")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| controls.element.to_string()))}),

                html!("dt", {.text("Weight")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| controls.weight.to_string()))}),

                html!("dt", {.text("Align")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| format!("{:?}", controls.align)))}),

                html!("dt", {.text("Font size")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| controls.font_size.to_string()))}),

                html!("dt", {.text("Highlight color")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| format!("{:?}", controls.highlight_color)))}),

                html!("dt", {.text("Color")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| format!("{:?}", controls.color)))}),

                html!("dt", {.text("Italic")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| controls.italic.to_string()))}),

                html!("dt", {.text("Underline")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| controls.underline.to_string()))}),
            ])
        }))
    })
}

pub fn render_wysiwyg(state: Rc<text_editor::state::State>) -> Dom {
    html!("div", {
        .style("display", "block")
        .style("border", "green dashed 1px")
        .style("box-sizing", "border-box")
        .style("align-self", "baseline")
        .child(text_editor::dom::render_wysiwyg(state))
    })
    
}
pub fn render_wysiwyg_output(value: Rc<Mutable<Option<String>>>) -> Dom {
    html!("div", {
        .child(html!("wysiwyg-output-renderer", {
            .style("border", "red solid 1px")
            .style("display", "block")
            .style("box-sizing", "border-box")
            .property_signal("valueAsString", value.signal_cloned())
        }))
    })
}

fn render_text() -> Dom {
    let value = "[{\"children\":[{\"text\":\"text from rust\",\"font\":\"\\\"Shesek - Regular\\\", \\\"Architects Daughter - Regular\\\"\",\"fontSize\":14,\"color\":\"#AFCBF4FF\"}],\"element\":\"P1\"}]".to_string();
    let value = Some(value);
    // let value = None;

    let value_change = Rc::new(Mutable::new(value.clone()));

    let state = text_editor::state::State::new(
        ThemeId::HappyBrush,
        value.clone(),
        Some(Box::new(clone!(value_change => move |v| {
            value_change.set(Some(v.to_string()));
            log::info!("{:?}", v);
        })))
    );

    html!("div", {
        .style("display", "grid")
        .style("grid-template-columns", "auto 1fr")
        .style("grid-template-rows", "auto auto")

        .children(&mut [
            render_text_editor_controls(state.clone()),
            render_wysiwyg(state.clone()),
            render_wysiwyg_output(value_change.clone()),
            html!("div", {
                .style("display", "grid")
                .children(&mut [
            html!("button", {
                .text("Set back to default value")
                .event(clone!(state, value_change, value => move |_: events::Click| {
                    state.set_value(value.clone());
                    value_change.set(value.clone());
                }))
            }),
            html!("button", {
                .text("Select all")
                        .event(clone!(state => move |_: events::Click| {
                    state.select_all();
                }))
                    }),
                    html!("div", {
                        .children(&mut [
                            html!("label", {
                                .text("Happy brush")
                                .child(
                                    html!("input", {
                                        .property("type", "radio")
                                        .property("name", "them")
                                        .property("checked", true)
                                        .event(clone!(state => move |_: events::Change| {
                                            state.set_theme(ThemeId::HappyBrush)
                                        }))
                                    })
                                )
                            }),
                            html!("label", {
                                .text("Chalkboard")
                                .child(
                                    html!("input", {
                                        .property("type", "radio")
                                        .property("name", "them")
                                        .event(clone!(state => move |_: events::Change| {
                                            state.set_theme(ThemeId::Chalkboard)
                                        }))
                                    })
                                )
                            }),
                        ])
                    })
                ])
            })
        ])
    })
}
