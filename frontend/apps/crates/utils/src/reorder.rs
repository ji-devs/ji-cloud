use dominator::{clone, html, Dom, DomBuilder};
use gloo::{
    events::{EventListener, EventListenerOptions, EventListenerPhase},
    render::{request_animation_frame, AnimationFrame},
};
use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent, PointerEvent, ShadowRoot};

pub struct Reorder {
    rows: RefCell<Vec<Weak<ReorderRow>>>,
    gesture: RefCell<Option<Gesture>>,
    listeners: RefCell<Vec<EventListener>>,
    frame: RefCell<Option<AnimationFrame>>,
    on_move: Box<dyn Fn(usize, usize)>,
    suppress_click: Cell<bool>,
    disabled: Cell<bool>,
}

pub struct ReorderRow {
    owner: Weak<Reorder>,
    element: RefCell<Option<HtmlElement>>,
    current_index: Box<dyn Fn() -> Option<usize>>,
    can_drag: Box<dyn Fn() -> bool>,
}

struct Gesture {
    source: Rc<ReorderRow>,
    capture: HtmlElement,
    pointer: i32,
    origin: (f64, f64),
    point: (f64, f64),
    order: Vec<(usize, usize)>,
    phase: Phase,
    scrollport: Element,
}

enum Phase {
    Pending,
    Dragging(HtmlElement),
}

impl Reorder {
    pub fn new(on_move: impl Fn(usize, usize) + 'static) -> Rc<Self> {
        Rc::new(Self {
            rows: RefCell::new(Vec::new()),
            gesture: RefCell::new(None),
            listeners: RefCell::new(Vec::new()),
            frame: RefCell::new(None),
            on_move: Box::new(on_move),
            suppress_click: Cell::new(false),
            disabled: Cell::new(false),
        })
    }

    pub fn row(
        self: &Rc<Self>,
        current_index: impl Fn() -> Option<usize> + 'static,
        can_drag: impl Fn() -> bool + 'static,
    ) -> Rc<ReorderRow> {
        let row = Rc::new(ReorderRow {
            owner: Rc::downgrade(self),
            element: RefCell::new(None),
            current_index: Box::new(current_index),
            can_drag: Box::new(can_drag),
        });
        self.rows.borrow_mut().push(Rc::downgrade(&row));
        row
    }

