use crate::prelude::*;
use super::templates;

const INITIAL_MODE:ModulePageKind = ModulePageKind::GridResize;

pub type Page = Rc<ModulePage<PageRenderer, PageLoader, RawData, State>>;

pub fn render() -> Page {
    ModulePage::<PageRenderer, PageLoader, RawData, State>::render(
        PageRenderer{},
        PageLoader{}
    )
}

pub type RawData = ();

pub struct State {
    pub kind: Mutable<ModulePageKind>
}
impl State {
    fn new(data:RawData) -> Self {
        Self { 
            kind: Mutable::new(INITIAL_MODE) 
        }
    }
}

pub struct PageRenderer { 
}

pub struct PageLoader { 
}
impl StateLoader<RawData, State> for PageLoader {
    type FutureState = impl Future<Output = Option<Rc<State>>>;
    fn load_state(&self) -> Self::FutureState{ 
        let state = self.derive_state(());
        async move {
            Some(state)
        }
    }

    fn derive_state(&self, data:RawData) -> Rc<State> { 
        Rc::new(State::new(data))
    }
}
impl ModuleRenderer<State> for PageRenderer {
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type ChildrenSignal = impl SignalVec<Item = ModuleDom>;


    fn page_kind_signal(state: Rc<State>) -> Self::PageKindSignal {
        state.kind.signal()
    }

    fn children_signal(state: Rc<State>, kind:ModulePageKind) -> Self::ChildrenSignal {
        state.kind
            .signal()
            .map(clone!(state => move |kind| {
                vec![
                    Self::sidebar(state.clone(), kind),
                    Self::header(state.clone(), kind),
                    Self::main(state.clone(), kind),
                    Self::footer(state.clone(), kind),
                ]
                .into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap_ji())
                .collect()
            }))
            .to_signal_vec()
    }
}

impl PageRenderer {
    fn sidebar(state: Rc<State>, kind:ModulePageKind) -> Option<ModuleDom> {
        templates::sidebar(kind).map(move |el| {
            ModuleDom::Sidebar(Box::new(move |mixin:HtmlMixinPtr| {
                elem!(el, {
                    .apply(|dom| mixin(dom))
                    .child(html!("div", {
                        .style("display", "flex")
                        .children(vec![
                            html!("button", {
                                .text("empty")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::Empty);
                                }))
                            }),
                            html!("button", {
                                .text("edit-plain")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::GridPlain);
                                }))
                            }),
                            html!("button", {
                                .text("edit-resize")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::GridResize);
                                }))
                            }),
                            html!("button", {
                                .text("edit-resize-scrollable")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::GridResizeScrollable);
                                }))
                            }),
                            html!("button", {
                                .text("iframe")
                                .event(clone!(state => move |evt:events::Click| {
                                    state.kind.set(ModulePageKind::Iframe);
                                }))
                            }),
                        ])
                    }))
                })
            }))
        })
    }

    fn header(state: Rc<State>, kind: ModulePageKind) -> Option<ModuleDom> { 
        templates::header(kind).map(move |el| {
            ModuleDom::Header(Box::new(move |mixin:HtmlMixinPtr| {
                elem!(el, {
                    .apply(|dom| mixin(dom))
                })
            }))
        })
    }

    fn main(state: Rc<State>, kind: ModulePageKind) -> Option<ModuleDom> { 
        templates::main(kind).map(move |el| {
            ModuleDom::Main(Box::new(move |mixin:HtmlMixinPtr| {
                elem!(el, {
                    .apply(|dom| mixin(dom))
                })
            }))
        })
    }

    fn footer(state: Rc<State>, kind: ModulePageKind) -> Option<ModuleDom> { 
        templates::footer(kind).map(move |el| {
            ModuleDom::Footer(Box::new(move |mixin:HtmlMixinPtr| {
                elem!(el, {
                    .apply(|dom| mixin(dom))
                })
            }))
        })
    }
}
