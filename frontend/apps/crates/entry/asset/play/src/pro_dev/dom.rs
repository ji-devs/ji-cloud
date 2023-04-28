use super::actions;
use crate::pro_dev::actions::{page_back_signal, page_forward_signal, paginate};
use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    share_asset::ShareAsset,
    stickers::video::ext::YoutubeUrlExt,
};
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use serde::{Deserialize, Serialize};
use shared::{
    domain::{
        audio::AudioId,
        image::ImageId,
        meta::ResourceTypeId,
        module::body::_groups::design::VideoHost,
        pdf::PdfId,
        pro_dev::{
            unit::{ProDevUnit, Video},
            ProDevResponse,
        },
    },
    media::MediaLibrary,
};
use std::rc::Rc;
use utils::{
    asset::ResourceContentExt,
    events,
    languages::Language,
    path::{audio_lib_url, pdf_lib_url},
};
use web_sys::{HtmlElement, HtmlIFrameElement};

use super::state::ProDevPlayer;

// const INT_IFRAME_PADDING: usize = 30;
// const INT_INITIAL_HEIGHT: usize = 3000;

const STR_SHARE_COURSE: &str = "Share course";

impl ProDevPlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        actions::load_data(state.clone());

        html!("div", {
            .child_signal(state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                if let Some(pro_dev) = pro_dev {
                    Some(state.render_pro_dev_landing(&pro_dev))
                } else {
                    None
                }
            })))
            .child_signal(state.active_unit.signal_cloned().map(clone!(state => move|active_unit| {
                active_unit.map(|_unit_id| {
                    // let options = AssetPlayerOptions::ProDev(ProDevPlayerOptions {
                    //     is_student: state.player_options.is_student,
                    //     ..Default::default()
                    // });
                    state.render_play_unit()

                })
            })))
        })
    }

    fn render_play_unit(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("dialog-overlay", {
            .prop("slot", "dialog")
            .prop("open", true)
            .prop("autoClose", false)
            .child(html!("pro-dev-player", {
                .child_signal(state.active_unit_signal().map(clone!(state => move|unit| {
                    unit.map(|unit| {
                        state.render_active_unit(unit)
                    })
                })))
                .child_signal(state.active_unit_signal().map(move|unit| {
                    unit.map(|unit| {
                        html!("div", {
                            .prop("slot", "title")
                            .children(&mut [
                                html!("div", {
                                    .style("text-align", "center")
                                    .text(&unit.display_name)
                                }),
                                html!("div", {
                                    .style("text-align", "center")
                                    .text(&unit.description)
                                })
                            ])
                        })
                    })
                }))
                .child_signal(state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                    pro_dev.map(clone!(state => move |pro_dev| {
                        html!("div", {
                            .prop("slot", "navigation")
                            .style("display", "flex")
                            .child_signal(state.active_unit.signal_cloned().map(clone!(state=> move |_active_unit| {
                                Some(
                                    html!("button", {  // back arrow button
                                        .text("<")
                                        .style("order", "0")
                                        .prop_signal("hidden", page_back_signal(Rc::clone(&state)))
                                        .event(clone!(state => move |_: events::Click| {
                                            let index = state.active_unit.get().unwrap_or(0);
                                            let current_page = state.current_page.get().unwrap_or(0);
                                            if index > 0 {
                                                state.played_unit(index);
                                                state.active_unit.set(Some(index - 1));
                                                if (index + 1) % 10 == 1 && current_page > 0 {
                                                    state.current_page.set(Some(current_page - 1));
                                                }
                                            }
                                        }))
                                    }))
                            })))
                            .child_signal(state.current_page.signal_cloned().map(clone!(
                                state, pro_dev => move |_page| {
                                    Some(paginate(&state, &pro_dev))
                                }
                            )))
                            .child_signal(state.active_unit.signal_cloned().map(clone!(state, pro_dev => move |_active_unit| {
                                Some(
                                    html!("button", {  // forward arrow button
                                        .text(">")
                                        .style("order", "2")
                                        .prop_signal("hidden", page_forward_signal(Rc::clone(&state), &pro_dev))
                                        .event(clone!(state, pro_dev => move |_: events::Click| {
                                                let index = state.active_unit.get().unwrap_or(0);
                                                let current_page = state.current_page.get().unwrap_or(0);
                                                let num_pages = (pro_dev.pro_dev_data.units.len() + 9) / 10;
                                                if index < (pro_dev.pro_dev_data.units.len() - 1) {
                                                    state.played_unit(index);
                                                    state.active_unit.set(Some(index + 1));
                                                    if (index + 1) % 10 == 0  && (current_page < (num_pages - 1))  {
                                                        state.current_page.set(Some(current_page + 1));
                                                    }
                                                }
                                            }))
                                    }))
                            })))
                        })
                    }))
                })))
                .child(html!("fa-button", {
                    .prop("slot", "close")
                    .prop("icon", "fa-light fa-xmark")
                    .event(clone!(state => move |_: events::Click| {
                        state.active_unit.set(None);
                    }))
                }))
            }))
        })
    }

    fn played_unit(self: &Rc<Self>, index: usize) {
        let played_length = self.played_units.lock_mut().len();
        let played_units = self.played_units.get_cloned();
        if !played_units.contains(&index) {
            self.played_units.lock_mut().insert(played_length, index);
        }
    }

    fn render_pro_dev_landing(self: &Rc<Self>, pro_dev: &ProDevResponse) -> Dom {
        let state = self;
        let language = Language::code_to_display_name(&pro_dev.pro_dev_data.language);

        html!("jig-play-course-main", {
            .prop("name", &pro_dev.pro_dev_data.display_name)
            .prop("description", &pro_dev.pro_dev_data.description)
            .prop("language", language)
            .prop("author", &pro_dev.author_name.to_owned().unwrap_or_default())
            .prop("itemsCount", pro_dev.pro_dev_data.units.len())
            .prop("hasAdditionalResources", !pro_dev.pro_dev_data.additional_resources.is_empty())
            .child(
                ModuleThumbnail::new_hight_res(
                    pro_dev.id.into(),
                    pro_dev.pro_dev_data.cover.clone(),
                    ThumbnailFallback::Asset,
                    state.player_options.draft_or_live,
                ).render(Some("thumbnail"))
            )
            .children_signal_vec(state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                match pro_dev {
                    Some(pro_dev) => {
                        pro_dev.pro_dev_data.units.iter().enumerate().map(clone!(state => move |(i, unit)| {
                            state.render_unit(unit, i)
                        })).collect()
                    }
                    None => todo!()
                }
                // units.iter().enumerate().map(clone!(state => move |(i, unit)| {
                //     state.render_unit(unit, i)
                // })).collect()
            })).to_signal_vec())
            .children(pro_dev.pro_dev_data.additional_resources.iter().map(|resource| {
                html!("a", {
                    .prop("slot", "additional-resources")
                    .prop("target", "_BLANK")
                    .prop("title", &resource.display_name)
                    .prop("href", resource.resource_content.get_link())
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-file")
                    }))
                    .text(" ")
                    .text_signal(state.resource_name_signal(resource.resource_type_id))
                })
            }))
            .child(html!("fa-button", {
                .prop("slot", "play")
                .prop("icon", "fa-solid fa-circle-play")
                .event(clone!(state => move |_: events::Click| {
                    state.active_unit.set(Some(0));
                }))
            }))
            .child(ShareAsset::new(pro_dev.clone().into()).render(
                html!("button-empty", {
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-share-nodes")
                    }))
                    .text(STR_SHARE_COURSE)
                }),
                Some("share")
            ))
            // .child_signal(state.active_unit.signal_cloned().map(|active_unit| {
            //     active_unit.map(|active_unit| {
            //         html!("div", {
            //             .text(&active_unit.0.to_string())
            //         })
            //     })
            // }))
        })
    }

    fn render_unit(self: &Rc<Self>, unit: &ProDevUnit, i: usize) -> Dom {
        let state = self;
        html!("jig-play-course-item", {
            .prop("slot", "items")
            .prop("name", &unit.display_name)
            .prop("description", &unit.description)
            .prop("index", i + 1)
            .prop_signal("done", state.played_units.signal_ref(move |played_units| played_units.contains(&i)))
            // .child(
            //     ModuleThumbnail::new(
            //         unit_id.into(),
            //         unit.unit_data.modules.get(0).cloned(),
            //         ThumbnailFallback::Asset,
            //         state.player_options.draft_or_live,
            //     ).render(Some("thumbnail"))
            // )
            .child(html!("fa-button", {
                .prop("slot", "play-button")
                .prop("icon", "fa-solid fa-play")
            }))
            .event(clone!(state => move |_: events::Click| {
                let played_units_len = state.played_units.lock_ref().len();
                state.active_unit.set(Some(i));
                state.played_units.lock_mut().insert(played_units_len, i);
            }))
        })
    }

    fn render_active_unit(self: &Rc<Self>, unit: ProDevUnit) -> Dom {
        match unit.value {
            shared::domain::pro_dev::unit::ProDevUnitValue::ImageId(image_id) => {
                self.render_active_image(image_id)
            }
            shared::domain::pro_dev::unit::ProDevUnitValue::AudioId(audio_id) => {
                self.render_active_audio(audio_id)
            }
            shared::domain::pro_dev::unit::ProDevUnitValue::Link(url) => {
                self.render_active_link(url)
            }
            shared::domain::pro_dev::unit::ProDevUnitValue::PdfId(pdf_id) => {
                self.render_active_pdf(pdf_id)
            }
            shared::domain::pro_dev::unit::ProDevUnitValue::Video(video) => {
                self.render_active_video(video)
            }
        }
    }

    fn active_unit_signal(self: &Rc<Self>) -> impl Signal<Item = Option<ProDevUnit>> {
        map_ref! {
            let active_unit = self.active_unit.signal(),
            let pro_dev = self.pro_dev.signal_cloned() => move {
                match (active_unit, pro_dev) {
                    (Some(active_unit), Some(pro_dev)) => {
                        Some(pro_dev.pro_dev_data.units[*active_unit].clone())
                    },
                    _ => None
                }
            }
        }
    }

    fn render_active_video(self: &Rc<Self>, video: Video) -> Dom {
        let mut_host = Mutable::new(video.host.clone());

        html!("div", {
            .prop("slot", "player-window")
            .child_signal(mut_host.signal_cloned().map(move|host| {
                match host {
                    VideoHost::Youtube(youtube_video) => Some(
                        html!("video-youtube-player" => HtmlElement, {
                            .prop("videoId", youtube_video.url.get_id())
                            .apply(|mut dom| {
                                if let Some(start_at) = video.start_at {
                                    dom = dom.prop("start", start_at);
                                }
                                if let Some(end_at) = video.end_at {
                                    dom = dom.prop("end", end_at);
                                }
                                dom
                            })
                            .style("display", "block")
                            .style("width", "100%")
                            .style("height", "100%")
                        })
                    ),
                }
            }))
        })
    }

    fn render_active_image(self: &Rc<Self>, image: ImageId) -> Dom {
        let mut_image = Mutable::new(image.clone());

        html!("div", {
            .prop("slot", "player-window")
            .child_signal(mut_image.signal_cloned().map(move|image| {
                    Some(
                        html!("img-ji", {
                            // would like to get rid if the styles here
                            .style("position", "relative")
                            .style("object-fit", "contain")
                            .prop("size", "full")
                            .prop("id", image.0.to_string())
                            .prop("lib", "user")
                        })
                    )
            }))
        })
    }

    pub fn render_active_link(self: &Rc<Self>, link: url::Url) -> Dom {
        // let state = self;
        html!("div", {
            .prop("slot", "player-window")
            .child(html!("iframe" => HtmlIFrameElement, {
                .style("width", "100%")
                .style("height", "100%")
                .style("border", "none")
                .prop("src", link.to_string())
            }))
        })
    }

    pub fn render_active_pdf(self: &Rc<Self>, pdf_id: PdfId) -> Dom {
        // let state = self;
        let resp = pdf_lib_url(MediaLibrary::User, pdf_id);

        html!("div", {
            .prop("slot", "player-window")
            .child(html!("iframe" => HtmlIFrameElement, {
                .style("width", "100%")
                .style("height", "100%")
                .style("position", "relative")
                .prop("src", resp)
            }))
        })
    }

    pub fn render_active_audio(self: &Rc<Self>, audio_id: AudioId) -> Dom {
        // let state = self;
        let resp = audio_lib_url(MediaLibrary::User, audio_id);

        html!("div", {
            .prop("slot", "player-window")
            .child(html!("audio", {
                .style("width", "100%")
                .style("border", "none")
                .prop("src", resp)
                .prop("controls", true)
            }))
        })
    }

    fn resource_name_signal(
        self: &Rc<Self>,
        resource_type_id: ResourceTypeId,
    ) -> impl Signal<Item = String> {
        let state = Rc::clone(self);

        state
            .resource_types
            .signal_cloned()
            .map(move |resource_types| {
                let resource_type = resource_types
                    .iter()
                    .find(|resource_type| resource_type_id == resource_type.id);

                match resource_type {
                    None => String::new(),
                    Some(resource_type) => resource_type.display_name.to_owned(),
                }
            })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct IframeMessageData {
    kind: String,
    height: usize,
}
