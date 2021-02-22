use std::collections::HashMap;
use serde::Serialize;
use wasm_bindgen::JsValue;
use std::collections::BTreeMap;

fn b_tree_to_vec<'a, K, V>(tree: &'a BTreeMap<K, V>) -> Vec<(&K, &V)> {
    tree.iter().collect::<Vec<(&K, &V)>>()
}

pub fn b_tree_to_js<'a, K, V>(tree: &'a BTreeMap<K, V>) -> JsValue
where K: Serialize,
    V: Serialize {
    let vec = b_tree_to_vec(&tree);
    serde_wasm_bindgen::to_value(&vec).unwrap()
}



fn map_to_vec<'a, K, V>(tree: &'a HashMap<K, V>) -> Vec<(&K, &V)> {
    tree.iter().collect::<Vec<(&K, &V)>>()
}

pub fn map_to_js<'a, K, V>(map: &'a HashMap<K, V>) -> JsValue
where K: Serialize,
    V: Serialize {
    let vec = map_to_vec(&map);
    serde_wasm_bindgen::to_value(&vec).unwrap()
}
