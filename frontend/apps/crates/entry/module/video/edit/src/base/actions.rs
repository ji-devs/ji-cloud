use super::{
    footer::state::Footer, header::state::Header, main::state::Main, overlay::state::Overlay,
    sidebar::state::Sidebar, state::*,
};
use components::{
    module::_common::edit::prelude::*,
    stickers::{embed::state::Embed, state::Sticker},
};
use shared::domain::module::body::video::{Mode, ModuleData as RawData, Step};
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

    BaseInit {
        force_step,
        force_theme: None,
        base: base.clone(),
        main: Rc::new(Main::new(base.clone())),
        sidebar: Rc::new(Sidebar::new(base.clone())),
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base)),
    }
}

impl Base {
    #[must_use]
    pub(super) fn get_embed_sticker(&self) -> Option<Rc<Embed>> {
        let stickers = self.stickers.list.lock_ref(); ///////////////////////////////////// TODO: why not just get self.embed?

        let embed = stickers
            .iter()
            .find(|sticker| matches!(sticker, Sticker::Embed(_)))
            .map(|sticker| match sticker {
                Sticker::Embed(embed) => embed,
                _ => unreachable!("should not be possible"),
            });

        let embed = embed.map(|embed| Rc::clone(&embed));

        embed
    }
}
