use super::state::*;
use crate::callback_future::CallbackFuture;
use awsm_web::dom::StyleExt;
use futures::future::join_all;
use itertools::Itertools;
use js_sys::Reflect;
use shared::{
    api::endpoints::{self},
    domain::module::{
        body::{
            Body, BodyExt, ModeExt, StepExt,
            _groups::cards::{CardContent, CardPair},
        },
        LiteModule, ModuleBody, ModuleCreatePath, ModuleCreateRequest, ModuleKind,
    },
    media::PngImageFile,
};
use utils::{
    iframe::{IframeAction, IframeMessageExt, ModuleToJigEditorMessage},
    js_wrappers::set_event_listener_once,
    path::image_lib_url,
    prelude::*,
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlIFrameElement, Window};

impl PostPreview {
    pub fn next(&self) {
        let msg = IframeAction::new(ModuleToJigEditorMessage::Next);

        if msg.try_post_message_to_editor().is_err() {
            log::info!("Couldn't post message to top... redirect!");

            let route: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                *self.asset_id.unwrap_jig(), // TODO: handle all types of assets
                JigEditRoute::Landing,
            )))
            .into();
            dominator::routing::go_to_url(&route);
        }
    }

    pub fn publish(&self) {
        let msg = IframeAction::new(ModuleToJigEditorMessage::Publish);

        if msg.try_post_message_to_editor().is_err() {
            log::info!("Couldn't post message to top... redirect!");

            let route: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                *self.asset_id.unwrap_jig(), // TODO: handle all types of assets
                JigEditRoute::Landing,
            )))
            .into();
            dominator::routing::go_to_url(&route);
        }
    }

    pub fn duplicate_module<RawData, Mode, Step>(&self, target_kind: ModuleKind, raw_data: RawData)
    where
        RawData: BodyExt<Mode, Step> + 'static,
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
    {
        let target_body = raw_data.convert_to_body(target_kind).unwrap_ji();

        let req = ModuleCreateRequest {
            body: target_body,
            parent_id: self.asset_id.into(),
        };

        let asset_id = self.asset_id;

        self.loader.load(async move {
            let res = endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await;

            match res {
                Ok(res) => {
                    let module_id = res.id;

                    let module = LiteModule {
                        id: module_id,
                        kind: target_kind,
                        is_complete: raw_data.is_complete(),
                    };

                    let msg = IframeAction::new(ModuleToJigEditorMessage::AppendModule(module));

                    if msg.try_post_message_to_editor().is_err() {
                        log::info!("Couldn't post message to parent... redirect!");
                        let route: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                            *(asset_id.unwrap_jig()), // TODO: handle all types of assets
                            JigEditRoute::Module(module_id),
                        )))
                        .into();
                        dominator::routing::go_to_url(&route);
                    }
                }
                Err(_) => {
                    log::error!("request to create module failed!");
                }
            }
        });
    }

    pub fn print_cards<RawData, Mode, Step>(&self, raw_data: &RawData) -> anyhow::Result<()>
    where
        RawData: BodyExt<Mode, Step> + 'static,
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
    {
        let html = get_html_to_print(raw_data.as_body())?;

        let custom_elements_script = web_sys::window()
            .unwrap_ji()
            .document()
            .unwrap_ji()
            .query_selector("script[src$='/custom-elements.js']")
            .unwrap_ji()
            .unwrap_ji();

        let custom_elements_src = Reflect::get(&custom_elements_script, &JsValue::from_str("src"))
            .unwrap_ji()
            .as_string()
            .unwrap_ji();

        let scripts = vec![custom_elements_src];

        print(html, scripts);

        Ok(())
    }
}

const EL_NAME_LIST: &str = "module-card-print-list";
const EL_NAME_SINGLE: &str = "module-card-print-single";
const EL_NAME_DOUBLE: &str = "module-card-print-double";
fn get_html_to_print(body: Body) -> anyhow::Result<String> {
    let children = match body {
        ModuleBody::MemoryGame(_) | ModuleBody::Matching(_) | ModuleBody::CardQuiz(_) => {
            let pairs = get_card_pairs(body)?;
            cards_to_elements_singles(pairs)
        }
        ModuleBody::Flashcards(_) => {
            let pairs = get_card_pairs(body)?;
            cards_to_elements_doubles(pairs)
        }
        _ => {
            return Err(anyhow::anyhow!("Not a card game"));
        }
    };
    Ok(format!("<{EL_NAME_LIST}>{children}</{EL_NAME_LIST}>"))
}

