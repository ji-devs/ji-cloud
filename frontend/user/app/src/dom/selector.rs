use shipyard::EntityId;
use wasm_bindgen::JsCast;
use awsm_web::dom::{select, get_element_by_id};
use web_sys::Document;

pub fn entity_id(id:shipyard::EntityId) -> String {
    //gotta start with a letter and avoid special characters
    format!("e{}-{}", id.index(), id.gen())
}

pub fn todo<T: JsCast>(doc:&Document, id:EntityId) -> T {
    select(doc, &format!("#{}", entity_id(id)))
}

pub fn todo_toggle<T: JsCast>(doc:&Document, id:EntityId) -> T {
    select(doc, &format!("#{} .toggle", entity_id(id)))
}
pub fn todo_edit<T: JsCast>(doc:&Document, id:EntityId) -> T {
    select(doc, &format!("#{} .edit", entity_id(id)))
}
pub fn todo_label<T: JsCast>(doc:&Document, id:EntityId) -> T {
    select(doc, &format!("#{} .label", entity_id(id)))
}

pub fn todo_delete<T: JsCast>(doc:&Document, id:EntityId) -> T {
    select(doc, &format!("#{} .destroy", entity_id(id)))
}

pub fn clear_completed<T: JsCast>(doc:&Document) -> T {
    select(doc, ".clear-completed") 
}

pub fn toggle_all<T: JsCast>(doc:&Document) -> T {
    get_element_by_id(&doc, "toggle-all")
}

pub fn count_num<T: JsCast>(doc:&Document) -> T {
    get_element_by_id(&doc, "todo-count-num")
}

pub fn count_label<T: JsCast>(doc:&Document) -> T {
    get_element_by_id(&doc, "todo-count-label")
}


pub fn bottom_filter<T: JsCast>(doc:&Document, name:&str) -> T {
    get_element_by_id(&doc, &format!("filters-{}", name))
}
