use std::rc::Rc;

use utils::js_wrappers::download_url;

use super::QrDialog;

impl QrDialog {
    pub fn download(self: &Rc<Self>) {
        download_url(&self.file_label, &self.url)
    }
}
