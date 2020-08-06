use actix_web::web::Data;
use core::settings::Settings;

pub async fn epoch_page(settings: Data<Settings>) -> String {
    format!("{}", settings.epoch.as_millis())
}
