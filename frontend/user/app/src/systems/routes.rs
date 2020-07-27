use core::routes::{Route, UserRoute};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use shipyard::*;
use crate::components::*;

pub fn init(route:UniqueView<Route>, world:WorldView) {
    match *route {
        Route::User(route) => {
            match route {
                UserRoute::Signin => world.run(super::signin::init),
                _ => {}
            }
        },
        _ => { panic!("this spa can only handle user routes"); }
    }
}
