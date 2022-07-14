use std::{
    pin::Pin,
    task::{Context, Poll},
};

use super::state::*;
use awsm_web::dom::StyleExt;
use futures::{future::join_all, Future};
use js_sys::Reflect;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        jig::JigFocus,
        module::{
            body::{
                Body, BodyExt, ModeExt, StepExt,
                _groups::cards::{CardContent, CardPair},
            },
            LiteModule, ModuleBody, ModuleCreateRequest, ModuleId, ModuleKind,
        },
        CreateResponse,
    },
    error::EmptyError,
};
use utils::{
    iframe::{IframeAction, IframeMessageExt, ModuleToJigEditorMessage},
    prelude::*,
};
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlIFrameElement, Window};

use futures_channel::oneshot;

impl PostPreview {
    pub fn next(&self) {
        let msg = IframeAction::new(ModuleToJigEditorMessage::Next);

        if msg.try_post_message_to_editor().is_err() {
            log::info!("Couldn't post message to top... redirect!");

            let route: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                *self.asset_id.unwrap_jig(), // TODO: handle all types of assets
                JigFocus::Modules,           // only module focused jigs are should be here
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
                JigFocus::Modules,           // only module focused jigs are should be here
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
            let res = api_with_auth::<CreateResponse<ModuleId>, EmptyError, ModuleCreateRequest>(
                endpoints::module::Create::PATH,
                endpoints::module::Create::METHOD,
                Some(req),
            )
            .await;

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
                            JigFocus::Modules,        // only module focused jigs are should be here
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
        let texts_json = get_card_texts_json(raw_data.as_body())?;

        let kind_str = RawData::kind().as_str();

        let html = format!("<{kind_str}-print cards='{texts_json}'></{kind_str}-print>");

        let root = if cfg!(feature = "local") {
            "http://localhost:4103"
        } else if cfg!(feature = "sandbox") {
           "https://frontend.sandbox.jicloud.org/"
        } else if cfg!(feature = "release") {
           "https://frontend.jicloud.org/"
        } else {
            ""
        };

        let scripts = vec![format!("{root}/module/{kind_str}/edit/custom-elements.js")];

        print(html, scripts);

        Ok(())
    }
}

fn get_card_texts_json(body: Body) -> anyhow::Result<String> {
    match body {
        ModuleBody::MemoryGame(memory) => {
            let pairs = memory.content.unwrap_or_default().base.pairs;
            let texts = cards_to_text_singles(pairs)?;
            let json = serde_json::to_string(&texts).unwrap_ji();
            Ok(json)
        }
        ModuleBody::Flashcards(flashcards) => {
            let pairs = flashcards.content.unwrap_or_default().base.pairs;
            let texts = cards_to_text_doubles(pairs)?;
            let json = serde_json::to_string(&texts).unwrap_ji();
            Ok(json)
        }
        // ModuleBody::Matching(matching) => {
        //     let pairs = matching.content.unwrap_or_default().base.pairs;
        //     let texts = cards_to_text_singles(pairs)?;
        //     let json = serde_json::to_string(&texts).unwrap_ji();
        //     Ok(json)
        // },
        // ModuleBody::CardQuiz(card_quiz) => {
        //     let pairs = card_quiz.content.unwrap_or_default().base.pairs;
        //     let texts = cards_to_text_singles(pairs)?;
        //     let json = serde_json::to_string(&texts).unwrap_ji();
        //     Ok(json)
        // },
        _ => Err(anyhow::anyhow!("Not a card game")),
    }
}

fn cards_to_text_singles(cards: Vec<CardPair>) -> anyhow::Result<Vec<String>> {
    let mut texts = Vec::with_capacity(cards.len() * 2);
    let err = Err(anyhow::anyhow!("Contains image"));
    for pair in cards {
        match pair.0.card_content {
            CardContent::Text(text) => texts.push(text),
            CardContent::Image(_) => {
                return err;
            }
        };
        match pair.1.card_content {
            CardContent::Text(text) => texts.push(text),
            CardContent::Image(_) => {
                return err;
            }
        };
    }
    Ok(texts)
}

fn cards_to_text_doubles(cards: Vec<CardPair>) -> anyhow::Result<Vec<(String, String)>> {
    let mut texts = Vec::with_capacity(cards.len() * 2);
    let err = Err(anyhow::anyhow!("Contains image"));
    for pair in cards {
        let a = match pair.0.card_content {
            CardContent::Text(text) => text,
            CardContent::Image(_) => {
                return err;
            }
        };
        let b = match pair.1.card_content {
            CardContent::Text(text) => text,
            CardContent::Image(_) => {
                return err;
            }
        };
        texts.push((a, b));
    }
    Ok(texts)
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

fn add_script(window: Window, src: String) -> SimpleFuture<()> {
    SimpleFuture::new(Box::new(move |resolve| {
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

// might be worth putting in utils
pub struct SimpleFuture<T>
where
    T: std::fmt::Debug + 'static,
{
    rx: oneshot::Receiver<T>,
}

impl<T> SimpleFuture<T>
where
    T: std::fmt::Debug + 'static,
{
    pub fn new(c: Box<dyn FnOnce(Box<dyn FnOnce(T)>)>) -> Self {
        let (tx, rx) = oneshot::channel();

        (c)(Box::new(move |val| {
            tx.send(val).unwrap_ji();
        }));

        Self { rx }
    }
}

impl<T> Future for SimpleFuture<T>
where
    T: std::fmt::Debug + 'static,
{
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Future::poll(Pin::new(&mut self.rx), cx).map(|t| t.unwrap_ji())
    }
}

// might be worth putting in utils
fn set_event_listener_once<E>(source: &EventTarget, event_name: &str, callback: Box<dyn FnOnce(E)>)
where
    E: FromWasmAbi + 'static,
{
    let closure = Closure::once(callback);
    let _ = source.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());

    closure.forget();
}

// might be worth putting in utils
// fn set_event_listener<E>(source: &EventTarget, event_name: &str, callback: Box<dyn Fn(E)>)
// where
//     E: FromWasmAbi + 'static,
// {
//     let closure = Closure::wrap(callback);
//     let _ = source.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());

//     closure.forget();
// }
