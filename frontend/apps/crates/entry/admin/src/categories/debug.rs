#![allow(dead_code)]
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(debug_assertions)] {
        pub const INIT_EXPANDED:bool = false;
        pub const INIT_EDITING:bool = false;
    } else {
        pub const INIT_EXPANDED:bool = false;
        pub const INIT_EDITING:bool = false;
    }
}
