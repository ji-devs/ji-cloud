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
    Pending { grab_offset: (f64, f64) },
    Dragging(Preview),
}

struct Preview {
    marker: HtmlElement,
    rows: Vec<PreviewRow>,
    dragged: DraggedRow,
    target: Option<usize>,
}

struct DraggedRow {
    source: HtmlElement,
    placeholder: HtmlElement,
    grab_offset: (f64, f64),
    styles: Vec<SavedStyle>,
}

struct PreviewRow {
    translate: SavedStyle,
    offset: f64,
}

struct SavedStyle {
    element: HtmlElement,
    property: &'static str,
    value: String,
    priority: String,
}

struct Destination {
    from: usize,
    to: usize,
    source_position: usize,
    target_position: usize,
    shift: f64,
    left: f64,
    top: f64,
    width: f64,
    height: f64,
}

impl SavedStyle {
    fn new(element: HtmlElement, property: &'static str) -> Self {
        let style = element.style();
        Self {
            value: style.get_property_value(property).unwrap_or_default(),
            priority: style.get_property_priority(property),
            element,
            property,
        }
    }

    fn restore(&self) {
        let style = self.element.style();
        if self.value.is_empty() {
            let _ = style.remove_property(self.property);
        } else {
            let _ = style.set_property_with_priority(self.property, &self.value, &self.priority);
        }
    }
}

impl DraggedRow {
    fn new(source: HtmlElement, grab_offset: (f64, f64)) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let rect = source.get_bounding_client_rect();
        let computed = window.get_computed_style(&source).unwrap().unwrap();
        let placeholder: HtmlElement = document.create_element("div").unwrap().unchecked_into();
        placeholder.style().set_css_text("display:block;box-sizing:border-box;visibility:hidden;pointer-events:none;flex-shrink:0");
        let _ = placeholder.set_attribute("aria-hidden", "true");
        let _ = placeholder.set_attribute("inert", "");
        if let Some(slot) = source.get_attribute("slot") {
            let _ = placeholder.set_attribute("slot", &slot);
        }

        for property in ["margin-top", "margin-right", "margin-bottom", "margin-left"] {
            let value = computed.get_property_value(property).unwrap_or_default();
            let _ = placeholder.style().set_property(property, &value);
        }
        let width = format!("{}px", rect.width());
        let height = format!("{}px", rect.height());
        let _ = placeholder.style().set_property("width", &width);
        let _ = placeholder.style().set_property("height", &height);
        source
            .parent_node()
            .unwrap()
            .insert_before(&placeholder, Some(&source))
            .unwrap();

        let properties = [
            ("position", "fixed"),
            ("box-sizing", "border-box"),
            ("width", width.as_str()),
            ("height", height.as_str()),
            ("min-width", "0"),
            ("min-height", "0"),
            ("max-width", "none"),
            ("max-height", "none"),
            ("margin-top", "0"),
            ("margin-right", "0"),
            ("margin-bottom", "0"),
            ("margin-left", "0"),
            ("left", "0"),
            ("top", "0"),
            ("right", "auto"),
            ("bottom", "auto"),
            ("translate", "none"),
            ("transform", "none"),
            ("transition", "none"),
            ("pointer-events", "none"),
            ("opacity", "0.5"),
            ("z-index", "2147483647"),
        ];
        let styles = properties
            .iter()
            .map(|(property, _)| SavedStyle::new(source.clone(), property))
            .collect();
        for (property, value) in properties {
            let _ = source.style().set_property(property, value);
        }

        Self {
            source,
            placeholder,
            grab_offset,
            styles,
        }
    }

    fn update(&self, point: (f64, f64)) {
        let style = self.source.style();
        let _ = style.set_property("left", &format!("{}px", point.0 - self.grab_offset.0));
        let _ = style.set_property("top", &format!("{}px", point.1 - self.grab_offset.1));
    }
}

