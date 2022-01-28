use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints::jig, domain::jig::JigPlayerSettings};
use utils::prelude::*;

use super::state::ShareJig;

impl ShareJig {
    pub(super) fn generate_student_code(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let req = shared::domain::jig::player::JigPlayerSessionCreateRequest {
                jig_id: state.jig_id,
                settings: JigPlayerSettings::default(),
            };

            match jig::player::Create::api_with_auth(Some(req)).await {
                Err(_) => todo!(),
                Ok(res) => {
                    let code = format!("{:04}", res.index.0);
                    state.student_code.set(Some(code));
                },
            };
        }));
    }
}
