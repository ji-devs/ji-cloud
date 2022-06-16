use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    badge::{Badge, BadgeId},
    user::public_user::PublicUser,
};

pub struct BadgeDetails {
    pub badge_id: BadgeId,
    pub badge: Mutable<Option<Badge>>,
    pub members: MutableVec<PublicUser>,
    pub loader: AsyncLoader,
}

impl BadgeDetails {
    pub fn new(badge_id: BadgeId) -> Rc<Self> {
        Rc::new(Self {
            badge_id,
            badge: Mutable::new(None),
            members: MutableVec::new(),
            loader: AsyncLoader::new(),
        })
    }
}
