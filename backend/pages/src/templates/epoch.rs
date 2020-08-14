use actix_web::web::Data;
use core::settings::RuntimeSettings;

pub async fn epoch_page(settings: Data<RuntimeSettings>) -> String {
    format!("{}", settings.epoch.as_millis())
}
