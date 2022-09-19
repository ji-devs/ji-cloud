use js_sys::RegExp;
use std::rc::Rc;
use web_sys::File;

use super::FileInput;

impl FileInput {
    pub fn on_file_change(self: &Rc<Self>, file: File) {
        self.validate_error_size(&file);
        self.validate_error_mime_type(&file);

        if !self.error_size.get() && !self.error_mime_type.get() {
            self.value.set(Some(file.clone()));
            (self.on_change)(Some(file));
        } else {
            // not sure we should delete file in here
            self.delete_file();
        }
    }

    pub fn delete_file(&self) {
        self.value.set(None);
        (self.on_change)(None);
    }

    fn validate_error_mime_type(&self, file: &File) {
        if !validate_accepts(&self.accept, &file.type_()) {
            self.error_mime_type.set(true);
        } else {
            self.error_mime_type.set(false);
        }
    }

    fn validate_error_size(&self, file: &File) {
        let valid = file.size() as u64 <= self.max_size as u64;
        if valid {
            self.error_size.set(false);
        } else {
            self.error_size.set(true);
        }
    }
}

fn validate_accepts(accepts: &str, mime: &str) -> bool {
    accepts
        .split(",")
        .map(|accept| validate_accept(accept, mime))
        .any(|accepted| accepted)
}

fn validate_accept(accept: &str, mime: &str) -> bool {
    let accept = accept.trim();
    let regex = accept.replace("*", ".*");

    // log::info!("{regex:?}");

    let regex = RegExp::new(&regex, "");

    // web_sys::console::log_1(&regex);

    regex.test(mime)
}
