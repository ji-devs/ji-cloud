use std::sync::Arc;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};

use wasm_bindgen::UnwrapThrowExt;
use dominator::{Dom, class, html, clone, events, link};
use web_sys::Url;

pub struct Header {
}

impl Header {
    pub fn render() -> Dom {
        html!("div", {
        })
    }
    pub fn render_right_auth() -> Dom {
        html!("div", { .class("center") 
            .text("TODO")
        })
    }
    pub fn render_right_guest() -> Dom {
        html!("div", { .class("center") 
            .text("TODO")
        })
    }
}

/*
return html`
            <header>
                <div class="left">
                    <img class="logo" src=${Path.ui("ji-logo.png")} @click=${on_home.bind(this)} />
                </div>
                <div class="center">
                    <div class="welcome">
                        Welcome to Ji Cloud!
                    </div>
                </div>
                ${!this.has_credentials
                    ? html`
                        <div class="right">
                            <div class="link" @click=${on_signin.bind(this)}>Sign in</div>
                            <div class="slash">/</div>
                            <div class="link" @click=${on_register.bind(this)}>Register</div>
                        </div>`
                    : html`
                        <div class="right">
                            <div class="link" @click=${on_profile.bind(this)}>Profile</div>
                            <div class="slash">/</div>
                            <div class="link" @click=${on_signout.bind(this)}>Sign out</div>
                        </div>`
                }
            </header>
        `;
        */
