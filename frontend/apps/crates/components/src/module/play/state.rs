use std::rc::Rc;
use std::cell::RefCell;
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
use crate::font_loader::{FontLoader, Font};
use utils::{settings::SETTINGS, prelude::*, iframe::*};
use std::marker::PhantomData;

pub struct GenericState <RawData, RawMode, Base> 
where
    RawData: BodyExt<RawMode> + 'static,
    RawMode: 'static,
    Base: BaseExt + 'static,
{
    pub(super) phase: Mutable<Rc<Phase<RawData, Base>>>,
    pub(super) jig: RefCell<Option<Jig>>,
    pub(super) opts: StateOpts<RawData>,
    pub(super) raw_loader: AsyncLoader,
    pub(super) page_body_switcher: AsyncLoader,
    phantom: PhantomData<RawMode>,
}

pub trait DomRenderable {
    fn render(state: Rc<Self>) -> Dom;
}

pub trait BaseExt: DomRenderable {
}

pub enum Phase <RawData, Base> 
{
    Init,
    WaitingIframe(Rc<Box<dyn Fn(RawData)>>),
    Playing(Rc<Base>),
}

impl <RawData, Base> Phase <RawData, Base> 
{
    pub fn waiting_iframe(&self) -> bool {
        match self {
            Self::WaitingIframe(_) => true,
            _ => false
        }
    }
}

pub enum InitSource {
    ForceRaw,
    Load,
    IframeData
}
#[derive(Debug, Clone)]
pub struct StateOpts<RawData> {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub force_raw: Option<RawData>, 
    pub force_raw_even_in_iframe: bool,
    pub skip_load_jig: bool,
    pub load_fonts: bool,
}

impl <RawData> StateOpts<RawData> {
    pub fn new(jig_id: JigId, module_id: ModuleId) -> Self {
        Self {
            jig_id,
            module_id,
            force_raw: None,
            force_raw_even_in_iframe: false,
            skip_load_jig: false,
            load_fonts: true,
        }
    }
}

impl <RawData, RawMode, Base> GenericState <RawData, RawMode, Base> 
where
    RawData: BodyExt<RawMode> + 'static,
    RawMode: 'static,
    Base: BaseExt + 'static,
{
    pub fn new<InitFromRawFn, InitFromRawOutput>(
        opts: StateOpts<RawData>, 
        init_from_raw: InitFromRawFn, 
    ) -> Rc<Self>
    where
        InitFromRawFn: Fn(JigId, ModuleId, Option<Jig>, RawData, InitSource) -> InitFromRawOutput + Clone + 'static,
        InitFromRawOutput: Future<Output = Base>,
        <RawData as TryFrom<ModuleBody>>::Error: std::fmt::Debug
    {
        

        let _self = Rc::new(Self {
            opts,
            jig: RefCell::new(None),
            phase: Mutable::new(Rc::new(Phase::Init)),
            raw_loader: AsyncLoader::new(),
            page_body_switcher: AsyncLoader::new(),
            phantom: PhantomData
        });

        _self.raw_loader.load(clone!(_self => async move {

            if _self.opts.load_fonts {
                FontLoader::new().load_all().await;
            }

            if !_self.opts.skip_load_jig {
                *_self.jig.borrow_mut() = {

                        let path = endpoints::jig::Get::PATH.replace("{id}",&_self.opts.jig_id.0.to_string());

                        match api_with_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::Get::METHOD, None).await {
                            Ok(resp) => {
                                Some(resp.jig)
                            },
                            Err(_) => {
                                panic!("error loading jig!")
                            },
                        }
                };
            }
            let wait_iframe = should_get_iframe_data();

            let raw:Option<RawData> = _self.opts.force_raw.clone()
                .and_then(|raw| {
                    if !wait_iframe || _self.opts.force_raw_even_in_iframe {
                        Some(raw)
                    } else {
                        None
                    }
                });

            let raw_source = match raw {
                Some(raw) => Some((raw, InitSource::ForceRaw)),
                None => {
                    if wait_iframe {
                        _self.phase.set(Rc::new(Phase::WaitingIframe(
                            Rc::new(Box::new(clone!(init_from_raw, _self => move |raw| {
                                _self.raw_loader.load(clone!(init_from_raw, _self => async move {

                                    let (jig_id, module_id, jig) = (
                                        _self.opts.jig_id.clone(),
                                        _self.opts.module_id.clone(),
                                        _self.jig.borrow().clone()
                                    );
                                    let base = init_from_raw(jig_id, module_id, jig, raw, InitSource::IframeData).await;

                                    _self.phase.set(Rc::new(Phase::Playing(Rc::new(base))));
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
                                Some((body.try_into().unwrap_ji(), InitSource::Load))
                            },
                            Err(_) => {
                                panic!("error loading module!")
                            }
                        }
                    } 
                }
            };

            if let Some((raw, init_source)) = raw_source {

                let (jig_id, module_id, jig) = (
                    _self.opts.jig_id.clone(),
                    _self.opts.module_id.clone(),
                    _self.jig.borrow().clone()
                );
                let base = init_from_raw(jig_id, module_id, jig, raw, init_source).await;

                _self.phase.set(Rc::new(Phase::Playing(Rc::new(base))));
            }
        }));

        _self
    }
}

