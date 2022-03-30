use dominator::{clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;

use super::{actions, state::*};

use utils::events;

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Deleting the category \"{category}\" will also remove it from any images or JIGs associated with it. Are you sure you want to delete this category?";
const STR_DELETE_CONFIRM: &str = "Delete category";
const STR_DELETE_CANCEL: &str = "Don't delete";

pub struct CategoriesPage {}

impl CategoriesPage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        actions::load_categories(state.clone());

        html!("empty-fragment", {
            .child_signal(state.deleting.signal_cloned().map(clone!(state => move |deleting| {
                deleting.map(|content_state| html!("modal-confirm", {
                        .property("dangerous", true)
                        .property("title", STR_DELETE_TITLE)
                        .property("content", STR_DELETE_CONTENT.replace("{category}", &content_state.cat.name.get_cloned()))
                        .property("cancel_text", STR_DELETE_CANCEL)
                        .property("confirm_text", STR_DELETE_CONFIRM)
                        .event(clone!(state => move |_evt: events::CustomCancel| state.deleting.set(None)))
                        .event(clone!(state => move |_evt: events::CustomConfirm| {
                            state.deleting.set(None);
                            actions::delete_category(content_state.clone());
                        }))
                    }))
            })))
            .child(
                html!("category-page", {
                    .children(&mut [
                        html!("button-expand", {
                            .property("slot", "expand")
                            .property("expanded", false)
                            .event(clone!(state => move |evt:events::CustomToggle| {
                                let flag = evt.value();
                                for cat in state.categories.lock_ref().iter() {
                                    actions::toggle_expand_all(cat, flag);
                                }
                            }))
                        }),
                        html!("category-button-add", {
                            .property("slot", "add")
                            .event(clone!(state => move |_evt:events::Click| {
                                actions::add_category_root(state.clone());
                            }))
                        }),
                        html!("div", {
                            .property("slot", "middle")
                            .children_signal_vec(state.categories.signal_vec_cloned().map(clone!(state => move |category| {
                                CategoryDom::render(None, category, state.clone())
                            })))
                        }),

                    ])
                })
            )
            .child(
                html!("window-loader-block", {
                    .property_signal("visible", state.loader.is_loading())
                })
            )

        })
    }
}

pub struct CategoryDom {}

impl CategoryDom {
    pub fn render(parent: Option<Rc<Category>>, cat: Rc<Category>, state: Rc<State>) -> Dom {
        html!("dropdown-tree", {
            .property_signal("label", cat.name.signal_cloned())
            .property_signal("expanded", cat.expanded.signal())
            .property_signal("hasChildren", cat.has_children_signal())
            .property("isChild", parent.is_some())
            .event(clone!(cat => move |_evt:events::ExpandAll| {
                actions::toggle_expand_all(&cat, true)
            }))
            .event(clone!(cat => move |_evt:events::CollapseAll| {
                actions::toggle_expand_all(&cat, false)
            }))
            .event(|_evt:events::Change| {
                log::info!("CHANGE EVENT");
            })
            .child(ContentDom::render(parent, cat.clone(), state.clone()))
            .child(html!("div", {
                .property("slot", "children")
                .children_signal_vec(cat.children.signal_vec_cloned().map(clone!(state, cat => move |category| {
                    CategoryDom::render(Some(cat.clone()), category, state.clone())
                })))
            }))
        })
    }
}

pub struct ContentDom {}

impl ContentDom {
    pub fn render(parent: Option<Rc<Category>>, cat: Rc<Category>, state: Rc<State>) -> Dom {
        let content_state = Rc::new(ContentState::new(parent, cat, state));

        let _visible_signal = Mutable::new(false);
        html!("menu-ellipses", {
            .property("slot", "content")
            .children(ContentLineDom::render(content_state.clone()))
            .child(MenuDom::render(content_state.clone()))

            .after_inserted(clone!(content_state => move |elem| {
                *content_state.menu_ref.borrow_mut() = Some(elem);
            }))
        })
    }
}

pub struct ContentLineDom {}

impl ContentLineDom {
    pub fn render(content_state: Rc<ContentState>) -> Vec<Dom> {
        let mut children: Vec<Dom> = vec![html!("input-text-content", {
            .property("slot", "content")
            .property_signal("editing", content_state.cat.editing.signal_cloned())
            .property_signal("value", content_state.cat.name.signal_cloned())
            .event(clone!(content_state => move |evt: events::CustomChange| {
                actions::rename_category(&content_state.cat, content_state.state.clone(), evt.value());
            }))
        })];

        if content_state.parent.is_none() {
            children.push(html!("button-expand", {
                .property("slot", "content")
                .property("expanded", false)
                .event(clone!(content_state => move |evt: events::CustomToggle| {
                    actions::toggle_expand_all(&content_state.cat, evt.value());
                }))
            }));
        }

        children
    }
}

pub struct MenuDom {}

impl MenuDom {
    pub fn render(content_state: Rc<ContentState>) -> Dom {
        html!("div", {
            .property("slot", "menu-content")
            .children(&mut [
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("Add")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        actions::add_category_child(content_state.clone());
                    }))
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("Move up")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        actions::move_category(content_state.clone(), actions::Direction::Up);
                    }))
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("Move down")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        actions::move_category(content_state.clone(), actions::Direction::Down);
                    }))
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("Rename")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        content_state.cat.editing.set(true);
                        content_state.close_menu();

                    }))
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "red")
                    .property("hoverColor", "red")
                    .text("Delete")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        content_state.close_menu();
                        content_state.state.deleting.set(Some(content_state.clone()));
                    }))
                }),
            ])
        })
    }
}
