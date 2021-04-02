use std::rc::Rc;
use utils::fetch::{api_upload_file, api_with_auth};
use web_sys::File;
use shared::{api::{ApiEndpoint, endpoints}, domain::{CreateResponse, audio::AudioId}, error::EmptyError};
use super::state::{AudioInputMode, State};


pub async fn file_change(state: Rc<State>, file: File) {
    state.mode.set(AudioInputMode::Uploading);
    let res = upload_file(file).await;
    if let Ok(audio_id) = res {
        state.options.value.set(Some(audio_id));
        state.mode.set(AudioInputMode::Success);
    } else {
        log::error!("Error uploading audio file");
        state.mode.set(AudioInputMode::Record);
    }
}

async fn upload_file(file: File) -> Result<AudioId, ()> {
    match api_with_auth::<CreateResponse<AudioId>, EmptyError, ()>(
        &endpoints::audio::user::Create::PATH,
        endpoints::audio::user::Create::METHOD,
        None
    ).await {
        Ok(resp) => {
            let CreateResponse { id } = resp;

            let path = endpoints::audio::user::Upload::PATH.replace("{id}", &id.0.to_string());
            match api_upload_file(&path, &file, endpoints::audio::user::Upload::METHOD).await {
                Ok(_) => {
                    Ok(resp.id)
                },
                Err(_) => Err(()),
            }
        },
        Err(_) => Err(()),
    }
}
