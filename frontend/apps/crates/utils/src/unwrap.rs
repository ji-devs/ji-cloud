use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

/*
 * debug switching between unwrap_throw() and unwrap()
 * just use unwrap_ji() and expect_ji() everywhere
 * potentially can add more instrumentaton for debugging here
 * TODO - get right line numbers!
 */

pub trait UnwrapJiExt<T>: Sized {
    #[track_caller]
    fn unwrap_ji(self) -> T {
        self.expect_ji("`unwrap_ji` failed")
    }

    #[track_caller]
    fn expect_ji(self, message: &str) -> T;
}
cfg_if! {
    if #[cfg(feature = "debug_log")] {
        impl<T> UnwrapJiExt<T> for Option<T> {
            fn expect_ji(self, message: &str) -> T {
                self.expect(message)
            }
        }

        impl<T, E> UnwrapJiExt<T> for Result<T, E>
        where
            E: core::fmt::Debug,
        {
            fn expect_ji(self, message: &str) -> T {
                self.expect(message)
            }
        }
    } else {

        impl<T> UnwrapJiExt<T> for Option<T> {
            fn expect_ji(self, message: &str) -> T {
                self.expect_throw(message)
            }
        }

        impl<T, E> UnwrapJiExt<T> for Result<T, E>
        where
            E: core::fmt::Debug,
        {
            fn expect_ji(self, message: &str) -> T {
                self.expect_throw(message)
            }
        }
    }
}
