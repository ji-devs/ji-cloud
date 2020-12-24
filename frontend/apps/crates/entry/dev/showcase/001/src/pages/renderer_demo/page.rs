use crate::prelude::*;
use super::app::*;

pub struct Page {
    module: Rc<ModulePage<PageRenderer, PageLoader, RawData, State>>,
}

pub struct PageRenderer { 
}

pub struct PageLoader { 
    state: RefCell<Option<Rc<State>>>
}

impl Page {
    pub fn render() -> Self {
        let loader = PageLoader{
            state: RefCell::new(None)
        };
        Self {
            module: ModulePage::<PageRenderer, PageLoader, RawData, State>::render(
                PageRenderer{},
                loader       
            ),
            
        }
    }
}

type RawData = ();

struct State {
    app: RefCell<Option<App>>
}

impl State {
    fn new(data:RawData) -> Self {
        Self { 
            app: RefCell::new(None)
        }
    }
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
        let state = Rc::new(State::new(data));
        *self.state.borrow_mut() = Some(state.clone());
        state
    }
}
impl ModuleRenderer<State> for PageRenderer {
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type ChildrenSignal = impl SignalVec<Item = ModuleDom>;


    fn page_kind_signal(state: Rc<State>) -> Self::PageKindSignal {
        always(ModulePageKind::Iframe) 
    }

    fn children_signal(state: Rc<State>, kind:ModulePageKind) -> Self::ChildrenSignal {

        always(vec![
            ModuleDom::Main(Box::new(move |mixin:HtmlMixinPtr| {
                html!("div", {
                    .style("width", "100%")
                    .style("height", "100%")
                    .apply(|dom| mixin(dom))
                    .child(html!("canvas" => HtmlCanvasElement, {
                        .style("width", "100%")
                        .style("height", "100%")
                        .after_inserted(clone!(state => move |canvas| {
                            *state.app.borrow_mut() = Some(App::new(canvas));
                        }))
                    }))
                })
            }))
        ]).to_signal_vec()
    }
}