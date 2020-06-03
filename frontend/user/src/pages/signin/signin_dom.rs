use std::rc::Rc;
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use dominator::{Dom, svg, class, text, html, clone, events, link};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use ji_cloud_shared::{
    user::UserRole,
    auth::SigninSuccess,
    frontend::routes::Route,
};
use crate::path;

use super::signin::{
    signin_google,
    on_signin_success
};

#[derive(Clone, Copy)]
enum State {
    Input,
    Validating,
    Complete,
    Error(ErrorType),
}
#[derive(Clone, Copy)]
enum ErrorType {
    NoUser
}

pub struct SigninDom {
    state: Mutable<State>
}

impl ErrorType {
    fn element(&self) -> Dom {
        html!("div", {
            .class(["mt-2","text-center","text-sm","leading-5","text-red-600","max-w"])
            .text("couldn't login!")
        })
    }
}

impl SigninDom {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            state: Mutable::new(State::Input)
        })
    }
    pub fn render(comp: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(comp.state.signal().map(move |state| {
                Some(match state {
                    State::Input => Self::render_form(comp.clone(), None),
                    State::Error(err_type) => Self::render_form(comp.clone(), Some(err_type)),
                    State::Validating => Self::render_validating(),
                    State::Complete => Self::render_complete(),
                })
            }))
        })
    }
    fn render_complete() -> Dom {
        text("you are now logged in :)")
    }
    fn render_validating() -> Dom {
        text("validating...")
    }

    fn on_login_happy(comp: Rc<Self>) -> impl FnOnce(SigninSuccess) {
        move |status| {
            on_signin_success(&status.csrf);
        }
    }
    fn on_login_sad(comp: Rc<Self>) -> impl FnOnce() {
        move || {
            comp.state.set(State::Error(ErrorType::NoUser));
        }
    }

    fn render_form(comp: Rc<Self>, err:Option<ErrorType>) -> Dom {
        html!("div", {
            .class(["min-h-screen","bg-gray-50","flex","flex-col","justify-center","py-12","sm:px-6","lg:px-8"])
            .children(&mut [
                html!("div", {
                    .class(["sm:mx-auto","sm:w-full","sm:max-w-md"])
                    .children(&mut [
                        html!("img", {
                            .class(["mx-auto","h-12","w-auto"])
                            .attribute("src", &path::ui("ji-logo.png"))
                            .attribute("alt", "Workflow")
                        }),
                        html!("h2", {
                            .class(["mt-6","text-center","text-3xl","leading-9","font-extrabold","text-gray-900"])
                            .children(&mut [
                                text("Sign in to your account"),
                            ])
                        }),
                        html!("p", {
                            .class(["mt-2","text-center","text-sm","leading-5","text-gray-600","max-w"])
                            .children(&mut [
                                text("Or "),
                                html!("a", {
                                    .attribute("href", Route::Register.into()) 
                                    .class(["font-medium","text-indigo-600","hover:text-indigo-500","focus:outline-none","focus:underline","transition","ease-in-out","duration-150"])
                                    .children(&mut [
                                        text("register here"),
                                    ])
                                }),
                            ])
                        }),
                        err.map(|err| {
                            html!("p", {
                                .children(&mut [ err.element() ])
                            })
                        }).unwrap_or(Dom::empty())
                    ])
                }),
                html!("div", {
                    .class(["mt-8","sm:mx-auto","sm:w-full","sm:max-w-md"])
                    .children(&mut [
                        html!("div", {
                            .class(["bg-white","py-8","px-4","shadow","sm:rounded-lg","sm:px-10"])
                            .children(&mut [
                                html!("form", {
                                    .attribute("action", "#")
                                    .attribute("method", "POST")
                                    .children(&mut [
                                        html!("div", {
                                            .children(&mut [
                                                html!("label", {
                                                    .attribute("for", "email")
                                                    .class(["block","text-sm","font-medium","leading-5","text-gray-700"])
                                                    .children(&mut [
                                                        text("Email address"),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(["mt-1","rounded-md","shadow-sm"])
                                                    .children(&mut [
                                                        html!("input", {
                                                            .attribute("id", "email")
                                                            .attribute("type", "email")
                                                            .attribute("required", "")
                                                            .class(["appearance-none","block","w-full","px-3","py-2","border","border-gray-300","rounded-md","placeholder-gray-400","focus:outline-none","focus:shadow-outline-blue","focus:border-blue-300","transition","duration-150","ease-in-out","sm:text-sm","sm:leading-5"])
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }),
                                        html!("div", {
                                            .class("mt-6")
                                            .children(&mut [
                                                html!("label", {
                                                    .attribute("for", "password")
                                                    .class(["block","text-sm","font-medium","leading-5","text-gray-700"])
                                                    .children(&mut [
                                                        text("Password"),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(["mt-1","rounded-md","shadow-sm"])
                                                    .children(&mut [
                                                        html!("input", {
                                                            .attribute("id", "password")
                                                            .attribute("type", "password")
                                                            .attribute("required", "")
                                                            .class(["appearance-none","block","w-full","px-3","py-2","border","border-gray-300","rounded-md","placeholder-gray-400","focus:outline-none","focus:shadow-outline-blue","focus:border-blue-300","transition","duration-150","ease-in-out","sm:text-sm","sm:leading-5"])
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }),
                                        html!("div", {
                                            .class(["mt-6","flex","items-center","justify-between"])
                                            .children(&mut [
                                                html!("div", {
                                                    .class(["flex","items-center"])
                                                    .children(&mut [
                                                        html!("input", {
                                                            .attribute("id", "remember_me")
                                                            .attribute("type", "checkbox")
                                                            .class(["form-checkbox","h-4","w-4","text-indigo-600","transition","duration-150","ease-in-out"])
                                                        }),
                                                        html!("label", {
                                                            .attribute("for", "remember_me")
                                                            .class(["ml-2","block","text-sm","leading-5","text-gray-900"])
                                                            .children(&mut [
                                                                text("Remember me"),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(["text-sm","leading-5"])
                                                    .children(&mut [
                                                        html!("a", {
                                                            .attribute("href", "#")
                                                            .class(["font-medium","text-indigo-600","hover:text-indigo-500","focus:outline-none","focus:underline","transition","ease-in-out","duration-150"])
                                                            .children(&mut [
                                                                text("Forgot your password?"),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }),
                                        html!("div", {
                                            .class("mt-6")
                                            .children(&mut [
                                                html!("span", {
                                                    .class(["block","w-full","rounded-md","shadow-sm"])
                                                    .children(&mut [
                                                        html!("button", {
                                                            .attribute("type", "submit")
                                                            .class(["w-full","flex","justify-center","py-2","px-4","border","border-transparent","text-sm","font-medium","rounded-md","text-white","bg-indigo-600","hover:bg-indigo-500","focus:outline-none","focus:border-indigo-700","focus:shadow-outline-indigo","active:bg-indigo-700","transition","duration-150","ease-in-out"])
                                                            .children(&mut [
                                                                text("Sign in"),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("mt-6")
                                    .children(&mut [
                                        html!("div", {
                                            .class("relative")
                                            .children(&mut [
                                                html!("div", {
                                                    .class(["absolute","inset-0","flex","items-center"])
                                                    .children(&mut [
                                                        html!("div", {
                                                            .class(["w-full","border-t","border-gray-300"])
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(["relative","flex","justify-center","text-sm","leading-5"])
                                                    .children(&mut [
                                                        html!("span", {
                                                            .class(["px-2","bg-white","text-gray-500"])
                                                            .children(&mut [
                                                                text("Or continue with"),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }),
                                        html!("div", {
                                            .class(["mt-6","grid","grid-cols-3","gap-3"])
                                            .children(&mut [
                                                html!("div", {
                                                    .children(&mut [
                                                        html!("span", {
                                                            .class(["w-full","inline-flex","rounded-md","shadow-sm"])
                                                            .children(&mut [
                                                                html!("button", {
                                                                    .event(clone!(comp => move |evt:events::Click| {
                                                                        signin_google(Self::on_login_happy(comp.clone()), Self::on_login_sad(comp.clone()));
                                                                    }))
                                                                    .attribute("type", "button")
                                                                    .class(["w-full","inline-flex","justify-center","py-2","px-4","border","border-gray-300","rounded-md","bg-white","text-sm","leading-5","font-medium","text-gray-500","hover:text-gray-400","focus:outline-none","focus:border-blue-300","focus:shadow-outline-blue","transition","duration-150","ease-in-out"])
                                                                    .attribute("aria-label", "Sign in with Facebook")
                                                                    .children(&mut [
                                                                        html!("img", {
                                                                            .class("h-5")
                                                                            .attribute("src", &path::ui("social/google-g.svg"))
                                                                        }),
                                                                    ])
                                                                }),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .children(&mut [
                                                        html!("span", {
                                                            .class(["w-full","inline-flex","rounded-md","shadow-sm"])
                                                            .children(&mut [
                                                                html!("button", {
                                                                    .attribute("type", "button")
                                                                    .class(["w-full","inline-flex","justify-center","py-2","px-4","border","border-gray-300","rounded-md","bg-white","text-sm","leading-5","font-medium","text-gray-500","hover:text-gray-400","focus:outline-none","focus:border-blue-300","focus:shadow-outline-blue","transition","duration-150","ease-in-out"])
                                                                    .attribute("aria-label", "Sign in with Twitter")
                                                                    .children(&mut [
                                                                        svg!("svg", {
                                                                            .class(["h-5","h-5"])
                                                                            .attribute("fill", "currentColor")
                                                                            .attribute("viewBox", "0 0 20 20")
                                                                            .children(&mut [
                                                                                svg!("path", {
                                                                                    .attribute("d", "M6.29 18.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0020 3.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.073 4.073 0 01.8 7.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 010 16.407a11.616 11.616 0 006.29 1.84")
                                                                                }),
                                                                            ])
                                                                        }),
                                                                    ])
                                                                }),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .children(&mut [
                                                        html!("span", {
                                                            .class(["w-full","inline-flex","rounded-md","shadow-sm"])
                                                            .children(&mut [
                                                                html!("button", {
                                                                    .attribute("type", "button")
                                                                    .class(["w-full","inline-flex","justify-center","py-2","px-4","border","border-gray-300","rounded-md","bg-white","text-sm","leading-5","font-medium","text-gray-500","hover:text-gray-400","focus:outline-none","focus:border-blue-300","focus:shadow-outline-blue","transition","duration-150","ease-in-out"])
                                                                    .attribute("aria-label", "Sign in with GitHub")
                                                                    .children(&mut [
                                                                        svg!("svg", {
                                                                            .class(["h-5","h-5"])
                                                                            .attribute("fill", "currentColor")
                                                                            .attribute("viewBox", "0 0 20 20")
                                                                            .children(&mut [
                                                                                svg!("path", {
                                                                                    .attribute("fill-rule", "evenodd")
                                                                                    .attribute("d", "M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z")
                                                                                    .attribute("clip-rule", "evenodd")
                                                                                }),
                                                                            ])
                                                                        }),
                                                                    ])
                                                                }),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }),
                                    ])
                                }),
                            ])
                        }),
                    ])
                }),
            ])
            })
    }
}