impl Drop for DraggedRow {
    fn drop(&mut self) {
        for style in &self.styles {
            style.restore();
        }
        self.placeholder.remove();
    }
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
                                    state
                                        .destination(g)
                                        .map(|destination| (destination.from, destination.to))
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
            if let Phase::Dragging(preview) = gesture.phase {
                preview.marker.remove();
                for row in preview.rows {
                    row.translate.restore();
                }
                drop(preview.dragged);
                self.suppress_click.set(true);
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
        let rect = element.get_bounding_client_rect();
        let scrollport = scrollport(&element);
        let _ = capture.set_pointer_capture(event.pointer_id());
        *self.gesture.borrow_mut() = Some(Gesture {
            source: Rc::clone(row),
            capture,
            pointer: event.pointer_id(),
            origin: point,
            point,
            order: self.order(),
            phase: Phase::Pending {
                grab_offset: (point.0 - rect.left(), point.1 - rect.top()),
            },
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
            if let Phase::Pending { grab_offset } = g.phase {
                if (x - g.origin.0).hypot(y - g.origin.1) >= 6.0 {
                    let document = web_sys::window().unwrap().document().unwrap();
                    let source = g.source.element.borrow().clone().unwrap();
                    let marker: HtmlElement =
                        document.create_element("div").unwrap().unchecked_into();
                    marker.style().set_css_text("position:fixed;box-sizing:border-box;margin:0;background:var(--light-blue-1, #f3f7fd);border:2px dashed var(--light-blue-5, #c4d9f5);pointer-events:none;z-index:2147483646;border-radius:12px");
                    let _ = marker.set_attribute("aria-hidden", "true");
                    if let Some(slot) = source.get_attribute("slot") {
                        let _ = marker.set_attribute("slot", &slot);
                    }
                    source
                        .parent_node()
                        .unwrap()
                        .insert_before(&marker, Some(&source))
                        .unwrap();
                    let rows = self
                        .registered()
                        .into_iter()
                        .map(|(_, _, element)| PreviewRow {
                            translate: SavedStyle::new(element, "translate"),
                            offset: 0.0,
                        })
                        .collect();
                    g.phase = Phase::Dragging(Preview {
                        marker,
                        rows,
                        dragged: DraggedRow::new(source, grab_offset),
                        target: None,
                    });
                }
            }
            let destination = self.destination(g);
            if let Phase::Dragging(preview) = &mut g.phase {
                preview.target = destination
                    .as_ref()
                    .map(|destination| destination.target_position);
                preview.dragged.update(g.point);
                for (position, row) in preview.rows.iter_mut().enumerate() {
                    let offset = destination.as_ref().map_or(0.0, |destination| {
                        if position > destination.source_position
                            && position <= destination.target_position
                        {
                            -destination.shift
                        } else if position >= destination.target_position
                            && position < destination.source_position
                        {
                            destination.shift
                        } else {
                            0.0
                        }
                    });
                    if row.offset == offset {
                        continue;
                    }
                    row.offset = offset;
                    if offset == 0.0 {
                        row.translate.restore();
                    } else {
                        let _ = row
                            .translate
                            .element
                            .style()
                            .set_property("translate", &format!("0 {}px", row.offset));
                    }
                }
                let style = preview.marker.style();
                let _ = style.set_property(
                    "display",
                    if destination.is_some() {
                        "block"
                    } else {
                        "none"
                    },
                );
                if let Some(destination) = destination {
                    let (top, bottom) = viewport(&g.scrollport);
                    let _ = style.set_property("left", &format!("{}px", destination.left));
                    let _ = style.set_property("top", &format!("{}px", destination.top));
                    let _ = style.set_property("width", &format!("{}px", destination.width));
                    let _ = style.set_property("height", &format!("{}px", destination.height));
                    let _ = style.set_property(
                        "clip-path",
                        &format!(
                            "inset({}px 0 {}px 0)",
                            (top - destination.top).max(0.0),
                            (destination.top + destination.height - bottom).max(0.0)
                        ),
                    );
                }
            }
        }

        self.schedule_frame();
    }

    fn destination(&self, g: &Gesture) -> Option<Destination> {
        if g.order != self.order() || !(g.source.can_drag)() {
            return None;
        }
        let Phase::Dragging(preview) = &g.phase else {
            return None;
        };
        let rows = self.registered();
        let from = (g.source.current_index)()?;
        let bounds: Vec<_> = rows
            .iter()
            .zip(&preview.rows)
            .map(|((_, _, element), row)| {
                let element = if element == &preview.dragged.source {
                    &preview.dragged.placeholder
                } else {
                    element
                };
                let rect = element.get_bounding_client_rect();
                (
                    rect.left(),
                    rect.top() - row.offset,
                    rect.width(),
                    rect.height(),
                )
            })
            .collect();
        let first = bounds.first()?;
        let last = bounds.last()?;
        let (top, bottom) = viewport(&g.scrollport);
        if g.point.0 < first.0
            || g.point.0 > first.0 + first.2
            || g.point.1 < top.max(first.1)
            || g.point.1 > bottom.min(last.1 + last.3)
        {
            return None;
        }
        let source_position = rows.iter().position(|(index, _, _)| *index == from)?;
        let source = bounds[source_position];
        let gap = |target_position: usize| {
            let target = bounds[target_position];
            if target_position > source_position {
                let shift = bounds[source_position + 1].1 - source.1;
                (target.1 + target.3 - source.3, shift)
            } else if target_position < source_position {
                let previous = bounds[source_position - 1];
                let shift = source.1 + source.3 - previous.1 - previous.3;
                (target.1, shift)
            } else {
                (source.1, 0.0)
            }
        };
        let sticky_target = preview.target.filter(|target| {
            let (gap_top, _) = gap(*target);
            g.point.1 >= gap_top && g.point.1 <= gap_top + source.3
        });
        let target_position = sticky_target.unwrap_or_else(|| {
            bounds
                .iter()
                .zip(&preview.rows)
                .enumerate()
                .filter(|(position, (rect, row))| {
                    *position != source_position && g.point.1 >= rect.1 + row.offset + rect.3 / 2.0
                })
                .count()
        });
        let (gap_top, shift) = gap(target_position);
        Some(Destination {
            from,
            to: rows[target_position].0,
            source_position,
            target_position,
            shift,
            left: source.0,
            top: gap_top,
            width: source.2,
            height: source.3,
        })
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
                let rect = match &g.phase {
                    Phase::Dragging(preview) => {
                        Some(preview.dragged.placeholder.get_bounding_client_rect())
                    }
                    Phase::Pending { .. } => None,
                };
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
