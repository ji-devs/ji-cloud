pub mod direct;
pub mod spa;
pub mod epoch;

use handlebars::Handlebars;
use std::sync::Arc;

pub fn register_templates() -> Arc<Handlebars<'static>> {
    let mut hb = Handlebars::new();

    hb.register_template_file("spa", "./handlebars/spa.hbs").expect("unable to parse spa template");
    hb.register_template_file("home", "./handlepars/home.hbs").expect("unable to parse home template");
    hb.register_template_file("not-found", "./handlebars/404.hbs").expect("unable to parse 404 template");
    hb.register_template_file("no-auth", "./handlebars/no-auth.hbs").expect("unable to parse no-auth template");
    Arc::new(hb)
}
