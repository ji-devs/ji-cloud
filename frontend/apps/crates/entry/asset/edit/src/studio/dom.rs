use components::page_header::{PageHeader, PageHeaderConfig, PageLinks};
use dominator::{events, html, on_click_go_to_url, Dom};
use utils::{
    asset,
    routes::{AssetRoute, HomeRoute, Route},
};

pub fn render_studio() -> Dom {
    html!("asset-edit-studio", {
        .children(&mut [
            PageHeader::new(PageHeaderConfig {
                slot: Some("header"),
                active_page: Some(PageLinks::Create),
                ..Default::default()
            }).render(),
            html!("button-rect", {
                .prop("slot", "jig-create")
                .prop("color", "red")
                .prop("kind", "filled")
                .prop("size", "regular")
                .event(|_: events::Click| {
                    asset::create_jig();
                })
                .text("Create a JIG")
            }),
            html!("button-rect", {
                .prop("slot", "jig-gallery")
                .prop("color", "blue")
                .prop("kind", "text")
                .prop("size", "regular")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Asset(AssetRoute::JigGallery).to_string()
                }))
                .text("My JIGs")
            }),
            html!("button-rect", {
                .prop("slot", "playlist-create")
                .prop("color", "red")
                .prop("kind", "filled")
                .prop("size", "regular")
                .event(|_: events::Click| {
                    asset::create_playlist();
                })
                .text("Create a Playlist")
            }),
            html!("button-rect", {
                .prop("slot", "playlist-gallery")
                .prop("color", "blue")
                .prop("kind", "text")
                .prop("size", "regular")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Asset(AssetRoute::PlaylistGallery).to_string()
                }))
                .text("My Playlists")
            }),
            html!("button-rect", {
                .prop("slot", "resource-create")
                .prop("color", "red")
                .prop("kind", "filled")
                .prop("size", "regular")
                .event(|_: events::Click| {
                    asset::create_resource();
                })
                .text("Add a Resource")
            }),
            html!("button-rect", {
                .prop("slot", "resource-gallery")
                .prop("color", "blue")
                .prop("kind", "text")
                .prop("size", "regular")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Asset(AssetRoute::ResourceGallery).to_string()
                }))
                .text("My Resources")
            }),
            html!("button-rect", {
                .prop("slot", "course-create")
                .prop("color", "red")
                .prop("kind", "filled")
                .prop("size", "regular")
                .event(|_: events::Click| {
                    asset::create_course();
                })
                .text("Create a Course")
            }),
            html!("button-rect", {
                .prop("slot", "course-gallery")
                .prop("color", "blue")
                .prop("kind", "text")
                .prop("size", "regular")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Asset(AssetRoute::CourseGallery).to_string()
                }))
                .text("My Courses")
            }),
            html!("button-rect", {
                .prop("slot", "help")
                .prop("kind", "outline")
                .prop("size", "regular")
                .prop("color", "white")
                .prop("href", Route::Home(HomeRoute::Help).to_string())
                .text("Help")
            }),
        ])
    })
}