fn get_card_pairs(body: Body) -> anyhow::Result<Vec<CardPair>> {
    match body {
        ModuleBody::MemoryGame(memory) => Ok(memory.content.unwrap_or_default().base.pairs),
        ModuleBody::Flashcards(flashcards) => Ok(flashcards.content.unwrap_or_default().base.pairs),
        ModuleBody::Matching(matching) => Ok(matching.content.unwrap_or_default().base.pairs),
        ModuleBody::CardQuiz(card_quiz) => Ok(card_quiz.content.unwrap_or_default().base.pairs),
        _ => Err(anyhow::anyhow!("Not a card game")),
    }
}

fn cards_to_elements_singles(cards: Vec<CardPair>) -> String {
    cards
        .into_iter()
        .map(|card| [card.0.card_content, card.1.card_content])
        .flatten()
        .map(|card_content| {
            format!(
                "<{EL_NAME_SINGLE} {attributes}></{EL_NAME_SINGLE}>",
                attributes = get_attributes_for_card_elements(&card_content, "")
            )
        })
        .collect_vec()
        .join("")
}

fn cards_to_elements_doubles(cards: Vec<CardPair>) -> String {
    cards
        .iter()
        .map(|card| {
            format!(
                "<{EL_NAME_DOUBLE} {attributes_0} {attributes_1}></{EL_NAME_DOUBLE}>",
                attributes_0 = get_attributes_for_card_elements(&card.0.card_content, "A"),
                attributes_1 = get_attributes_for_card_elements(&card.1.card_content, "B")
            )
        })
        .collect_vec()
        .join("")
}

fn get_attributes_for_card_elements(card: &CardContent, attr_postfix: &str) -> String {
    match &card {
        CardContent::Text(text) => {
            format!("card{attr_postfix}='{text}' kind{attr_postfix}='text'")
        }
        CardContent::Image(image) => {
            let url = image
                .as_ref()
                .map(|image| image_lib_url(image.lib, PngImageFile::Resized, image.id))
                .unwrap_or_default();
            format!("card{attr_postfix}='{url}' kind{attr_postfix}='image'")
        }
    }
}

fn print(html: String, scripts: Vec<String>) {
    spawn_local(async move {
        let document = web_sys::window().unwrap_ji().document().unwrap_ji();
        let body = document.body().unwrap_ji();

        let iframe: HtmlIFrameElement = document
            .create_element("iframe")
            .unwrap_ji()
            .dyn_into()
            .unwrap_ji();

        iframe.set_style("height", "0");
        iframe.set_style("width", "0");
        iframe.set_style("border", "0");

        let _ = body.append_child(&iframe);

        let iframe_window = iframe.content_window().unwrap_ji();

        let iframe_body = iframe_window.document().unwrap_ji().body().unwrap_ji();

        // Can't use dominator dominator::append_dom since the html! macro uses document.createElement but `document` has to be the iframe document
        iframe_body.set_inner_html(&html);

        let scripts = scripts
            .into_iter()
            .map(|script| add_script(iframe_window.clone(), script));
        join_all(scripts).await;

        // give css a chance to load, with simple tests 1 second was enough
        // querying the document for the css link and listening to it's load event might make more sense
        gloo_timers::future::TimeoutFuture::new(1_000).await;

        set_event_listener_once(
            &iframe_window,
            "afterprint",
            Box::new(move |_: web_sys::Event| {
                let _ = body.remove_child(&iframe);
            }),
        );

        let _ = iframe_window.print();
    });
}

fn add_script(window: Window, src: String) -> CallbackFuture<()> {
    CallbackFuture::new(Box::new(move |resolve| {
        let document = window.document().unwrap_ji();
        let body = window.document().unwrap_ji().body().unwrap_ji();

        let script = document.create_element("script").unwrap_ji();

        // using reflect rather then `dyn_into` `HtmlScriptElement` because dyn_into expects the script to be on the same document
        // but this script element is on the iframes document
        let _ = Reflect::set(
            &script,
            &JsValue::from_str("type"),
            &JsValue::from_str("module"),
        );
        let _ = Reflect::set(&script, &JsValue::from_str("src"), &JsValue::from_str(&src));

        let callback: Box<dyn FnOnce(_)> = Box::new(move |_: web_sys::Event| {
            resolve(());
        });

        set_event_listener_once(&script, "load", callback);
        let _ = body.append_child(&script);
    }))
}
