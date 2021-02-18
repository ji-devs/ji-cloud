use wasm_bindgen::prelude::*;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;


pub fn get_random_string(length: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}


#[wasm_bindgen(inline_js = "
export function add_styles(contents) {
    let head = document.head;
    var link = document.createElement('style');
    link.textContent = contents;
    head.appendChild(link);
}
")]
extern "C" {
    pub fn add_styles(s: &str);
}

