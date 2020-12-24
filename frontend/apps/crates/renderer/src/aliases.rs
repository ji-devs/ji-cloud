use beach_map::{BeachMap, DefaultVersion};
use web_sys::HtmlCanvasElement;
use awsm_web::{
    dom::resize::ResizeObserver,
    webgl::WebGl2Renderer
};
use shipyard::*;
use std::rc::Rc;


pub type Gl<'a> = NonSendSync<UniqueView<'a, WebGl2Renderer>>;
pub type GlMut<'a> = NonSendSync<UniqueViewMut<'a, WebGl2Renderer>>;