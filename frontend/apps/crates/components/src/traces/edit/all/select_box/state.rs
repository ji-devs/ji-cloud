use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};

pub struct SelectBox {
    pub menu_pos: Mutable<Option<(f64, f64)>>,
}

impl SelectBox {
    pub fn new() -> Self {
        Self {
            menu_pos: Mutable::new(None),
        }
    }
    pub fn menu_pos_signal(
        &self,
        active_signal: impl Signal<Item = bool>,
    ) -> impl Signal<Item = Option<(f64, f64)>> {
        map_ref! {
            let active = active_signal,
            let pos = self.menu_pos.signal_cloned()
                => {
                    if !*active {
                        None
                    } else {
                        *pos
                    }
                }
        }
    }
}
