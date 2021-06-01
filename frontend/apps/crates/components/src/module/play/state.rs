use std::rc::Rc;
use shared::{
    api::endpoints::{ApiEndpoint, self, jig::module::*},
    error::{EmptyError, MetadataNotFound},
    domain::jig::{*, module::{*, body::{Body, BodyExt}}},
};
use dominator::{clone, Dom};
use futures_signals::{
    map_ref,
    signal::{self, Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use std::convert::{TryFrom, TryInto};
use std::future::Future;
use dominator_helpers::{
    signals::OptionSignal,
    futures::AsyncLoader,
};
use utils::{settings::SETTINGS, prelude::*, iframe::*};

pub struct GenericState <RawData, Main> 
where
    RawData: BodyExt + 'static,
    Main: MainExt + 'static,
{
    pub(super) phase: Mutable<Rc<Phase<RawData, Main>>>,
    pub(super) opts: StateOpts<RawData>,
    pub(super) raw_loader: AsyncLoader,
    pub(super) page_body_switcher: AsyncLoader,
}

pub trait DomRenderable {
    fn render(state: Rc<Self>) -> Dom;
}

pub trait MainExt: DomRenderable {
}

pub enum Phase <RawData, Main> 
where
    RawData: BodyExt + 'static,
    Main: MainExt + 'static,
{
    Init,
    WaitingIframe(Rc<Box<dyn Fn(RawData)>>),
    Playing(Rc<Main>),
}

impl <RawData, Main> Phase <RawData, Main> 
where
    RawData: BodyExt + 'static,
    Main: MainExt + 'static,
{
    pub fn waiting_iframe(&self) -> bool {
        match self {
            Self::WaitingIframe(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateOpts<RawData> {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub force_raw: Option<RawData>, 
    pub force_raw_even_in_iframe: bool,
}

impl <RawData> StateOpts<RawData> {
    pub fn new(jig_id: JigId, module_id: ModuleId) -> Self {
        Self {
            jig_id,
            module_id,
            force_raw: None,
            force_raw_even_in_iframe: false,
        }
    }
}

impl <RawData, Main> GenericState <RawData, Main> 
where
    RawData: BodyExt + 'static,
    Main: MainExt + 'static,
{
    pub fn new<InitFromRawFn, InitFromRawOutput>(
        opts: StateOpts<RawData>, 
        init_from_raw: InitFromRawFn, 
    ) -> Rc<Self>
    where
        InitFromRawFn: Fn(JigId, ModuleId, RawData) -> InitFromRawOutput + Clone + 'static,
        InitFromRawOutput: Future<Output = Main>,
        <RawData as TryFrom<ModuleBody>>::Error: std::fmt::Debug
    {
        

        let _self = Rc::new(Self {
            opts,
            phase: Mutable::new(Rc::new(Phase::Init)),
            raw_loader: AsyncLoader::new(),
            page_body_switcher: AsyncLoader::new(),
        });

        _self.raw_loader.load(clone!(_self => async move {

            let wait_iframe = should_get_iframe_data();

            let raw:Option<RawData> = _self.opts.force_raw.clone()
                .and_then(|raw| {
                    if !wait_iframe || _self.opts.force_raw_even_in_iframe {
                        Some(raw)
                    } else {
                        None
                    }
                });

            let raw = match raw {
                Some(raw) => Some(raw),
                None => {
                    if wait_iframe {
                        _self.phase.set(Rc::new(Phase::WaitingIframe(
                            Rc::new(Box::new(clone!(init_from_raw, _self => move |raw| {
                                _self.raw_loader.load(clone!(init_from_raw, _self => async move {
                                    let main = init_from_raw(_self.opts.jig_id.clone(), _self.opts.module_id.clone(), raw).await;

                                    _self.phase.set(Rc::new(Phase::Playing(Rc::new(main))));
                                }));
                            })))
                        )));

                        None
                    } else {
                        let path = Get::PATH
                            .replace("{id}",&_self.opts.jig_id.0.to_string())
                            .replace("{module_id}",&_self.opts.module_id.0.to_string());

                        match api_with_auth::<ModuleResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
                            Ok(resp) => {
                                let body = resp.module.body.unwrap_ji();
                                Some(body.try_into().unwrap_ji())
                            },
                            Err(_) => {
                                panic!("error loading module!")
                            }
                        }
                    } 
                }
            };

            if let Some(raw) = raw {
                let main = init_from_raw(_self.opts.jig_id.clone(), _self.opts.module_id.clone(), raw).await;

                _self.phase.set(Rc::new(Phase::Playing(Rc::new(main))));
            }
        }));

        _self
    }
}
