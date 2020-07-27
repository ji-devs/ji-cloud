use std::rc::Rc;
use shipyard::*;
use crate::utils::templates::TemplateManager;
use super::uniques::DomRoot;

pub type LocalView<'a, T> = NonSendSync<ViewMut<'a, T>>;
pub type LocalViewMut<'a, T> = NonSendSync<ViewMut<'a, T>>;
pub type LocalUniqueView<'a, T> = NonSendSync<UniqueView<'a, T>>;
pub type LocalUniqueViewMut<'a, T> = NonSendSync<UniqueViewMut<'a, T>>;

pub type WorldView<'a> = LocalUniqueView<'a, Rc<World>>;
pub type DocumentView<'a> = LocalUniqueView<'a, web_sys::Document>;
pub type DomRootView<'a> = LocalUniqueView<'a, DomRoot>;
pub type TemplateManagerView<'a> = LocalUniqueView<'a, TemplateManager>;
