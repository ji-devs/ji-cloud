use components::page_header::{PageHeader, PageHeaderConfig, PageLinks};
use dominator::{events, html, on_click_go_to_url, Dom};
use utils::routes::{AssetRoute, HomeRoute, Route};

use super::actions;

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
                .prop("size", "small")
                .event(|_: events::Click| {
                    actions::create_jig();
                })
                .text("Create a JIG")
            }),
            html!("button-rect", {
                .prop("slot", "jig-gallery")
                .prop("color", "blue")
                .prop("kind", "text")
                .prop("size", "small")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Asset(AssetRoute::JigGallery).to_string()
                }))
                .text("My JIGs")
            }),
            html!("button-rect", {
                .prop("slot", "course-create")
                .prop("color", "red")
                .prop("kind", "filled")
                .prop("size", "small")
                .event(|_: events::Click| {
                    actions::create_course();
                })
                .text("Create a course")
            }),
            html!("button-rect", {
                .prop("slot", "course-gallery")
                .prop("color", "blue")
                .prop("kind", "text")
                .prop("size", "small")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Asset(AssetRoute::CourseGallery).to_string()
                }))
                .text("My Courses")
            }),
            html!("button-rect", {
                .prop("slot", "resource-create")
                .prop("color", "red")
                .prop("kind", "filled")
                .prop("size", "small")
                .event(|_: events::Click| {
                    actions::create_resource();
                })
                .text("Create a resource")
            }),
            html!("button-rect", {
                .prop("slot", "resource-gallery")
                .prop("color", "blue")
                .prop("kind", "text")
                .prop("size", "small")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Asset(AssetRoute::ResourceGallery).to_string()
                }))
                .text("My resources")
            }),
            html!("button-rect", {
                .prop("slot", "pro-dev-create")
                .prop("color", "red")
                .prop("kind", "filled")
                .prop("size", "small")
                .event(|_: events::Click| {
                    // actions::create_pro_dev();
                })
                .text("Create a Pro-Dev course")
            }),
            html!("button-rect", {
                .prop("slot", "pro-dev-gallery")
                .prop("color", "blue")
                .prop("kind", "text")
                .prop("size", "small")
                // .apply(move |dom| on_click_go_to_url!(dom, {
                //     Route::Asset(AssetRoute::ProDevGallery).to_string()
                // }))
                .text("My Pro-Dev Courses")
            }),
            html!("button-rect", {
                .prop("slot", "help")
                .prop("kind", "outline")
                .prop("size", "small")
                .prop("color", "white")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Home(HomeRoute::Help).to_string()
                }))
                .prop("href", "???")
                .text("Help")
            }),
        ])
    })
}
