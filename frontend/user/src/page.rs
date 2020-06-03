use std::sync::Arc;
use lazy_static::lazy_static;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use ji_cloud_shared::frontend::routes::Route;
use wasm_bindgen::UnwrapThrowExt;
use dominator::{Dom, class, html, clone, events};
use web_sys::Url;
use crate::router::route_signal;
use crate::settings::SETTINGS;
use crate::header::Header;
use crate::pages::signin::SigninDom;
use crate::pages::register::RegisterDom;
use crate::pages::profile::ProfileDom;

pub struct Page {
}

impl Page {
    pub fn new() -> Self {
        Self { }
    }
    
    pub fn render(&self) -> Dom {

        html!("main", {
            .children_signal_vec(
                route_signal() 
                    .map(|route| {
                        vec![
                            Header::render(),
                            {
                                match route {
                                    Route::Signin => {
                                        SigninDom::render(SigninDom::new())
                                    },
                                    Route::Register => {
                                        RegisterDom::render(RegisterDom::new())
                                    },
                                    Route::Profile => {
                                        ProfileDom::render(ProfileDom::new()) 
                                    },
                                    _ => {
                                        NotFound::render()
                                    }
                                }
                            }
                        ]
                    })
                    .to_signal_vec()
            )
        })
    }
}

struct NotFound {
}

impl NotFound {
    pub fn render() -> Dom {
        html!("div", { .text("Not found!") } )
    }
}
