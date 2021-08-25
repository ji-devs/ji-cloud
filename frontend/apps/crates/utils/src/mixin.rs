use dominator::DomBuilder;
use web_sys::HtmlElement;

pub type MixinStub = fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>;
