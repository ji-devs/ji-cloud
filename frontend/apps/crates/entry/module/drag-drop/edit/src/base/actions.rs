use components::module::_common::edit::prelude::*;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Audio,
            Instructions,
            drag_drop::{Mode, Step, Content as RawContent, ModuleData as RawData, ItemKind as RawItemKind, Interactive as RawInteractive},
            _groups::design::Trace as RawTrace,
        }
    }
};
use super::{
    state::*,
    footer::state::Footer,
    header::state::Header,
    main::state::Main,
    overlay::state::Overlay,
    sidebar::state::Sidebar
};
use dominator::clone;
use futures_signals::signal::{ReadOnlyMutable, Mutable};
use utils::prelude::*;
use components::{
    text_editor::state::State as TextEditorState,
    audio_mixer::AudioMixer,
};

pub async fn init_from_raw(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {

    let force_step = {
        if init_args.source == InitSource::ForceRaw { 
            crate::debug::settings().step
        } else {
            None
        }
    };


    let base = Base::new(init_args).await;
    
    BaseInit {
        force_step,
        force_theme: None,
        base: base.clone(),
        main: Rc::new(Main::new(base.clone())),
        sidebar: Rc::new(Sidebar::new(base.clone())),
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base.clone())),
    }
}


impl Base {
    /*
     * The traces themselves are managed by the component
     * Callbacks here are fired from there and need only to manage
     * meta and history
     */
    pub fn on_trace_added(&self, raw_trace: RawTrace) {
        /*
        self.traces_meta.lock_mut().push_cloned(TraceMeta::new(None, None));

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces.push(DragDropTrace {
                    trace: raw_trace,
                    audio: None,
                    text: None,
                })
            }
        });
        */
    }

    pub fn on_trace_deleted(&self, index: usize) {
        /*
        self.traces_meta.lock_mut().remove(index);

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces.remove(index);
            }
        });
        */
    }

    pub fn on_trace_changed(&self, index: usize, raw_trace: RawTrace) {
        /*
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces[index].trace = raw_trace;
            }
        });
        */
    }

    pub fn set_drag_item_selected(&self, index: usize) {
        let list = &*self.stickers.list.lock_ref();
        let kind = &list[index].kind;

        if std::mem::discriminant(&*kind.lock_ref()) == std::mem::discriminant(&ItemKind::Static) {
            kind.set(ItemKind::Interactive(Interactive::new(None)));
            self.history.push_modify(move |raw| {
                if let Some(content) = &mut raw.content {
                    content.items[index].kind = RawItemKind::Interactive(RawInteractive::default());
                }
            });
        }

        self.drag_item_selected_index.set(Some(index));
    }

    pub fn set_drag_item_deselected(&self, index: usize) {
        let list = &*self.stickers.list.lock_ref();
        let kind = &list[index].kind;

        if std::mem::discriminant(&*kind.lock_ref()) != std::mem::discriminant(&ItemKind::Static) {
            kind.set(ItemKind::Static);
            self.history.push_modify(move |raw| {
                if let Some(content) = &mut raw.content {
                    content.items[index].kind = RawItemKind::Static;
                }
            });
        }
        self.drag_item_selected_index.set(None);
    }

    pub fn set_drag_item_meta_audio(&self, index: usize, audio: Option<Audio>) {
        let list = &*self.stickers.list.lock_ref();
        let kind = &list[index].kind;
        match kind.get_cloned() {
            ItemKind::Interactive(data) => {
                data.audio.set(audio.clone());
                self.history.push_modify(move |raw| {
                    if let Some(content) = &mut raw.content {
                        match &mut content.items[index].kind {
                            RawItemKind::Interactive(data) => {
                                data.audio = audio;
                            },
                            RawItemKind::Static => {
                                panic!("saving audio on static item!?");
                            },
                        }
                    }
                });
            },
            _ => {
                panic!("setting audio on static item!?");
            }
        }
    }
}
