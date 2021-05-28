use std::rc::Rc;
use utils::prelude::*;
use web_sys::File;
use shared::{
    media::MediaLibrary,
    api::{ApiEndpoint, endpoints}, 
    domain::{CreateResponse, audio::AudioId},
    error::EmptyError,
};
use super::state::{AudioInputMode, State};

impl State {
    //Internal only, prevents callback cycles
    pub(super) fn set_audio_id(&self, audio_id: Option<AudioId>) {
        //Change the mutable for affecting all DOM rendering stuff
        //with _eventual consistency_
        self.mode.set_neq(match audio_id {
            Some(audio_id) => AudioInputMode::Stopped(audio_id),
            None => AudioInputMode::Empty,
        });

        //Call the callback for precise unskipped updates
        if let Some(on_change) = &self.on_change {
            (on_change) (audio_id.map(|id| (id, MediaLibrary::User)));
        }
    }


    //Intended for externally forcing the state
    //e.g. for undo/redo compatability
    pub fn set_audio_id_ext(&self, audio_id: Option<AudioId>) {
        self.mode.set_neq(match audio_id {
            Some(audio_id) => AudioInputMode::Stopped(audio_id),
            None => AudioInputMode::Empty,
        });
    }
}

pub async fn file_change(state: Rc<State>, file: File) {
    state.mode.set(AudioInputMode::Uploading);
    let res = upload_file(file).await;
    if let Ok(audio_id) = res {
        state.set_audio_id(Some(audio_id));
    } else {
        log::error!("Error uploading audio file");
        state.mode.set(AudioInputMode::Empty);
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
