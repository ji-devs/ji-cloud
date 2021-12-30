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
                    SidebarItem::new(AdminRoute::ImageAdd, profile, &curr_route),
                    SidebarItem::new(AdminRoute::ImageSearch(None), profile, &curr_route),
                    SidebarItem::new(AdminRoute::ImageTags, profile, &curr_route),
                    SidebarItem::new(AdminRoute::Curation(AdminCurationRoute::Table), profile, &curr_route),
                    SidebarItem::new(AdminRoute::Categories, profile, &curr_route),
                    SidebarItem::new(AdminRoute::Locale, profile, &curr_route),
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
            AdminRoute::Curation(_) => "curation",
            AdminRoute::Landing => "",
        };

        let locked = match profile {
            None => true,
            Some(user) => !target_route.allowed_user_scope(&user.scopes),
        };

        let selected = match curr_route {
            AdminRoute::ImageMeta(_, _) | AdminRoute::ImageSearch(_) => match target_route {
                AdminRoute::ImageMeta(_, _) | AdminRoute::ImageSearch(_) => true,
                _ => false,
            },
            _ => std::mem::discriminant(&target_route) == std::mem::discriminant(curr_route),
        };

        Rc::new(Self {
            locked,
            selected,
            id,
            route: target_route,
        })
    }
}
