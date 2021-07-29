use std::ops::Index;

use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, Mutable},
    signal_vec::{SignalVecExt, MutableVec},
};

pub struct HistoryState<T, OnSaveFn, OnUndoRedoFn> 
where
    T: Clone,
    OnSaveFn: Fn(T),
    OnUndoRedoFn: Fn(T),
{
    on_save: OnSaveFn,
    on_undoredo: OnUndoRedoFn,
    history: MutableVec<T>,
    cursor: Mutable<usize>
}

impl <T, OnSaveFn, OnUndoRedoFn> HistoryState <T, OnSaveFn, OnUndoRedoFn> 
where
    T: Clone + 'static,
    OnSaveFn: Fn(T) + 'static,
    OnUndoRedoFn: Fn(T) + 'static,
{
    pub fn new(init:T, on_save: OnSaveFn, on_undoredo: OnUndoRedoFn) -> Self {
        Self {
            on_save,
            on_undoredo,
            history: MutableVec::new_with_values(vec![init]),
            cursor: Mutable::new(0)
        }
    }

    // Just getters, don't actually change anything
    pub fn get_current(&self) -> T {
        let mut cursor = self.cursor.lock_mut();
        self.history.lock_ref().index(*cursor).clone()
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


    // Setters, and they call self.on_save()
    // undo and redo also return the value
    // so that it can differentiate between an update and a pop
    pub fn undo(&self) {
        let mut cursor = self.cursor.lock_mut();
        if *cursor > 0 {
            *cursor -= 1;
            let value = self.history.lock_ref().index(*cursor).clone();
            (self.on_save)(value.clone());
            (self.on_undoredo)(value);
        } 

    }
    pub fn redo(&self) {
        let mut cursor = self.cursor.lock_mut();
        let len = self.history.lock_ref().len();
        if *cursor < len-1 {
            *cursor += 1;
            let value = self.history.lock_ref().index(*cursor).clone();
            (self.on_save)(value.clone());
            (self.on_undoredo)(value);
        }
    }

    pub fn push(&self, value:T) {
        let mut history = self.history.lock_mut();
        let mut cursor = self.cursor.lock_mut();


        //Delete everything after this point in history
        while history.len() > 0 && *cursor < history.len()-1 {
            history.remove(history.len()-1);
        }

        history.push_cloned(value.clone());

        *cursor += 1;

        (self.on_save)(value);
    }


    /// Helper to push new state more easily.
    /// Clones the current state
    /// and expects the caller to modify it before pushing
    /// internally calls push()
    pub fn push_modify<M: FnOnce(&mut T)>(&self, modify:M) {

        let mut value = {
            let cursor = self.cursor.get();
            let old_ref = self.history.lock_ref();
            old_ref.index(cursor).clone()
        };

        modify(&mut value);

        self.push(value);
    }

    /// Helper to save without pushing new state
    pub fn save_current_modify<M: FnOnce(&mut T)>(&self, modify:M) {
        let mut value = self.get_current();
        modify(&mut value);

        (self.on_save) (value);
    }
}
