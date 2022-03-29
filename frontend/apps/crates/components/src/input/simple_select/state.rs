use std::rc::Rc;

use futures_signals::signal::Mutable;

use super::SimpleSelectItem;

pub struct SimpleSelect<T: SimpleSelectItem, P, L> {
    pub(super) label: Option<L>,
    pub(super) placeholder: Option<P>,
    pub(super) value: Mutable<Option<T>>,
    pub(super) values: Vec<T>,
    pub(super) on_change: Option<Box<dyn Fn(Option<T>)>>,
}

impl<T: SimpleSelectItem + 'static, P, L> SimpleSelect<T, P, L> {
    pub fn new(
        label: Option<L>,
        placeholder: Option<P>,
        init_value: Option<T>,
        values: Vec<T>,
        on_change: impl Fn(Option<T>) + 'static,
    ) -> Rc<Self> {
        Self::_new(label, placeholder, init_value, values, Some(on_change))
    }

    pub fn new_no_handler(
        label: Option<L>,
        placeholder: Option<P>,
        init_value: Option<T>,
        values: Vec<T>,
    ) -> Rc<Self> {
        Self::_new(
            label,
            placeholder,
            init_value,
            values,
            None::<fn(Option<_>)>,
        )
    }

    fn _new(
        label: Option<L>,
        placeholder: Option<P>,
        init_value: Option<T>,
        values: Vec<T>,
        on_change: Option<impl Fn(Option<T>) + 'static>,
    ) -> Rc<Self> {
        Rc::new(Self {
            label,
            placeholder,
            value: Mutable::new(init_value),
            values,
            on_change: on_change.map(|f| Box::new(f) as Box<_>),
        })
    }
}

impl<T: SimpleSelectItem, P, L> SimpleSelect<T, P, L> {
    pub fn get_value(&self) -> Option<T> {
        self.value.get_cloned()
    }
}
