use futures_signals::{
    signal::{ReadOnlyMutable, SignalExt},
    signal_vec::SignalVec,
};
use shared::domain::user::UserProfile;
use std::rc::Rc;
use utils::routes::*;

pub struct Sidebar {
    pub route: AdminRoute,
    pub profile: ReadOnlyMutable<Option<Option<UserProfile>>>,
}

impl Sidebar {
    pub fn new(
        route: AdminRoute,
        profile: ReadOnlyMutable<Option<Option<UserProfile>>>,
    ) -> Rc<Self> {
        Rc::new(Self { route, profile })
    }

    pub fn item_signal_vec(&self) -> impl SignalVec<Item = Rc<SidebarItem>> {
        let curr_route = self.route.clone();

        self.profile
            .signal_ref(move |profile| match profile {
                None => Vec::new(),
                Some(profile) => vec![
                    SidebarItem::new(AdminRoute::Images, profile, &curr_route),
                    SidebarItem::new(AdminRoute::ImageAdd, profile, &curr_route),
                    SidebarItem::new(AdminRoute::ImageSearch(None), profile, &curr_route),
                    SidebarItem::new(AdminRoute::ImageTags, profile, &curr_route),
                    SidebarItem::new(
                        AdminRoute::JigCuration(AdminJigCurationRoute::Table),
                        profile,
                        &curr_route,
                    ),
                    SidebarItem::new(
                        AdminRoute::ResourceCuration(AdminResourceCurationRoute::Table),
                        profile,
                        &curr_route,
                    ),
                    SidebarItem::new(
                        AdminRoute::CourseCuration(AdminCourseCurationRoute::Table),
                        profile,
                        &curr_route,
                    ),
                    SidebarItem::new(
                        AdminRoute::PlaylistCuration(AdminPlaylistCurationRoute::Table),
                        profile,
                        &curr_route,
                    ),
                    SidebarItem::new(
                        AdminRoute::Users(AdminUsersRoute::Table),
                        profile,
                        &curr_route,
                    ),
                    SidebarItem::new(
                        AdminRoute::Schools(AdminSchoolsRoute::Table),
                        profile,
                        &curr_route,
                    ),
                    SidebarItem::new(AdminRoute::Categories, profile, &curr_route),
                    SidebarItem::new(AdminRoute::Locale, profile, &curr_route),
                    SidebarItem::new(AdminRoute::Export, profile, &curr_route),
                ],
            })
            .to_signal_vec()
    }
}

pub struct SidebarItem {
    pub locked: bool,
    pub selected: bool,
    pub id: &'static str,
    pub route: AdminRoute,
}

impl SidebarItem {
    pub fn new(
        target_route: AdminRoute,
        profile: &Option<UserProfile>,
        curr_route: &AdminRoute,
    ) -> Rc<Self> {
        let id = match target_route {
            AdminRoute::Categories => "category",
            AdminRoute::Locale => "locale",
            AdminRoute::ImageAdd => "image-add",
            AdminRoute::ImageTags => "image-tags",
            AdminRoute::ImageMeta(_, _) => "image-search",
            AdminRoute::ImageSearch(_) => "image-search",
            AdminRoute::Users(_) => "users",
            AdminRoute::JigCuration(_) => "jig-curation",
            AdminRoute::ResourceCuration(_) => "resource-curation",
            AdminRoute::CourseCuration(_) => "course-curation",
            AdminRoute::PlaylistCuration(_) => "playlist-curation",
            AdminRoute::Schools(_) => "schools",
            AdminRoute::Images => "images",
            AdminRoute::Export => "export",
            AdminRoute::Landing => "",
        };

        let locked = match profile {
            None => true,
            Some(user) => !target_route.allowed_user_scope(&user.scopes),
        };

        let selected = match curr_route {
            AdminRoute::ImageMeta(_, _) | AdminRoute::ImageSearch(_) => {
                matches!(
                    target_route,
                    AdminRoute::ImageMeta(_, _) | AdminRoute::ImageSearch(_)
                )
            }
            _ => std::mem::discriminant(&target_route) == { std::mem::discriminant(curr_route) },
        };

        Rc::new(Self {
            locked,
            selected,
            id,
            route: target_route,
        })
    }
}
