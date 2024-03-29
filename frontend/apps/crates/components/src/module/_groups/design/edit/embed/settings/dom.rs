use super::state::EmbedSettings;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::_groups::design::DoneAction;
use std::rc::Rc;

use crate::{
    module::_common::edit::settings::prelude::*,
    stickers::embed::types::{EmbedHost, YoutubeEmbed},
};

impl EmbedSettings {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        // can use lock_ref instead of signal as embed type can't chance while on this page/step
        match &*state.embed.lock_ref() {
            Some(embed) => match &*embed.host.lock_ref() {
                EmbedHost::Youtube(youtube) => {
                    render_settings(Rc::new(state.render_settings_youtube(youtube)))
                }
                EmbedHost::Vimeo(_) => state.render_no_settings("Vimeo"),
                EmbedHost::GoogleDoc(_) => state.render_no_settings("Google Doc"),
                EmbedHost::GoogleForm(_) => state.render_no_settings("Google Form"),
                EmbedHost::GoogleSheet(_) => state.render_no_settings("Google Sheet"),
                EmbedHost::GoogleSlide(_) => state.render_no_settings("Google Slide"),
                EmbedHost::Edpuzzle(_) => state.render_no_settings("Edpuzzle"),
                EmbedHost::Puzzel(_) => state.render_no_settings("Puzzel"),
                EmbedHost::Quizlet(_) => state.render_no_settings("Quizlet"),
                EmbedHost::Thinglink(_) => state.render_no_settings("Thinglink"),
                EmbedHost::Sutori(_) => state.render_no_settings("Sutori"),
            },
            None => state.render_no_embed_selected(),
        }
    }

    fn render_no_settings(self: &Rc<Self>, embed_name: &str) -> Dom {
        html!("div", {
            .text(&format!("No settings for {embed_name}"))
        })
    }
    fn render_no_embed_selected(self: &Rc<Self>) -> Dom {
        html!("div", {
            .text("Please select an embed in step 2 before editing settings")
        })
    }

    fn render_settings_youtube(self: &Rc<Self>, youtube: &Rc<YoutubeEmbed>) -> ModuleSettings {
        let state = self;
        ModuleSettings {
            lines: vec![
                ModuleSettingsLine::new(
                    LineKind::VideoPlay,
                    vec![
                        Some(
                            SettingsButtonBuilder::new(
                                SettingsButtonKind::PlayClick,
                                clone!(youtube => move || youtube.autoplay.signal().map(|autoplay| !autoplay)),
                            )
                            .on_click(clone!(state, youtube => move || state.toggle_autoplay(&youtube)))
                            .build()
                        ),
                        // Some(SettingsButton::new_click(
                        //     SettingsButtonKind::Loop,
                        //     clone!(state => move || {
                        //         youtube.done_action.signal_ref(|done_action| {
                        //             matches!(done_action, Some(DoneAction::Loop))
                        //         })
                        //     }),
                        //     clone!(state => move || {
                        //         state.set_unset_next_action(Some(DoneAction::Loop));
                        //     }),
                        // )),
                        Some(
                            SettingsButtonBuilder::new(
                                SettingsButtonKind::VideoCaptions,
                                clone!(youtube => move || youtube.captions.signal()),
                            )
                            .on_click(clone!(state, youtube => move || state.toggle_captions(&youtube)))
                            .build()
                        ),
                        Some(
                            SettingsButtonBuilder::new(
                                SettingsButtonKind::Mute,
                                clone!(youtube => move || youtube.muted.signal()),
                            )
                            .on_click(clone!(state, youtube => move || state.toggle_muted(&youtube)))
                            .build()
                        ),
                    ],
                ),
                ModuleSettingsLine::new(
                    LineKind::Next,
                    vec![
                        Some(
                            SettingsButtonBuilder::new(
                                SettingsButtonKind::ContinueClick,
                                clone!(youtube => move || {
                                    youtube.done_action.signal_ref(|done_action| {
                                        matches!(done_action, None)
                                    })
                                }),
                            )
                            .on_click(clone!(state, youtube => move || state.set_unset_next_action(&youtube, None)))
                            .build()
                        ),
                        Some(
                            SettingsButtonBuilder::new(
                                SettingsButtonKind::ContinueAutomatically,
                                clone!(youtube => move || {
                                    youtube.done_action.signal_ref(|done_action| {
                                        matches!(done_action, Some(DoneAction::Next))
                                    })
                                }),
                            )
                            .on_click(clone!(state, youtube => move || state.set_unset_next_action(&youtube, Some(DoneAction::Next))))
                            .build()
                        ),
                    ],
                ),
            ],
        }
    }
}
