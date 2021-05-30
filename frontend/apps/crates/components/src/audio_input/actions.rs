use std::rc::Rc;
use utils::prelude::*;
use web_sys::File;
use shared::{
    media::MediaLibrary,
    api::{ApiEndpoint, endpoints}, 
    error::EmptyError,
    domain::{
        CreateResponse,
        jig::module::body::Audio,
        audio::AudioId,
    }
    
};
use super::state::{AudioInputMode, State};

impl State {
    //Internal only, prevents callback cycles
    pub(super) fn set_audio(&self, audio: Option<Audio>) {
        //Change the mutable for affecting all DOM rendering stuff
        //with _eventual consistency_
        self.mode.set_neq(match audio.clone() {
            Some(audio) => AudioInputMode::Stopped(audio),
            None => AudioInputMode::Empty,
        });

        //Callbacks for immediate, unskipped updates
        match audio {
            Some(audio) => {
                if let Some(on_add) = &self.callbacks.on_add {
                    (on_add) (audio);
                }
            },
            None => {
                if let Some(on_delete) = &self.callbacks.on_delete {
                    (on_delete) ();
                }
            }
        }
    }


    //Intended for externally forcing the state
    //e.g. for undo/redo compatability
    pub fn set_audio_ext(&self, audio: Option<Audio>) {
        self.mode.set_neq(match audio {
            Some(audio) => AudioInputMode::Stopped(audio),
            None => AudioInputMode::Empty,
        });
    }
}

pub async fn file_change(state: Rc<State>, file: File) {
    state.mode.set(AudioInputMode::Uploading);
    let res = upload_file(file).await;
    if let Ok(audio_id) = res {
        state.set_audio(Some(Audio { id: audio_id, lib: MediaLibrary::User}));
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
