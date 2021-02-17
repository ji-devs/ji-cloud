use dominator::{Dom, DomBuilder, html, clone, with_node};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use dominator_helpers::futures::AsyncLoader;
use super::actions;
use shared::domain::session::*;

//The popup dom
pub struct RegisterOauthPage {
}

impl RegisterOauthPage {
    pub fn render(data: OauthData) -> Dom {
        let loader = AsyncLoader::new();

        loader.load(async move {
            let req = match data {
                OauthData::Google(code) => {
                    CreateSessionOAuthRequest::Google {
                        code,
                        redirect_kind: OAuthUrlKind::Register
                    }
                }
            };
            actions::finalize(req).await;
        });

        Dom::with_state(loader, |loader| {
            html!("div", {
                .text_signal(loader.is_loading().map(|loading| {
                    if loading {
                        "loading"
                    } else {
                        "done!"
                    }
                }))
            })
        })
    }
}

