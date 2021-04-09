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
    image_search::{self, state::ImageSearchOptions},
    audio_input::{self, options::AudioInputOptions, state::State as AudioState},
    color_select::{self, state::ColorSelectConfig},
    text_editor_controls,
};
use shared::domain::audio::AudioId;
use std::pin::Pin;
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTemplateElement, DocumentFragment, Document};
use utils::{prelude::*, themes::ThemeId};

pub struct Page { }

impl Page {
    pub fn render() -> Dom {
        render_audio_input()
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

pub fn render_image_search() -> Dom {
    let opts = ImageSearchOptions {
            background_only: Some(false),
            upload: Some(()),
            filters: Some(()),
            value: Mutable::new(None),
    };

    html!("div", {
        .style("padding", "30px")
        .child(image_search::dom::render(opts, None))
    })
}


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
    html!("div", {
        .style("padding", "30px")
        .child(color_select::dom::render(ColorSelectConfig {
            theme: Some(ThemeId::HappyBrush),
            // theme: None,
            value: Rc::new(Mutable::new(None))
        }, None))
    })
}


pub fn render_text_editor_controls() -> Dom {
    let state = text_editor_controls::state::State::new();
    html!("div", {
        .style("padding", "10px")
        .style("width", "492px")
        .child(html!("dl", {
            .children(&mut [
                html!("dt", {.text("Font")}),
                html!("dd", {.text_signal(state.font.signal_cloned().map(|font| font.to_string()))}),

                html!("dt", {.text("Element")}),
                html!("dd", {.text_signal(state.element.signal_cloned().map(|element| element.to_string()))}),

                html!("dt", {.text("Weight")}),
                html!("dd", {.text_signal(state.weight.signal_cloned().map(|weight| weight.to_string()))}),

                html!("dt", {.text("Align")}),
                html!("dd", {.text_signal(state.align.signal_cloned().map(|align| format!("{:?}", align)))}),

                html!("dt", {.text("Font size")}),
                html!("dd", {.text_signal(state.font_size.signal_cloned().map(|font_size| font_size.to_string()))}),

                html!("dt", {.text("Highlight color")}),
                html!("dd", {.text_signal(state.highlight_color.signal_cloned().map(|highlight_color| format!("{:?}", highlight_color)))}),

                html!("dt", {.text("Color")}),
                html!("dd", {.text_signal(state.color.signal_cloned().map(|color| format!("{:?}", color)))}),

                html!("dt", {.text("Bold")}),
                html!("dd", {.text_signal(state.bold.signal_cloned().map(|bold| bold.to_string()))}),

                html!("dt", {.text("Italic")}),
                html!("dd", {.text_signal(state.italic.signal_cloned().map(|italic| italic.to_string()))}),

                html!("dt", {.text("Underline")}),
                html!("dd", {.text_signal(state.underline.signal_cloned().map(|underline| underline.to_string()))}),
            ])
        }))
        .child(html!("br"))
        .child(html!("br"))
        .child(html!("br"))
        .child(text_editor_controls::dom::render(
            state.clone()
        ))
    })
}
