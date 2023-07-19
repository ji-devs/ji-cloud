use std::error::Error;
use std::sync::Arc;
use std::{borrow::Cow, sync::Mutex};

use dominator::html;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use gloo::timers::callback::Timeout;
use gloo::utils::document;
use lazy_static::lazy_static;

use crate::icon;
use crate::unwrap::UnwrapJiExt;

lazy_static! {
    static ref INITIALIZED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

thread_local! {
    static TOASTS: MutableVec<(i64, Toast)> = Default::default();
}

pub fn success(s: &'static str) {
    Toast {
        msg: Cow::Borrowed(s),
        kind: ToastKind::Success,
    }
    .enqueue();
}
pub fn success_string(s: String) {
    Toast {
        msg: Cow::Owned(s),
        kind: ToastKind::Success,
    }
    .enqueue();
}
pub fn error(s: &'static str) {
    Toast {
        msg: Cow::Borrowed(s),
        kind: ToastKind::Error,
    }
    .enqueue();
}
pub fn error_string(s: String) {
    Toast {
        msg: Cow::Owned(s),
        kind: ToastKind::Error,
    }
    .enqueue();
}
pub fn notice(s: &'static str) {
    Toast {
        msg: Cow::Borrowed(s),
        kind: ToastKind::Notice,
    }
    .enqueue();
}
pub fn notice_string(s: String) {
    Toast {
        msg: Cow::Owned(s),
        kind: ToastKind::Notice,
    }
    .enqueue();
}

#[derive(Clone, Copy, Debug)]
enum ToastKind {
    Success,
    Error,
    Notice,
}

#[derive(Clone, Debug)]
struct Toast {
    msg: Cow<'static, str>,
    kind: ToastKind,
}
impl Toast {
    fn enqueue(self) {
        if !*INITIALIZED.lock().unwrap_ji() {
            init();
        }
        let id = js_sys::Math::random() as i64;
        TOASTS.with(|toasts| {
            toasts.lock_mut().push_cloned((id, self));
        });
        Timeout::new(6_000, move || {
            TOASTS.with(|toasts| {
                toasts.lock_mut().retain(|el| el.0 != id);
            });
        })
        .forget();
    }

    fn icon(&self) -> &'static str {
        match self.kind {
            ToastKind::Success => "fa-solid fa-check",
            ToastKind::Error => "fa-solid fa-xmark",
            ToastKind::Notice => "fa-solid fa-exclamation",
        }
    }
    fn icon_background_color(&self) -> &'static str {
        match self.kind {
            ToastKind::Success => "var(--green-3)",
            ToastKind::Error => "var(--light-red-1)",
            ToastKind::Notice => "var(--light-blue-3)",
        }
    }
    fn icon_circle_color(&self) -> &'static str {
        match self.kind {
            ToastKind::Success => "var(--dark-green-1)",
            ToastKind::Error => "var(--red-alert)",
            ToastKind::Notice => "var(--main-blue)",
        }
    }
}
pub fn init() {
    *INITIALIZED.lock().unwrap_ji() = true;
    TOASTS.with(|toasts| {
        dominator::append_dom(
            &document().body().unwrap_ji(),
            html!("div", {
                .style("position", "fixed")
                .style("bottom", "16px")
                .style("right", "10px")
                .style("display", "grid")
                .style("justify-items", "end")
                .style("gap", "16px")
                .children_signal_vec(toasts.signal_vec_cloned().map(|(_, toast)| {
                    html!("div", {
                        .style("padding-inline", "16px")
                        .style("height", "64px")
                        .style("display", "grid")
                        .style("grid-template-columns", "32px 1fr")
                        .style("gap", "8px")
                        .style("border-radius", "16px")
                        .style("place-content", "center")
                        .style("align-items", "center")
                        .style("box-shadow", "2px 2px 6px 0 rgba(0, 0, 0, 0.16)")
                        .style("background-color", toast.icon_background_color())
                        .style("font-size", "16px")
                        .style("color", "var(--dark-gray-6)")
                        .child(icon!(toast.icon(), {
                            .style("height", "32px")
                            .style("display", "grid")
                            .style("place-content", "center")
                            .style("border-radius", "50%")
                            .style("background-color", toast.icon_circle_color())
                            .style("color", toast.icon_background_color())
                        }))
                        .text(&toast.msg)
                    })
                }))
            }),
        )
    });
}
