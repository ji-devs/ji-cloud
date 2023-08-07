use futures_signals::signal::{Mutable, Signal};
use shared::domain::{UpdateNonNullable, UpdateNullable};

#[derive(Clone)]
pub struct EditableField<U> {
    inner: U,
}

#[derive(Clone)]
pub struct NonNullable<T> {
    update: Mutable<UpdateNonNullable<T>>,
    value: Mutable<T>,
}

#[derive(Clone)]
pub struct Nullable<T> {
    update: Mutable<UpdateNullable<T>>,
    value: Mutable<Option<T>>,
}

impl<T> Default for Nullable<T> {
    fn default() -> Self {
        Self {
            update: Default::default(),
            value: Default::default(),
        }
    }
}

impl<T> From<T> for EditableField<NonNullable<T>> {
    fn from(value: T) -> Self {
        Self {
            inner: NonNullable {
                update: Mutable::default(),
                value: Mutable::new(value),
            },
        }
    }
}

impl<T: Clone> From<EditableField<NonNullable<T>>> for UpdateNonNullable<T> {
    fn from(value: EditableField<NonNullable<T>>) -> Self {
        value.inner.update.get_cloned()
    }
}

impl<T: Clone> EditableField<NonNullable<T>> {
    pub fn signal(&self) -> impl Signal<Item = T> {
        self.inner.value.signal_cloned()
    }

    pub fn get(&self) -> T {
        self.inner.value.get_cloned()
    }

    pub fn set(&self, value: T) {
        self.inner.value.set(value.clone());
        self.inner.update.set(UpdateNonNullable::Change(value));
    }

    pub fn changed(&self) -> bool {
        let update = self.inner.update.get_cloned();
        !matches!(update, UpdateNonNullable::Keep)
    }

    pub fn changed_signal(&self) -> impl Signal<Item = bool> {
        self.inner
            .update
            .signal_ref(|update| !matches!(update, UpdateNonNullable::Keep))
    }
}

impl<T> From<Option<T>> for EditableField<Nullable<T>> {
    fn from(value: Option<T>) -> Self {
        Self {
            inner: Nullable {
                value: Mutable::new(value),
                ..Default::default()
            },
        }
    }
}

impl<T> Default for EditableField<Nullable<T>> {
    fn default() -> Self {
        Self {
            inner: Nullable {
                update: Mutable::default(),
                value: Mutable::new(None),
            },
        }
    }
}

impl<T: Clone> From<EditableField<Nullable<T>>> for UpdateNullable<T> {
    fn from(value: EditableField<Nullable<T>>) -> Self {
        value.inner.update.get_cloned()
    }
}

impl<T: Clone> EditableField<Nullable<T>> {
    pub fn signal(&self) -> impl Signal<Item = Option<T>> {
        self.inner.value.signal_cloned()
    }

    pub fn get(&self) -> Option<T> {
        self.inner.value.get_cloned()
    }

    pub fn set(&self, value: Option<T>) {
        match value {
            Some(value) => {
                self.inner.value.set(Some(value.clone()));
                self.inner.update.set(UpdateNullable::Change(value));
            }
            None => {
                self.inner.value.set(None);
                self.inner.update.set(UpdateNullable::Unset);
            }
        }
    }

    pub fn changed(&self) -> bool {
        let update = self.inner.update.get_cloned();
        !matches!(update, UpdateNullable::Keep)
    }

    pub fn changed_signal(&self) -> impl Signal<Item = bool> {
        self.inner
            .update
            .signal_ref(|update| !matches!(update, UpdateNullable::Keep))
    }
}
