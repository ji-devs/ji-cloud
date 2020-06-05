use handlebars::Handlebars;
use std::sync::Arc;

pub fn register_templates() -> Arc<Handlebars<'static>> {
    let mut hb = Handlebars::new();

    hb.register_template_file("spa", "./templates/spa.hbs").expect("unable to parse spa template");
    hb.register_template_file("home", "./templates/home.hbs").expect("unable to parse home template");
    hb.register_template_file("not-found", "./templates/404.hbs").expect("unable to parse 404 template");
    hb.register_template_file("no-auth", "./templates/no-auth.hbs").expect("unable to parse no-auth template");
    Arc::new(hb)
}
