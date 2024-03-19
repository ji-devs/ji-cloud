use std::rc::Rc;

use qrcode_generator::QrCodeEcc;
use utils::{routes::Route, unwrap::UnwrapJiExt};
use wasm_bindgen::JsValue;

use super::QrDialogCallbacks;

pub struct QrDialog {
    pub url: String,
    pub file_label: String,
    pub callbacks: QrDialogCallbacks,
}

impl QrDialog {
    pub fn new(route: Route, file_label: String, callbacks: QrDialogCallbacks) -> Rc<Self> {
        let url = qr_core_file_from_route(route);
        Rc::new(Self {
            url,
            file_label,
            callbacks,
        })
    }
}

fn file_to_object_url(filetype: &str, data: &str) -> String {
    let data = JsValue::from_str(data);
    let blob = web_sys::Blob::new_with_str_sequence_and_options(
        &js_sys::Array::from_iter(vec![data]),
        web_sys::BlobPropertyBag::new().type_(filetype),
    )
    .unwrap_ji();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap_ji();
    url
}

pub fn qr_core_file_from_route(route: Route) -> String {
    let result: String =
        qrcode_generator::to_svg_to_string(route.to_string(), QrCodeEcc::High, 200, None::<&str>)
            .unwrap();

    let url = file_to_object_url("image/svg+xml", &result);

    url
}
