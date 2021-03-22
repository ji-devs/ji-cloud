use std::ops::Index;

use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, Mutable},
    signal_vec::{SignalVecExt, MutableVec},
};

pub struct HistoryState<T: Clone> {
    history: MutableVec<T>,
    cursor: Mutable<usize>
}

impl <T: Clone> HistoryState <T> {
    pub fn new(init:T) -> Self {
        Self {
            history: MutableVec::new_with_values(vec![init]),
            cursor: Mutable::new(0)
        }
    }

    pub fn undoable(&self) -> impl Signal<Item = bool> {
        self.cursor.signal().map(|cursor| cursor > 0)
    }

    pub fn redoable(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let len = self.history.signal_vec_cloned().len(),
            let cursor = self.cursor.signal()
                => {
                    *cursor < *len-1
                }
        }
    }

    pub fn undo_unchecked(&self) -> Option<T> {
        let mut cursor = self.cursor.lock_mut();
        if *cursor > 0 {
            *cursor -= 1;
            Some(self.history.lock_ref().index(*cursor).clone())
        } else {
            None
        }
    }
    pub fn redo_unchecked(&self) -> Option<T> {
        let mut cursor = self.cursor.lock_mut();
        let len = self.history.lock_ref().len();
        if *cursor < len-1 {
            *cursor += 1;
            Some(self.history.lock_ref().index(*cursor).clone())
        } else {
            None
        }
    }


    //Clones the current state
    //and expects the caller to modify it before pushing
    pub fn push_mix<F: FnOnce(&mut T)>(&self, callback:F) {

        let mut value = {
            let cursor = self.cursor.get();
            let old_ref = self.history.lock_ref();
            old_ref.index(cursor).clone()
        };

        callback(&mut value);

        self.push(value);
    }

    pub fn push(&self, value:T) {
        let mut history = self.history.lock_mut();
        let mut cursor = self.cursor.lock_mut();


        //Delete everything after this point in history
        while history.len() > 0 && *cursor < history.len()-1 {
            history.remove(history.len()-1);
        }

        history.push_cloned(value);

        *cursor += 1;
    }

}
