use crate::schools::Schools;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::admin::InviteSchoolUserFailure;
use shared::domain::billing::{AccountUser, AdminSchool, SchoolId};
use std::rc::Rc;
use web_sys::HtmlTextAreaElement;

pub struct SchoolDetails {
    pub parent: Rc<Schools>,
    pub school_id: SchoolId,
    pub school: Mutable<Option<AdminSchool>>,
    pub users: MutableVec<Rc<AccountUser>>,
    pub current_action: Mutable<CurrentAction>,
    pub errored_users: Mutable<Vec<String>>,
}

impl SchoolDetails {
    pub fn new(parent: Rc<Schools>, school_id: SchoolId) -> Rc<Self> {
        Rc::new(Self {
            parent,
            school_id,
            school: Mutable::new(None),
            users: MutableVec::new(),
            current_action: Mutable::new(CurrentAction::Viewing),
            errored_users: Mutable::new(vec![]),
        })
    }
}

#[derive(Clone, Debug)]
pub enum CurrentAction {
    Viewing,
    AddUsers(Mutable<Option<Rc<HtmlTextAreaElement>>>),
    AddingUsers,
    Results(Vec<InviteSchoolUserFailure>),
}
