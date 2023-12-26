use std::rc::Rc;

use utils::routes::ClassroomCodesRoute;

pub struct Codes {
    pub route: ClassroomCodesRoute,
}

impl Codes {
    pub fn new(route: ClassroomCodesRoute) -> Rc<Self> {
        Rc::new(Self { route })
    }
}
