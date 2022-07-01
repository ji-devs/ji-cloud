use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    circle::{Circle, CircleId},
    user::public_user::PublicUser,
};

use crate::state::Community;

pub struct CircleDetails {
    pub circle_id: CircleId,
    pub circle: Mutable<Option<Circle>>,
    pub members: MutableVec<PublicUser>,
    pub loader: AsyncLoader,
    pub community_state: Rc<Community>,
}

impl CircleDetails {
    pub fn new(community_state: Rc<Community>, circle_id: CircleId) -> Rc<Self> {
        Rc::new(Self {
            circle_id,
            circle: Mutable::new(None),
            members: MutableVec::new(),
            loader: AsyncLoader::new(),
            community_state,
        })
    }
}
