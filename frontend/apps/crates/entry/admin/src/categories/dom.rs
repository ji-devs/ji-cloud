use dominator::{clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;

use super::{actions, state::*};

use utils::events;

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Deleting the category \"{category}\" will also remove it from any images or JIGs associated with it. Are you sure you want to delete this category?";
const STR_DELETE_CONFIRM: &str = "Yes, delete";
const STR_DELETE_CANCEL: &str = "Don't delete";

pub struct CategoriesPage {}

impl CategoriesPage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        actions::load_categories(state.clone());

        html!("empty-fragment", {
            .child_signal(state.deleting.signal_cloned().map(clone!(state => move |deleting| {
                deleting.map(|content_state| html!("modal-confirm", {
                        .prop("dangerous", true)
                        .prop("title", STR_DELETE_TITLE)
                        .prop("content", STR_DELETE_CONTENT.replace("{category}", &content_state.cat.name.get_cloned()))
                        .prop("cancel_text", STR_DELETE_CANCEL)
                        .prop("confirm_text", STR_DELETE_CONFIRM)
                        .prop("confirmIcon", "core/menus/delete-white.svg")
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
                            .prop("slot", "expand")
                            .prop("expanded", false)
                            .event(clone!(state => move |evt:events::CustomToggle| {
                                let flag = evt.value();
                                for cat in state.categories.lock_ref().iter() {
                                    actions::toggle_expand_all(cat, flag);
                                }
                            }))
                        }),
                        html!("category-button-add", {
                            .prop("slot", "add")
                            .event(clone!(state => move |_evt:events::Click| {
                                actions::add_category_root(state.clone());
                            }))
                        }),
                        html!("div", {
                            .prop("slot", "middle")
                            .children_signal_vec(state.categories.signal_vec_cloned().map(clone!(state => move |category| {
                                CategoryDom::render(None, category, state.clone())
                            })))
                        }),

                    ])
                })
            )
            .child(
                html!("window-loader-block", {
                    .prop_signal("visible", state.loader.is_loading())
                })
            )

        })
    }
}

pub struct CategoryDom {}

impl CategoryDom {
    pub fn render(parent: Option<Rc<Category>>, cat: Rc<Category>, state: Rc<State>) -> Dom {
        html!("dropdown-tree", {
            .prop_signal("label", cat.name.signal_cloned())
            .prop_signal("expanded", cat.expanded.signal())
            .prop_signal("hasChildren", cat.has_children_signal())
            .prop("isChild", parent.is_some())
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
                .prop("slot", "children")
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
            .prop("slot", "content")
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
            .prop("slot", "content")
            .prop_signal("editing", content_state.cat.editing.signal_cloned())
            .prop_signal("value", content_state.cat.name.signal_cloned())
            .event(clone!(content_state => move |evt: events::CustomChange| {
                actions::rename_category(&content_state.cat, content_state.state.clone(), evt.value());
            }))
        })];

        if content_state.parent.is_none() {
            children.push(html!("button-expand", {
                .prop("slot", "content")
                .prop("expanded", false)
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
            .prop("slot", "menu-content")
            .children(&mut [
                html!("button-rect", {
                    .prop("kind", "text")
                    .prop("color", "darkGray")
                    .prop("hoverColor", "blue")
                    .text("Add")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        actions::add_category_child(content_state.clone());
                    }))
                }),
                html!("button-rect", {
                    .prop("kind", "text")
                    .prop("color", "darkGray")
                    .prop("hoverColor", "blue")
                    .text("Move up")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        actions::move_category(content_state.clone(), actions::Direction::Up);
                    }))
                }),
                html!("button-rect", {
                    .prop("kind", "text")
                    .prop("color", "darkGray")
                    .prop("hoverColor", "blue")
                    .text("Move down")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        actions::move_category(content_state.clone(), actions::Direction::Down);
                    }))
                }),
                html!("button-rect", {
                    .prop("kind", "text")
                    .prop("color", "darkGray")
                    .prop("hoverColor", "blue")
                    .text("Rename")
                    .event(clone!(content_state => move |_evt: events::Click| {
                        content_state.cat.editing.set(true);
                        content_state.close_menu();

                    }))
                }),
                html!("button-rect", {
                    .prop("kind", "text")
                    .prop("color", "red")
                    .prop("hoverColor", "red")
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
