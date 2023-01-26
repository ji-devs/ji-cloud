// use super::state::AdminUser;
// use dominator::{clone, html, Dom};
// use std::rc::Rc;
// use utils::{events, routes::AdminUsersRoute};

// impl AdminUser {
//     pub fn render(self: Rc<Self>) -> Dom {
//         let state = self;
//         html!("admin-user-details", {
//             .prop("slot", "user-details")
//             .child(html!("window-loader-block", {
//                 .prop("slot", "loader")
//                 .prop_signal("visible", state.user.loader.is_loading())
//             }))
//             .children(&mut [
//                 html!("button-rect", {
//                     .prop("slot", "back")
//                     .prop("color", "blue")
//                     .prop("kind", "text")
//                     .text("Back")
//                     .event(clone!(state => move |_: events::Click| {
//                         let route = AdminUsersRoute::Table;
//                         state.users_state.navigate_to(route);
//                     }))
//                 }),
//                 // html!("div", {
//                 //     .prop("slot", "inputs")
//                 //     .children(&mut [
//                 //         html!("input-wrapper", {
//                 //             .prop("label", "Username")
//                 //             .children(&mut [
//                 //                 html!("input", {
//                 //                     .prop("readOnly", true)
//                 //                     .prop("value", &state.user.username)
//                 //                 }),
//                 //             ])
//                 //         }),
//                 //         html!("input-wrapper", {
//                 //             .prop("label", "First Name")
//                 //             .children(&mut [
//                 //                 html!("input", {
//                 //                     .prop("readOnly", true)
//                 //                     .prop("value", &state.user.first_name)
//                 //                 }),
//                 //             ])
//                 //         }),
//                 //         html!("input-wrapper", {
//                 //             .prop("label", "Last Name")
//                 //             .children(&mut [
//                 //                 html!("input", {
//                 //                     .prop("readOnly", true)
//                 //                     .prop("value", &state.user.last_name)
//                 //                 }),
//                 //             ])
//                 //         }),
//                 //         html!("input-wrapper", {
//                 //             .prop("label", "Last Name")
//                 //             .children(&mut [
//                 //                 html!("input", {
//                 //                     .prop("readOnly", true)
//                 //                     .prop("value", &state.user.last_name)
//                 //                 }),
//                 //             ])
//                 //         }),
//                 //         html!("input-wrapper", {
//                 //             .prop("label", "Last Name")
//                 //             .children(&mut [
//                 //                 html!("input", {
//                 //                     .prop("readOnly", true)
//                 //                     .prop("value", &state.user.last_name)
//                 //                 }),
//                 //             ])
//                 //         }),
//                 //         html!("input-wrapper", {
//                 //             .prop("label", "Last Name")
//                 //             .children(&mut [
//                 //                 html!("input", {
//                 //                     .prop("readOnly", true)
//                 //                     .prop("value", &state.user.last_name)
//                 //                 }),
//                 //             ])
//                 //         }),
//                 //     ])
//                 // }),
//             ])
//         })
//     }
// }
