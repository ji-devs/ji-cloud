use std::rc::Rc;
use utils::{path::audio_lib_url, prelude::*};
use web_sys::File;
use shared::{
    api::{ApiEndpoint, endpoints}, 
    domain::{CreateResponse, audio::AudioId}, 
    media::MediaLibrary,
    error::EmptyError
};
use super::state::{AudioInputMode, State};

impl State {
    //Internal only, prevents callback cycles
    pub(super) fn set_audio_id(&self, audio_id: Option<AudioId>) {
        //Change the mutable for affecting all DOM rendering stuff
        //with _eventual consistency_
        self.audio_id.set_neq(audio_id);

        //Call the callback for precise unskipped updates
        if let Some(on_change) = &self.on_change {
            (on_change)(audio_id);
        }
    }


    //Intended for externally forcing the state
    //e.g. for undo/redo compatability
    pub fn set_audio_id_ext(&self, audio_id: Option<AudioId>) {
        //TODO - decide if we should imperatively force the player state here
        //or rather handle it via match patterns and guards only
        self.audio_id.set_neq(audio_id);
    }
}

pub async fn file_change(state: Rc<State>, file: File) {
    state.mode.set(AudioInputMode::Uploading);
    let res = upload_file(file).await;
    if let Ok(audio_id) = res {
        state.set_audio_id(Some(audio_id));
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