    pub fn list(
        self: &Rc<Self>,
    ) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        let state = Rc::clone(self);
        move |dom| {
            dom.after_inserted(clone!(state => move |_| state.listen()))
                .after_removed(move |_| {
                    state.cancel();
                    state.listeners.borrow_mut().clear();
                })
        }
    }

    fn listen(self: &Rc<Self>) {
        let window = web_sys::window().unwrap();
        let mut listeners = self.listeners.borrow_mut();
        for name in [
            "pointermove",
            "pointerup",
            "pointercancel",
            "lostpointercapture",
        ] {
            let weak = Rc::downgrade(self);
            listeners.push(EventListener::new_with_options(
                &window,
                name,
                listener_options(),
                move |event| {
                    let Some(state) = weak.upgrade() else { return };
                    let Some(event) = event.dyn_ref::<PointerEvent>() else {
                        return;
                    };
                    if !state
                        .gesture
                        .borrow()
                        .as_ref()
                        .map_or(false, |g| g.pointer == event.pointer_id())
                    {
                        return;
                    }
                    match name {
                        "pointermove" => {
                            event.prevent_default();
                            state.update(event.client_x() as f64, event.client_y() as f64);
                        }
                        "pointerup" => {
                            state.update(event.client_x() as f64, event.client_y() as f64);
                            let destination = state.gesture.borrow().as_ref().and_then(|g| {
                                if matches!(g.phase, Phase::Dragging(_)) {
                                    state.destination(g).map(|(from, to, _, _, _)| (from, to))
                                } else {
                                    None
                                }
                            });
                            state.cancel();
                            if let Some((from, to)) = destination {
                                if from != to {
                                    (state.on_move)(from, to);
                                }
                            }
                        }
                        _ => state.cancel(),
                    }
                },
            ));
        }
        let weak = Rc::downgrade(self);
        listeners.push(EventListener::new(&window, "blur", move |_| {
            if let Some(state) = weak.upgrade() {
                state.cancel();
            }
        }));
        let weak = Rc::downgrade(self);
        listeners.push(EventListener::new(&window, "keydown", move |event| {
            if event
                .dyn_ref::<KeyboardEvent>()
                .map_or(false, |event| event.key() == "Escape")
            {
                if let Some(state) = weak.upgrade() {
                    state.cancel();
                }
            }
        }));
        let weak = Rc::downgrade(self);
        listeners.push(EventListener::new_with_options(
            &window,
            "click",
            listener_options(),
            move |event| {
                if let Some(state) = weak.upgrade() {
                    if event
                        .dyn_ref::<MouseEvent>()
                        .map_or(false, |event| event.detail() != 0)
                        && state.suppress_click.replace(false)
                    {
                        event.prevent_default();
                        event.stop_immediate_propagation();
                    }
                }
            },
        ));
        let weak = Rc::downgrade(self);
        listeners.push(EventListener::new_with_options(
            &window,
            "pointerdown",
            listener_options(),
            move |_| {
                if let Some(state) = weak.upgrade() {
                    state.suppress_click.set(false);
                }
            },
        ));
    }

    fn registered(&self) -> Vec<(usize, Rc<ReorderRow>, HtmlElement)> {
        self.rows.borrow_mut().retain(|row| row.strong_count() > 0);
        let mut rows: Vec<_> = self
            .rows
            .borrow()
            .iter()
            .filter_map(|row| {
                let row = row.upgrade()?;
                let index = (row.current_index)()?;
                let element = row.element.borrow().clone()?;
                if !element.is_connected() {
                    return None;
                }
                Some((index, row, element))
            })
            .collect();
        rows.sort_by_key(|(index, _, _)| *index);
        rows
    }

    fn order(&self) -> Vec<(usize, usize)> {
        self.registered()
            .iter()
            .map(|(index, row, _)| (Rc::as_ptr(row) as usize, *index))
            .collect()
    }

    pub fn set_disabled(&self, disabled: bool) {
        self.disabled.set(disabled);
        if disabled {
            self.cancel();
        }
    }

    pub fn cancel(&self) {
        self.frame.borrow_mut().take();
        let gesture = self.gesture.borrow_mut().take();
        if let Some(gesture) = gesture {
            if let Phase::Dragging(marker) = gesture.phase {
                marker.remove();
                self.suppress_click.set(true);
                if let Some(element) = gesture.source.element.borrow().as_ref() {
                    let _ = element.style().remove_property("opacity");
                }
            }
            let _ = gesture.capture.release_pointer_capture(gesture.pointer);
        }
    }

    fn start(self: &Rc<Self>, row: &Rc<ReorderRow>, capture: HtmlElement, event: &PointerEvent) {
        if self.disabled.get() || !event.is_primary() || event.button() != 0 || !(row.can_drag)() {
            return;
        }
        self.cancel();
        let Some(element) = row.element.borrow().clone() else {
            return;
        };
        if (row.current_index)().is_none() {
            return;
        }
        let point = (event.client_x() as f64, event.client_y() as f64);
        let scrollport = scrollport(&element);
        let _ = capture.set_pointer_capture(event.pointer_id());
        *self.gesture.borrow_mut() = Some(Gesture {
            source: Rc::clone(row),
            capture,
            pointer: event.pointer_id(),
            origin: point,
            point,
            order: self.order(),
            phase: Phase::Pending,
            scrollport,
        });
    }

    fn update(self: &Rc<Self>, x: f64, y: f64) {
        let valid = self
            .gesture
            .borrow()
            .as_ref()
            .map_or(false, |g| g.order == self.order() && (g.source.can_drag)());
        if !valid {
            self.cancel();
            return;
        }
        {
            let mut gesture = self.gesture.borrow_mut();
            let Some(g) = gesture.as_mut() else { return };
            g.point = (x, y);
            if matches!(g.phase, Phase::Pending) && (x - g.origin.0).hypot(y - g.origin.1) >= 6.0 {
                let document = web_sys::window().unwrap().document().unwrap();
                let marker: HtmlElement = document.create_element("div").unwrap().unchecked_into();
                marker.style().set_css_text("position:fixed;height:3px;background:var(--main-blue, #2469d8);pointer-events:none;z-index:2147483647;border-radius:2px");
                let _ = document.body().unwrap().append_child(&marker);
                if let Some(element) = g.source.element.borrow().as_ref() {
                    let _ = element.style().set_property("opacity", "0.45");
                }
                g.phase = Phase::Dragging(marker);
            }
            if let Phase::Dragging(marker) = &g.phase {
                let destination = self.destination(g);
                let _ = marker.style().set_property(
                    "display",
                    if destination.is_some() {
                        "block"
                    } else {
                        "none"
                    },
                );
                if let Some((_, _, left, top, width)) = destination {
                    let _ = marker.style().set_property("left", &format!("{}px", left));
                    let _ = marker.style().set_property("top", &format!("{}px", top));
                    let _ = marker
                        .style()
                        .set_property("width", &format!("{}px", width));
                }
            }
        }
        self.schedule_frame();
    }

    fn destination(&self, g: &Gesture) -> Option<(usize, usize, f64, f64, f64)> {
        if g.order != self.order() || !(g.source.can_drag)() {
            return None;
        }
        let rows = self.registered();
        let from = (g.source.current_index)()?;
        let first = rows.first()?.2.get_bounding_client_rect();
        let last = rows.last()?.2.get_bounding_client_rect();
        let (top, bottom) = viewport(&g.scrollport);
        let left = first.left();
        let right = first.right();
        if g.point.0 < left
            || g.point.0 > right
            || g.point.1 < top.max(first.top())
            || g.point.1 > bottom.min(last.bottom())
        {
            return None;
        }
        let boundary = rows
            .iter()
            .position(|(_, _, element)| {
                let rect = element.get_bounding_client_rect();
                g.point.1 < rect.top() + rect.height() / 2.0
            })
            .unwrap_or(rows.len());
        let source_position = rows.iter().position(|(index, _, _)| *index == from)?;
        let target_position = boundary
            .saturating_sub(usize::from(boundary > source_position))
            .min(rows.len() - 1);
        let to = rows[target_position].0;
        let line = if boundary == rows.len() {
            last.bottom()
        } else {
            rows[boundary].2.get_bounding_client_rect().top()
        };
        Some((from, to, left, line.clamp(top, bottom), right - left))
    }

    fn schedule_frame(self: &Rc<Self>) {
        if self.frame.borrow().is_some()
            || !self
                .gesture
                .borrow()
                .as_ref()
                .map_or(false, |g| matches!(g.phase, Phase::Dragging(_)))
        {
            return;
        }
        let weak = Rc::downgrade(self);
        *self.frame.borrow_mut() = Some(request_animation_frame(move |_| {
            let Some(state) = weak.upgrade() else { return };
            state.frame.borrow_mut().take();
            let point = state.gesture.borrow().as_ref().map(|g| {
                let (top, bottom) = viewport(&g.scrollport);
                let rect = g
                    .source
                    .element
                    .borrow()
                    .as_ref()
                    .map(|element| element.get_bounding_client_rect());
                if rect.map_or(false, |rect| {
                    g.point.0 >= rect.left() && g.point.0 <= rect.right()
                }) && g.point.1 >= top
                    && g.point.1 <= bottom
                {
                    let speed = if g.point.1 < top + 36.0 {
                        -10
                    } else if g.point.1 > bottom - 36.0 {
                        10
                    } else {
                        0
                    };
                    g.scrollport
                        .set_scroll_top(g.scrollport.scroll_top() + speed);
                }
                g.point
            });
            if let Some((x, y)) = point {
                state.update(x, y);
            }
        }));
    }
}

