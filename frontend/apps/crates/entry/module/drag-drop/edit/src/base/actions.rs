use super::{
    footer::state::Footer, header::state::Header, main::state::Main, overlay::state::Overlay,
    sidebar::state::Sidebar, state::*,
};
use components::module::_common::edit::prelude::*;
use shared::domain::jig::module::body::{
    Audio, Transform,
    _groups::design::Trace as RawTrace,
    drag_drop::{
        Interactive as RawInteractive, ItemKind as RawItemKind, Mode, ModuleData as RawData, Step,
        TargetArea,
    },
};
use std::rc::Rc;

pub async fn init_from_raw(
    init_args: BaseInitFromRawArgs<RawData, Mode, Step>,
) -> BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {
    let force_step = {
        if init_args.source == InitSource::ForceRaw {
            crate::debug::settings().step
        } else {
            None
        }
    };

    let base = Base::new(init_args).await;

    let sidebar = Rc::new(Sidebar::new(base.clone()));

    BaseInit {
        force_step,
        force_theme: None,
        base: base.clone(),
        main: Rc::new(Main::new(base.clone(), sidebar.clone())),
        sidebar,
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base)),
    }
}

impl Base {
    /*
     * The traces themselves are managed by the component
     * Callbacks here are fired from there and need only to manage
     * meta and history
     */
    pub fn on_trace_added(&self, raw_trace: RawTrace) {
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.target_areas.push(TargetArea { trace: raw_trace })
            }
        });
    }

    pub fn on_trace_deleted(&self, index: usize) {
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.target_areas.remove(index);
            }
        });
    }

    pub fn on_trace_changed(&self, index: usize, raw_trace: RawTrace) {
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.target_areas[index].trace = raw_trace;
            }
        });
    }

    pub fn set_drag_item_selected(&self, index: usize) {
        let list = &*self.stickers.list.lock_ref();
        let kind = &list[index].kind;

        if std::mem::discriminant(&*kind.lock_ref()) == std::mem::discriminant(&ItemKind::Static) {
            let data = RawInteractive {
                audio: None,
                target_transform: None,
            };

            kind.set(ItemKind::Interactive(Interactive::new(data.clone())));

            self.history.push_modify(move |raw| {
                if let Some(content) = &mut raw.content {
                    content.items[index].kind = RawItemKind::Interactive(data);
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

    pub fn set_drag_item_audio(&self, index: usize, audio: Option<Audio>) {
        let list = &*self.stickers.list.lock_ref();
        let item = &list[index];
        let data = item.get_interactive_unchecked();

        data.audio.set(audio.clone());
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                match &mut content.items[index].kind {
                    RawItemKind::Interactive(data) => {
                        data.audio = audio;
                    }
                    RawItemKind::Static => {
                        panic!("saving audio on static item!?");
                    }
                }
            }
        });
    }

    pub fn set_drag_item_target_transform(&self, index: usize, transform: Transform) {
        let list = &*self.stickers.list.lock_ref();
        let item = &list[index];
        let data = item.get_interactive_unchecked();

        data.target_transform.set(Some(transform.clone()));

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                match &mut content.items[index].kind {
                    RawItemKind::Interactive(data) => {
                        data.target_transform = Some(transform);
                    }
                    RawItemKind::Static => {
                        panic!("saving offset on static item!?");
                    }
                }
            }
        });
    }
}
