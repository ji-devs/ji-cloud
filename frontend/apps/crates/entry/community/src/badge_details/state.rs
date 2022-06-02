use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::badge::{Badge, BadgeId};

pub struct BadgeDetails {
    pub badge_id: BadgeId,
    pub badge: Mutable<Option<Badge>>,
    pub loader: AsyncLoader,
}

impl BadgeDetails {
    pub fn new(badge_id: BadgeId) -> Rc<Self> {
        Rc::new(Self {
            badge_id,
            badge: Mutable::new(None),
            loader: AsyncLoader::new(),
        })
    }
}
