use crate::schools::details::school_name::state::SchoolNameState;
use crate::schools::Schools;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_vec::MutableVec;
use serde_json::Value;
use shared::domain::admin::InviteSchoolUserFailure;
use shared::domain::billing::{
    Account, AccountUser, AdminSchool, SchoolId, SchoolName, UpdateSchoolAccountRequest,
};
use shared::domain::image::ImageId;
use std::rc::Rc;
use utils::editable_field::{EditableField, NonNullable, Nullable};
use web_sys::HtmlTextAreaElement;

pub struct SchoolDetails {
    pub parent: Rc<Schools>,
    pub school_id: SchoolId,
    pub school: Mutable<Option<EditableAdminSchool>>,
    pub account: Mutable<Option<Account>>,
    pub users: MutableVec<Rc<AccountUser>>,
    pub current_action: Mutable<CurrentAction>,
    pub editing_name: Mutable<Option<Rc<SchoolNameState>>>,
}

impl SchoolDetails {
    pub fn new(parent: Rc<Schools>, school_id: SchoolId) -> Rc<Self> {
        Rc::new(Self {
            parent,
            school_id,
            school: Mutable::new(None),
            account: Mutable::new(None),
            users: MutableVec::new(),
            current_action: Mutable::new(CurrentAction::Viewing),
            editing_name: Mutable::new(None),
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

#[derive(Clone)]
pub struct EditableAdminSchool {
    pub id: SchoolId,
    pub internal_school_name: Option<SchoolName>,
    pub verified: bool,
    pub school_name: EditableField<NonNullable<String>>,
    pub email: EditableField<NonNullable<String>>,
    pub location: EditableField<Nullable<Value>>,
    pub description: EditableField<Nullable<String>>,
    pub profile_image: EditableField<Nullable<ImageId>>,
    pub website: EditableField<Nullable<String>>,
    pub organization_type: EditableField<Nullable<String>>,
}

impl EditableAdminSchool {
    pub fn changed_signal(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let school_name = self.school_name.changed_signal(),
            let email = self.email.changed_signal(),
            let location = self.location.changed_signal(),
            let description = self.description.changed_signal(),
            let profile_image = self.profile_image.changed_signal(),
            let website = self.website.changed_signal(),
            let organization_type = self.organization_type.changed_signal()
            => {
                *school_name || *email || *location || *description || *profile_image || *website || *organization_type
            }
        }
    }
}

impl From<AdminSchool> for EditableAdminSchool {
    fn from(value: AdminSchool) -> Self {
        Self {
            id: value.id,
            internal_school_name: value.internal_school_name,
            verified: value.verified,
            school_name: value.school_name.into(),
            email: value.email.into(),
            location: value.location.into(),
            description: value.description.into(),
            profile_image: value.profile_image.into(),
            website: value.website.into(),
            organization_type: value.organization_type.into(),
        }
    }
}

impl From<EditableAdminSchool> for UpdateSchoolAccountRequest {
    fn from(value: EditableAdminSchool) -> Self {
        Self {
            email: value.email.into(),
            school_name: value.school_name.into(),
            location: value.location.into(),
            description: value.description.into(),
            profile_image: value.profile_image.into(),
            website: value.website.into(),
            organization_type: value.organization_type.into(),
        }
    }
}