impl ReorderRow {
    pub fn item(
        self: &Rc<Self>,
    ) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        let row = Rc::clone(self);
        move |dom| {
            dom.after_inserted(
                clone!(row => move |element| *row.element.borrow_mut() = Some(element)),
            )
            .after_removed(move |_| {
                if let Some(owner) = row.owner.upgrade() {
                    owner.cancel();
                }
                row.element.borrow_mut().take();
            })
        }
    }

    pub fn grip(self: &Rc<Self>, slot: Option<&str>) -> Dom {
        let row = self;
        html!("button" => HtmlElement, {
            .attr("type", "button")
            .attr("aria-label", "Drag to reorder; use the menu to move with a keyboard")
            .attr("title", "Drag to reorder")
            .apply_if(slot.is_some(), |dom| dom.attr("slot", slot.unwrap()))
            .style("touch-action", "none")
            .style("user-select", "none")
            .style("cursor", "grab")
            .style("border", "0")
            .style("background", "transparent")
            .style("color", "var(--main-blue)")
            .style("font-size", "20px")
            .style("padding", "0 4px")
            .style("min-width", "24px")
            .style("height", "24px")
            .text("⠿")
            .event_with_options(&dominator::EventOptions::preventable(), |event: dominator::events::DragStart| {
                event.prevent_default();
                event.stop_propagation();
            })
            .event_with_options(&dominator::EventOptions::preventable(), |event: dominator::events::Click| {
                event.prevent_default();
                event.stop_propagation();
            })
            .apply(clone!(row => move |dom| {
                let listener = Rc::new(RefCell::new(None));
                dom.after_inserted(clone!(row, listener => move |element: HtmlElement| {
                    let weak = Rc::downgrade(&row);
                    let capture = element.clone();
                    *listener.borrow_mut() = Some(EventListener::new_with_options(&element, "pointerdown", listener_options(), move |event| {
                        event.stop_propagation();
                        let Some(event) = event.dyn_ref::<PointerEvent>() else {
                            return;
                        };

                        if let Some(row) = weak.upgrade() {
                            if let Some(owner) = row.owner.upgrade() {
                                owner.start(&row, capture.clone(), event);
                            }
                        }
                    }));
                })).after_removed(move |_| {
                    listener.borrow_mut().take();
                })
            }))
        })
    }
}

