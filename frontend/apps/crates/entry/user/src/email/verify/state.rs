use std::rc::Rc;

pub struct VerifyEmailPage {
    pub token: String,
}

impl VerifyEmailPage {
    pub fn new(token: String) -> Rc<Self> {
        Rc::new(Self { token })
    }
}
