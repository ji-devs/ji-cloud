use dominator::{html, Dom};
use utils::{routes::*, events};

pub struct SidebarDom {}
impl SidebarDom {
    pub fn render(route: AdminRoute) -> Dom {

        html!("admin-sidebar", {
            .property("section", {
                match route {
                    AdminRoute::Categories => "category", 
                    AdminRoute::Locale => "locale", 
                    AdminRoute::ImageAdd => "image-add", 
                    AdminRoute::ImageTags => "image-tags", 
                    AdminRoute::ImageMeta(id, is_new) => "image-search", 
                    AdminRoute::ImageSearch(query) => "image-search",
                    _ => ""
                }
            })
            .event(|evt:events::CustomRoute| {
                match evt.route().as_ref() {
                    "image-add" => {
                        let route:String = Route::Admin(AdminRoute::ImageAdd).into();
                        dominator::routing::go_to_url(&route);
                    },
                    "image-search" => {
                        let route:String = Route::Admin(AdminRoute::ImageSearch(None)).into();
                        dominator::routing::go_to_url(&route);
                    },
                    "image-tags" => {
                        let route:String = Route::Admin(AdminRoute::ImageTags).into();
                        dominator::routing::go_to_url(&route);
                    },
                    "jig" => {
                        /*
                        let route:String = Route::Admin(AdminRoute::ImageAdd).into();
                        dominator::routing::go_to_url(&route);
                        */
                    },
                    "category" => {
                        let route:String = Route::Admin(AdminRoute::Categories).into();
                        dominator::routing::go_to_url(&route);
                    },
                    "locale" => {
                        let route:String = Route::Admin(AdminRoute::Locale).into();
                        dominator::routing::go_to_url(&route);
                    },
                    _ => {
                    }
                }
            })
        })
    }
}
