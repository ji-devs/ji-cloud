pub mod direct;
pub mod spa;
pub mod epoch;
pub mod info;

use handlebars::Handlebars;
use std::sync::Arc;
use crate::settings::HANDLEBARS_PATH;

pub fn register_templates() -> Arc<Handlebars<'static>> {
    let mut hb = Handlebars::new();

    hb.register_template_file("info", &get_path("info.hbs")).expect("unable to parse info template");
    hb.register_template_file("spa", &get_path("spa.hbs")).expect("unable to parse spa template");
    hb.register_template_file("home", &get_path("home.hbs")).expect("unable to parse home template");
    hb.register_template_file("not-found", &get_path("404.hbs")).expect("unable to parse 404 template");
    hb.register_template_file("no-auth", &get_path("no-auth.hbs")).expect("unable to parse no-auth template");
    Arc::new(hb)
}

fn get_path(path:&str) -> String {
    format!("{}/{}", HANDLEBARS_PATH, path)
}