fn listener_options() -> EventListenerOptions {
    EventListenerOptions {
        phase: EventListenerPhase::Capture,
        passive: false,
    }
}

fn composed_parent(element: &Element) -> Option<Element> {
    element
        .assigned_slot()
        .map(Into::into)
        .or_else(|| element.parent_element())
        .or_else(|| {
            element
                .get_root_node()
                .dyn_into::<ShadowRoot>()
                .ok()
                .map(|root| root.host())
        })
}

fn scrollport(element: &Element) -> Element {
    let window = web_sys::window().unwrap();
    let mut parent = composed_parent(element);
    while let Some(element) = parent {
        if let Ok(Some(style)) = window.get_computed_style(&element) {
            let overflow = style.get_property_value("overflow-y").unwrap_or_default();
            if matches!(overflow.as_str(), "auto" | "scroll") {
                return element;
            }
        }
        parent = composed_parent(&element);
    }
    window.document().unwrap().scrolling_element().unwrap()
}

fn viewport(scrollport: &Element) -> (f64, f64) {
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    if window.document().unwrap().scrolling_element().as_ref() == Some(scrollport) {
        return (0.0, height);
    }
    let rect = scrollport.get_bounding_client_rect();
    (rect.top().max(0.0), rect.bottom().min(height))
}
