use once_cell::sync::Lazy;
use fluent_bundle::{FluentResource, FluentArgs, FluentValue, concurrent::FluentBundle};
use unic_langid::LanguageIdentifier;
use wasm_bindgen::prelude::*;

static FLUENT_BUNDLE: Lazy<FluentBundle<FluentResource>> = Lazy::new(|| {
    let res = FluentResource::try_new(include_str!("../../../../../../localization/fluent/entry/user/english.ftl").to_string())
        .expect_throw("failed to parse English FTL");

    let lang_id: LanguageIdentifier = "en-US".parse().expect_throw("language id failed");

    let mut bundle = FluentBundle::new(vec![lang_id]);

    bundle.add_resource(res)
        .expect_throw("failed to add English bundle");

    bundle
});

pub fn test_message(id: &str, name:&str) -> String {
    let msg = FLUENT_BUNDLE.get_message(id).unwrap_throw();
    let pattern = msg.value.unwrap_throw();

    let mut args = FluentArgs::new();
    args.add("name", FluentValue::from(name));
    let mut errors = vec![];
    FLUENT_BUNDLE.format_pattern(pattern, Some(&args), &mut errors).to_string()
}
