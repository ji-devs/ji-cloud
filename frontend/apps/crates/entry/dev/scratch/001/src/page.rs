#![allow(unused_imports)]
#![allow(dead_code)]

use std::rc::Rc;

use dominator::{clone, events, html, Dom};
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::SignalVecExt,
};

use components::{
    color_select,
    image::search::{
        self as image_search,
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions},
    },
    text_editor,
};

use utils::{prelude::*, themes::ThemeId};

pub struct Page {}

impl Page {
    pub fn render() -> Dom {
        render_image_search()
    }
}

struct State {
    pub current_step: Mutable<u32>,
}

impl State {
    pub fn new(initial_step: u32) -> Rc<Self> {
        Rc::new(Self {
            current_step: Mutable::new(initial_step),
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
            render_button(4, "Test", state),
        ])
    })
}

fn render_button(step: u32, label: &str, state: Rc<State>) -> Dom {
    html!("circle-button", {
        .property("text", format!("{}", step))
        .property("label", label)
        .property("slot", format!("btn-{}", step))
        .property_signal("active", state.current_step.signal().map(move |current_step| {
            current_step == step
        }))
        .event(clone!(step, state => move |_evt: events::Click| {
            state.current_step.set(step);
        }))
    })
}

pub fn render_image_search() -> Dom {
    let opts = ImageSearchOptions {
        kind: ImageSearchKind::Background,
        ..ImageSearchOptions::default()
    };
    let callbacks = ImageSearchCallbacks::new(Some(|image| {
        log::info!("{:?}", image);
    }));
    let state = image_search::state::State::new(opts, callbacks);

    html!("div", {
        .style("padding", "30px")
        .child(image_search::dom::render(Rc::new(state), None))
    })
}

// pub fn render_audio_input() -> Dom {
//     let opts:AudioInputOptions = AudioInputOptions {
//         //ummm... this is a lie... I guess... but w/e
//         //in the usual case of supplying a Some the real type is inferred
//         // on_change: None,
//         on_change: Some(Box::new(|h| {
//             log::info!("Audio chagne: {:?}", h);
//         })),
//         audio_id: None,
//     };

//     let state = Rc::new(AudioState::new(opts));

//     html!("div", {
//         .style("padding", "30px")
//         .child(audio::input::dom::render(state.clone(), None))
//         .child(html!("button", {
//             .text("Set to Some")
//             .event(clone!(state => move |_: events::Click| {
//                 let uuid = uuid::Uuid::parse_str("30a326ec-991f-11eb-afad-7717bd5f5028").unwrap_ji();
//                 state.set_audio_id_ext(Some(AudioId(uuid)));
//             }))
//         }))
//         .child(html!("button", {
//             .text("Set to None")
//             .event(clone!(state => move |_: events::Click| {
//                 state.set_audio_id_ext(None);
//             }))
//         }))
//     })
// }

pub fn render_color_select() -> Dom {
    let theme_id = Mutable::new(ThemeId::HappyBrush);
    let state = color_select::state::State::new((*theme_id).clone(), None, None, Some(|_| {}));
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
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| controls.font))}),

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

                html!("dt", {.text("box color")}),
                html!("dd", {.text_signal(state.controls.signal_cloned().map(|controls| format!("{:?}", controls.box_color)))}),

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
        .style("justify-self", "start")
        .child(text_editor::dom::render_wysiwyg(state))
    })
}
pub fn render_wysiwyg_output(value: Rc<Mutable<Option<String>>>, theme: Mutable<ThemeId>) -> Dom {
    html!("div", {
        .style("display", "block")
        .style("border", "red solid 1px")
        .style("box-sizing", "border-box")
        .style("align-self", "baseline")
        .style("justify-self", "start")
        .child(html!("wysiwyg-output-renderer", {
            .property_signal("valueAsString", value.signal_cloned())
            .property_signal("theme", theme.signal_cloned().map(|theme| theme.as_str_id()))
        }))
    })
}

fn render_text() -> Dom {
    let value =
        shared::domain::jig::module::body::_groups::design::Text::value_from_str("text from rust");
    let value = Some(value);
    // let value = None;

    let value_change = Rc::new(Mutable::new(value.clone()));

    let callbacks = text_editor::callbacks::Callbacks::new(
        Some(Box::new(|_v: &str| {})),
        Some(Box::new(clone!(value_change => move |v: &str| {
            value_change.set(Some(v.to_string()));
            // log::info!("{:?}", v);
        }))),
        Some(Box::new(|| {
            log::info!("On blur");
        })),
    );
    let theme = Mutable::new(ThemeId::HappyBrush);
    let state = text_editor::state::State::new((*theme).clone(), value.clone(), callbacks);

    html!("div", {
        .style("display", "grid")
        .style("grid-template-columns", "auto 1fr")
        .style("grid-template-rows", "auto auto")

        .children(&mut [
            render_text_editor_controls(state.clone()),
            render_wysiwyg(state.clone()),
            render_wysiwyg_output(value_change.clone(), theme.clone()),
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
                                        .event(clone!(theme => move |_: events::Change| {
                                            theme.set(ThemeId::HappyBrush);
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
                                        .event(clone!(theme => move |_: events::Change| {
                                            theme.set(ThemeId::Chalkboard)
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
